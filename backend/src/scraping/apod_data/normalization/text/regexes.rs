pub const TAG_REGEX: &str = r#"<(?:[^"]|".+?")+?>"#;

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use regex::Regex;

  fn helper(text: &str, compare: &[&str]) {
    let re = Regex::new(TAG_REGEX).unwrap();
    assert_eq!(re.find_iter(text).count(), compare.len());
    for (i, m) in re.find_iter(text).enumerate() {
      assert_eq!(m.as_str(), compare[i]);
    }
  }

  #[test]
  fn tag_regex_finds_tags() {
    helper("Text <i> Text", &["<i>"]);
    helper("Text <i>Content</i> Text", &["<i>", "</i>"]);
    helper(
      "Text <a href=\"www.google.de\">Link</a> Text",
      &["<a href=\"www.google.de\">", "</a>"],
    );
    helper(
      "Text <ahref=\"www.google.de\">Link</a> Text",
      &["<ahref=\"www.google.de\">", "</a>"],
    );
    helper(
      "Text <a href=\"www.google.de>\">Link</a> Text",
      &["<a href=\"www.google.de>\">", "</a>"],
    );
    helper(
      "Text <a href=\"www.google.de\" >Link</a> Text",
      &["<a href=\"www.google.de\" >", "</a>"],
    );
    helper(
      "Text <?=/a> Text",
      &["<?=/a>"],
    );
  }
}
