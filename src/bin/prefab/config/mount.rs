use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Mount{
    pub destination: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_type: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::Mount;
    use serde_json;

    #[test]
    fn serialize_mount() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&mount_prototype()).unwrap()).unwrap();

        let expected = serde_json::json!({
            "destination": "/some/mount/destination",
            "source": "/some/mount/source",
            "options": ["ro", "bind"],
            "type": "none"
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_mount() {
        let json = r#"{
            "destination": "/some/mount/destination",
            "source": "/some/mount/source",
            "options": ["ro", "bind"],
            "type": "none"
        }"#;

        let mount: Mount = serde_json::from_str(json).unwrap();

        assert_eq!(mount_prototype(), mount);
    }

    #[test]
    fn deserialize_mount_optional_fields() {
        let mount: Mount = serde_json::from_str(r#"{
            "destination": "/some/mount/destination"
        }"#).unwrap();

        let expected = Mount{
            source: None,
            options: None,
            mount_type: None,

            ..mount_prototype()
        };

        assert_eq!(expected, mount);
    }

    fn mount_prototype() -> Mount {
        Mount{
            destination: String::from("/some/mount/destination"),
            source: Some(String::from("/some/mount/source")),
            options: Some(vec![String::from("ro"), String::from("bind")]),
            mount_type: Some(String::from("none")),
        }
    }
}
