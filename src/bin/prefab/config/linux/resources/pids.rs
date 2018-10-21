#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Pids {
    pub limit: i64,
}

#[cfg(test)]
mod tests {
    use super::Pids;
    use serde_json;

    #[test]
    fn serialize_pids() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&pids_prototype()).unwrap()).unwrap();

        let expected = json!({"limit": 32771});

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_pids() {
        let json = r#"{"limit": 32771}"#;

        let pids: Pids = serde_json::from_str(json).unwrap();

        assert_eq!(pids_prototype(), pids);
    }

    fn pids_prototype() -> Pids {
        Pids {limit: 32771}
    }
}
