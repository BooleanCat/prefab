use serde_derive::{Serialize, Deserialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Arg {
    pub index: usize,
    pub value: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_two: Option<u64>,

    pub op: String,
}

#[cfg(test)]
mod tests {
    use super::Arg;
    use serde_json;

    #[test]
    fn serialize_arg() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&arg_prototype()).unwrap()).unwrap();

        let expected = serde_json::json!({
            "index": 2,
            "value": 42,
            "valueTwo": 7,
            "op": "SCMP_CMP_LT"
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_arg() {
        let json = r#"{
            "index": 2,
            "value": 42,
            "valueTwo": 7,
            "op": "SCMP_CMP_LT"
        }"#;

        let arg: Arg = serde_json::from_str(json).unwrap();

        assert_eq!(arg_prototype(), arg);
    }

    #[test]
    fn deserialize_arg_optional_fields() {
        let arg: Arg = serde_json::from_str(r#"{
            "index": 2,
            "value": 42,
            "op": "SCMP_CMP_LT"
        }"#).unwrap();

        let expected = Arg{
            value_two: None,

            ..arg_prototype()
        };

        assert_eq!(expected, arg);
    }

    fn arg_prototype() -> Arg {
        Arg {
            index: 2,
            value: 42,
            value_two: Some(7),
            op: String::from("SCMP_CMP_LT"),
        }
    }
}
