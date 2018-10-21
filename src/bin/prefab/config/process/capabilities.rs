#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Capabilities{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inheritable: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permitted: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambient: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::Capabilities;
    use serde_json;

    #[test]
    fn serialize_capabilities() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&capabilities_prototype()).unwrap()).unwrap();

        let expected = json!({
            "effective": ["CAP_CHOWN"],
            "bounding": ["CAP_DAC_OVERRIDE"],
            "inheritable": ["CAP_FSETID"],
            "permitted": ["CAP_FOWNER"],
            "ambient": ["CAP_MKNOD"]
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_capabilities() {
        let json = r#"{
            "effective": ["CAP_CHOWN"],
            "bounding": ["CAP_DAC_OVERRIDE"],
            "inheritable": ["CAP_FSETID"],
            "permitted": ["CAP_FOWNER"],
            "ambient": ["CAP_MKNOD"]
        }"#;

        let capabilities: Capabilities = serde_json::from_str(json).unwrap();

        assert_eq!(capabilities_prototype(), capabilities);
    }

    #[test]
    fn deserialize_capabilities_optional_fields() {
        let capabilities: Capabilities = serde_json::from_str("{}").unwrap();
        let expected = Capabilities{
            effective: None,
            bounding: None,
            inheritable: None,
            permitted: None,
            ambient: None,
        };

        assert_eq!(expected, capabilities);
    }

    fn capabilities_prototype() -> Capabilities {
        Capabilities{
            effective: Some(vec![String::from("CAP_CHOWN")]),
            bounding: Some(vec![String::from("CAP_DAC_OVERRIDE")]),
            inheritable: Some(vec![String::from("CAP_FSETID")]),
            permitted: Some(vec![String::from("CAP_FOWNER")]),
            ambient: Some(vec![String::from("CAP_MKNOD")]),
        }
    }
}
