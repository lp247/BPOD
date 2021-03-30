mod getter;
mod normalization;
mod translation;

use super::error::ScrapeResult;
use crate::apod::APOD;
use crate::APODRequestClient;
use getter::{get_description, get_img_url, get_meta, get_title};

pub async fn get_apod_data(date: &str, client: &APODRequestClient) -> ScrapeResult<Option<APOD>> {
  let year = &date[2..4];
  let month = &date[5..7];
  let day = &date[8..10];
  let url = format!("https://apod.nasa.gov/apod/ap{}{}{}.html", year, month, day);
  let page_response = client.get(&url).await?;
  if page_response.status().as_u16() == 404 {
    return Ok(None);
  }
  let page = page_response.text().await.expect("Could not get text");

  let description = get_description(&page);
  let img_url = get_img_url(&page);
  let title = get_title(&page);
  let meta = get_meta(&page);

  Ok(Some(APOD {
    id: None,
    date: String::from(date),
    img_url,
    title,
    description,
    meta,
  }))
}
