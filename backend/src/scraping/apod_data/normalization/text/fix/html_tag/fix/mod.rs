mod opening_a_tag;

use super::detect::Tag;
use opening_a_tag::fix_opening_a_tag;

pub fn fix_tag(tag: &str, tag_type: &Tag) -> String {
  match tag_type {
    Tag::Br => String::from("<br>"),
    Tag::OpeningB => String::from("<b>"),
    Tag::ClosingB => String::from("</b>"),
    Tag::OpeningI => String::from("<i>"),
    Tag::ClosingI => String::from("</i>"),
    Tag::OpeningA => fix_opening_a_tag(tag),
    Tag::ClosingA => String::from("</a>"),
    Tag::OpeningCenter => String::from("<center>"),
    Tag::ClosingCenter => String::from("</center>"),
    Tag::OpeningP => String::from("<p>"),
    Tag::ClosingP => String::from("</p>"),
  }
}
