mod apod;
mod scraping;

use chrono::{TimeZone, Utc};
use regex::Regex;
use reqwest::{
  header::{HeaderMap, HeaderValue},
  Client, Response,
};
use scraping::{get_apod_data, get_apod_thumbnail, ScrapeResult};
use std::{thread, time};
use tokio_postgres::NoTls;

pub struct APODRequestClient {
  client: Client,
}

impl APODRequestClient {
  fn new() -> APODRequestClient {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(
      "Accept-Language",
      HeaderValue::from_static("de,de-DE;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6"),
    );
    headers.insert(
      "Accept-Encoding",
      HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert("Accept", HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"));

    let client = Client::builder()
      .user_agent(
          "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36 Edg/89.0.774.57",
      )
      .default_headers(headers)
      .build()
      .unwrap();
    APODRequestClient { client }
  }
  async fn get(&self, url: &str) -> Response {
    let host = Regex::new("://(.+?)[/$]")
      .unwrap()
      .captures(url)
      .expect("Could not match URL")
      .get(1)
      .expect("Could not find host")
      .as_str();
    self
      .client
      .get(url)
      .header("Host", host)
      .send()
      .await
      .expect("Could not get resource")
  }
}

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> () {
  // Connect to the database.
  let (client, connection) = tokio_postgres::connect(
    "host=localhost user=postgres password=admin dbname=bpod",
    NoTls,
  )
  .await
  .unwrap();

  // The connection object performs the actual communication with the database,
  // so spawn it off to run on its own.
  tokio::spawn(async move {
    if let Err(e) = connection.await {
      eprintln!("connection error: {}", e);
    }
  });

  client
    .execute(
      "CREATE TABLE IF NOT EXISTS pictures (
                id SERIAL PRIMARY KEY,
                date DATE NOT NULL,
                img_url VARCHAR(2048) NOT NULL,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                meta TEXT NOT NULL
            );",
      &[],
    )
    .await
    .unwrap();

  let last_date = Utc.ymd(1996, 1, 1);
  let mut counter = Utc::today();
  let reqwest_client = APODRequestClient::new();

  while counter >= last_date {
    let date_str = format!("{}", counter.format("%Y-%m-%d"));
    let apod = match get_apod_data(date_str.as_str(), &reqwest_client).await {
      Ok(Some(apod)) => apod,
      Ok(None) => {
        counter = counter - chrono::Duration::days(1);
        continue;
      }
      Err(err) => panic!(err),
    };

    match get_apod_thumbnail(&apod, &reqwest_client).await {
      Ok(_) => (),
      Err(err) => println!("Could not get thumbnail: {}", err),
    }

    println!(
      // "Date: {}, Image URL: {}, Title: {}, Description: {}, Credit: {}, Image editor: {}, Text author: {}, Copyright: {}, License: {}",
      "Date: {}, Image URL: {}, Title: {}",
      apod.date,
      apod.img_url,
      apod.title,
      // apod.description,
    );
    // apod.save(client).await.unwrap();

    let half_sec = time::Duration::from_millis(500);
    thread::sleep(half_sec);

    counter = counter - chrono::Duration::days(1);
  }

  ()
}
