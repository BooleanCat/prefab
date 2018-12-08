mod device;
mod resources;
mod network;
mod hyperv;

use self::device::Device;
use self::resources::Resources;
use self::network::Network;
use self::hyperv::Hyperv;
use serde_derive::{Serialize, Deserialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Windows {
    pub layer_folders: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Resources>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub servicing: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_flushes_during_boot: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyperv: Option<Hyperv>,
}

#[cfg(test)]
mod tests {
    use super::Windows;
    use serde_json;

    #[test]
    fn serialize_windows() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&windows_prototype()).unwrap()).unwrap();

        let expected = serde_json::json!({
            "layerFolders": ["C:\\foo\\bar", "C:\\bar\\baz"],
            "devices": [],
            "resources": {},
            "network": {},
            "servicing": true,
            "ignoreFlushesDuringBoot": true,
            "hyperv": {}
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_windows() {
        let json = r#"{
            "layerFolders": ["C:\\foo\\bar", "C:\\bar\\baz"],
            "devices": [],
            "resources": {},
            "network": {},
            "servicing": true,
            "ignoreFlushesDuringBoot": true,
            "hyperv": {}
        }"#;

        let windows: Windows = serde_json::from_str(json).unwrap();

        assert_eq!(windows_prototype(), windows);
    }

    #[test]
    fn deserialize_windows_optional_fields() {
        let windows: Windows = serde_json::from_str(r#"{
            "layerFolders": ["C:\\foo\\bar", "C:\\bar\\baz"]}
        "#).unwrap();

        let expected = Windows{
            devices: None,
            resources: None,
            network: None,
            servicing: None,
            ignore_flushes_during_boot: None,
            hyperv: None,

            ..windows_prototype()
        };

        assert_eq!(expected, windows);
    }

    fn windows_prototype() -> Windows {
        Windows {
            layer_folders: vec![String::from("C:\\foo\\bar"), String::from("C:\\bar\\baz")],
            devices: Some(vec![]),
            resources: Some(Default::default()),
            network: Some(Default::default()),
            servicing: Some(true),
            ignore_flushes_during_boot: Some(true),
            hyperv: Some(Default::default()),
        }
    }
}
