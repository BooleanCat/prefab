mod capped_cpu;
mod capped_memory;
mod anet;

use self::capped_cpu::CappedCpu;
use self::capped_memory::CappedMemory;
use self::anet::Anet;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Solaris {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limitpriv: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_shm_memory: Option<String>,

    #[serde(rename = "cappedCPU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capped_cpu: Option<CappedCpu>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capped_memory: Option<CappedMemory>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub anet: Option<Vec<Anet>>,
}

#[cfg(test)]
mod tests {
    use super::Solaris;
    use serde_json;

    #[test]
    fn serialize_solaris() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&solaris_prototype()).unwrap()).unwrap();
        let expected = json!({
            "milestone": "svc:/milestone/container:default",
            "limitpriv": "default",
            "maxShmMemory": "512m",
            "cappedCPU": {},
            "cappedMemory": {},
            "anet": []
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_solaris() {
        let json = r#"{
            "milestone": "svc:/milestone/container:default",
            "limitpriv": "default",
            "maxShmMemory": "512m",
            "cappedCPU": {},
            "cappedMemory": {},
            "anet": []
        }"#;

        let solaris: Solaris = serde_json::from_str(json).unwrap();

        assert_eq!(solaris_prototype(), solaris);
    }

    #[test]
    fn deserialize_solaris_optional_fields() {
        let solaris: Solaris = serde_json::from_str("{}").unwrap();
        let expected = Solaris{
            milestone: None,
            limitpriv: None,
            max_shm_memory: None,
            capped_cpu: None,
            capped_memory: None,
            anet: None,
        };

        assert_eq!(expected, solaris);
    }

    fn solaris_prototype() -> Solaris {
        Solaris {
            milestone: Some(String::from("svc:/milestone/container:default")),
            limitpriv: Some(String::from("default")),
            max_shm_memory: Some(String::from("512m")),
            capped_cpu: Some(Default::default()),
            capped_memory: Some(Default::default()),
            anet: Some(vec![]),
        }
    }
}
