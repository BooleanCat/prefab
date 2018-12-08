use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct CappedMemory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::CappedMemory;
    use serde_json;

    #[test]
    fn serialize_capped_memory() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&capped_memory_prototype()).unwrap()).unwrap();

        let expected = serde_json::json!({
            "physical": "512m",
            "swap": "256m"
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_capped_memory() {
        let json = r#"{
            "physical": "512m",
            "swap": "256m"
        }"#;

        let capped_memory: CappedMemory = serde_json::from_str(json).unwrap();

        assert_eq!(capped_memory_prototype(), capped_memory);
    }

    #[test]
    fn deserialize_capped_memory_optional_fields() {
        let capped_memory: CappedMemory = serde_json::from_str("{}").unwrap();

        let expected = CappedMemory{
            physical: None,
            swap: None,
        };

        assert_eq!(expected, capped_memory);
    }

    fn capped_memory_prototype() -> CappedMemory {
        CappedMemory {
            physical: Some(String::from("512m")),
            swap: Some(String::from("256m")),
        }
    }
}
