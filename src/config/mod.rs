mod process;
mod root;
mod mount;

pub use self::process::{Process, ConsoleSize, RLimit, Capabilities, User};
pub use self::root::Root;
pub use self::mount::Mount;

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
}

#[cfg(test)]
mod tests {
    use super::{Config, Root};
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
            "hostname": "pikachu"
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
            "hostname": "pikachu"
        }"#;

        let config: Config = serde_json::from_str(json).unwrap();

        assert_eq!(config_prototype(), config);
    }

    #[test]
    fn deserialize_config_optional_fields() {
        let config: Config = serde_json::from_str(r#"{
            "ociVersion": "foo",
            "root": {
                "path": "/foo/bar"
            }
        }"#).unwrap();

        let expected = Config{
            mounts: None,
            process: None,
            hostname: None,

            oci_version: String::from("foo"),
            root: Root{
                path: String::from("/foo/bar"),
                readonly: None,
            },
        };

        assert_eq!(expected, config);
    }

    fn config_prototype() -> Config {
        Config{
            oci_version: String::from("foo"),
            root: Default::default(),
            mounts: Some(vec![]),
            process: Some(Default::default()),
            hostname: Some(String::from("pikachu")),
        }
    }
}
