use super::super::normalize_url;
use regex::Regex;

pub fn normalize_opening_a_tag(tag: &str) -> String {
  if Regex::new(r#"<a href="[\S]+?">"#).unwrap().is_match(tag) {
    return String::from(tag);
  }

  let href_attr_regex = r"(?:ref|href|rhef|hre|hef|hrf|HREF)";
  let link_url_regex = r"(?P<url>[\S\s]+?)";
  let link_regex = format!(
    r#"<[aA]\s*{href_attr}\s*=\s*"?{link_url}(?:>|"[\S\s]*>|"</a>)"#,
    href_attr = href_attr_regex,
    link_url = link_url_regex,
  );
  let url = Regex::new(&link_regex)
    .unwrap()
    .captures(tag)
    .unwrap()
    .name("url")
    .unwrap()
    .as_str();
  format!(r#"<a href="{}">"#, normalize_url(url))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn changes_uppercase_to_lowercase() {
    assert_eq!(
      normalize_opening_a_tag(r#"<A href="http://www.google.de">"#),
      r#"<a href="http://www.google.de">"#
    );
  }

  #[test]
  fn inserts_missing_space_between_tag_name_and_href_attr() {
    assert_eq!(
      normalize_opening_a_tag(r#"<ahref="http://www.google.de">"#),
      r#"<a href="http://www.google.de">"#
    );
  }

  #[test]
  fn fixes_bad_href_attr_name() {
    assert_eq!(
      normalize_opening_a_tag(r#"<a rhef="http://www.google.de">"#),
      r#"<a href="http://www.google.de">"#
    );
    assert_eq!(
      normalize_opening_a_tag(r#"<a ref="http://www.google.de">"#),
      r#"<a href="http://www.google.de">"#
    );
    assert_eq!(
      normalize_opening_a_tag(r#"<a hre="http://www.google.de">"#),
      r#"<a href="http://www.google.de">"#
    );
    assert_eq!(
      normalize_opening_a_tag(r#"<a hef="http://www.google.de">"#),
      r#"<a href="http://www.google.de">"#
    );
    assert_eq!(
      normalize_opening_a_tag(r#"<a hrf="http://www.google.de">"#),
      r#"<a href="http://www.google.de">"#
    );
    assert_eq!(
      normalize_opening_a_tag(r#"<a HREF="http://www.google.de">"#),
      r#"<a href="http://www.google.de">"#
    );
  }

  #[test]
  fn inserts_missing_quotes() {
    assert_eq!(
      normalize_opening_a_tag(r#"<a href=http://www.google.de>"#),
      r#"<a href="http://www.google.de">"#
    );
  }

  #[test]
  fn fixes_a_closing_tag_as_end() {
    assert_eq!(
      normalize_opening_a_tag(r#"<a href="http://www.google.de"</a>"#),
      r#"<a href="http://www.google.de">"#
    );
  }

  #[test]
  fn removes_spaces_around_equal_sign() {
    assert_eq!(
      normalize_opening_a_tag(r#"<a href = "http://www.google.de">"#),
      r#"<a href="http://www.google.de">"#
    );
  }

  #[test]
  fn fixes_new_lines_in_tag() {
    assert_eq!(
      normalize_opening_a_tag(
        "<a href=\n\"https://www.smithsonianmag.com/history/decoding-antikythera-mechanism-first-computer-180953979/\"\n>"
      ),
      "<a href=\"https://www.smithsonianmag.com/history/decoding-antikythera-mechanism-first-computer-180953979/\">"
    )
  }
}
