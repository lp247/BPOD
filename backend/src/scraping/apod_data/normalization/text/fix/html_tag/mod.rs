mod check;
mod detect;
mod fix;

use crate::scraping::ScrapeResult;
use check::tag_syntax_is_valid;
use detect::detect_tag;
use fix::fix_tag;

pub fn normalize_html_tag(tag: &str) -> ScrapeResult<String> {
  if tag_syntax_is_valid(tag) {
    return Ok(String::from(tag));
  }

  let tag_type = detect_tag(tag)?;
  Ok(fix_tag(tag, &tag_type))
}
