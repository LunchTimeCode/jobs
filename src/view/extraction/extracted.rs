use maud::{html, Markup};
use rocket::{form::Form, response::content};

use crate::extractor;

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
            header{
               h1{"Dates"}
            }

            @for date_view in date_views {
                ({date_view})
            }
        }
    }
}

pub fn date_view(date: extractor::datefinder::Date) -> Markup {
    let line = extractor::datefinder::format_line(date.line(), date.value().to_string());

    html! {
        aside{
            h2{"Date: "({date.value()})}
                p {
                   "Position in html: " ({date.index()})
                }
                p {
                        (line)
                }

        }
    }
}
