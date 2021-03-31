mod check;
mod fix;
mod regexes;

use crate::scraping::{ScrapeError, ScrapeResult};
use check::check_html;
use fix::{normalize_html_tag, replace_matches};
use regex::Regex;
use regexes::TAG_REGEX;

pub fn normalize_text(text: &str) -> ScrapeResult<String> {
  // TODO: Fix &ccedil; &oacute; &eacute; &aacute; &amp; &oslash;
  let new_lines_removed = Regex::new(r"\n+").unwrap().replace_all(text, " ");

  let tags_fixed = replace_matches(&new_lines_removed, TAG_REGEX, normalize_html_tag)?;

  let missing_closing_link_tag_fixed =
    Regex::new(r"(?P<first_tag><a[^>]+?>)(?P<content>[^<]+?)(?P<add>[^\w]*)(?P<end>(?:<a|$))")
      .unwrap()
      .replace_all(&tags_fixed, "${first_tag}${content}</a>${add}${end}");

  let non_content_moved_before = Regex::new(r#"(<(?:i|b|a href=".+?")>)(\s+)(\S)"#)
    .unwrap()
    .replace_all(&missing_closing_link_tag_fixed, "$2$1$3");

  let non_content_moved_after = Regex::new(r#"(\S)(\s+)(</[iba]>)"#)
    .unwrap()
    .replace_all(&non_content_moved_before, "$1$3$2");

  let tag_colon_order_fixed = Regex::new(r"(?P<c_tag></[ib]>)\s?:")
    .unwrap()
    .replace_all(&non_content_moved_after, ":${c_tag}");

  let multiple_spaces_fixed = Regex::new(r" {2,}")
    .unwrap()
    .replace_all(&tag_colon_order_fixed, " ");

  let spaces_around_br_removed = Regex::new(r"\s?<br>\s?")
    .unwrap()
    .replace_all(&multiple_spaces_fixed, "<br>");

  let trimmed = spaces_around_br_removed.trim();

  match check_html(trimmed) {
    Err(ScrapeError::HTMLFixing(err_message)) => {
      panic!("HTML fix error ({}) in {}", err_message, trimmed)
    }
    _ => (),
  }

  Ok(String::from(trimmed))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn fixes_tags() {
    assert_eq!(
      normalize_text(r#"This is a text with a <a href=www.google.de>Link</a> within it."#).unwrap(),
      r#"This is a text with a <a href="www.google.de">Link</a> within it."#
    );
    assert_eq!(
      normalize_text(
        r#"<a href="https://www.eso.org/public/">ESO</a>/<a href="https://www.eso.org/public/teles-instr/lasilla/mpg22/wfi/">WFI</a> (visible);"#
      ).unwrap(),
      r#"<a href="https://www.eso.org/public/">ESO</a>/<a href="https://www.eso.org/public/teles-instr/lasilla/mpg22/wfi/">WFI</a> (visible);"#
    );
  }

  #[test]
  fn adds_missing_closing_link_tags() {
    assert_eq!(
      normalize_text(
        r#"This is a text with a <a href="www.google.de">Link without end-tag <a href="www.google.de">and another Link</a>."#
      ).unwrap(),
      r#"This is a text with a <a href="www.google.de">Link without end-tag</a> <a href="www.google.de">and another Link</a>."#
    );
    assert_eq!(
      normalize_text(
        r#"This is a text with a lonely <a href="www.google.de">Link without end-tag."#
      )
      .unwrap(),
      r#"This is a text with a lonely <a href="www.google.de">Link without end-tag</a>."#
    );
  }

  #[test]
  fn moves_spaces_out_of_b_to_front() {
    assert_eq!(
      normalize_text(r#"Here is<b> bold</b> Text"#).unwrap(),
      r#"Here is <b>bold</b> Text"#
    );
  }

  #[test]
  fn moves_spaces_out_of_b_to_back() {
    assert_eq!(
      normalize_text(r#"Here is <b>bold </b>Text"#).unwrap(),
      r#"Here is <b>bold</b> Text"#
    );
  }

  #[test]
  fn moves_spaces_out_of_b_to_front_and_back() {
    assert_eq!(
      normalize_text(r#"Here is<b> bold </b>Text"#).unwrap(),
      r#"Here is <b>bold</b> Text"#
    );
  }

  #[test]
  fn leaves_spaces_as_is_in_good_b() {
    assert_eq!(
      normalize_text(r#"Here is <b>bold</b> Text"#).unwrap(),
      r#"Here is <b>bold</b> Text"#
    );
  }

  #[test]
  fn moves_spaces_out_of_i_to_front() {
    assert_eq!(
      normalize_text(r#"Here is<i> italic</i> Text"#).unwrap(),
      r#"Here is <i>italic</i> Text"#
    );
  }

  #[test]
  fn moves_spaces_out_of_i_to_back() {
    assert_eq!(
      normalize_text(r#"Here is <i>italic </i>Text"#).unwrap(),
      r#"Here is <i>italic</i> Text"#
    );
  }

  #[test]
  fn moves_spaces_out_of_i_to_front_and_back() {
    assert_eq!(
      normalize_text(r#"Here is<i> italic </i>Text"#).unwrap(),
      r#"Here is <i>italic</i> Text"#
    );
  }

  #[test]
  fn leaves_spaces_as_is_in_good_i() {
    assert_eq!(
      normalize_text(r#"Here is <i>italic</i> Text"#).unwrap(),
      r#"Here is <i>italic</i> Text"#
    );
  }

  #[test]
  fn moves_spaces_out_of_a_to_front() {
    assert_eq!(
      normalize_text(r#"Here is<a href="www.google.de"> Link</a> Text"#).unwrap(),
      r#"Here is <a href="www.google.de">Link</a> Text"#
    );
  }

  #[test]
  fn moves_spaces_out_of_a_to_back() {
    assert_eq!(
      normalize_text(r#"Here is <a href="www.google.de">Link </a>Text"#).unwrap(),
      r#"Here is <a href="www.google.de">Link</a> Text"#
    );
  }

  #[test]
  fn moves_spaces_out_of_a_to_front_and_back() {
    assert_eq!(
      normalize_text(r#"Here is<a href="www.google.de"> Link </a>Text"#).unwrap(),
      r#"Here is <a href="www.google.de">Link</a> Text"#
    );
  }

  #[test]
  fn leaves_spaces_as_is_in_good_a() {
    assert_eq!(
      normalize_text(r#"Here is <a href="www.google.de">Link</a> Text"#).unwrap(),
      r#"Here is <a href="www.google.de">Link</a> Text"#
    );
  }

  #[test]
  fn moves_colon_into_ib() {
    assert_eq!(
      normalize_text("Some <i>Text</i>: Here").unwrap(),
      "Some <i>Text:</i> Here"
    )
  }

  #[test]
  fn removes_caret_in_url() {
    assert_eq!(
      normalize_text(r#"A <a href="https://www.nasa.gov/>">Link</a> with caret in URL"#).unwrap(),
      r#"A <a href="https://www.nasa.gov/">Link</a> with caret in URL"#
    )
  }
}
