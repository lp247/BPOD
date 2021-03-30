use super::super::normalization::normalize_text;
use super::super::translation::html_to_markdown;
use regex::Regex;

pub fn get_description(page: &str) -> String {
  let regex =
    Regex::new(r#"<.+?>\s*Explanation:\s*<.+?>\s*(?P<explanation>[\s\S]+?)\s*<p>"#).unwrap();
  let raw_description = regex
    .captures(page)
    .unwrap()
    .name("explanation")
    .unwrap()
    .as_str();
  html_to_markdown(&normalize_text(raw_description))
}
