#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct IntelRdt {
    #[serde(rename = "closID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clos_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub l3_cache_schema: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_bw_schema: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::IntelRdt;
    use serde_json;

    #[test]
    fn serialize_intel_rdt() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&intel_rdt_prototype()).unwrap()).unwrap();

        let expected = json!({
            "closID": "guaranteed_group",
            "l3CacheSchema": "L3:0=7f0;1=1f",
            "memBwSchema": "MB:0=20;1=70"
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_intel_rdt() {
        let json = r#"{
            "closID": "guaranteed_group",
            "l3CacheSchema": "L3:0=7f0;1=1f",
            "memBwSchema": "MB:0=20;1=70"
        }"#;

        let intel_rdt: IntelRdt = serde_json::from_str(json).unwrap();

        assert_eq!(intel_rdt_prototype(), intel_rdt);
    }

    #[test]
    fn deserialize_intel_rdt_optional_fields() {
        let intel_rdt: IntelRdt = serde_json::from_str("{}").unwrap();

        let expected = IntelRdt{
            clos_id: None,
            l3_cache_schema: None,
            mem_bw_schema: None,
        };

        assert_eq!(expected, intel_rdt);
    }

    fn intel_rdt_prototype() -> IntelRdt {
        IntelRdt {
            clos_id: Some(String::from("guaranteed_group")),
            l3_cache_schema: Some(String::from("L3:0=7f0;1=1f")),
            mem_bw_schema: Some(String::from("MB:0=20;1=70")),
        }
    }
}
