use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Cpu {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shares: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    maximum: Option<u16>,
}

#[cfg(test)]
mod tests {
    use super::Cpu;
    use serde_json;

    #[test]
    fn serialize_cpu() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&cpu_prototype()).unwrap()).unwrap();

        let expected = serde_json::json!({
            "count": 123,
            "shares": 42,
            "maximum": 100
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_cpu() {
        let json = r#"{
            "count": 123,
            "shares": 42,
            "maximum": 100
        }"#;

        let cpu: Cpu = serde_json::from_str(json).unwrap();

        assert_eq!(cpu_prototype(), cpu);
    }

    #[test]
    fn deserialize_cpu_optional_fields() {
        let cpu: Cpu = serde_json::from_str("{}").unwrap();
        let expected = Cpu{
            count: None,
            shares: None,
            maximum: None,
        };

        assert_eq!(expected, cpu);
    }

    fn cpu_prototype() -> Cpu {
        Cpu {
            count: Some(123),
            shares: Some(42),
            maximum: Some(100),
        }
    }
}
