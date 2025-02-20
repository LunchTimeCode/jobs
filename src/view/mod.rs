use maud::{html, Markup};
use rocket::{form::Form, response::content, Route};

use crate::extractor;

pub fn page(markup: Markup) -> Markup {
    html! {
       html color-mode="user" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta name="description" content="Jobs";
                ({frontend::resources()})
                ({title("Jobs")})
            }

            body {
                (markup)
        }
       }
    }
}

fn title(title: impl Into<String>) -> Markup {
    html! {
    title { ({title.into()}) }
    }
}

pub mod frontend {
    use maud::{html, Markup, PreEscaped};

    const HTMX: &str = r#"<script src="/_assets/htmx.js"></script>"#;
    const FAVICON: &str = r#"<link rel="icon" href="/_assets/favicon.ico" type="image/x-icon">"#;
    const CSS: &str = r#"<link rel="stylesheet" href="/_assets/mvp.css">"#;

    pub fn resources() -> Markup {
        html! {
        (PreEscaped(HTMX))
        (PreEscaped(FAVICON))
        (PreEscaped(CSS))
           }
    }
}

#[get("/")]
pub fn body() -> content::RawHtml<String> {
    content::RawHtml(page(body_m()).into_string())
}

fn body_m() -> Markup {
    html! {
    body {
         header {
             (navigation())
         }
         main id="main_target"{
             section id="tools"{
                 header{
                        h1{"Job Seeking Tools"}
                        p{"For a dear friend"}
                 }


                 (tool(
                        "data-collection-33.svg",
                        "Extract",
                        "Extract what matters",
                        "extractor",
                        "Extracter Tool"
                 ))
                 (tool(
                        "track.svg",
                        "Tracker",
                        "Track what matters",
                        "tracker",
                        "Tracker Tool"
                 ))
             }

         }

        }
    }
}

pub fn tool(img: &str, title: &str, description: &str, link: &str, link_text: &str) -> Markup {
    let img = format!("/_assets/{}", img);
    let link = format!("/{}", link);
    html! {
        aside {
            img alt="HTML only" src=(img) height="150";
            h3 {
                ({title})
            }
            p {
                ({description})
            }
            p {
                a hx-target="#main_target" hx-trigger="click" hx-get=(link) {
                    em {
                        ({link_text})
                    }
                }
            }
        }
    }
}

pub fn navigation() -> Markup {
    html! {

        nav {
                a href="/" {
                    img alt="lunchtime/jobs" src="./_assets/profile.png" height="70";
                }
                ul {
                    li {
                        a href="/" {
                            "Home"
                        }

                    }
                    li {
                        a href="about" {
                            "About"
                        }
                    }
                    li {
                        a href="test" {
                            "Test"
                        }
                    }

                }
            }

    }
}

#[get("/extractor")]
pub fn extractor_route() -> content::RawHtml<String> {
    content::RawHtml(extractor_view().into_string())
}

pub fn extractor_view() -> Markup {
    html! {
        form hx-post="/extract" hx-target="#extracted" {
            input type="text" name="url" placeholder="URL" required="true";
            button type="submit" {
                "Extract Dates"
            }
        }
        div id="extracted" {

        }
    }
}

#[derive(FromForm)]
pub struct UrlInput {
    // The raw, undecoded value. You _probably_ want `String` instead.
    url: String,
}

#[post("/extract", data = "<url_input>")]
pub async fn extract_route(url_input: Form<UrlInput>) -> content::RawHtml<String> {
    let dates = extractor::extract_dates(&url_input.url).await;
    content::RawHtml(extract(dates).into_string())
}

pub fn extract(dates: Vec<extractor::datefinder::Date>) -> Markup {
    let date_views = dates
        .iter()
        .map(|d| date_view(d.clone()))
        .collect::<Vec<Markup>>();
    html! {
        section{
            h1{"Dates"}
            ul{
                @for date_view in date_views {
                    li{
                        ({date_view})
                    }
                }
            }
        }
    }
}

pub fn date_view(date: extractor::datefinder::Date) -> Markup {
    let line = extractor::datefinder::format_line(date.line(), date.value().to_string());

    html! {
        section{
                p {
                    "Date: "({date.value()})
                }
                p {
                   "Position in html: " ({date.index()})
                }
                section{
                    h2{"Context"}
                    p {
                        (line)
                    }
                }
        }


    }
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/", routes![body, extractor_route, extract_route])
}
