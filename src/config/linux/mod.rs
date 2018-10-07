mod namespace;
mod id_mapping;
mod device;
mod resources;

use self::namespace::Namespace;
use self::id_mapping::IdMapping;
use self::device::Device;
use self::resources::Resources;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Linux {
    #[serde(skip_serializing_if = "Option::is_none")]
    namespaces: Option<Vec<Namespace>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    uid_mappings: Option<Vec<IdMapping>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gid_mappings: Option<Vec<IdMapping>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    devices: Option<Vec<Device>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cgroups_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<Resources>,
}

#[cfg(test)]
mod tests {
    use super::Linux;
    use serde_json;

    #[test]
    fn serialize_linux() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&linux_prototype()).unwrap()).unwrap();

        let expected = json!({
            "namespaces": [],
            "uidMappings": [],
            "gidMappings": [],
            "devices": [],
            "cgroupsPath": "/path/to/cgroups",
            "resources": {}
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_linux() {
        let json = r#"{
            "namespaces": [],
            "uidMappings": [],
            "gidMappings": [],
            "devices": [],
            "cgroupsPath": "/path/to/cgroups",
            "resources": {}
        }"#;

        let linux: Linux = serde_json::from_str(json).unwrap();

        assert_eq!(linux_prototype(), linux);
    }

    #[test]
    fn deserialize_linux_optional_fields() {
        let linux: Linux = serde_json::from_str("{}").unwrap();

        let expected = Linux{
            namespaces: None,
            uid_mappings: None,
            gid_mappings: None,
            devices: None,
            cgroups_path: None,
            resources: None,
        };

        assert_eq!(expected, linux);
    }

    fn linux_prototype() -> Linux {
        Linux {
            namespaces: Some(vec![]),
            uid_mappings: Some(vec![]),
            gid_mappings: Some(vec![]),
            devices: Some(vec![]),
            cgroups_path: Some(String::from("/path/to/cgroups")),
            resources: Some(Default::default()),
        }
    }
}

