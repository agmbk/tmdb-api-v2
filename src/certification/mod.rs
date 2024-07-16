#[cfg(feature = "commands")]
pub mod list;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Certification {
    pub certification: String,
    pub meaning: String,
    pub order: usize,
}
