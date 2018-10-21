#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Memory {
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::Memory;
    use serde_json;

    #[test]
    fn serialize_memory() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&memory_prototype()).unwrap()).unwrap();

        let expected = json!({"limit": 1000});

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_memory() {
        let json = r#"{"limit": 1000}"#;

        let memory: Memory = serde_json::from_str(json).unwrap();

        assert_eq!(memory_prototype(), memory);
    }

    #[test]
    fn deserialize_memory_optional_fields() {
        let memory: Memory = serde_json::from_str("{}").unwrap();
        let expected = Memory{limit: None};

        assert_eq!(expected, memory);
    }

    fn memory_prototype() -> Memory {
        Memory {limit: Some(1000)}
    }
}
