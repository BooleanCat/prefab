use serde_derive::{Serialize, Deserialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Cpu {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_runtime: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_period: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mems: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::Cpu;
    use serde_json;

    #[test]
    fn serialize_cpu() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&cpu_prototype()).unwrap()).unwrap();

        let expected = serde_json::json!({
            "shares": 1024,
            "quota": 1000000,
            "period": 500000,
            "realtimeRuntime": 950000,
            "realtimePeriod": 1000000,
            "cpus": "2-3",
            "mems": "0-7"
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_cpu() {
        let json = r#"{
            "shares": 1024,
            "quota": 1000000,
            "period": 500000,
            "realtimeRuntime": 950000,
            "realtimePeriod": 1000000,
            "cpus": "2-3",
            "mems": "0-7"
        }"#;

        let cpu: Cpu = serde_json::from_str(json).unwrap();

        assert_eq!(cpu_prototype(), cpu);
    }

    #[test]
    fn deserialize_cpu_optional_fields() {
        let cpu: Cpu = serde_json::from_str(r#"{
            "allow": true
        }"#).unwrap();

        let expected = Cpu{
            shares: None,
            quota: None,
            period: None,
            realtime_runtime: None,
            realtime_period: None,
            cpus: None,
            mems: None,
        };

        assert_eq!(expected, cpu);
    }

    fn cpu_prototype() -> Cpu {
        Cpu {
            shares: Some(1024),
            quota: Some(1000000),
            period: Some(500000),
            realtime_runtime: Some(950000),
            realtime_period: Some(1000000),
            cpus: Some(String::from("2-3")),
            mems: Some(String::from("0-7")),
        }
    }
}
