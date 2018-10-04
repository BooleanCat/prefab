mod hook;

pub use self::hook::Hook;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Hooks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prestart: Option<Vec<Hook>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub poststart: Option<Vec<Hook>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub poststop: Option<Vec<Hook>>,
}

#[cfg(test)]
mod tests {
    use super::Hooks;
    use serde_json;

    #[test]
    fn serialize_hooks() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&hooks_prototype()).unwrap()).unwrap();

        let expected = json!({
            "prestart": [],
            "poststart": [],
            "poststop": []
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_hooks() {
        let json = r#"{
            "prestart": [],
            "poststart": [],
            "poststop": []
        }"#;

        let hooks: Hooks = serde_json::from_str(json).unwrap();

        assert_eq!(hooks_prototype(), hooks);
    }

    #[test]
    fn deserialize_hooks_optional_fields() {
        let hooks: Hooks = serde_json::from_str("{}").unwrap();

        let expected = Hooks{
            prestart: None,
            poststart: None,
            poststop: None,
        };

        assert_eq!(expected, hooks);
    }

    fn hooks_prototype() -> Hooks {
        Hooks{
            prestart: Some(vec![]),
            poststart: Some(vec![]),
            poststop: Some(vec![]),
        }
    }
}
