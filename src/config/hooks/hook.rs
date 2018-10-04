#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Hook {
    pub path: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<isize>,
}

#[cfg(test)]
mod tests {
    use super::Hook;
    use serde_json;

    #[test]
    fn serialize_hook() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&hook_prototype()).unwrap()).unwrap();

        let expected = json!({
            "path": "/foo/bar",
            "args": ["foo", "bar"],
            "env": ["bar", "baz"],
            "timeout": 42
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_hook() {
        let json = r#"{
            "path": "/foo/bar",
            "args": ["foo", "bar"],
            "env": ["bar", "baz"],
            "timeout": 42
        }"#;

        let hook: Hook = serde_json::from_str(json).unwrap();

        assert_eq!(hook_prototype(), hook);
    }

    #[test]
    fn deserialize_hooks_optional_fields() {
        let hook: Hook = serde_json::from_str(r#"{"path": "/foo/bar"}"#).unwrap();

        let expected = Hook{
            args: None,
            env: None,
            timeout: None,

            path: String::from("/foo/bar"),
        };

        assert_eq!(expected, hook);
    }

    fn hook_prototype() -> Hook {
        Hook{
            path: String::from("/foo/bar"),
            args: Some(vec![String::from("foo"), String::from("bar")]),
            env: Some(vec![String::from("bar"), String::from("baz")]),
            timeout: Some(42),
        }
    }
}
