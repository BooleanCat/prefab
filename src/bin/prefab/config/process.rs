mod console_size;
mod rlimit;
mod capabilities;
mod user;

use self::console_size::ConsoleSize;
use self::rlimit::RLimit;
use self::capabilities::Capabilities;
use self::user::User;
use serde_derive::{Serialize, Deserialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Process {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_size: Option<ConsoleSize>,

    pub cwd: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,

    pub args: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rlimits: Option<Vec<RLimit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub apparmor_profile: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_new_privileges: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_score_adj: Option<isize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selinux_label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[cfg(test)]
mod tests {
    use super::Process;
    use serde_json;

    #[test]
    fn serialize_process() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&process_prototype()).unwrap()).unwrap();

        let expected = serde_json::json!({
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
            "capabilities": {},
            "noNewPrivileges": true,
            "oomScoreAdj": 14,
            "selinuxLabel": "foo",
            "user": {
                "uid": 0,
                "gid": 0
            }
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
            "capabilities": {},
            "noNewPrivileges": true,
            "oomScoreAdj": 14,
            "selinuxLabel": "foo",
            "user": {
                "uid": 0,
                "gid": 0
            }
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
            terminal: None,
            console_size: None,
            env: None,
            rlimits: None,
            apparmor_profile: None,
            capabilities: None,
            no_new_privileges: None,
            oom_score_adj: None,
            selinux_label: None,
            user: None,

            ..process_prototype()
        };

        assert_eq!(expected, process);
    }

    fn process_prototype() -> Process {
        Process{
            terminal: Some(true),
            console_size: Some(Default::default()),
            cwd: String::from("/foo/bar"),
            env: Some(vec![String::from("FOO=BAR")]),
            args: vec![String::from("foo"), String::from("bar")],
            rlimits: Some(vec![]),
            apparmor_profile: Some(String::from("so-secure")),
            capabilities: Some(Default::default()),
            no_new_privileges: Some(true),
            oom_score_adj: Some(14),
            selinux_label: Some(String::from("foo")),
            user: Some(Default::default()),
        }
    }
}
