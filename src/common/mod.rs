use std::fmt;
use std::fmt::Debug;

pub mod country;
pub mod credits;
pub mod external_ids;
pub mod image;
pub mod keyword;
pub mod language;
pub mod release_date;
pub mod status;
pub mod video;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PaginatedResult<T> {
    pub page: u64,
    pub total_results: u64,
    pub total_pages: u64,
    pub results: Vec<T>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Movie,
    Tv,
    Collection,
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            MediaType::Movie => "movie",
            MediaType::Tv => "tv",
            MediaType::Collection => "collection",
        };

        write!(f, "{s}")
    }
}
