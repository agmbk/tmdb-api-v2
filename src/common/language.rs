#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Language {
    pub iso_639_1: String,
    pub name: String,
}
