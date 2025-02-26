use maud::{html, Markup};
use rocket::response::content;

pub fn extractor_view() -> Markup {
    html! {
        form hx-post="/extractor/extract" hx-target="#extracted" {
            input type="text" name="url" placeholder="URL" required="true";
            button type="submit" {
                "Extract Dates"
            }
        }
        div id="extracted" {

        }
    }
}

#[get("/")]
pub fn route() -> content::RawHtml<String> {
    content::RawHtml(extractor_view().into_string())
}
