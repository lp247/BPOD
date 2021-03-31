mod regexes;

use super::super::super::super::super::normalize_url;
use regex::Regex;
use regexes::LINK;

pub fn fix_opening_a_tag(tag: &str) -> String {
  let url = Regex::new(&LINK)
    .unwrap()
    .captures(tag)
    .unwrap()
    .name("url")
    .unwrap()
    .as_str();
  format!(r#"<a href="{}">"#, normalize_url(url))
}
