use chrono::{Duration, TimeZone, Utc};
use image::{imageops::CatmullRom, load_from_memory};
use regex::Regex;
use std::{fmt, fs, path::Path, thread, time};
use tokio_postgres::{Client, Error as PGError, NoTls};

type ScrapeResult<T> = std::result::Result<T, ScrapeError>;

#[derive(Debug, Clone)]
pub struct ScrapeError;

impl fmt::Display for ScrapeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error while scraping")
    }
}

pub struct APODListEntry {
    date: String,
    title: String,
    page_url: String,
}

pub struct APODInfo {
    id: Option<u32>,
    date: String,
    img_url: String,
    title: String,
    description: String,
    meta: String,
}

impl APODInfo {
    pub fn new(
        id: Option<u32>,
        date: String,
        img_url: String,
        title: String,
        description: String,
        meta: String,
    ) -> Self {
        Self {
            id,
            date,
            img_url,
            title,
            description,
            meta,
        }
    }
    pub async fn save(&self, client: Client) -> Result<u64, PGError> {
        match self.id {
            Some(_) => self.update(client).await,
            None => self.create(client).await,
        }
    }
    async fn update(&self, client: Client) -> Result<u64, PGError> {
        let stmt = format!(
            "UPDATE pictures SET date = '{}', url = '{}', title = '{}', description = '{}', meta = '{}' WHERE id = {};",
            self.date, self.img_url, self.title.replace("'", "''"), self.description.replace("'", "''"), self.meta.replace("'", "''"), self.id.unwrap()
        );
        client.execute(stmt.as_str(), &[]).await
    }
    async fn create(&self, client: Client) -> Result<u64, PGError> {
        let stmt = format!(
            "INSERT INTO pictures (date, url, title, description, meta) VALUES ('{}', '{}', '{}', '{}', '{}');",
            self.date, self.img_url, self.title.replace("'", "''"), self.description.replace("'", "''"), self.meta.replace("'", "''")
        );
        client.execute(stmt.as_str(), &[]).await
    }
}

pub struct APODList<'a> {
    list_entries: Vec<APODListEntry>,
    url_root: &'a str,
    client: reqwest::Client,
}

