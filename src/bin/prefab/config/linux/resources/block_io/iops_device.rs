#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct RateDevice {
    pub major: i64,
    pub minor: i64,
    pub rate: u64,
}

#[cfg(test)]
mod tests {
    use super::RateDevice;
    use serde_json;

    #[test]
    fn serialize_rate_device() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&rate_device_prototype()).unwrap()).unwrap();

        let expected = json!({
            "major": 8,
            "minor": 0,
            "rate": 300
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_rate_device() {
        let json = r#"{
            "major": 8,
            "minor": 0,
            "rate": 300
        }"#;

        let rate_device: RateDevice = serde_json::from_str(json).unwrap();

        assert_eq!(rate_device_prototype(), rate_device);
    }

    fn rate_device_prototype() -> RateDevice {
        RateDevice {
            major: 8,
            minor: 0,
            rate: 300,
        }
    }
}
