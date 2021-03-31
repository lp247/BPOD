mod extract;

use super::super::super::super::super::normalize_url;
use extract::extract_url;

pub fn fix_opening_a_tag(tag: &str) -> String {
  let url = extract_url(tag);

  // The url may not be given. In this case remove the opening link tag
  // altogether. The remaining </a> tag will be removed in artifacts clean up.
  if url.is_empty() {
    return String::new();
  }

  format!(r#"<a href="{}">"#, normalize_url(url))
}
