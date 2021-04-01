use super::super::normalization::normalize_text;
use crate::scraping::ScrapeResult;
use regex::Regex;

pub fn get_description(page: &str) -> ScrapeResult<String> {
  let regex =
    Regex::new(r#"<.+?>\s*Explanation:\s*<.+?>\s*(?P<explanation>[\s\S]+?)\s*<p>"#).unwrap();
  let raw_description = regex
    .captures(page)
    .unwrap()
    .name("explanation")
    .unwrap()
    .as_str();
  normalize_text(raw_description, false)
}
