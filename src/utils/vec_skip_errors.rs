//! Deserializes a [Vec], skipping errors.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

#[allow(dead_code)]
pub(crate) fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    serializer.serialize_some(value)
}

pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: for<'de_bis> Deserialize<'de_bis>,
{
    let value = Vec::<Value>::deserialize(deserializer)?;

    Ok(value.into_iter().flat_map(serde_json::from_value).collect())
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct Test {
        #[serde(deserialize_with = "super::deserialize")]
        vec: Vec<u64>,
    }

    #[test]
    fn deserialize() {
        let json = r#"{"vec":[1,"2",3]}"#;
        let test = serde_json::from_str::<Test>(json).unwrap();
        assert_eq!(test, Test { vec: vec![1, 3] });
    }
}
