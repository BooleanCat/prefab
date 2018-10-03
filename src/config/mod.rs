mod process;
mod root;
mod mount;

pub use self::process::{Process, ConsoleSize};
pub use self::root::Root;
pub use self::mount::Mount;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Config {
    pub oci_version: String,
    pub root: Root,

    #[serde(default)]
    pub mounts: Vec<Mount>,

    #[serde(default)]
    pub process: Process,
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
                "readonly": false
            },
            "mounts": [],
            "process": {
                "terminal": false,
                "consoleSize": {
                    "height": 0,
                    "width": 0
                },
                "cwd": "",
                "env": [],
                "args": []
            }
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
                "terminal": false,
                "consoleSize": {
                    "height": 0,
                    "width": 0
                },
                "cwd": "",
                "env": [],
                "args": []
            }
        }"#;

        let config: Config = serde_json::from_str(json).unwrap();

        assert_eq!(config_prototype(), config);
    }

    #[test]
    fn deserialize_config_optional_fields() {
        let config: Config = serde_json::from_str(r#"{
            "ociVersion": "foo",
            "root": {
                "path": "/foo/bar",
                "readonly": true
            }
        }"#).unwrap();

        let expected = Config{
            oci_version: String::from("foo"),
            root: Root{
                path: String::from("/foo/bar"),
                readonly: true,
            },
            ..Default::default()
        };

        assert_eq!(expected, config);
    }

    fn config_prototype() -> Config {
        Config{
            oci_version: String::from("foo"),
            root: Default::default(),
            mounts: Default::default(),
            process: Default::default(),
        }
    }
}
