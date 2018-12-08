use serde_derive::{Serialize, Deserialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct WeightDevice {
    pub major: i64,
    pub minor: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf_weight: Option<u16>,
}

#[cfg(test)]
mod tests {
    use super::WeightDevice;
    use serde_json;

    #[test]
    fn serialize_weight_device() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&weight_device_prototype()).unwrap()).unwrap();

        let expected = serde_json::json!({
            "major": 8,
            "minor": 0,
            "weight": 500,
            "leafWeight": 300
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_weight_device() {
        let json = r#"{
            "major": 8,
            "minor": 0,
            "weight": 500,
            "leafWeight": 300
        }"#;

        let weight_device: WeightDevice = serde_json::from_str(json).unwrap();

        assert_eq!(weight_device_prototype(), weight_device);
    }

    #[test]
    fn deserialize_weight_device_optional_fields() {
        let weight_device: WeightDevice = serde_json::from_str(r#"{
            "major": 8,
            "minor": 0
        }"#).unwrap();

        let expected = WeightDevice{
            weight: None,
            leaf_weight: None,

            ..weight_device_prototype()
        };

        assert_eq!(expected, weight_device);
    }

    fn weight_device_prototype() -> WeightDevice {
        WeightDevice {
            major: 8,
            minor: 0,
            weight: Some(500),
            leaf_weight: Some(300),
        }
    }
}
