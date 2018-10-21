mod namespace;
mod id_mapping;
mod device;
mod resources;
mod intel_rdt;
mod seccomp;

use self::namespace::Namespace;
use self::id_mapping::IdMapping;
use self::device::Device;
use self::resources::Resources;
use self::intel_rdt::IntelRdt;
use self::seccomp::Seccomp;
use std::collections::HashMap;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Linux {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<Namespace>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid_mappings: Option<Vec<IdMapping>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid_mappings: Option<Vec<IdMapping>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroups_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Resources>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub intel_rdt: Option<IntelRdt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysctl: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seccomp: Option<Seccomp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rootfs_propagation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_paths: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_paths: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_label: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::Linux;
    use serde_json;
    use std::collections::HashMap;

    #[test]
    fn serialize_linux() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&linux_prototype()).unwrap()).unwrap();

        let expected = json!({
            "namespaces": [],
            "uidMappings": [],
            "gidMappings": [],
            "devices": [],
            "cgroupsPath": "/path/to/cgroups",
            "resources": {},
            "intelRdt": {},
            "sysctl": {},
            "seccomp": {"defaultAction": ""},
            "rootfsPropagation": "slave",
            "maskedPaths": ["/proc/kcore"],
            "readonlyPaths": ["/proc/sys"],
            "mountLabel": "system_u:object_r:svirt_sandbox_file_t:s0:c715,c811"
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
            "resources": {},
            "intelRdt": {},
            "sysctl": {},
            "seccomp": {"defaultAction": ""},
            "rootfsPropagation": "slave",
            "maskedPaths": ["/proc/kcore"],
            "readonlyPaths": ["/proc/sys"],
            "mountLabel": "system_u:object_r:svirt_sandbox_file_t:s0:c715,c811"
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
            intel_rdt: None,
            sysctl: None,
            seccomp: None,
            rootfs_propagation: None,
            masked_paths: None,
            readonly_paths: None,
            mount_label: None,
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
            intel_rdt: Some(Default::default()),
            sysctl: Some(HashMap::new()),
            seccomp: Some(Default::default()),
            rootfs_propagation: Some(String::from("slave")),
            masked_paths: Some(vec![String::from("/proc/kcore")]),
            readonly_paths: Some(vec![String::from("/proc/sys")]),
            mount_label: Some(String::from("system_u:object_r:svirt_sandbox_file_t:s0:c715,c811")),
        }
    }
}

