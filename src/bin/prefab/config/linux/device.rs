#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Device {
    #[serde(rename = "type")]
    device_type: String,

    path: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    major: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    minor: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    file_mode: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gid: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::Device;
    use serde_json;

    #[test]
    fn serialize_device() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&device_prototype()).unwrap()).unwrap();

        let expected = json!({
            "type": "c",
            "path": "/dev/fuse",
            "major": 10,
            "minor": 229,
            "fileMode": 438,
            "uid": 1,
            "gid": 1
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_device() {
        let json = r#"{
            "type": "c",
            "path": "/dev/fuse",
            "major": 10,
            "minor": 229,
            "fileMode": 438,
            "uid": 1,
            "gid": 1
        }"#;

        let device: Device = serde_json::from_str(json).unwrap();

        assert_eq!(device_prototype(), device);
    }

    #[test]
    fn deserialize_device_optional_fields() {
        let device: Device = serde_json::from_str(r#"{
            "type": "c",
            "path": "/dev/fuse"
        }"#).unwrap();

        let expected = Device{
            major: None,
            minor: None,
            file_mode: None,
            uid: None,
            gid: None,

            ..device_prototype()
        };

        assert_eq!(expected, device);
    }

    fn device_prototype() -> Device {
        Device {
            device_type: String::from("c"),
            path: String::from("/dev/fuse"),
            major: Some(10),
            minor: Some(229),
            file_mode: Some(438),
            uid: Some(1),
            gid: Some(1),
        }
    }
}
