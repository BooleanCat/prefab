use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Hyperv {
    #[serde(rename = "utilityVMPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    utility_vm_path: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::Hyperv;
    use serde_json;

    #[test]
    fn serialize_hyperv() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&hyperv_prototype()).unwrap()).unwrap();
        let expected = serde_json::json!({"utilityVMPath": "C:\\path\\to\\utilityvm"});

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_hyperv() {
        let json = r#"{"utilityVMPath": "C:\\path\\to\\utilityvm"}"#;
        let hyperv: Hyperv = serde_json::from_str(json).unwrap();

        assert_eq!(hyperv_prototype(), hyperv);
    }

    #[test]
    fn deserialize_hyperv_optional_fields() {
        let hyperv: Hyperv = serde_json::from_str("{}").unwrap();
        let expected = Hyperv{utility_vm_path: None};

        assert_eq!(expected, hyperv);
    }

    fn hyperv_prototype() -> Hyperv {
        Hyperv {utility_vm_path: Some(String::from("C:\\path\\to\\utilityvm"))}
    }
}
