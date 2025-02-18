pub fn regexes() -> Vec<regex::Regex> {
    vec![
        regex::Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap(),
        regex::Regex::new(r"\d{2}-\d{2}-\d{4}").unwrap(),
        regex::Regex::new(r"\d{2}/\d{2}/\d{4}").unwrap(),
        regex::Regex::new(r"\d{4}/\d{2}/\d{2}").unwrap(),
        regex::Regex::new(r"\d{2}-\d{2}-\d{2}").unwrap(),
        regex::Regex::new(r"\d{2}/\d{2}/\d{2}").unwrap(),
        regex::Regex::new(r"\d{2}-\d{2}-\d{2}").unwrap(),
        regex::Regex::new(r"\d{2}/\d{2}/\d{2}").unwrap(),
        regex::Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap(),
        regex::Regex::new(r"\d{2}-\d{2}-\d{4}").unwrap(),
        regex::Regex::new(r"\d{2}/\d{2}/\d{4}").unwrap(),
        regex::Regex::new(r"\d{4}/\d{2}/\d{2}").unwrap(),
        regex::Regex::new(r"\d{2}-\d{2}-\d{2}").unwrap(),
        regex::Regex::new(r"\d{2}/\d{2}/\d{2}").unwrap(),
        regex::Regex::new(r"\d{2}-\d{2}-\d{2}").unwrap(),
        regex::Regex::new(r"\d{2}/\d{2}/\d{2}").unwrap(),
        regex::Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap(),
        regex::Regex::new(r"\d{2}-\d{2}-\d{4}").unwrap(),
        regex::Regex::new(r"\d{2}/\d{2}/\d{4}").unwrap(),
    ]
}

#[allow(unused)]
pub struct Date {
    value: String,
    index: usize,
}

impl Date {
    fn new(value: String, index: usize) -> Self {
        Self { value, index }
    }
}

pub fn find_dates_in_line(res: Vec<regex::Regex>, line: &str) -> Vec<String> {
    let mut dates = vec![];
    for regex in res {
        for date in regex.find_iter(line) {
            dates.push(date.as_str().to_string());
        }
    }
    dates
}

pub fn find_dates(text: &str) -> Vec<Date> {
    let regexes = regexes();
    let lines = text.lines();
    let mut dates: Vec<Date> = vec![];
    for (i, line) in lines.enumerate() {
        let line_dates = find_dates_in_line(regexes.clone(), line);
        for date in line_dates {
            dates.push(Date::new(date, i));
        }
    }

    dates
}
