mod syscall;

use self::syscall::Syscall;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Seccomp {
    pub default_action: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub architectures: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub syscalls: Option<Vec<Syscall>>,
}

#[cfg(test)]
mod tests {
    use super::Seccomp;
    use serde_json;

    #[test]
    fn serialize_seccomp() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&seccomp_prototype()).unwrap()).unwrap();

        let expected = json!({
            "defaultAction": "SCMP_ACT_ALLOW",
            "architectures": ["SCMP_ARCH_X86"],
            "syscalls": []
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_seccomp() {
        let json = r#"{
            "defaultAction": "SCMP_ACT_ALLOW",
            "architectures": ["SCMP_ARCH_X86"],
            "syscalls": []
        }"#;

        let seccomp: Seccomp = serde_json::from_str(json).unwrap();

        assert_eq!(seccomp_prototype(), seccomp);
    }

    #[test]
    fn deserialize_seccomp_optional_fields() {
        let seccomp: Seccomp = serde_json::from_str(r#"{
            "defaultAction": "SCMP_ACT_ALLOW"
        }"#).unwrap();

        let expected = Seccomp{
            architectures: None,
            syscalls: None,

            ..seccomp_prototype()
        };

        assert_eq!(expected, seccomp);
    }

    fn seccomp_prototype() -> Seccomp {
        Seccomp {
            default_action: String::from("SCMP_ACT_ALLOW"),
            architectures: Some(vec![String::from("SCMP_ARCH_X86")]),
            syscalls: Some(vec![]),
        }
    }
}
