use crate::scraping::ScrapeResult;
use regex::Regex;

pub fn replace_matches(
    text: &str,
    regex: &str,
    replace_fn: impl Fn(&str) -> ScrapeResult<String>,
) -> ScrapeResult<String> {
    let match_re = Regex::new(regex).unwrap();
    let matches = match_re.find_iter(&text);

    let mut result = String::new();
    let mut last_end: usize = 0;
    let text_end_index: usize = text.len();
    for matched in matches {
        let start = matched.start();
        let end = matched.end();

        if start > last_end {
            result += &text[last_end..start];
        }

        let replaced = replace_fn(&text[start..end])?;
        result += &replaced;

        last_end = end;
    }

    if last_end < text_end_index {
        result += &text[last_end..text_end_index];
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scraping::ScrapeResult;
    use pretty_assertions::assert_eq;

    fn replacer(_: &str) -> ScrapeResult<String> {
        Ok(String::from("<num>"))
    }

    #[test]
    fn works() {
        let text = "Some 0 random 891 Text with 46 a couple 3 of 1953043 numbers";
        let reference = "Some <num> random <num> Text with <num> a couple <num> of <num> numbers";
        let re = r"\d+";
        assert_eq!(replace_matches(text, re, replacer).unwrap(), reference);
    }
}
