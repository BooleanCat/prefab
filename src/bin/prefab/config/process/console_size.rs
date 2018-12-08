use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct ConsoleSize {
    pub height: usize,
    pub width: usize,
}

#[cfg(test)]
mod tests {
    use super::ConsoleSize;
    use serde_json;

    #[test]
    fn serialize_console_size() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&ConsoleSize{height: 100, width: 200}).unwrap()).unwrap();

        let expected = serde_json::json!({
            "height": 100,
            "width": 200
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_console_size() {
        let json = r#"{
            "height": 100,
            "width": 200
        }"#;

        let console_size: ConsoleSize = serde_json::from_str(json).unwrap();
        let expected = ConsoleSize{height: 100, width: 200};

        assert_eq!(expected, console_size);
    }
}
