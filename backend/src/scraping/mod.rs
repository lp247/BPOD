mod apod_data;
mod apod_thumbnail;
mod error;

pub use apod_data::get_apod_data;
pub use apod_thumbnail::get_apod_thumbnail;
pub use error::{ScrapeError, ScrapeResult};