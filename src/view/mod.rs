use maud::{html, Markup};
use rocket::{response::content, Route};

mod extraction;

pub use extraction::api as ex_api;

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

                }
            }

    }
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/", routes![body])
}
