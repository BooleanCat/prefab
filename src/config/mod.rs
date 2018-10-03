mod process;
mod root;

pub use self::process::{Process, ConsoleSize};
pub use self::root::Root;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Config {
    pub oci_version: String,
    pub root: Root,

    #[serde(default)]
    pub process: Process,
}

#[cfg(test)]
mod tests {
    use super::{Config, Root, Process, ConsoleSize};
    use serde_json;

    #[test]
    fn serialize_config() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&config_prototype()).unwrap()).unwrap();

        let expected = json!({
            "ociVersion": "foo",
            "root": {
                "path": "/foo/bar",
                "readonly": true
            },
            "process": {
                "terminal": true,
                "consoleSize": {
                    "height": 100,
                    "width": 200
                },
                "cwd": "/foo/bar",
                "env": ["FOO=BAR"]
            }
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_config() {
        let json = r#"{
            "ociVersion": "foo",
            "root": {
                "path": "/foo/bar",
                "readonly": true
            },
            "process": {
                "terminal": true,
                "consoleSize": {
                    "height": 100,
                    "width": 200
                },
                "cwd": "/foo/bar",
                "env": ["FOO=BAR"]
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
            root: Root {
                path: String::from("/foo/bar"),
                readonly: true,
            },
            process: Process{
                terminal: true,
                console_size: ConsoleSize{
                    height: 100,
                    width: 200,
                },
                cwd: String::from("/foo/bar"),
                env: vec![String::from("FOO=BAR")],
            },
        }
    }
}
