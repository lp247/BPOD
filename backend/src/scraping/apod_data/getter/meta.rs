use super::super::normalization::normalize_text;
use super::super::translation::html_to_markdown;
use super::utils::get_title_meta_block;
use regex::Regex;

pub fn get_meta(page: &str) -> String {
  let title_meta_block = get_title_meta_block(page);
  let meta_block = Regex::new(r"<[^>]+?>[\s\S]+?</[^>]+?>\s*(?:<br>)?\s*(?P<amb>[\s\S]+)")
    .expect("Regex for additional meta block invalid")
    .captures(title_meta_block)
    .expect("Could not find additional meta block content")
    .name("amb")
    .expect("Could not get meta block content")
    .as_str();
  html_to_markdown(&normalize_text(meta_block).replace("*", ""))
}
