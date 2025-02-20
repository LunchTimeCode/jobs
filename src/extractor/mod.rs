pub mod datefinder;

pub async fn extract_dates(url: &str) -> Vec<datefinder::Date> {
    let res = reqwest::get(url).await.unwrap().text().await.unwrap();

    datefinder::find_dates(&res)
}
