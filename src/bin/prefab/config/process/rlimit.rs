use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct RLimit{
    #[serde(rename = "type")]
    pub rlimit_type: String,

    pub soft: u64,
    pub hard: u64,
}

#[cfg(test)]
mod tests {
    use super::RLimit;
    use serde_json;

    #[test]
    fn serialize_rlimit() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&rlimit_prototype()).unwrap()).unwrap();

        let expected = serde_json::json!({
            "type": "RLIMIT_MSGQUEUE",
            "soft": 1234,
            "hard": 5678
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_rlimit() {
        let json = r#"{
            "type": "RLIMIT_MSGQUEUE",
            "soft": 1234,
            "hard": 5678
        }"#;

        let rlimit: RLimit = serde_json::from_str(json).unwrap();

        assert_eq!(rlimit_prototype(), rlimit);
    }

    fn rlimit_prototype() -> RLimit {
        RLimit{
            rlimit_type: String::from("RLIMIT_MSGQUEUE"),
            soft: 1234,
            hard: 5678,
        }
    }
}
