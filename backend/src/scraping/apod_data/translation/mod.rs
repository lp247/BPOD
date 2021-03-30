use regex::{Captures, Regex};

pub fn html_to_markdown(html: &str) -> String {
  let br_translated: String = html.replace("<br>", "\n");

  let center_removed: String = Regex::new(r#"<center>(?P<content>[\s\S]+?)</center>"#)
    .unwrap()
    .replace_all(br_translated.as_str(), "$content")
    .into_owned();

  let italic_translated: String = Regex::new(r#"<i>(?P<content>[\s\S]+?)</i>"#)
    .unwrap()
    .replace_all(center_removed.as_str(), "**$content**")
    .into_owned();

  let bold_translated: String = Regex::new(r#"<b>(?P<content>[\s\S]+?)</b>"#)
    .unwrap()
    .replace_all(italic_translated.as_str(), "*$content*")
    .into_owned();

  let links_translated: String = Regex::new(r#"<a href="(?P<url>.+?)">(?P<text>[\s\S]+?)</a>"#)
    .unwrap()
    .replace_all(bold_translated.as_str(), |captures: &Captures| {
      let url = captures.name("url").unwrap().as_str();
      let text = captures.name("text").unwrap().as_str();
      format!("[{}]({})", text, url)
    })
    .into_owned();

  let artifacts_removed: String = Regex::new(r#"\s?(?:</a>|</b>)\s?"#)
    .unwrap()
    .replace_all(
      links_translated.as_str(),
      |captures: &regex::Captures| match captures
        .get(0)
        .expect("Could not get artifact")
        .as_str()
        .contains(" ")
      {
        true => " ",
        false => "",
      },
    )
    .into_owned();

  let trimmed = artifacts_removed.trim();

  let test_re = Regex::new(r"(<|>|^\s|\s$|\*\s*:|\*:[^\s]|\n{3,}| {2,})").unwrap();
  if test_re.is_match(&trimmed) {
    let captures = test_re.captures(&trimmed).unwrap();
    panic!(format!(
      "Text not translated successfully - Found '{}' in '{}'\nUnformatted input: {}",
      &captures[1], trimmed, html
    ));
  }
  String::from(trimmed)
}
