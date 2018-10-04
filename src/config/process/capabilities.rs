#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Capabilities{
    #[serde(default)]
    pub effective: Vec<String>,

    #[serde(default)]
    pub bounding: Vec<String>,

    #[serde(default)]
    pub inheritable: Vec<String>,

    #[serde(default)]
    pub permitted: Vec<String>,

    #[serde(default)]
    pub ambient: Vec<String>,
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
            effective: vec![],
            bounding: vec![],
            inheritable: vec![],
            permitted: vec![],
            ambient: vec![],
        };

        assert_eq!(expected, capabilities);
    }

    fn capabilities_prototype() -> Capabilities {
        Capabilities{
            effective: vec![String::from("CAP_CHOWN")],
            bounding: vec![String::from("CAP_DAC_OVERRIDE")],
            inheritable: vec![String::from("CAP_FSETID")],
            permitted: vec![String::from("CAP_FOWNER")],
            ambient: vec![String::from("CAP_MKNOD")],
        }
    }
}
