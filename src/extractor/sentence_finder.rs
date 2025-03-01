use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct SentenceMatch {
    text: String,
}

impl SentenceMatch {
    pub fn text(&self) -> &str {
        &self.text
    }
}

pub fn find_sentences(html: &str) -> Vec<SentenceMatch> {
    let mut sentences = Vec::new();
    let mut current_sentence = String::new();

    // Remove HTML tags
    let text = html.replace("<[^>]*>", "");

    for (i, c) in text.chars().enumerate() {
        current_sentence.push(c);

        // Check for sentence endings
        if c == '.' || c == '!' || c == '?' {
            // Check if next char is whitespace or end of string
            if i + 1 >= text.len() || text.chars().nth(i + 1).unwrap().is_whitespace() {
                let cleaned = strip_html_and_refs(current_sentence.trim());

                sentences.push(SentenceMatch { text: cleaned });
                current_sentence.clear();
            }
        }
    }

    // Add final sentence if any
    if !current_sentence.trim().is_empty() {
        let cleaned = strip_html_and_refs(current_sentence.trim());
        sentences.push(SentenceMatch { text: cleaned });
    }

    remove_quotes(&mut sentences);
    sentences.sort();
    sentences.dedup();
    sentences
}

pub fn strip_html_and_refs(input: &str) -> String {
    let mut result = String::new();
    let mut inside_tag = false;
    let mut skip_refs = false;
    let no_js = strip_javascript_sections(input);
    let mut chars = no_js.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '<' {
            inside_tag = true;
            continue;
        }
        if c == '>' {
            inside_tag = false;
            continue;
        }
        if !inside_tag {
            // Check for src= or href= patterns
            if (c == 's' && chars.peek() == Some(&'r') && chars.clone().nth(1) == Some('c'))
                || (c == 'h'
                    && chars.peek() == Some(&'r')
                    && chars.clone().nth(1) == Some('e')
                    && chars.clone().nth(2) == Some('f'))
            {
                skip_refs = true;
                // Skip the rest of the pattern
                while let Some(next_c) = chars.peek() {
                    if next_c.is_whitespace() {
                        break;
                    }
                    chars.next();
                }
                continue;
            }
            if skip_refs {
                if c.is_whitespace() {
                    skip_refs = false;
                }
                continue;
            }
            if !c.is_whitespace() || !result.ends_with(' ') {
                result.push(c);
            }
        }
    }

    // Remove any remaining HTML tags
    result.replace(regex::Regex::new(r"<[^>]+>").unwrap().as_str(), "")
}

#[allow(clippy::all)]
pub fn strip_javascript_sections(input: &str) -> String {
    let mut result = String::new();
    let mut in_script = false;
    let mut current_script_start = String::new();
    let without_json = strip_json_sections(input);
    let mut chars = without_json.chars().peekable();

    while let Some(c) = chars.next() {
        if !in_script {
            // Look for script start
            if c == '<' {
                current_script_start.clear();
                current_script_start.push(c);
            } else if !current_script_start.is_empty() {
                current_script_start.push(c);
                if current_script_start.to_lowercase() == "<script" {
                    in_script = true;
                    // Skip until we find the closing >
                    while let Some(sc) = chars.next() {
                        if sc == '>' {
                            break;
                        }
                    }
                    continue;
                } else if current_script_start.len() >= 7 {
                    // Not a script tag, add back the collected chars
                    result.push_str(&current_script_start);
                    current_script_start.clear();
                }
            } else {
                result.push(c);
            }
        } else {
            // In script section, look for </script>
            if c == '<' {
                let mut closing = String::from("<");
                while let Some(nc) = chars.next() {
                    closing.push(nc);
                    if closing.to_lowercase() == "</script>" {
                        in_script = false;
                        break;
                    }
                    if closing.len() > 9 {
                        break;
                    }
                }
            }
        }
    }

    result
}

#[allow(clippy::all)]
pub fn strip_json_sections(input: &str) -> String {
    let mut result = String::new();
    let mut in_json = false;
    let mut current_json_start = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if !in_json {
            // Look for potential JSON array/object start
            if c == '[' || c == '{' {
                current_json_start.clear();
                current_json_start.push(c);
                if let Some(nc) = chars.peek() {
                    if *nc == '"' || nc.is_whitespace() {
                        in_json = true;
                        continue;
                    } else {
                        result.push(c);
                    }
                }
            } else {
                result.push(c);
            }
        } else {
            // Count brackets to handle nested structures
            if c == ']' || c == '}' {
                if chars.peek().map_or(true, |nc| nc.is_whitespace()) {
                    in_json = false;
                }
            }
        }
    }

    result
}

fn remove_quotes(sentences: &mut Vec<SentenceMatch>) {
    for sentence in sentences.iter_mut() {
        sentence.text = sentence.text.replace('"', "");
        sentence.text = sentence.text.trim().to_string();
    }
    sentences.retain(|s| !s.text.contains("});"));
    sentences.retain(|s| !s.text.contains("{("));
}
