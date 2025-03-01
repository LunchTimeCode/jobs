use sentence_finder::SentenceMatch;
use serde::{Deserialize, Serialize};

pub mod datefinder;
pub mod sentence_finder;

#[derive(Debug, Serialize, Deserialize)]
pub struct Matches {
    dates: Vec<datefinder::Date>,
    sentences: Vec<SentenceMatch>,
}

impl Matches {
    pub fn new(dates: Vec<datefinder::Date>, sentences: Vec<SentenceMatch>) -> Self {
        Matches { dates, sentences }
    }
    pub fn dates(&self) -> &Vec<datefinder::Date> {
        &self.dates
    }
    pub fn sentences(&self) -> &Vec<SentenceMatch> {
        &self.sentences
    }
}

pub async fn extract(html: &str) -> Matches {
    let dates = datefinder::find_dates(html);
    let sentences = sentence_finder::find_sentences(html);

    Matches::new(dates, sentences)
}

pub fn generate_markdown_report(matches: &Matches) -> String {
    let mut report = String::new();

    report.push_str("# Extraction Report\n\n");

    report.push_str("## Dates Found\n\n");
    for date in matches.dates() {
        report.push_str(&format!("- {}\n", date));
    }

    report.push_str("\n## Sentences Found\n\n");
    for sentence in matches.sentences() {
        report.push_str(&format!("- {}\n", sentence.text()));
    }

    report
}

pub fn generate_ai_markdown_report(matches: &Matches) -> String {
    let mut report = String::new();

    report.push_str("# Instruction\n\n");

    report.push_str("Find and show all relevant dates and breakdown the job posting, only use what is inside this report\n\n");
    report.push_str("Break down all of this Extraction, including the sentences and do not leave out one bit of information\n\n");
    report.push_str("Ignore JSON, Javascript, and html tags\n\n");
    report
        .push_str("Make sure to include when this job posting got posted, changed or uploaded\n\n");

    report.push_str("You got this, you are a very good AI agent\n\n");
    report.push_str(
        "Make sure you get this right or people will loose jobs and may even not able to eat\n\n",
    );

    report.push_str("# Extraction\n\n");

    report.push_str("## Dates Found\n\n");
    for date in matches.dates() {
        report.push_str(&format!("- {}\n", date));
    }

    report.push_str("\n## Sentences Found\n\n");
    for sentence in matches.sentences() {
        report.push_str(&format!("- {}\n", sentence.text()));
    }

    report
}
