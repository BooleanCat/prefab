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

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Process {
    #[serde(default)]
    pub terminal: bool,

    #[serde(default)]
    pub console_size: ConsoleSize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct ConsoleSize {
    pub height: usize,
    pub width: usize,
}

#[cfg(test)]
mod tests {
    use super::{Config, Root, Process, ConsoleSize};
    use serde_json;

    #[test]
    fn serialize_deserialize_config() {
        let root = Root{path: String::from("/foo/bar"), readonly: true};
        let process = Process{terminal: true, console_size: ConsoleSize{height: 1, width: 1}};
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
    fn deserialize_process_terminal_optional() {
        let process: Process = serde_json::from_str("{}").unwrap();
        let expected = Process{terminal: false, ..Default::default()};

        assert_eq!(expected, process);
    }

    #[test]
    fn deserialize_process_console_size_optional() {
        let process: Process = serde_json::from_str("{}").unwrap();
        let expected = Process{console_size: Default::default(), ..Default::default()};

        assert_eq!(expected, process);
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
