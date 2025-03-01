use rocket::{http::Header, response::content};

use crate::{extractor, ServerState};

#[get("/report?<url>")]
pub async fn report(url: String, state: &ServerState) -> Downloadable {
    let mut state = state.get().await;
    let html = state.get_html(&url).await;

    let matches = extractor::extract(html).await;
    let report = extractor::generate_markdown_report(&matches);

    let content = rocket::response::content::RawText(report);
    let more = Header::new("Content-Disposition", "attachment; filename=report.md");
    Downloadable {
        inner: content,
        more,
    }
}

#[get("/ai_report?<url>")]
pub async fn ai_report(url: String, state: &ServerState) -> Downloadable {
    let mut state = state.get().await;
    let html = state.get_html(&url).await;

    let matches = extractor::extract(html).await;
    let report = extractor::generate_ai_markdown_report(&matches);

    let content = rocket::response::content::RawText(report);
    let more = Header::new("Content-Disposition", "attachment; filename=report.md");
    Downloadable {
        inner: content,
        more,
    }
}

#[derive(Responder)]
#[response(status = 200)]
pub struct Downloadable {
    inner: content::RawText<String>,
    more: Header<'static>,
}
