pub const LINK: &str = r#"<[aA]\s*(?:ref|href|rhef|hre|hef|hrf|HREF)\s*=\s*"?(?P<url>[\S\s]+?)(?:>$|"[\S\s]*>$|"</a>$)"#;

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use regex::Regex;

  fn helper(text: &str, url: &str) {
    let re = Regex::new(LINK).unwrap();
    assert!(re.is_match(text));
    assert_eq!(
      re.captures(text).unwrap().name("url").unwrap().as_str(),
      url
    );
  }

  #[test]
  fn can_interpret_all_variants() {
    helper("<a href=\"www.google.de\">", "www.google.de");
    helper("<ahref=\"www.google.de\">", "www.google.de");
    helper("<a HREF=\"www.google.de\">", "www.google.de");
    helper("<a hrf=\"www.google.de\">", "www.google.de");
    helper("<a hef=\"www.google.de\">", "www.google.de");
    helper("<a ref=\"www.google.de\">", "www.google.de");
    helper("<A href=\"www.google.de\">", "www.google.de");
    helper("<a href=www.google.de>", "www.google.de");
    helper("<a href=\"www.google.de\" id=\"Test\">", "www.google.de");
    helper("<a href=\"www.google.de>\">", "www.google.de>");
    helper("<a href=\"www.google.de\" >", "www.google.de");
  }
}
