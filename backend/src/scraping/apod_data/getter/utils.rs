use regex::Regex;

pub fn get_title_meta_block(page: &str) -> &str {
  let full_title_meta_block = Regex::new(r"<center>[\s\S]+?</center>")
    .expect("Regex for full meta block invalid")
    .find_iter(page)
    .nth(1)
    .expect("Could not find meta block")
    .as_str();
  Regex::new(r"<center>\s*(?P<content>\S[\s\S]+?\S)\s*</center>")
    .expect("Regex for meta block content invalid")
    .captures(full_title_meta_block)
    .expect("Could not find meta block content")
    .name("content")
    .expect("Could not get meta block content")
    .as_str()
}
