use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Video {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub site: String,
    pub key: String,
    pub published_at: DateTime<Utc>,
    pub size: u64,
    pub iso_639_1: String,
    pub iso_3166_1: String,
}
