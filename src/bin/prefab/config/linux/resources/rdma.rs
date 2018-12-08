use serde_derive::{Serialize, Deserialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Rdma {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hca_handles: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hca_objects: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::Rdma;
    use serde_json;

    #[test]
    fn serialize_rdma() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&rdma_prototype()).unwrap()).unwrap();

        let expected = serde_json::json!({
            "hcaHandles": 3,
            "hcaObjects": 10000
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_rdma() {
        let json = r#"{
            "hcaHandles": 3,
            "hcaObjects": 10000
        }"#;

        let rdma: Rdma = serde_json::from_str(json).unwrap();

        assert_eq!(rdma_prototype(), rdma);
    }

    #[test]
    fn deserialize_rdma_optional_fields() {
        let rdma: Rdma = serde_json::from_str("{}").unwrap();

        let expected = Rdma{
            hca_handles: None,
            hca_objects: None,
        };

        assert_eq!(expected, rdma);
    }

    fn rdma_prototype() -> Rdma {
        Rdma {
            hca_handles: Some(3),
            hca_objects: Some(10000),
        }
    }
}
