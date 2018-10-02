#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Process {
    #[serde(default)]
    pub terminal: bool,

    #[serde(default)]
    pub console_size: ConsoleSize,

    pub cwd: String,

    #[serde(default)]
    pub env: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct ConsoleSize {
    pub height: usize,
    pub width: usize,
}

#[cfg(test)]
mod tests {
    use super::{Process, ConsoleSize};
    use serde_json;

    #[test]
    fn serialize_process() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&process_prototype()).unwrap()).unwrap();

        let expected = json!({
            "terminal": true,
            "consoleSize": {
                "height": 0,
                "width": 0
            },
            "cwd": "/foo/bar",
            "env": ["FOO=BAR"]
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_process() {
        let json = r#"{
            "terminal": true,
            "consoleSize": {
                "height": 0,
                "width": 0
            },
            "cwd": "/foo/bar",
            "env": ["FOO=BAR"]
        }"#;

        let process: Process = serde_json::from_str(json).unwrap();

        assert_eq!(process_prototype(), process);
    }

    #[test]
    fn deserialize_process_optional_fields() {
        let process: Process = serde_json::from_str(r#"{
            "cwd": "/foo/bar"
        }"#).unwrap();
        let expected = Process{
            terminal: false,
            console_size: Default::default(),
            env: vec![],

            cwd: String::from("/foo/bar"),
        };

        assert_eq!(expected, process);
    }

    #[test]
    fn serialize_console_size() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&ConsoleSize{height: 100, width: 200}).unwrap()).unwrap();

        let expected = json!({
            "height": 100,
            "width": 200
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_console_size() {
        let json = r#"{
            "height": 100,
            "width": 200
        }"#;

        let console_size: ConsoleSize = serde_json::from_str(json).unwrap();
        let expected = ConsoleSize{height: 100, width: 200};

        assert_eq!(expected, console_size);
    }

    fn process_prototype() -> Process {
        Process{
            terminal: true,
            console_size: Default::default(),
            cwd: String::from("/foo/bar"),
            env: vec![String::from("FOO=BAR")],
        }
    }
}
