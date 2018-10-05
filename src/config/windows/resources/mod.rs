mod memory;

pub use self::memory::Memory;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Resources {
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<Memory>,
}

#[cfg(test)]
mod tests {
    use super::Resources;
    use serde_json;

    #[test]
    fn serialize_resources() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&resources_prototype()).unwrap()).unwrap();

        let expected = json!({
            "memory": {}
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_resources() {
        let json = r#"{
            "memory": {}
        }"#;

        let resources: Resources = serde_json::from_str(json).unwrap();

        assert_eq!(resources_prototype(), resources);
    }

    #[test]
    fn deserialize_resources_optional_fields() {
        let resources: Resources = serde_json::from_str("{}").unwrap();
        let expected = Resources{
            memory: None,
        };

        assert_eq!(expected, resources);
    }

    fn resources_prototype() -> Resources {
        Resources {
            memory: Some(Default::default()),
        }
    }
}
