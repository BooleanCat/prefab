#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Device {
    pub id: String,
    pub id_type: String,
}

#[cfg(test)]
mod tests {
    use super::Device;
    use serde_json;

    #[test]
    fn serialize_device() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&device_prototype()).unwrap()).unwrap();

        let expected = json!({
            "id": "24E552D7-6523-47F7-A647-D3465BF1F5CA",
            "idType": "class"
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_device() {
        let json = r#"{
            "id": "24E552D7-6523-47F7-A647-D3465BF1F5CA",
            "idType": "class"
        }"#;

        let device: Device = serde_json::from_str(json).unwrap();

        assert_eq!(device_prototype(), device);
    }

    fn device_prototype() -> Device {
        Device {
            id: String::from("24E552D7-6523-47F7-A647-D3465BF1F5CA"),
            id_type: String::from("class"),
        }
    }
}
