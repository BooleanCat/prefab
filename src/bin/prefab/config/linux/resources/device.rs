use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Device {
    pub allow: bool,

    #[serde(rename = "type")]
    device_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    major: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    minor: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    access: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::Device;
    use serde_json;

    #[test]
    fn serialize_device() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&device_prototype()).unwrap()).unwrap();

        let expected = serde_json::json!({
            "allow": true,
            "type": "c",
            "major": 10,
            "minor": 229,
            "access": "rw"
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_device() {
        let json = r#"{
            "allow": true,
            "type": "c",
            "major": 10,
            "minor": 229,
            "access": "rw"
        }"#;

        let device: Device = serde_json::from_str(json).unwrap();

        assert_eq!(device_prototype(), device);
    }

    #[test]
    fn deserialize_device_optional_fields() {
        let device: Device = serde_json::from_str(r#"{
            "allow": true
        }"#).unwrap();

        let expected = Device{
            device_type: None,
            major: None,
            minor: None,
            access: None,

            ..device_prototype()
        };

        assert_eq!(expected, device);
    }

    fn device_prototype() -> Device {
        Device {
            allow: true,
            device_type: Some(String::from("c")),
            major: Some(10),
            minor: Some(229),
            access: Some(String::from("rw")),
        }
    }
}
