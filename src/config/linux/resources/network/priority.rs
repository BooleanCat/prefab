#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Priority {
    pub name: String,
    pub priority: u32,
}

#[cfg(test)]
mod tests {
    use super::Priority;
    use serde_json;

    #[test]
    fn serialize_priority() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&priority_prototype()).unwrap()).unwrap();

        let expected = json!({
            "name": "eth0",
            "priority": 100
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_priority() {
        let json = r#"{
            "name": "eth0",
            "priority": 100
        }"#;

        let priority: Priority = serde_json::from_str(json).unwrap();

        assert_eq!(priority_prototype(), priority);
    }

    fn priority_prototype() -> Priority {
        Priority {
            name: String::from("eth0"),
            priority: 100,
        }
    }
}
