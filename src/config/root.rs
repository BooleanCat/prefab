#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Root {
    pub path: String,

    #[serde(default)]
    pub readonly: bool,
}

#[cfg(test)]
mod tests {
    use super::Root;
    use serde_json;

    #[test]
    fn serialize_root() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&root_prototype()).unwrap()).unwrap();

        let expected = json!({
            "path": "/foo/bar",
            "readonly": true
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_root() {
        let json = r#"{
            "path": "/foo/bar",
            "readonly": true
        }"#;

        let root: Root = serde_json::from_str(json).unwrap();

        assert_eq!(root_prototype(), root);
    }

    #[test]
    fn deserialize_root_optional_fields() {
        let root: Root = serde_json::from_str(r#"{"path": "/foo/bar"}"#).unwrap();
        let expected = Root{
            path: String::from("/foo/bar"),
            readonly: false,
        };

        assert_eq!(expected, root);
    }

    fn root_prototype() -> Root {
        Root {
            path: String::from("/foo/bar"),
            readonly: true,
        }
    }
}