impl<'a> APODList<'a> {
    pub fn new() -> APODList<'a> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Accept-Language",
            reqwest::header::HeaderValue::from_static(
                "de,de-DE;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6",
            ),
        );
        headers.insert(
            "Accept-Encoding",
            reqwest::header::HeaderValue::from_static("gzip, deflate, br"),
        );
        headers.insert("Accept", reqwest::header::HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"));

        let client = reqwest::Client::builder()
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36 Edg/89.0.774.57",
            )
            .default_headers(headers)
            .build()
            .unwrap();

        APODList {
            list_entries: vec![],
            url_root: "https://apod.nasa.gov/apod/",
            client,
        }
    }

    pub async fn get_apod(&mut self, date: &str) -> ScrapeResult<Option<APODInfo>> {
        if self.list_entries.len() == 0 {
            self.retrieve_list().await.unwrap();
        }
        let target_list_entry = match self.list_entries.iter().find(|&e| e.date == date) {
            Some(value) => value,
            None => return Ok(None),
        };
        let sub_page = reqwest::get(format!("{}{}", self.url_root, target_list_entry.page_url))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let description = get_description(&sub_page, self.url_root);
        let img_url = get_img_url(&sub_page, self.url_root);
        let title = get_title(&sub_page, self.url_root);
        let meta = get_meta(&sub_page, self.url_root);

        let num_attempts: u64 = 5;
        for attempt in 0..num_attempts {
            match self.store_thumbnail(&img_url, date).await {
                Ok(_) => break,
                Err(_) => match attempt == num_attempts - 1 {
                    true => return Err(ScrapeError),
                    false => {
                        let wait_time = time::Duration::from_secs((attempt + 1) * 2);
                        thread::sleep(wait_time);
                        continue;
                    }
                },
            }
        }

        Ok(Some(APODInfo {
            id: None,
            date: String::from(date),
            img_url,
            title,
            description,
            meta,
        }))
    }

    async fn retrieve_list(&mut self) -> ScrapeResult<()> {
        let list_page = reqwest::get(format!("{}archivepixFull.html", self.url_root))
            .await
            .map_err(|_| ScrapeError)?
            .text()
            .await
            .map_err(|_| ScrapeError)?;
        let entry_re = Regex::new(r#"(.+?):\s+<a href="(.+?)">(.+?)</a><br>"#).unwrap();
        let year_re = Regex::new(r#"(\d{4})"#).unwrap();
        let month_re = Regex::new(r#"([a-zA-Z]+)"#).unwrap();
        let day_re = Regex::new(r#"\s(\d{2})"#).unwrap();
        for capture in entry_re.captures_iter(&list_page) {
            let unformatted_date = &capture[1];
            let year_capture = year_re.captures(&unformatted_date).unwrap();
            let month_capture = month_re.captures(&unformatted_date).unwrap();
            let day_capture = day_re.captures(&unformatted_date).unwrap();
            let year = &year_capture[1];
            let day = &day_capture[1];
            let month = match &month_capture[1] {
                "January" => "01",
                "February" => "02",
                "March" => "03",
                "April" => "04",
                "May" => "05",
                "June" => "06",
                "July" => "07",
                "August" => "08",
                "September" => "09",
                "October" => "10",
                "November" => "11",
                "December" => "12",
                _ => panic!("Bad Month name"),
            };
            let date = format!("{}-{}-{}", year, month, day);
            let sub_page_url = String::from(&capture[2]);
            let title = String::from(&capture[3]);
            self.list_entries.push(APODListEntry {
                date,
                title,
                page_url: sub_page_url,
            });
        }
        Ok(())
    }

    async fn store_thumbnail(&self, img_url: &str, date: &str) -> ScrapeResult<()> {
        let (image_url, host) = if img_url.starts_with("https://www.youtube.com/embed") {
            Regex::new(r#"https://www.youtube.com/embed/(.+?)(?:\?.*|$)"#)
                .map(|re| match re.captures(&img_url) {
                    Some(captures) => (
                        format!("https://img.youtube.com/vi/{}/0.jpg", &captures[1]),
                        String::from("img.youtube.com"),
                    ),
                    None => panic!("No Youtube ID found in {}!", img_url),
                })
                .unwrap()
        } else if img_url.starts_with("https://apod.nasa.gov/apod/image")
            && !img_url.contains(".swf")
            && !img_url.contains(".html")
        {
            (String::from(img_url), String::from("apod.nasa.gov"))
        } else {
            return Ok(());
        };

        let response: reqwest::Response = self
            .client
            .get(&image_url)
            .header("Host", host)
            .header("Accept", "image/jpeg")
            .send()
            .await
            .map_err(|_| ScrapeError)?;

        let img_bytes = response.bytes().await.map_err(|_| ScrapeError)?;
        let img = load_from_memory(&img_bytes).unwrap();
        let thumbnail = img.resize_to_fill(250, 250, image::imageops::CatmullRom);
        let thumbnail_file_name = format!("{}.png", date);
        let thumbnail_file_path = Path::new(&dirs::home_dir().unwrap())
            .join("bpod")
            .join(thumbnail_file_name);
        thumbnail
            .save_with_format(thumbnail_file_path, image::ImageFormat::Png)
            .unwrap();
        Ok(())
    }
}

fn get_description(page: &str, url_root: &str) -> String {
    let regex =
        Regex::new(r#"<.+?>\s*Explanation:\s*<.+?>\s*(?P<explanation>[\s\S]+?)\s*<p>"#).unwrap();
    let raw_description = regex
        .captures(page)
        .unwrap()
        .name("explanation")
        .unwrap()
        .as_str();
    normalize_text(raw_description, url_root)
}

fn get_img_url(page: &str, url_root: &str) -> String {
    // TODO: Get image source not from image tag but from enclosing link
    let regex = Regex::new(
        r#"<(?:IMG SRC|img src|iframe[\s\S]+?src|object[\s\S]+?data)=["'](?P<url>.+?)["']"#,
    )
    .unwrap();
    let captures = regex.captures(page).expect("Could not find image source");
    let url = captures
        .name("url")
        .expect("URL not found in image source")
        .as_str();
    match url.starts_with("http") {
        true => String::from(url),
        false => format!("{}{}", url_root, url),
    }
}

fn get_meta_block(page: &str) -> &str {
    let full_meta_block = Regex::new(r"<center>[\s\S]+?</center>")
        .expect("Regex for full meta block invalid")
        .find_iter(page)
        .nth(1)
        .expect("Could not find meta block")
        .as_str();
    Regex::new(r"<center>\s*(?P<content>\S[\s\S]+?\S)\s*</center>")
        .expect("Regex for meta block content invalid")
        .captures(full_meta_block)
        .expect("Could not find meta block content")
        .name("content")
        .expect("Could not get meta block content")
        .as_str()
}

fn get_additional_meta_block(page: &str) -> &str {
    let meta_block = get_meta_block(page);
    Regex::new(r"<[^>]+?>[\s\S]+?</[^>]+?>\s*(?:<br>)?\s*(?P<amb>[\s\S]+)")
        .expect("Regex for additional meta block invalid")
        .captures(meta_block)
        .expect("Could not find additional meta block content")
        .name("amb")
        .expect("Could not get meta block content")
        .as_str()
}

fn get_title(page: &str, url_root: &str) -> String {
    let meta_block = get_meta_block(page);
    let regex =
        Regex::new(r"<[^>]+?>\s*(\S[\s\S]+?\S)\s*</[^>]+?>").expect("Regex for title invalid");
    let raw_title = regex
        .find_iter(meta_block)
        .nth(0)
        .expect("Could not find title")
        .as_str();
    normalize_text(raw_title, url_root).replace("*", "")
}

fn get_meta(page: &str, url_root: &str) -> String {
    normalize_text(get_additional_meta_block(page), url_root).replace("*", "")
}

fn normalize_text(text: &str, url_root: &str) -> String {
    let space_fixed: String = Regex::new(r"(?:\s|<br>|</br>)+")
        .unwrap()
        .replace_all(text, " ")
        .into_owned();
    let bad_link_closing_tag_fixed = Regex::new(r"(?:<\?=/a>|<a/>|</a/>|</A>)")
        .unwrap()
        .replace_all(space_fixed.as_str(), "</a>")
        .into_owned();
    let missing_closing_link_tag_fixed =
        Regex::new(r"(?P<first_tag><[aA][^>]+?>)(?P<content>[^>]+?)(?P<second_tag><[aA])")
            .unwrap()
            .replace_all(
                bad_link_closing_tag_fixed.as_str(),
                "${first_tag}${content}</a>${second_tag}",
            )
            .into_owned();
    let i_fixed: String = Regex::new(r"<[iI]>\s*(?P<content>[\S\s]+?)\s*</[iI]>")
        .unwrap()
        .replace_all(missing_closing_link_tag_fixed.as_str(), "**$content**")
        .into_owned();
    let b_fixed: String = Regex::new(r"<[bB]>\s*(?P<content>[\S\s]+?)\s*</[bB]>")
        .unwrap()
            .replace_all(i_fixed.as_str(), "*$content*")
        .into_owned();
    let colon_order_fixed: String = Regex::new(r"(?P<stars>\*+):")
        .unwrap()
        .replace_all(b_fixed.as_str(), ":$stars")
        .into_owned();
    let mailto_fixed = Regex::new(r#"mailto:[\s\S]+""#)
        .unwrap()
        .replace_all(colon_order_fixed.as_str(), |captures: &regex::Captures| {
            captures
                .get(0)
                .unwrap()
                .as_str()
                .replace(" ", "")
                .replace("@at@", "@")
                .replace("[at]", "@")
                .replace(".dot.", ".")
                .replace("[dot]", ".")
                .replace(".d.o.t.", ".")
        })
        .into_owned();
    let href_attr_regex = r"(?:ref|href|rhef|hre|hef|hrf|HREF)";
    let link_text_regex = r"(?P<text>.+?)";
    let link_url_regex = r"(?P<url>\S+?)";
    let link_regex = format!(
        r#"<[aA]\s*{href_attr}\s*=\s*"?{link_url}"?(?:>|\s>|\s.*?>|</a>){link_text}</a>"#,
        href_attr = href_attr_regex,
        link_text = link_text_regex,
        link_url = link_url_regex,
    );
    let link_fixed: String = Regex::new(link_regex.as_str())
        .unwrap()
        .replace_all(mailto_fixed.as_str(), |captures: &regex::Captures| {
            let url = captures.name("url").unwrap().as_str();
            let text = captures.name("text").unwrap().as_str();
            match url.starts_with("http") || url.starts_with("mailto:") {
                true => format!("[{}]({})", text, url),
                false => format!("[{}]({}{})", text, url_root, url),
            }
        })
        .into_owned();

    let artifacts_fixed: String = Regex::new(r#"\s?(?:</a>|</b>)\s?"#)
        .unwrap()
        .replace_all(
            link_fixed.as_str(),
            |captures: &regex::Captures| match captures
                .get(0)
                .expect("Could not get artifact")
                .as_str()
                .contains(" ")
            {
                true => " ",
                false => "",
            },
        )
        .into_owned();

    let trimmed = artifacts_fixed.trim();

    let test_re = Regex::new(r"(<|>|\s{2,}|^\s|\s$|mailto:\)|\*\s*:|:$)").unwrap();
    if test_re.is_match(&trimmed) {
        let captures = test_re.captures(&trimmed).unwrap();
        panic!(format!(
            "Text not fixed completely - Found {} in {}\nUnformatted input: {}",
            &captures[1], trimmed, text
        ));
    }
    String::from(trimmed)
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
    let mut apod_list = APODList::new();

    while counter >= last_date {
        let date_str = format!("{}", counter.format("%Y-%m-%d"));
        let apod = match apod_list.get_apod(date_str.as_str()).await {
            Ok(Some(apod)) => apod,
            Ok(None) => {
                counter = counter - Duration::days(1);
                continue;
            }
            Err(err) => panic!(err),
        };
        println!(
            // "Date: {}, Image URL: {}, Title: {}, Description: {}, Credit: {}, Image editor: {}, Text author: {}, Copyright: {}, License: {}",
            "Date: {}, Image URL: {}, Title: {}\n    Meta: {}",
            apod.date,
            apod.img_url,
            apod.title,
            apod.meta,
            // apod.description,
        );
        // apod.save(client).await.unwrap();

        let half_sec = time::Duration::from_millis(500);
        thread::sleep(half_sec);

        counter = counter - Duration::days(1);
    }

    ()
}
