mod process;
mod root;
mod mount;
mod hooks;
mod vm;

pub use self::process::{Process, ConsoleSize, RLimit, Capabilities, User};
pub use self::root::Root;
pub use self::mount::Mount;
pub use self::hooks::{Hooks, Hook};
pub use self::vm::{Vm, Hypervisor, Kernel, Image};

use std::collections::HashMap;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Config {
    pub oci_version: String,
    pub root: Root,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<Mount>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<Process>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<Hooks>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm: Option<Vm>,
}

#[cfg(test)]
mod tests {
    use super::Config;
    use serde_json;

    #[test]
    fn serialize_config() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&config_prototype()).unwrap()).unwrap();

        let expected = json!({
            "ociVersion": "foo",
            "root": {
                "path": "",
            },
            "mounts": [],
            "process": {
                "cwd": "",
                "args": []
            },
            "hostname": "pikachu",
            "hooks": {},
            "annotations": {
                "foo": "bar",
                "bar": "baz"
            },
            "vm": {"kernel": {"path": ""}}
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_config() {
        let json = r#"{
            "ociVersion": "foo",
            "root": {
                "path": ""
            },
            "mounts": [],
            "process": {
                "cwd": "",
                "args": []
            },
            "hostname": "pikachu",
            "hooks": {},
            "annotations": {
                "foo": "bar",
                "bar": "baz"
            },
            "vm": {"kernel": {"path": ""}}
        }"#;

        let config: Config = serde_json::from_str(json).unwrap();

        assert_eq!(config_prototype(), config);
    }

    #[test]
    fn deserialize_config_optional_fields() {
        let config: Config = serde_json::from_str(r#"{
            "ociVersion": "foo",
            "root": {"path": ""}
        }"#).unwrap();

        let expected = Config{
            mounts: None,
            process: None,
            hostname: None,
            hooks: None,
            annotations: None,
            vm: None,

            oci_version: String::from("foo"),
            root: Default::default(),
        };

        assert_eq!(expected, config);
    }

    macro_rules! hashmap {
        ($( $key: expr => $val: expr ),*) => {{
            let mut map = ::std::collections::HashMap::new();
            $( map.insert($key, $val); )*
            map
        }}
    }

    fn config_prototype() -> Config {
        Config{
            oci_version: String::from("foo"),
            root: Default::default(),
            mounts: Some(vec![]),
            process: Some(Default::default()),
            hostname: Some(String::from("pikachu")),
            hooks: Some(Default::default()),
            annotations: Some(hashmap![
                String::from("foo") => String::from("bar"),
                String::from("bar") => String::from("baz")
            ]),
            vm: Some(Default::default()),
        }
    }
}
