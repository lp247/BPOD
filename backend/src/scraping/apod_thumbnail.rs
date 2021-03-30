use super::error::{ScrapeError, ScrapeResult};
use crate::apod::APOD;
use crate::APODRequestClient;
use image::load_from_memory;
use regex::Regex;
use reqwest::Response;
use std::{path::Path, thread, time};

async fn get_apod_thumbnail_once(apod: &APOD, client: &APODRequestClient) -> ScrapeResult<()> {
  let image_url = if apod.img_url.starts_with("https://www.youtube.com/embed") {
    Regex::new(r#"https://www.youtube.com/embed/(.+?)(?:\?.*|$)"#)
      .map(|re| match re.captures(&apod.img_url) {
        Some(captures) => format!("https://img.youtube.com/vi/{}/0.jpg", &captures[1]),
        None => panic!("No Youtube ID found in {}!", apod.img_url),
      })
      .unwrap()
  } else if apod.img_url.starts_with("https://apod.nasa.gov/apod/image")
    && !apod.img_url.contains(".swf")
    && !apod.img_url.contains(".html")
  {
    String::from(&apod.img_url)
  } else {
    return Err(ScrapeError::ResourceUnsupported);
  };

  let response: Response = client.get(&image_url).await?;
  let img_bytes = response.bytes().await.map_err(|_| ScrapeError::Parsing)?;
  let img = load_from_memory(&img_bytes).map_err(|_| ScrapeError::Image)?;
  let thumbnail = img.resize_to_fill(250, 250, image::imageops::CatmullRom);
  let thumbnail_file_name = format!("{}.png", apod.date);
  let thumbnail_file_path = Path::new(&dirs::home_dir().unwrap())
    .join("bpod")
    .join(thumbnail_file_name);
  thumbnail
    .save_with_format(thumbnail_file_path, image::ImageFormat::Png)
    .map_err(|_| ScrapeError::FileSystem)?;

  Ok(())
}

pub async fn get_apod_thumbnail(apod: &APOD, client: &APODRequestClient) -> ScrapeResult<()> {
  let num_attempts: u64 = 5;
  for attempt in 0..num_attempts {
    if get_apod_thumbnail_once(&apod, &client).await.is_ok() {
      return Ok(());
    }
    let wait_time = time::Duration::from_secs((attempt + 1) * 2);
    thread::sleep(wait_time);
  }
  return Err(ScrapeError::Network);
}
