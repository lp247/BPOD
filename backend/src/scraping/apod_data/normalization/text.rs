use super::html_tag::normalize_html_tag;
use crate::scraping::{ScrapeError, ScrapeResult};
use regex::{Captures, Regex};

pub fn normalize_text(text: &str) -> String {
  // TODO: Fix &ccedil; &oacute; &eacute; &aacute; &amp; &oslash;
  let tags_fixed = Regex::new(r"<[^>]+?>")
    .unwrap()
    .replace_all(text, |captures: &Captures| {
      normalize_html_tag(captures.get(0).unwrap().as_str())
    });

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

  // let tag_colon_order_fixed = Regex::new(r"(?P<c_tag></[^>]+?>):")
  //   .unwrap()
  //   .replace_all(&space_in_tags_removed, ":${c_tag}");

  let multiple_spaces_fixed = Regex::new(r" {2,}")
    .unwrap()
    .replace_all(&non_content_moved_after, " ");

  let multiple_new_line_fixed = Regex::new(r"\n{3,}")
    .unwrap()
    .replace_all(&multiple_spaces_fixed, "\n\n");

  let trimmed = multiple_new_line_fixed.trim();

  match check_html(trimmed) {
    Err(ScrapeError::HTMLFixing(err_message)) => {
      panic!("HTML fix error ({}) in {}", err_message, trimmed)
    }
    _ => (),
  }

  String::from(trimmed)
}

fn check_html(html: &str) -> ScrapeResult<()> {
  check_empty_start_of_text(html)?;
  check_empty_end_of_text(html)?;
  check_colon_style_tag_order(html)?;
  check_multiple_new_lines(html)?;
  check_multiple_spaces(html)?;
  check_tag_case(html)?;
  check_link_opening_tag_format(html)?;
  check_closing_tag_format(html)
}

fn check_empty_start_of_text(html: &str) -> ScrapeResult<()> {
  if Regex::new(r"^\s").unwrap().is_match(html) {
    return Err(ScrapeError::HTMLFixing(String::from(
      "Empty space at beginning of text",
    )));
  }
  Ok(())
}

fn check_empty_end_of_text(html: &str) -> ScrapeResult<()> {
  if Regex::new(r"\s$").unwrap().is_match(html) {
    return Err(ScrapeError::HTMLFixing(String::from(
      "Empty space at end of text",
    )));
  }
  Ok(())
}

fn check_colon_style_tag_order(html: &str) -> ScrapeResult<()> {
  if Regex::new(r"</[bi]>\s*:").unwrap().is_match(html) {
    return Err(ScrapeError::HTMLFixing(String::from(
      "Colon after closing </b> or </i> tag",
    )));
  }
  Ok(())
}

fn check_multiple_new_lines(html: &str) -> ScrapeResult<()> {
  if Regex::new(r"\n{3,}").unwrap().is_match(html) {
    return Err(ScrapeError::HTMLFixing(String::from(
      "More than 2 consecutive newlines",
    )));
  }
  Ok(())
}

fn check_multiple_spaces(html: &str) -> ScrapeResult<()> {
  if Regex::new(r" {2,}").unwrap().is_match(html) {
    return Err(ScrapeError::HTMLFixing(String::from(
      "More than 1 consecutive space",
    )));
  }
  Ok(())
}

fn check_tag_case(html: &str) -> ScrapeResult<()> {
  if Regex::new(r"</?[A-Z]").unwrap().is_match(html) {
    return Err(ScrapeError::HTMLFixing(String::from("Upper case tag")));
  }
  Ok(())
}

