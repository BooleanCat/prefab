#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Namespace {
    #[serde(rename = "type")]
    pub namespace_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::Namespace;
    use serde_json;

    #[test]
    fn serialize_namespace() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&namespace_prototype()).unwrap()).unwrap();

        let expected = json!({
            "type": "pid",
            "path": "/proc/self/ns/pid"
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_namespace() {
        let json = r#"{
            "type": "pid",
            "path": "/proc/self/ns/pid"
        }"#;

        let namespace: Namespace = serde_json::from_str(json).unwrap();

        assert_eq!(namespace_prototype(), namespace);
    }

    #[test]
    fn deserialize_namespace_optional_fields() {
        let namespace: Namespace = serde_json::from_str(r#"{
            "type": ""
        }"#).unwrap();

        let expected = Namespace{
            path: None,

            namespace_type: Default::default(),
        };

        assert_eq!(expected, namespace);
    }

    fn namespace_prototype() -> Namespace {
        Namespace {
            namespace_type: String::from("pid"),
            path: Some(String::from("/proc/self/ns/pid")),
        }
    }
}
