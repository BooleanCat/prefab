use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct IdMapping {
    #[serde(rename = "containerID")]
    container_id: u32,

    #[serde(rename = "hostID")]
    host_id: u32,

    size: u32,
}

#[cfg(test)]
mod tests {
    use super::IdMapping;
    use serde_json;

    #[test]
    fn serialize_id_mapping() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&id_mapping_prototype()).unwrap()).unwrap();

        let expected = serde_json::json!({
            "containerID": 0,
            "hostID": 100,
            "size": 1
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_id_mapping() {
        let json = r#"{
            "containerID": 0,
            "hostID": 100,
            "size": 1
        }"#;

        let id_mapping: IdMapping = serde_json::from_str(json).unwrap();

        assert_eq!(id_mapping_prototype(), id_mapping);
    }

    fn id_mapping_prototype() -> IdMapping {
        IdMapping {
            container_id: 0,
            host_id: 100,
            size: 1,
        }
    }
}
