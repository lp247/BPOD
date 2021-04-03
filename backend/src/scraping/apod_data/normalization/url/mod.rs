mod check;

use check::is_rel_website_url;

pub fn normalize_url(url: &str) -> String {
    // TODO move fixes here to own module
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
    } else if url_without_invalid_chars.starts_with("//") {
        format!("https:{}", url_without_invalid_chars)
    } else if is_rel_website_url(&url_without_invalid_chars) {
        format!("https://apod.nasa.gov/apod/{}", url_without_invalid_chars)
    } else {
        url_without_invalid_chars
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

    #[test]
    fn adds_missing_protocol() {
        assert_eq!(normalize_url("//www.google.de"), "https://www.google.de")
    }
}
