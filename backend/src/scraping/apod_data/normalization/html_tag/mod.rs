mod opening_a_tag;

use opening_a_tag::normalize_opening_a_tag;
use regex::Regex;

pub fn normalize_html_tag(tag: &str) -> String {
  let tag_syntax_good = Regex::new(r#"^(?:</?[a-z]+>|<a href="\S+?">)$"#)
    .unwrap()
    .is_match(tag);
  if tag_syntax_good {
    if tag == "</br>" {
      return String::from("<br>");
    }
    return String::from(tag);
  }

  let is_opening_a_tag = Regex::new(r"^<[aA](?:\s|href)").unwrap().is_match(tag);
  if is_opening_a_tag {
    return normalize_opening_a_tag(tag);
  }

  let is_closing_tag = tag.contains("/");
  if is_closing_tag {
    let tag_name = Regex::new(r"[^a-z]").unwrap().replace_all(tag, "");
    return format!("</{}>", &tag_name);
  }

  String::from(tag.to_lowercase())
}
