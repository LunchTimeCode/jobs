use rocket::Route;

pub mod control;
pub mod extracted;
pub mod reports;

pub fn api() -> (&'static str, Vec<Route>) {
    (
        "/extractor",
        routes![
            control::route,
            extracted::extract_route,
            reports::report,
            reports::ai_report
        ],
    )
}
