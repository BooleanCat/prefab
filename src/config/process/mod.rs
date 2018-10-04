mod console_size;
mod rlimit;
mod capabilities;

pub use self::console_size::ConsoleSize;
pub use self::rlimit::RLimit;
pub use self::capabilities::Capabilities;

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

    pub args: Vec<String>,

    #[serde(default)]
    pub rlimits: Vec<RLimit>,

    #[serde(default)]
    pub apparmor_profile: String,

    #[serde(default)]
    pub capabilities: Capabilities,

    #[serde(default)]
    pub no_new_privileges: bool,
}

#[cfg(test)]
mod tests {
    use super::Process;
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
            "env": ["FOO=BAR"],
            "args": ["foo", "bar"],
            "rlimits": [],
            "apparmorProfile": "so-secure",
            "capabilities": {
                "effective": [],
                "bounding": [],
                "inheritable": [],
                "permitted": [],
                "ambient": []
            },
            "noNewPrivileges": true
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
            "env": ["FOO=BAR"],
            "args": ["foo", "bar"],
            "rlimits": [],
            "apparmorProfile": "so-secure",
            "capabilities": {
                "effective": [],
                "bounding": [],
                "inheritable": [],
                "permitted": [],
                "ambient": []
            },
            "noNewPrivileges": true
        }"#;

        let process: Process = serde_json::from_str(json).unwrap();

        assert_eq!(process_prototype(), process);
    }

    #[test]
    fn deserialize_process_optional_fields() {
        let process: Process = serde_json::from_str(r#"{
            "cwd": "/foo/bar",
            "args": ["foo", "bar"]
        }"#).unwrap();
        let expected = Process{
            terminal: false,
            console_size: Default::default(),
            env: vec![],
            rlimits: vec![],
            apparmor_profile: String::from(""),
            capabilities: Default::default(),
            no_new_privileges: false,

            cwd: String::from("/foo/bar"),
            args: vec![String::from("foo"), String::from("bar")],
        };

        assert_eq!(expected, process);
    }

    fn process_prototype() -> Process {
        Process{
            terminal: true,
            console_size: Default::default(),
            cwd: String::from("/foo/bar"),
            env: vec![String::from("FOO=BAR")],
            args: vec![String::from("foo"), String::from("bar")],
            rlimits: Default::default(),
            apparmor_profile: String::from("so-secure"),
            capabilities: Default::default(),
            no_new_privileges: true,
        }
    }
}
