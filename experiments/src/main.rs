mod extractor;

#[tokio::main]
async fn main() {
    
    let res = read_test_file();
    let dates = extractor::lines_with_dates(res);
    dates.iter().for_each(|d|{
     println!("{}", d);
    }
    );
    
   
}


fn read_test_file() -> String {
    let path = "test.html";
    let content = std::fs::read_to_string(path).unwrap();
    content
}

// example request
async fn get() -> String {
   let res = reqwest::get("https://jobs.coopjobs.ch/offene-stellen/plongeur-en-restauration/557c3892-aeb9-49c6-9fe0-fb4076bcf723")
        .await.unwrap()
        .text()
        .await.unwrap();
   res
}