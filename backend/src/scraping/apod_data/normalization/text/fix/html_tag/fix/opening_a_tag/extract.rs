use regex::Regex;

pub fn extract_url(tag: &str) -> &str {
  let href_attr_regex_str = r"(?:ref|href|rhef|hre|hef|hrf|HREF|h ref|hreff)";
  let url_regex_str = r"(?P<url>[\S\s]*?)";
  let end_regex_str = r#"(?:>$|"[\S\s]*>$|"</a>$)"#;
  let href_url_con_regex_str = r#"(?:\s*=\s*"|=|")"#;
  let link_opening_tag_regex_str = format!(
    r#"<l?[aA]\s*{href_attr}{href_url_con}{url}{end}"#,
    href_attr = href_attr_regex_str,
    href_url_con = href_url_con_regex_str,
    url = url_regex_str,
    end = end_regex_str
  );
  Regex::new(link_opening_tag_regex_str.as_str())
    .unwrap()
    .captures(tag)
    .unwrap()
    .name("url")
    .unwrap()
    .as_str()
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn can_interpret_all_variants() {
    assert_eq!(extract_url("<a href=\"www.google.de\">"), "www.google.de");
    assert_eq!(extract_url("<ahref=\"www.google.de\">"), "www.google.de");
    assert_eq!(extract_url("<a HREF=\"www.google.de\">"), "www.google.de");
    assert_eq!(extract_url("<a hrf=\"www.google.de\">"), "www.google.de");
    assert_eq!(extract_url("<a hef=\"www.google.de\">"), "www.google.de");
    assert_eq!(extract_url("<a ref=\"www.google.de\">"), "www.google.de");
    assert_eq!(extract_url("<A href=\"www.google.de\">"), "www.google.de");
    assert_eq!(extract_url("<a href=www.google.de>"), "www.google.de");
    assert_eq!(
      extract_url("<a href=\"www.google.de\" id=\"Test\">"),
      "www.google.de"
    );
    assert_eq!(extract_url("<a href=\"www.google.de>\">"), "www.google.de>");
    assert_eq!(extract_url("<a href=\"www.google.de\" >"), "www.google.de");
    assert_eq!(
      extract_url("<a href=\"www.google.de\"</a>"),
      "www.google.de"
    );
    assert_eq!(extract_url("<a href=\"\">"), "");
    assert_eq!(extract_url("<a h ref=\"www.google.de\">"), "www.google.de");
    assert_eq!(extract_url("<a hreff=\"www.google.de\">"), "www.google.de");
    assert_eq!(
      extract_url("<a href\"http://www.google.de\">"),
      "http://www.google.de"
    );
    assert_eq!(extract_url("<a href =\"www.google.de\">"), "www.google.de");
    assert_eq!(extract_url("<a href= \"www.google.de\">"), "www.google.de");
    assert_eq!(extract_url("<a href = \"www.google.de\">"), "www.google.de");
    assert_eq!(extract_url("<la href=\"www.google.de\">"), "www.google.de");
  }
}
