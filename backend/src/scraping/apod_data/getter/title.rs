use super::super::normalization::normalize_text;
use super::super::translation::html_to_markdown;
use super::utils::get_title_meta_block;
use crate::scraping::ScrapeResult;
use regex::Regex;

pub fn get_title(page: &str) -> ScrapeResult<String> {
  let meta_block = get_title_meta_block(page);
  let regex =
    Regex::new(r"<[^>]+?>\s*(\S[\s\S]+?\S)\s*</[^>]+?>").expect("Regex for title invalid");
  let raw_title = regex
    .find_iter(meta_block)
    .nth(0)
    .expect("Could not find title")
    .as_str();
  let normalized_title = normalize_text(raw_title)?;
  Ok(html_to_markdown(&normalized_title).replace("*", ""))
}
