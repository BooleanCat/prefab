use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct CappedCpu {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ncpus: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::CappedCpu;
    use serde_json;

    #[test]
    fn serialize_capped_cpu() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&capped_cpu_prototype()).unwrap()).unwrap();
        let expected = serde_json::json!({"ncpus": "8"});

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_capped_cpu() {
        let json = r#"{"ncpus": "8"}"#;
        let capped_cpu: CappedCpu = serde_json::from_str(json).unwrap();

        assert_eq!(capped_cpu_prototype(), capped_cpu);
    }

    #[test]
    fn deserialize_capped_cpu_optional_fields() {
        let capped_cpu: CappedCpu = serde_json::from_str("{}").unwrap();
        let expected = CappedCpu{ncpus: None};

        assert_eq!(expected, capped_cpu);
    }

    fn capped_cpu_prototype() -> CappedCpu {
        CappedCpu {ncpus: Some(String::from("8"))}
    }
}
