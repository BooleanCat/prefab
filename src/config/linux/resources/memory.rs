#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Memory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel: Option<i64>,

    #[serde(rename = "kernelTCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_tcp: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub swapiness: Option<u64>,

    #[serde(rename = "disableOOMKiller")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_oom_killer: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::Memory;
    use serde_json;

    #[test]
    fn serialize_memory() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&memory_prototype()).unwrap()).unwrap();

        let expected = json!({
            "limit": 2048,
            "reservation": 128,
            "swap": 2048,
            "kernel": -1,
            "kernelTCP": -1,
            "swapiness": 0,
            "disableOOMKiller": false
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_memory() {
        let json = r#"{
            "limit": 2048,
            "reservation": 128,
            "swap": 2048,
            "kernel": -1,
            "kernelTCP": -1,
            "swapiness": 0,
            "disableOOMKiller": false
        }"#;

        let memory: Memory = serde_json::from_str(json).unwrap();

        assert_eq!(memory_prototype(), memory);
    }

    #[test]
    fn deserialize_memory_optional_fields() {
        let memory: Memory = serde_json::from_str(r#"{
            "allow": true
        }"#).unwrap();

        let expected = Memory{
            limit: None,
            reservation: None,
            swap: None,
            kernel: None,
            kernel_tcp: None,
            swapiness: None,
            disable_oom_killer: None,
        };

        assert_eq!(expected, memory);
    }

    fn memory_prototype() -> Memory {
        Memory {
            limit: Some(2048),
            reservation: Some(128),
            swap: Some(2048),
            kernel: Some(-1),
            kernel_tcp: Some(-1),
            swapiness: Some(0),
            disable_oom_killer: Some(false),
        }
    }
}
