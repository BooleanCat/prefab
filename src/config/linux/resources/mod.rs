mod device;
mod memory;
mod cpu;
mod block_io;

use self::device::Device;
use self::memory::Memory;
use self::cpu::Cpu;
use self::block_io::BlockIo;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Resources {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<Memory>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<Cpu>,

    #[serde(rename = "blockIO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_io: Option<BlockIo>,
}

#[cfg(test)]
mod tests {
    use super::Resources;
    use serde_json;

    #[test]
    fn serialize_resources() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&resources_prototype()).unwrap()).unwrap();

        let expected = json!({
            "devices": [],
            "memory": {},
            "cpu": {},
            "blockIO": {}
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_resources() {
        let json = r#"{
            "devices": [],
            "memory": {},
            "cpu": {},
            "blockIO": {}
        }"#;

        let resources: Resources = serde_json::from_str(json).unwrap();

        assert_eq!(resources_prototype(), resources);
    }

    #[test]
    fn deserialize_resources_optional_fields() {
        let resources: Resources = serde_json::from_str("{}").unwrap();

        let expected = Resources{
            devices: None,
            memory: None,
            cpu: None,
            block_io: None,
        };

        assert_eq!(expected, resources);
    }

    fn resources_prototype() -> Resources {
        Resources {
            devices: Some(vec![]),
            memory: Some(Default::default()),
            cpu: Some(Default::default()),
            block_io: Some(Default::default()),
        }
    }
}
