use std::{
    fmt::{self, Display},
};

use regex::Regex;

#[derive(Debug)]
pub struct LineWithMatch {
    pos: usize,
    section: Section,
}

impl Display for LineWithMatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Position in file: {}\n-------\nSection:\n{}\n-------------------\n",
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
    fn new(pos: usize, section: Section) -> Self {
        Self {pos, section }
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

            if !dates {
                None
            } else {
                let s = Section::new(before.cloned(), after.cloned(), l.clone());

                let m = LineWithMatch::new(pos, s);
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

pub fn find_dates(line: String) -> bool {
    line.contains("date") || line.contains("Date") || line.contains("DATE") || line.contains("Datum")
}
