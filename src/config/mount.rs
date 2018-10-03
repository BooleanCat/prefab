#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Mount{
    pub destination: String,

    #[serde(default)]
    pub source: String,

    #[serde(default)]
    pub options: Vec<String>,

    #[serde(default)]
    #[serde(rename = "type")]
    pub mount_type: String,
}

#[cfg(test)]
mod tests {
    use super::Mount;
    use serde_json;

    #[test]
    fn serialize_mount() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&mount_prototype()).unwrap()).unwrap();

        let expected = json!({
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
            destination: String::from("/some/mount/destination"),
            ..Default::default()
        };

        assert_eq!(expected, mount);
    }

    fn mount_prototype() -> Mount {
        Mount{
            destination: String::from("/some/mount/destination"),
            source: String::from("/some/mount/source"),
            options: vec![String::from("ro"), String::from("bind")],
            mount_type: String::from("none"),
        }
    }
}
