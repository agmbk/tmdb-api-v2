#[cfg(feature = "commands")]
pub mod list;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}
