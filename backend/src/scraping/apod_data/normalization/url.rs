use regex::Regex;

pub fn normalize_url(url: &str) -> String {
  let url_without_invalid_chars = url
    .replace("\n", "")
    .replace(" ", "")
    .replace("\t", "")
    .replace(">", "")
    .replace("<", "");
  if url_without_invalid_chars.starts_with("mailto:") {
    url_without_invalid_chars
      .replace("@at@", "@")
      .replace("[at]", "@")
      .replace(".dot.", ".")
      .replace("[dot]", ".")
      .replace(".d.o.t.", ".")
  } else {
    let top_level_domain = r"[a-zA-Z]{2,}";
    let domain_label = r"(?:[a-zA-Z0-9][a-zA-Z0-9\-]+?[a-zA-Z0-9]|[a-zA-Z0-9]{1,2})";
    let valid_full_url_regex = format!(
      r"^(?:https?://)?(?:{}\.)+{}",
      domain_label, top_level_domain
    );
    match Regex::new(valid_full_url_regex.as_str())
      .unwrap()
      .is_match(&url_without_invalid_chars)
    {
      true => String::from(url_without_invalid_chars),
      false => format!("https://apod.nasa.gov/apod/{}", url_without_invalid_chars),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn removes_whitespace() {
    assert_eq!(
      normalize_url("http://www.google .de"),
      "http://www.google.de"
    );
  }

  #[test]
  fn removes_new_line() {
    assert_eq!(
      normalize_url("http://www.google\n.de"),
      "http://www.google.de"
    );
  }

  #[test]
  fn removes_tab() {
    assert_eq!(
      normalize_url("http://www.google\t.de"),
      "http://www.google.de"
    );
  }

  #[test]
  fn removes_caret() {
    assert_eq!(
      normalize_url("http://www.google.de>"),
      "http://www.google.de"
    );
    assert_eq!(
      normalize_url("http://www.google.de<"),
      "http://www.google.de"
    );
  }

  #[test]
  fn changes_relative_to_absolute_url() {
    assert_eq!(
      normalize_url("images/test.jpeg"),
      "https://apod.nasa.gov/apod/images/test.jpeg"
    )
  }

  #[test]
  fn fixes_at_in_mailto() {
    assert_eq!(
      normalize_url("mailto:me@at@server.com"),
      "mailto:me@server.com"
    );
    assert_eq!(
      normalize_url("mailto:me[at]server.com"),
      "mailto:me@server.com"
    );
  }

  #[test]
  fn fixes_dot_in_mailto() {
    assert_eq!(
      normalize_url("mailto:me@server.dot.com"),
      "mailto:me@server.com"
    );
    assert_eq!(
      normalize_url("mailto:me@server[dot]com"),
      "mailto:me@server.com"
    );
    assert_eq!(
      normalize_url("mailto:me@server.d.o.t.com"),
      "mailto:me@server.com"
    );
  }
}
