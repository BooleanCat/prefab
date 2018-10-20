mod arg;

use self::arg::Arg;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Syscall {
    pub names: Vec<String>,
    pub action: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Arg>>,
}

#[cfg(test)]
mod tests {
    use super::Syscall;
    use serde_json;

    #[test]
    fn serialize_syscall() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&syscall_prototype()).unwrap()).unwrap();

        let expected = json!({
            "names": ["getcwd", "chmod"],
            "action": "SCMP_ACT_ERRNO",
            "args": []
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_syscall() {
        let json = r#"{
            "names": ["getcwd", "chmod"],
            "action": "SCMP_ACT_ERRNO",
            "args": []
        }"#;

        let syscall: Syscall = serde_json::from_str(json).unwrap();

        assert_eq!(syscall_prototype(), syscall);
    }

    #[test]
    fn deserialize_syscall_optional_fields() {
        let syscall: Syscall = serde_json::from_str(r#"{
            "names": ["getcwd", "chmod"],
            "action": "SCMP_ACT_ERRNO"
        }"#).unwrap();

        let expected = Syscall{
            args: None,

            ..syscall_prototype()
        };

        assert_eq!(expected, syscall);
    }

    fn syscall_prototype() -> Syscall {
        Syscall {
            names: vec![String::from("getcwd"), String::from("chmod")],
            action: String::from("SCMP_ACT_ERRNO"),
            args: Some(vec![]),
        }
    }
}
