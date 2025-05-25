use maud::{html, Markup};
use rocket::{form::Form, response::content};

use crate::{
    extractor::{self, Matches},
    ServerState,
};

#[get("/ai?<url>")]
pub async fn for_ai(url: String, state: &ServerState) -> String {
    let mut url = url;
    let without_slash = url.split_off(0);

    let mut state = state.get().await;
    let html = state.get_html(&without_slash).await;

    let matches = extractor::extract(html).await;
    let report = extractor::generate_markdown_report(&matches);

    rocket::response::content::RawText(report).0
}

#[derive(FromForm)]
pub struct UrlInput {
    // The raw, undecoded value. You _probably_ want `String` instead.
    url: String,
}

#[post("/extract", data = "<url_input>")]
pub async fn extract_route(
    url_input: Form<UrlInput>,
    state: &ServerState,
) -> content::RawHtml<String> {
    let mut state = state.get().await;
    let html = state.get_html(&url_input.url).await;

    let matches = extractor::extract(html).await;
    content::RawHtml(extract(matches, &url_input.url).into_string())
}

pub fn extract(matches: Matches, url: &str) -> Markup {
    let dates = matches.dates();
    let sentences = matches.sentences();
    let sentence_views = sentences
        .iter()
        .map(|s| sentence_view(s.clone()))
        .collect::<Vec<Markup>>();

    let date_views = dates
        .iter()
        .map(|d| date_view(d.clone()))
        .collect::<Vec<Markup>>();

    let url_with_query_ai = format!("/extractor/ai_report?url={}", url);

    html! {
        section{
            header{
                h1{"Extraction"}
            }

            a href=(url) {p {"Orignal URL: "(url)}}

            aside {
                img alt="HTML only" src=("/_assets/chatbot.svg") height="150";
                h3 {
                    ("AI Report")
                }
                p {
                    ("Upload this report to any AI Agent of your choise, for example ChatGPT")
                }
                a href=(url_with_query_ai) {button {"Download Report for AI"}}
            }

        }




    section{
          header{
                       h1{"Dates"}
          }
                    @for date_view in date_views {
                        ({date_view})
                    }
    }
        section{

                    @for sentence_view in sentence_views {
                        ({sentence_view})
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

pub fn sentence_view(sentence: extractor::sentence_finder::SentenceMatch) -> Markup {
    html! {
        aside{
                p {
                    ({sentence.text()})
                }
        }
    }
}
