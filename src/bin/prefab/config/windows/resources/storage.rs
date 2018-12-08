use serde_derive::{Serialize, Deserialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Storage {
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bps: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sandbox_size: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::Storage;
    use serde_json;

    #[test]
    fn serialize_storage() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&storage_prototype()).unwrap()).unwrap();

        let expected = serde_json::json!({
            "iops": 123,
            "bps": 42,
            "sandboxSize": 100
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_storage() {
        let json = r#"{
            "iops": 123,
            "bps": 42,
            "sandboxSize": 100
        }"#;

        let storage: Storage = serde_json::from_str(json).unwrap();

        assert_eq!(storage_prototype(), storage);
    }

    #[test]
    fn deserialize_storage_optional_fields() {
        let storage: Storage = serde_json::from_str("{}").unwrap();
        let expected = Storage{
            iops: None,
            bps: None,
            sandbox_size: None,
        };

        assert_eq!(expected, storage);
    }

    fn storage_prototype() -> Storage {
        Storage {
            iops: Some(123),
            bps: Some(42),
            sandbox_size: Some(100),
        }
    }
}
