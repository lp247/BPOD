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
    credit: String,
    img_editor: String,
    text_author: String,
    copyright: String,
    license: String,
}

impl APODInfo {
    pub fn new(
        id: Option<u32>,
        date: String,
        img_url: String,
        title: String,
        credit: String,
        description: String,
        copyright: String,
        license: String,
        text_author: String,
        img_editor: String,
    ) -> Self {
        Self {
            id,
            date,
            img_url,
            title,
            credit,
            description,
            copyright,
            license,
            img_editor,
            text_author,
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
            "UPDATE pictures SET date = '{}', url = '{}', title = '{}', credit = '{}', description = '{}' WHERE id = {};",
            self.date, self.img_url, self.title.replace("'", "''"), self.credit.replace("'", "''"), self.description.replace("'", "''"), self.id.unwrap()
        );
        client.execute(stmt.as_str(), &[]).await
    }
    async fn create(&self, client: Client) -> Result<u64, PGError> {
        let stmt = format!(
            "INSERT INTO pictures (date, url, title, credit, description) VALUES ('{}', '{}', '{}', '{}', '{}');",
            self.date, self.img_url, self.title.replace("'", "''"), self.credit.replace("'", "''"), self.description.replace("'", "''")
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

    pub async fn get_apod(&mut self, date: &str) -> Option<APODInfo> {
        if self.list_entries.len() == 0 {
            self.retrieve_list().await.unwrap();
        }
        let target_list_entry = self.list_entries.iter().find(|&e| e.date == date)?;
        let sub_page = reqwest::get(format!("{}{}", self.url_root, target_list_entry.page_url))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let description: String = Regex::new(r#"<.+?>\s*Explanation:\s*<.+?>\s*([\s\S]+?)\s*<p>"#)
            .map(|re| match re.captures(&sub_page) {
                Some(capture) => normalize_text(&capture[1], self.url_root),
                None => String::new(),
            })
            .unwrap();

        assert!(description.len() > 0);

        // TODO: Get image source not from image tag but from enclosing link
        let img_url: String = Regex::new(r#"<(?:IMG SRC|img src|iframe[\s\S]+?src)=["'](.+?)["']"#)
            .map(|re| match re.captures(&sub_page) {
                Some(capture) => match &capture[1].starts_with("http") {
                    true => normalize_text(&capture[1], self.url_root),
                    false => format!("{}{}", self.url_root, &capture[1]),
                },
                None => String::new(),
            })
            .unwrap();

        assert!(img_url.len() > 0);

        let credit: String = Regex::new(
            r#"<\w+?>[\s\S]*?Credit[\s\S]*?:\s*</\w+?>\s*([\s\S]+?)\s*;?\s*<(?:/center|p|i|b)"#,
        )
        .map(|re| match re.captures(&sub_page) {
            Some(capture) => normalize_text(&capture[1], self.url_root),
            None => String::new(),
        })
        .unwrap();

        let credit_test_re = Regex::new(r#"Credit"#).unwrap();
        assert!(!(credit.len() == 0 && credit_test_re.is_match(&sub_page)));

        let img_editor: String = Regex::new(
            r#"<\w+?>[\s\S]*?Processing[\s\S]*?:\s*</\w+?>\s*([\s\S]+?)\s*;?\s*<(?:/center|p|i|b)"#,
        )
        .map(|re| match re.captures(&sub_page) {
            Some(capture) => normalize_text(&capture[1], self.url_root),
            None => String::new(),
        })
        .unwrap();

        let img_editor_test_re = Regex::new(r#"Processing"#).unwrap();
        assert!(!(credit.len() == 0 && img_editor_test_re.is_match(&sub_page)));

        let copyright: String = Regex::new(
            r#"<\w+?>[\s\S]*?Copyright[\s\S]*?:\s*</\w+?>\s*([\s\S]+?)\s*;?\s*<(?:/center|p|i|b)"#,
        )
        .map(|re| match re.captures(&sub_page) {
            Some(capture) => normalize_text(&capture[1], self.url_root),
            None => String::new(),
        })
        .unwrap();

        let copyright_test_re = Regex::new(r#"Copyright"#).unwrap();
        assert!(!(credit.len() == 0 && copyright_test_re.is_match(&sub_page)));

        let license: String = Regex::new(
            r#"<\w+?>[\s\S]*?License[\s\S]*?:\s*</\w+?>\s*([\s\S]+?)\s*;?\s*<(?:/center|p|i|b)"#,
        )
        .map(|re| match re.captures(&sub_page) {
            Some(capture) => normalize_text(&capture[1], self.url_root),
            None => String::new(),
        })
        .unwrap();

        let license_test_re = Regex::new(r#"License"#).unwrap();
        assert!(!(credit.len() == 0 && license_test_re.is_match(&sub_page)));

        let text_author: String = Regex::new(
            r#"<\w+?>[\s\S]*?Text[\s\S]*?:\s*</\w+?>\s*([\s\S]+?)\s*;?\s*<(?:/center|p|i|b)"#,
        )
        .map(|re| match re.captures(&sub_page) {
            Some(capture) => normalize_text(&capture[1], self.url_root),
            None => String::new(),
        })
        .unwrap();

        let text_author_test_re = Regex::new(r#"Text"#).unwrap();
        assert!(!(credit.len() == 0 && text_author_test_re.is_match(&sub_page)));

        self.store_thumbnail(&img_url, date).await.unwrap();

        Some(APODInfo {
            id: None,
            date: String::from(date),
            img_url,
            title: String::from(&target_list_entry.title),
            credit,
            description,
            copyright,
            license,
            text_author,
            img_editor,
        })
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
                        format!("https://i.ytimg.com/vi/{}/maxresdefault.jpg", &captures[1]),
                        String::from("i.ytimg.com"),
                    ),
                    None => panic!("No Youtube ID found in {}!", img_url),
                })
                .unwrap()
        } else if img_url.starts_with("https://apod.nasa.gov/apod/image") {
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

fn normalize_text(text: &str, url_root: &str) -> String {
    let space_fix_regex = Regex::new(r"(?:\s{2,}|\n)").unwrap();
    let space_fixed: String = space_fix_regex.replace_all(text, " ").into_owned();
    let br_fixed: String = Regex::new(r"<br>")
        .unwrap()
        .replace_all(space_fixed.as_str(), " ")
        .into_owned();
    let i_fixed: String = Regex::new(r"<i>\s*(?P<content>.+?)\s*</i>")
        .unwrap()
        .replace_all(br_fixed.as_str(), "*$content*")
        .into_owned();
    let b_fixed: String = Regex::new(r"<b>\s*(?P<content>.+?)\s*</b>")
        .unwrap()
        .replace_all(i_fixed.as_str(), "**$content**")
        .into_owned();
    let href_attr_regex = r"(?:ref|href|rhef|hre|hef)";
    let link_text_regex = r"(?P<text>.+?)";
    let link_url_regex = r"(?P<url>\S+?)";
    let link_closing_tag_regex = r"(?:</a>|<\?=/a>)";
    let link_regex = format!(
        r#"<a\s+{href_attr}\s*=\s*"?{link_url}"?(?:>|\s>|\s.*?>){link_text}{link_closing_tag}"#,
        href_attr = href_attr_regex,
        link_text = link_text_regex,
        link_url = link_url_regex,
        link_closing_tag = link_closing_tag_regex,
    );
    let link_fixed: String = Regex::new(link_regex.as_str())
        .unwrap()
        .replace_all(b_fixed.as_str(), |captures: &regex::Captures| {
            let url = captures.name("url").unwrap().as_str();
            let text = captures.name("text").unwrap().as_str();
            match url.starts_with("http") {
                true => format!("[{}]({})", text, url),
                false => format!("[{}]({}{})", text, url_root, url),
            }
        })
        .into_owned();

    // </a> tag is too much in date 2020-10-30.
    let artifacts_fixed: String = Regex::new(r#"</a>"#)
        .unwrap()
        .replace_all(link_fixed.as_str(), "")
        .into_owned();

    let space_fixed_again: String = space_fix_regex
        .replace_all(artifacts_fixed.as_str(), " ")
        .into_owned();

    let trimmed: String = Regex::new(r"(?:^\s|\s$)")
        .unwrap()
        .replace_all(space_fixed_again.as_str(), "")
        .into_owned();

    let test_re = Regex::new(r"(<|>|\s{2,}|^\s|\s$)").unwrap();
    if test_re.is_match(&trimmed) {
        let captures = test_re.captures(&trimmed).unwrap();
        panic!(format!(
            "Text not fixed completely - Found {} in {}",
            &captures[1], trimmed
        ));
    }
    trimmed
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
                credit TEXT NOT NULL,
                description TEXT NOT NULL,
                img_editor TEXT,
                text_author TEXT,
                copyright TEXT,
                license TEXT
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
            Some(apod) => apod,
            None => {
                counter = counter - Duration::days(1);
                continue;
            },
        };
        println!(
            // "Date: {}, Image URL: {}, Title: {}, Description: {}, Credit: {}, Image editor: {}, Text author: {}, Copyright: {}, License: {}",
            "Date: {}, Image URL: {}, Title: {}",
            apod.date,
            apod.img_url,
            apod.title,
            // apod.description,
            // apod.credit,
            // apod.img_editor,
            // apod.text_author,
            // apod.copyright,
            // apod.license,
        );
        // apod.save(client).await.unwrap();

        let half_sec = time::Duration::from_millis(500);
        thread::sleep(half_sec);

        counter = counter - Duration::days(1);
    }

    ()
}
