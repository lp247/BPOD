use regex::{Captures, Regex};

pub fn html_to_markdown(html: &str) -> String {
  let br_translated = Regex::new(r" ?<br> ?").unwrap().replace_all(html, "");

  let center_removed = Regex::new(r#"<center>(?P<content>[\s\S]+?)</center>"#)
    .unwrap()
    .replace_all(&br_translated, "$content");

  let italic_translated = Regex::new(r#"<i>(?P<content>[\s\S]+?)</i>"#)
    .unwrap()
    .replace_all(&center_removed, "**$content**");

  let bold_translated = Regex::new(r#"<b>(?P<content>[\s\S]+?)</b>"#)
    .unwrap()
    .replace_all(&italic_translated, "*$content*");

  let links_translated = Regex::new(r#"<a href="(?P<url>.+?)">(?P<text>[\s\S]+?)</a>"#)
    .unwrap()
    .replace_all(&bold_translated, |captures: &Captures| {
      let url = captures.name("url").unwrap().as_str();
      let text = captures.name("text").unwrap().as_str();
      format!("[{}]({})", text, url)
    });

  let artifacts_removed = Regex::new(r#"\s?(?:</a>|</b>)\s?"#).unwrap().replace_all(
    &links_translated,
    |captures: &regex::Captures| match captures
      .get(0)
      .expect("Could not get artifact")
      .as_str()
      .contains(" ")
    {
      true => " ",
      false => "",
    },
  );

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
