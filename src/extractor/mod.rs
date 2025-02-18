mod datefinder;

#[allow(unused)]
pub fn extract_dates(url: &str) -> Vec<datefinder::Date> {
    datefinder::find_dates("")
}
