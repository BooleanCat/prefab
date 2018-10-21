#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Kernel {
    pub path: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub initrd: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::Kernel;
    use serde_json;

    #[test]
    fn serialize_kernel() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&kernel_prototype()).unwrap()).unwrap();

        let expected = json!({
            "path": "/foo/bar",
            "parameters": ["one=1", "two=2"],
            "initrd": "/foo/bar.img"
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_kernel() {
        let json = r#"{
            "path": "/foo/bar",
            "parameters": ["one=1", "two=2"],
            "initrd": "/foo/bar.img"
        }"#;

        let kernel: Kernel = serde_json::from_str(json).unwrap();

        assert_eq!(kernel_prototype(), kernel);
    }

    #[test]
    fn deserialize_kernel_optional_fields() {
        let kernel: Kernel = serde_json::from_str(r#"{"path": "/foo/bar"}"#).unwrap();
        let expected = Kernel{
            parameters: None,
            initrd: None,

            ..kernel_prototype()
        };

        assert_eq!(expected, kernel);
    }

    fn kernel_prototype() -> Kernel {
        Kernel {
            path: String::from("/foo/bar"),
            parameters: Some(vec![String::from("one=1"), String::from("two=2")]),
            initrd: Some(String::from("/foo/bar.img")),
        }
    }
}
