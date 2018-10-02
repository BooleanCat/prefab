mod process;

pub use self::process::{Process, ConsoleSize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Config {
    pub oci_version: String,
    pub root: Root,

    #[serde(default)]
    pub process: Process,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Root {
    pub path: String,

    #[serde(default)]
    pub readonly: bool,
}

#[cfg(test)]
mod tests {
    use super::{Config, Root, Process, ConsoleSize};
    use serde_json;

    #[test]
    fn serialize_deserialize_config() {
        let root = Root{path: String::from("/foo/bar"), readonly: true};
        let process = Process{
            terminal: true,
            console_size: ConsoleSize{height: 1, width: 1},
            cwd: String::from("/bar/baz"),
            env: vec![String::from("FOO=BAR")],
        };
        let config = Config{
            oci_version: String::from("foo"),
            root: root,
            process: process,
        };

        let deserialized_config: Config = serde_json::from_str(&serde_json::to_string(&config).unwrap()).unwrap();

        assert_eq!(deserialized_config, config);
    }

    #[test]
    fn deserialize_root_readonly_optional() {
        let root: Root = serde_json::from_str(r#"{"path": "/foo/bar"}"#).unwrap();
        let expected = Root{
            path: String::from("/foo/bar"),
            readonly: false,
        };

        assert_eq!(expected, root);
    }

    #[test]
    fn deserialize_config_process_optional() {
        let config: Config = serde_json::from_str(r#"{
            "ociVersion": "",
            "root":{
                "path": ""
            }
        }"#).unwrap();
        let expected = Config{process: Default::default(), ..Default::default()};

        assert_eq!(expected, config);
    }
}
