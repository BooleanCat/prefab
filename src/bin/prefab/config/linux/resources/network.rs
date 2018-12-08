mod priority;

use self::priority::Priority;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Network {
    #[serde(rename = "classID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_id: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priorities: Option<Vec<Priority>>,
}

#[cfg(test)]
mod tests {
    use super::Network;
    use serde_json;

    #[test]
    fn serialize_network() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&network_prototype()).unwrap()).unwrap();

        let expected = serde_json::json!({
            "classID": 1048577,
            "priorities": []
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_network() {
        let json = r#"{
            "classID": 1048577,
            "priorities": []
        }"#;

        let network: Network = serde_json::from_str(json).unwrap();

        assert_eq!(network_prototype(), network);
    }

    #[test]
    fn deserialize_network_optional_fields() {
        let network: Network = serde_json::from_str("{}").unwrap();

        let expected = Network{
            class_id: None,
            priorities: None,
        };

        assert_eq!(expected, network);
    }

    fn network_prototype() -> Network {
        Network {
            class_id: Some(1048577),
            priorities: Some(vec![]),
        }
    }
}
