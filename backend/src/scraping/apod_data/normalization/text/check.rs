use crate::scraping::{ScrapeError, ScrapeResult};
use regex::Regex;

pub fn check_html(html: &str) -> ScrapeResult<()> {
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
