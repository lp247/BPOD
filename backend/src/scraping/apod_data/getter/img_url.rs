use super::super::normalization::normalize_url;
use regex::Regex;

pub fn get_img_url(page: &str) -> String {
    // TODO: Get image source not from image tag but from enclosing link
    let regex =
    Regex::new(r#"<(?:IMG[\s\S]+?SRC|img[\s\S]+?src|iframe[\s\S]+?src|object[\s\S]+?data|param name="movie" value)=["'](?P<url>.+?)["']"#)
      .unwrap();
    let captures = regex.captures(page).expect("Could not find image source");
    let url = captures
        .name("url")
        .expect("URL not found in image source")
        .as_str();
    normalize_url(url)
}
