use serde_derive::{Serialize, Deserialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct HugepageLimit {
    pub page_size: String,
    pub limit: u64,
}

#[cfg(test)]
mod tests {
    use super::HugepageLimit;
    use serde_json;

    #[test]
    fn serialize_hugepage_limit() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&hugepage_limit_prototype()).unwrap()).unwrap();

        let expected = serde_json::json!({
            "pageSize": "2MB",
            "limit": 209715200
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_hugepage_limit() {
        let json = r#"{
            "pageSize": "2MB",
            "limit": 209715200
        }"#;

        let hugepage_limit: HugepageLimit = serde_json::from_str(json).unwrap();

        assert_eq!(hugepage_limit_prototype(), hugepage_limit);
    }

    fn hugepage_limit_prototype() -> HugepageLimit {
        HugepageLimit {
            page_size: String::from("2MB"),
            limit: 209715200,
        }
    }
}
