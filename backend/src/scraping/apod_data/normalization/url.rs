use regex::Regex;

pub fn normalize_url(url: &str) -> String {
  let url_without_new_line = url.replace("\n", "");
  let url_without_space = url_without_new_line.replace(" ", "");
  let top_level_domain = r"[a-zA-Z]{2,}";
  let domain_label = r"(?:[a-zA-Z0-9][a-zA-Z0-9\-]+?[a-zA-Z0-9]|[a-zA-Z0-9]{1,2})";
  if url_without_space.starts_with("mailto:") {
    url_without_space
      .replace("@at@", "@")
      .replace("[at]", "@")
      .replace(".dot.", ".")
      .replace("[dot]", ".")
      .replace(".d.o.t.", ".")
  } else {
    let valid_full_url_regex = format!(
      r"^(?:https?://)?(?:{}\.)+{}",
      domain_label, top_level_domain
    );
    match Regex::new(valid_full_url_regex.as_str())
      .unwrap()
      .is_match(&url_without_space)
    {
      true => String::from(url_without_space),
      false => format!("https://apod.nasa.gov/apod/{}", url_without_space),
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
