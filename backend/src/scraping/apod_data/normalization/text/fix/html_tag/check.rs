use regex::Regex;

pub fn tag_syntax_is_valid(tag: &str) -> bool {
  let tag_syntax_good = Regex::new(r#"^(?:</?[a-z]+>|<a href="[^<>\s]+?"(?:[\s\S]*?\S>|>)|<br />)$"#)
    .unwrap()
    .is_match(tag);
  tag_syntax_good && tag != "</br>"
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn detects_valid_tags() {
    assert_eq!(tag_syntax_is_valid("<i>"), true);
    assert_eq!(tag_syntax_is_valid("</i>"), true);
    assert_eq!(tag_syntax_is_valid("<b>"), true);
    assert_eq!(tag_syntax_is_valid("</b>"), true);
    assert_eq!(tag_syntax_is_valid(r#"<a href="www.google.de">"#), true);
    assert_eq!(
      tag_syntax_is_valid(r#"<a href="www.google.de" id="link">"#),
      true
    );
    assert_eq!(tag_syntax_is_valid("</a>"), true);
    assert_eq!(tag_syntax_is_valid("<br>"), true);
    assert_eq!(tag_syntax_is_valid("<br />"), true);
    assert_eq!(tag_syntax_is_valid("<center>"), true);
    assert_eq!(tag_syntax_is_valid("</center>"), true);
  }

  #[test]
  fn detects_invalid_tags() {
    assert_eq!(tag_syntax_is_valid("<a/>"), false);
    assert_eq!(tag_syntax_is_valid("<A>"), false);
    assert_eq!(tag_syntax_is_valid("<I>"), false);
    assert_eq!(tag_syntax_is_valid("<B>"), false);
    assert_eq!(tag_syntax_is_valid("</a/>"), false);
    assert_eq!(tag_syntax_is_valid("</?=a>"), false);
    assert_eq!(tag_syntax_is_valid("<?=/a>"), false);
    assert_eq!(tag_syntax_is_valid(r#"<ahref="www.google.de">"#), false);
    assert_eq!(tag_syntax_is_valid(r#"<a hrf="www.google.de">"#), false);
    assert_eq!(tag_syntax_is_valid(r#"<a ref="www.google.de">"#), false);
    assert_eq!(tag_syntax_is_valid(r#"<a href=www.google.de>"#), false);
    assert_eq!(tag_syntax_is_valid(r#"<a href="www.google.de>">"#), false);
    assert_eq!(tag_syntax_is_valid(r#"<a href="www.google.de" >"#), false);
    assert_eq!(tag_syntax_is_valid("</br>"), false);
    assert_eq!(tag_syntax_is_valid("<br/>"), false);
  }
}
