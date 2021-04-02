pub const TAG_REGEX: &str = r#"<(?:[^"]|".*?")+?"?>"#;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use regex::Regex;

    fn helper(text: &str, compare: &[&str]) {
        let re = Regex::new(TAG_REGEX).unwrap();
        assert_eq!(re.find_iter(text).count(), compare.len());
        for (i, m) in re.find_iter(text).enumerate() {
            assert_eq!(m.as_str(), compare[i]);
        }
    }

    #[test]
    fn tag_regex_finds_tags() {
        helper("Text <i> Text", &["<i>"]);
        helper("Text <i>Content</i> Text", &["<i>", "</i>"]);
        helper(
            "Text <a href=\"www.google.de\">Link</a> Text",
            &["<a href=\"www.google.de\">", "</a>"],
        );
        helper(
            "Text <ahref=\"www.google.de\">Link</a> Text",
            &["<ahref=\"www.google.de\">", "</a>"],
        );
        helper(
            "Text <a href=\"www.google.de>\">Link</a> Text",
            &["<a href=\"www.google.de>\">", "</a>"],
        );
        helper(
            "Text <a href=\"www.google.de\" >Link</a> Text",
            &["<a href=\"www.google.de\" >", "</a>"],
        );
        helper("Text <?=/a> Text", &["<?=/a>"]);
        helper(
            "Text <a href=\"image/1905/TotnBefore_Dai_3000.jpg\"</a> Text",
            &["<a href=\"image/1905/TotnBefore_Dai_3000.jpg\"</a>"],
        );
        helper("Text <a href=\"\"> Text", &["<a href=\"\">"]);
        helper("Text <p>Content</p> Text", &["<p>", "</p>"]);
        helper(
            "Text <a h ref=\"www.google.de\"> Text",
            &["<a h ref=\"www.google.de\">"],
        );
        helper(
            "Text <a href=www.google.de\"> Text",
            &["<a href=www.google.de\">"],
        );
        helper(
            "Text <a href\"www.google.de\"> Text",
            &["<a href\"www.google.de\">"],
        );
        helper(
            "Text <la href\"www.google.de\"> Text",
            &["<la href\"www.google.de\">"],
        );
        helper(
      "Text <href=\"http://cosmicdiary.org/fpatat/2009/01/19/ x-shooter-goes-on-sky-again-and-again-nights-2-3-and-4/\"> Text",
      &["<href=\"http://cosmicdiary.org/fpatat/2009/01/19/ x-shooter-goes-on-sky-again-and-again-nights-2-3-and-4/\">"],
    );
    }
}
