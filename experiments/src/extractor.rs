use core::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct LineWithMatch {
    date: String,
    pos: usize,
    section: Section,
}

impl Display for LineWithMatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Date: {}, Position in file: {}\n-------\nSection:\n{}\n-------------------\n",
            self.date,
            self.pos,
            self.section.to_string()
        )
    }
}

#[derive(Debug)]
pub struct Section {
    before: Option<String>,
    after: Option<String>,
    current: String,
}

impl Section {
    fn new(before: Option<String>, after: Option<String>, current: String) -> Self {
        Self {
            before,
            after,
            current,
        }
    }
}

impl Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}\n{:?}\n{:?}",
            self.before.clone().unwrap_or_default(),
            self.current,
            self.after.clone().unwrap_or_default()
        )
    }
}

impl LineWithMatch {
    fn new(date: String, pos: usize, section: Section) -> Self {
        Self {date, pos, section }
    }
}

pub fn lines_with_dates(file: String) -> Vec<LineWithMatch> {
    let lines = lines(file);
    let res: Vec<LineWithMatch> = lines
        .iter()
        .enumerate()
        .filter_map(|x| {
            let (pos, l) = x;
            let dates = find_dates(l.to_string());
            let mut before = None;
            let mut after = None;

            if pos != 0 {
                before = lines.get(pos - 1);
                if pos != lines.len() {
                    after = lines.get(pos + 1);
                }
            }

            if dates.is_empty() {
                None
            } else {
                let s = Section::new(before.cloned(), after.cloned(), l.clone());

                let m = LineWithMatch::new(dates.join(", "),pos, s);
                Some(m)
            }
        })
        .collect();
    res
}

pub fn lines(file: String) -> Vec<String> {
    let lines: Vec<String> = file.lines().map(|l| l.to_string()).collect();
    lines
}

pub fn find_dates(line: String) -> Vec<String> {
    let patterns =     vec![
        "\\d{2}-\\d{2}-\\d{4}",
    "[0-9]{2}/{1}[0-9]{2}/{1}[0-9]{4}",
    "\\d{1,2}-(January|February|March|April|May|June|July|August|September|October|November|December)-\\d{4}",
    "\\d{4}-\\d{1,2}-\\d{1,2}",
    "[0-9]{1,2}\\s(January|February|March|April|May|June|July|August|September|October|November|December)\\s\\d{4}",
    "\\d{1,2}-\\d{1,2}-\\d{4}"]
    
}
