use crate::scraping::{ScrapeError, ScrapeResult};
use regex::Regex;

#[derive(PartialEq)]
pub enum Tag {
  Br,
  OpeningA,
  ClosingA,
  OpeningB,
  ClosingB,
  OpeningI,
  ClosingI,
  OpeningCenter,
  ClosingCenter,
}

pub fn detect_tag(tag: &str) -> ScrapeResult<Tag> {
  if Regex::new(r"^<[aA](?:\s|href)").unwrap().is_match(tag) {
    return Ok(Tag::OpeningA);
  }

  let tag_includes_slash = tag.contains("/");
  let tag_name = Regex::new(r"[^a-zA-Z]")
    .unwrap()
    .replace_all(tag, "")
    .to_lowercase();

  if tag_includes_slash {
    match tag_name.as_str() {
      "br" => Ok(Tag::Br),
      "i" => Ok(Tag::ClosingI),
      "b" => Ok(Tag::ClosingB),
      "a" => Ok(Tag::ClosingA),
      "center" => Ok(Tag::ClosingCenter),
      _ => Err(ScrapeError::HTMLFixing(format!(
        "Could not detect tag {}",
        tag
      ))),
    }
  } else {
    match tag_name.as_str() {
      "br" => Ok(Tag::Br),
      "i" => Ok(Tag::OpeningI),
      "b" => Ok(Tag::OpeningB),
      "center" => Ok(Tag::OpeningCenter),
      _ => Err(ScrapeError::HTMLFixing(format!(
        "Could not detect tag {}",
        tag
      ))),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn detects_br_tags() {
    assert_eq!(detect_tag("<br>").unwrap() == Tag::Br, true);
    assert_eq!(detect_tag("</br>").unwrap() == Tag::Br, true);
    assert_eq!(detect_tag("<br/>").unwrap() == Tag::Br, true);
    assert_eq!(detect_tag("<br />").unwrap() == Tag::Br, true);
    assert_eq!(detect_tag("<BR>").unwrap() == Tag::Br, true);
  }

  #[test]
  fn detects_opening_i_tags() {
    assert_eq!(detect_tag("<i>").unwrap() == Tag::OpeningI, true);
    assert_eq!(detect_tag("<I>").unwrap() == Tag::OpeningI, true);
  }

  #[test]
  fn detects_closing_i_tags() {
    assert_eq!(detect_tag("</i>").unwrap() == Tag::ClosingI, true);
    assert_eq!(detect_tag("</I>").unwrap() == Tag::ClosingI, true);
    assert_eq!(detect_tag("<i/>").unwrap() == Tag::ClosingI, true);
    assert_eq!(detect_tag("<I/>").unwrap() == Tag::ClosingI, true);
  }

  #[test]
  fn detects_opening_b_tags() {
    assert_eq!(detect_tag("<b>").unwrap() == Tag::OpeningB, true);
    assert_eq!(detect_tag("<B>").unwrap() == Tag::OpeningB, true);
  }

  #[test]
  fn detects_closing_b_tags() {
    assert_eq!(detect_tag("</b>").unwrap() == Tag::ClosingB, true);
    assert_eq!(detect_tag("</B>").unwrap() == Tag::ClosingB, true);
    assert_eq!(detect_tag("<b/>").unwrap() == Tag::ClosingB, true);
    assert_eq!(detect_tag("<B/>").unwrap() == Tag::ClosingB, true);
  }

  #[test]
  fn detects_opening_center_tags() {
    assert_eq!(detect_tag("<center>").unwrap() == Tag::OpeningCenter, true);
    assert_eq!(detect_tag("<CENTER>").unwrap() == Tag::OpeningCenter, true);
  }

  #[test]
  fn detects_closing_center_tags() {
    assert_eq!(detect_tag("</center>").unwrap() == Tag::ClosingCenter, true);
    assert_eq!(detect_tag("</CENTER>").unwrap() == Tag::ClosingCenter, true);
    assert_eq!(detect_tag("<center/>").unwrap() == Tag::ClosingCenter, true);
    assert_eq!(detect_tag("<CENTER/>").unwrap() == Tag::ClosingCenter, true);
  }

  #[test]
  fn detects_opening_a_tags() {
    assert_eq!(
      detect_tag("<a href=\"www.google.de\">").unwrap() == Tag::OpeningA,
      true
    );
    assert_eq!(
      detect_tag("<ahref=\"www.google.de\">").unwrap() == Tag::OpeningA,
      true
    );
    assert_eq!(
      detect_tag("<a hrf=\"www.google.de\">").unwrap() == Tag::OpeningA,
      true
    );
    assert_eq!(
      detect_tag("<a HREF=\"www.google.de\">").unwrap() == Tag::OpeningA,
      true
    );
    assert_eq!(
      detect_tag("<A href=\"www.google.de\">").unwrap() == Tag::OpeningA,
      true
    );
    assert_eq!(
      detect_tag("<a href=www.google.de>").unwrap() == Tag::OpeningA,
      true
    );
    assert_eq!(
      detect_tag("<a hef=\"www.google.de\">").unwrap() == Tag::OpeningA,
      true
    );
    assert_eq!(
      detect_tag("<a href=\"www.google.de>\">").unwrap() == Tag::OpeningA,
      true
    );
    assert_eq!(
      detect_tag("<a href=\"www.google.de\" >").unwrap() == Tag::OpeningA,
      true
    );
    assert_eq!(
      detect_tag("<a href=\"image/1905/TotnBefore_Dai_3000.jpg\"</a>").unwrap() == Tag::OpeningA,
      true
    );
  }

  #[test]
  fn detects_closing_a_tags() {
    assert_eq!(detect_tag("</a>").unwrap() == Tag::ClosingA, true);
    assert_eq!(detect_tag("</A>").unwrap() == Tag::ClosingA, true);
    assert_eq!(detect_tag("<a/>").unwrap() == Tag::ClosingA, true);
    assert_eq!(detect_tag("<A/>").unwrap() == Tag::ClosingA, true);
    assert_eq!(detect_tag("<?=/a>").unwrap() == Tag::ClosingA, true);
  }
}
