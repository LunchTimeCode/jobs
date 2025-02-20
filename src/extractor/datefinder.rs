use std::fmt::Display;

pub fn regexes() -> Vec<regex::Regex> {
    vec![
        regex::Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap(),
        regex::Regex::new(r"\d{4}/\d{2}/\d{2}").unwrap(),
        regex::Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap(),
        regex::Regex::new(r"\d{4}/\d{2}/\d{2}").unwrap(),
        regex::Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap(),
    ]
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Date {
    value: String,
    index: usize,
    line: String,
}

impl Date {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn line(&self) -> &str {
        &self.line
    }

    pub fn key(&self) -> String {
        self.value().to_string()
    }
}

pub fn format_line(line: &str, date: String) -> String {
    let date_pos = line.find(&date).unwrap();

    let before = &line[..date_pos];

    let before_ten = &before
        .chars()
        .rev()
        .take(15)
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();

    let res = format!("{}{}", before_ten, date);

    res
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Date {
    pub fn new(value: String, index: usize, line: String) -> Self {
        Date { value, index, line }
    }
}

pub fn find_dates_in_line(res: Vec<regex::Regex>, line: &str) -> Vec<String> {
    let mut dates = vec![];
    for regex in res {
        for date in regex.find_iter(line) {
            dates.push(date.as_str().to_string());
        }
    }
    dates.sort();
    dates.dedup();
    dates
}

pub fn find_dates(text: &str) -> Vec<Date> {
    let regexes = regexes();
    let lines = text.lines();
    let lines_len = lines.clone().count();
    let mut dates: Vec<Date> = vec![];
    for (i, line) in lines.clone().enumerate() {
        let line_dates = find_dates_in_line(regexes.clone(), line);
        for date in line_dates {
            let _line_before = if lines_len > 0 {
                lines.clone().nth(i - 1).unwrap_or("")
            } else {
                ""
            };

            let _line_after = if lines_len > i {
                lines.clone().nth(i + 1).unwrap_or("")
            } else {
                ""
            };
            dates.push(Date::new(date, i, line.to_string()));
        }
    }

    dates.sort_by_key(|d| d.key());
    dates.dedup_by_key(|d| d.key());

    dates
}
