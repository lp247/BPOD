mod getter;
mod normalization;
mod translation;

use crate::apod::APOD;
use crate::APODRequestClient;
use getter::{get_description, get_img_url, get_meta, get_title};
use super::error::ScrapeResult;

pub async fn get_apod_data(date: &str, client: &APODRequestClient) -> ScrapeResult<Option<APOD>> {
  let year = &date[2..4];
  let month = &date[5..7];
  let day = &date[8..10];
  let url = format!("https://apod.nasa.gov/apod/ap{}{}{}.html", year, month, day);
  let sub_page = client
    .get(&url)
    .await
    .text()
    .await
    .expect("Could not get text");

  let description = get_description(&sub_page);
  let img_url = get_img_url(&sub_page);
  let title = get_title(&sub_page);
  let meta = get_meta(&sub_page);

  Ok(Some(APOD {
    id: None,
    date: String::from(date),
    img_url,
    title,
    description,
    meta,
  }))
}
