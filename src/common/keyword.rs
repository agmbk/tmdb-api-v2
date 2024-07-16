#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Keyword {
    pub id: u64,
    pub name: String,
}
