use std::fmt::{Display, Formatter, Result as FmtResult};

pub type ScrapeResult<T> = std::result::Result<T, ScrapeError>;

#[derive(Debug, Clone)]
pub enum ScrapeError {
  NotFound,
  Parsing,
  ResourceUnsupported,
  Dummy,
  FileSystem,
  Image,
  HTMLFixing(String),
}

impl Display for ScrapeError {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    let text = match self {
      NotFound => "ScrapeError: Not found",
      Parsing => "ScrapeError: Parsing",
      ResourceUnsupported => "ScrapeError: Resource unsupported",
      Dummy => "ScrapeError: Dummy",
      FileSystem => "ScrapeError: File system",
      Image => "ScrapeError: Image",
      HTMLFixing => "ScrapeError: HTML fixing",
    };
    write!(f, "{}", text)
  }
}