fn check_link_opening_tag_format(html: &str) -> ScrapeResult<()> {
  let num_any_link_tags = Regex::new(r"(?:<a\s|<ahref)")
    .unwrap()
    .find_iter(html)
    .count();
  let num_valid_link_tags = Regex::new(r#"<a href="\S+?">"#)
    .unwrap()
    .find_iter(html)
    .count();
  if num_any_link_tags != num_valid_link_tags {
    return Err(ScrapeError::HTMLFixing(String::from(
      "Link opening tag with bad format",
    )));
  }
  Ok(())
}

fn check_closing_tag_format(html: &str) -> ScrapeResult<()> {
  let all_closing_tags_valid = Regex::new(r"<\S+?>")
    .unwrap()
    .find_iter(html)
    .filter(|tag| tag.as_str().contains("/"))
    .all(|tag| Regex::new(r"</[a-z]+?>").unwrap().is_match(tag.as_str()));
  if !all_closing_tags_valid {
    return Err(ScrapeError::HTMLFixing(String::from(
      "Closing tag with bad format",
    )));
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn fixes_tags() {
    assert_eq!(
      normalize_text("This is a text with a <a href=www.google.de>Link</a> within it."),
      r#"This is a text with a <a href="www.google.de">Link</a> within it."#
    );
    assert_eq!(
      normalize_text("The <a href=\"https://en.wikipedia.org/wiki/Antikythera_mechanism\"\n>Antikythera mechanism</a>, pictured, is now widely regarded as the \n<a href=\"https://en.wikipedia.org/wiki/Computer#Pre-20th_century\"\n>first</a> <a href=\n\"https://www.smithsonianmag.com/history/decoding-antikythera-mechanism-first-computer-180953979/\"\n>computer</a>."),
      "The <a href=\"https://en.wikipedia.org/wiki/Antikythera_mechanism\">Antikythera mechanism</a>, pictured, is now widely regarded as the \n<a href=\"https://en.wikipedia.org/wiki/Computer#Pre-20th_century\">first</a> <a href=\"https://www.smithsonianmag.com/history/decoding-antikythera-mechanism-first-computer-180953979/\">computer</a>."
    );
    assert_eq!(
      normalize_text("<a href=\"https://www.eso.org/public/\">ESO</a>/<a\nhref=\"https://www.eso.org/public/teles-instr/lasilla/mpg22/wfi/\">WFI</a> (visible);"),
      "<a href=\"https://www.eso.org/public/\">ESO</a>/<a href=\"https://www.eso.org/public/teles-instr/lasilla/mpg22/wfi/\">WFI</a> (visible);"
    );
  }

  #[test]
  fn adds_missing_closing_link_tags() {
    assert_eq!(
      normalize_text(
        r#"This is a text with a <a href="www.google.de">Link without end-tag <a href="www.google.de">and another Link</a>."#
      ),
      r#"This is a text with a <a href="www.google.de">Link without end-tag</a> <a href="www.google.de">and another Link</a>."#
    );
    assert_eq!(
      normalize_text(
        r#"This is a text with a lonely <a href="www.google.de">Link without end-tag."#
      ),
      r#"This is a text with a lonely <a href="www.google.de">Link without end-tag</a>."#
    );
  }

  #[test]
  fn moves_spaces_out_of_b_to_front() {
    assert_eq!(
      normalize_text(r#"Here is<b> bold</b> Text"#),
      r#"Here is <b>bold</b> Text"#
    );
  }

  #[test]
  fn moves_spaces_out_of_b_to_back() {
    assert_eq!(
      normalize_text(r#"Here is <b>bold </b>Text"#),
      r#"Here is <b>bold</b> Text"#
    );
  }

  #[test]
  fn moves_spaces_out_of_b_to_front_and_back() {
    assert_eq!(
      normalize_text(r#"Here is<b> bold </b>Text"#),
      r#"Here is <b>bold</b> Text"#
    );
  }

  #[test]
  fn leaves_spaces_as_is_in_good_b() {
    assert_eq!(
      normalize_text(r#"Here is <b>bold</b> Text"#),
      r#"Here is <b>bold</b> Text"#
    );
  }

  #[test]
  fn moves_spaces_out_of_i_to_front() {
    assert_eq!(
      normalize_text(r#"Here is<i> italic</i> Text"#),
      r#"Here is <i>italic</i> Text"#
    );
  }

  #[test]
  fn moves_spaces_out_of_i_to_back() {
    assert_eq!(
      normalize_text(r#"Here is <i>italic </i>Text"#),
      r#"Here is <i>italic</i> Text"#
    );
  }

  #[test]
  fn moves_spaces_out_of_i_to_front_and_back() {
    assert_eq!(
      normalize_text(r#"Here is<i> italic </i>Text"#),
      r#"Here is <i>italic</i> Text"#
    );
  }

  #[test]
  fn leaves_spaces_as_is_in_good_i() {
    assert_eq!(
      normalize_text(r#"Here is <i>italic</i> Text"#),
      r#"Here is <i>italic</i> Text"#
    );
  }

  #[test]
  fn moves_spaces_out_of_a_to_front() {
    assert_eq!(
      normalize_text(r#"Here is<a href="www.google.de"> Link</a> Text"#),
      r#"Here is <a href="www.google.de">Link</a> Text"#
    );
  }

  #[test]
  fn moves_spaces_out_of_a_to_back() {
    assert_eq!(
      normalize_text(r#"Here is <a href="www.google.de">Link </a>Text"#),
      r#"Here is <a href="www.google.de">Link</a> Text"#
    );
  }

  #[test]
  fn moves_spaces_out_of_a_to_front_and_back() {
    assert_eq!(
      normalize_text(r#"Here is<a href="www.google.de"> Link </a>Text"#),
      r#"Here is <a href="www.google.de">Link</a> Text"#
    );
  }

  #[test]
  fn leaves_spaces_as_is_in_good_a() {
    assert_eq!(
      normalize_text(r#"Here is <a href="www.google.de">Link</a> Text"#),
      r#"Here is <a href="www.google.de">Link</a> Text"#
    );
  }

  #[test]
  fn merges_multiple_new_lines_into_max_two() {
    assert_eq!(normalize_text("Some\n\n\n\nnew lines"), "Some\n\nnew lines");
  }
}
