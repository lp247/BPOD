use regex::Regex;

pub fn is_rel_website_url(url: &str) -> bool {
    if Regex::new(r"^https?://").unwrap().is_match(url) {
        return false;
    }

    let top_level_domain = r"[a-zA-Z]{2,}";
    let domain_label = r"(?:[a-zA-Z0-9][a-zA-Z0-9\-]+?[a-zA-Z0-9]|[a-zA-Z0-9]{1,2})";
    let valid_full_url_regex = format!(r"^(?:{}\.)+(?P<tld>{})", domain_label, top_level_domain);

    match Regex::new(&valid_full_url_regex).unwrap().captures(url) {
        Some(cap) => match cap.name("tld").unwrap().as_str() {
            "html" | "jpg" | "jpeg" | "png" | "swf" | "gif" | "htm" | "asp" | "php" => true,
            _ => false,
        },
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_absolute_urls() {
        assert!(!is_rel_website_url("http://www.google.de"));
        assert!(!is_rel_website_url("www.google.de"));
        assert!(!is_rel_website_url("google.de"));
        assert!(!is_rel_website_url(
            "http://www.google.de/image/1905/TotnAfter_Dai_3000.jpg"
        ));
        assert!(!is_rel_website_url("www.google.de/ap160630.html"));
        assert!(!is_rel_website_url("google.de/images/test.jpeg"));
    }

    #[test]
    fn detects_relative_urls() {
        assert!(is_rel_website_url("image/1905/TotnAfter_Dai_3000.jpg"));
        assert!(is_rel_website_url("ap160630.html"));
        assert!(is_rel_website_url("images/test.jpeg"));
    }
}
