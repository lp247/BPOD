impl APOD {
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
  pub async fn save(&self, client: tokio_postgres::Client) -> Result<u64, tokio_postgres::Error> {
    match self.id {
      Some(_) => self.update(client).await,
      None => self.create(client).await,
    }
  }
  async fn update(&self, client: tokio_postgres::Client) -> Result<u64, tokio_postgres::Error> {
    let stmt = format!(
          "UPDATE pictures SET date = '{}', url = '{}', title = '{}', description = '{}', meta = '{}' WHERE id = {};",
          self.date, self.img_url, self.title.replace("'", "''"), self.description.replace("'", "''"), self.meta.replace("'", "''"), self.id.unwrap()
      );
    client.execute(stmt.as_str(), &[]).await
  }
  async fn create(&self, client: tokio_postgres::Client) -> Result<u64, tokio_postgres::Error> {
    let stmt = format!(
          "INSERT INTO pictures (date, url, title, description, meta) VALUES ('{}', '{}', '{}', '{}', '{}');",
          self.date, self.img_url, self.title.replace("'", "''"), self.description.replace("'", "''"), self.meta.replace("'", "''")
      );
    client.execute(stmt.as_str(), &[]).await
  }
}
