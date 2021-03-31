use regex::Regex;

pub fn extract_url(tag: &str) -> &str {
  Regex::new(r#"<[aA]\s*(?:ref|href|rhef|hre|hef|hrf|HREF|h ref)\s*=\s*"?(?P<url>[\S\s]*?)(?:>$|"[\S\s]*>$|"</a>$)"#)
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
    assert_eq!(extract_url("<a href=\"image/1905/TotnBefore_Dai_3000.jpg\"</a>"), "image/1905/TotnBefore_Dai_3000.jpg");
    assert_eq!(extract_url("<a href=\"\">"), "");
    assert_eq!(extract_url("<a h ref=\"www.google.de\">"), "www.google.de");
  }
}
