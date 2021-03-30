use std::fmt::{Display, Formatter, Result as FmtResult};

pub type ScrapeResult<T> = std::result::Result<T, ScrapeError>;

#[derive(Debug, Clone)]
pub enum ScrapeError {
  Parsing,
  ResourceUnsupported,
  Dummy,
  FileSystem,
  Image,
  HTMLFixing(String),
  Network,
}

impl Display for ScrapeError {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    match &*self {
      ScrapeError::Parsing => write!(f, "Parsing failed"),
      ScrapeError::ResourceUnsupported => write!(f, "The resource is unsupported"),
      ScrapeError::Dummy => write!(f, "THIS ERROR SHOULD NOT BE THROWN"),
      ScrapeError::FileSystem => write!(f, "Could not save or load file"),
      ScrapeError::Image => write!(f, "Could not load image"),
      ScrapeError::HTMLFixing(err_string) => {
        write!(f, "HTML fixing was unsuccessful ({})", err_string)
      }
      ScrapeError::Network => write!(f, "The network resource could not be retrieved"),
    }
  }
}
