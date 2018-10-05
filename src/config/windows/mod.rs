mod device;
mod resources;

pub use self::device::Device;
pub use self::resources::{Resources, Memory};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Windows {
    pub layer_folders: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
}

#[cfg(test)]
mod tests {
    use super::Windows;
    use serde_json;

    #[test]
    fn serialize_windows() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&windows_prototype()).unwrap()).unwrap();

        let expected = json!({
            "layerFolders": ["C:\\foo\\bar", "C:\\bar\\baz"],
            "devices": []
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_windows() {
        let json = r#"{
            "layerFolders": ["C:\\foo\\bar", "C:\\bar\\baz"],
            "devices": []
        }"#;

        let windows: Windows = serde_json::from_str(json).unwrap();

        assert_eq!(windows_prototype(), windows);
    }

    #[test]
    fn deserialize_windows_optional_fields() {
        let windows: Windows = serde_json::from_str(r#"{"layerFolders": []}"#).unwrap();
        let expected = Windows{
            devices: None,

            layer_folders: vec![],
        };

        assert_eq!(expected, windows);
    }

    fn windows_prototype() -> Windows {
        Windows {
            layer_folders: vec![String::from("C:\\foo\\bar"), String::from("C:\\bar\\baz")],
            devices: Some(vec![]),
        }
    }
}
