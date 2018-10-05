#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Hypervisor {
    pub path: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::Hypervisor;
    use serde_json;

    #[test]
    fn serialize_hypervisor() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&hypervisor_prototype()).unwrap()).unwrap();

        let expected = json!({
            "path": "/foo/bar",
            "parameters": ["one=1", "two=2"]
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_hypervisor() {
        let json = r#"{
            "path": "/foo/bar",
            "parameters": ["one=1", "two=2"]
        }"#;

        let hypervisor: Hypervisor = serde_json::from_str(json).unwrap();

        assert_eq!(hypervisor_prototype(), hypervisor);
    }

    #[test]
    fn deserialize_hypervisor_optional_fields() {
        let hypervisor: Hypervisor = serde_json::from_str(r#"{"path": "/foo"}"#).unwrap();
        let expected = Hypervisor{
            parameters: None,

            path: String::from("/foo"),
        };

        assert_eq!(expected, hypervisor);
    }

    fn hypervisor_prototype() -> Hypervisor {
        Hypervisor {
            path: String::from("/foo/bar"),
            parameters: Some(vec![String::from("one=1"), String::from("two=2")]),
        }
    }
}
