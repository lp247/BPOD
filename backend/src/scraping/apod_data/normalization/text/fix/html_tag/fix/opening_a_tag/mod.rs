mod extract;

use super::super::super::super::super::normalize_url;
use extract::extract_url;

pub fn fix_opening_a_tag(tag: &str) -> String {
  let url = extract_url(tag);
  format!(r#"<a href="{}">"#, normalize_url(url))
}
