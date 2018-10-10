mod device;
mod memory;
mod cpu;
mod block_io;
mod hugepage_limit;
mod network;
mod pids;

use self::device::Device;
use self::memory::Memory;
use self::cpu::Cpu;
use self::block_io::BlockIo;
use self::hugepage_limit::HugepageLimit;
use self::network::Network;
use self::pids::Pids;

#[serde(rename_all = "camelCase")]
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hugepage_limits: Option<Vec<HugepageLimit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pids: Option<Pids>,
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
            "blockIO": {},
            "hugepageLimits": [],
            "network": {},
            "pids": {"limit": 0}
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_resources() {
        let json = r#"{
            "devices": [],
            "memory": {},
            "cpu": {},
            "blockIO": {},
            "hugepageLimits": [],
            "network": {},
            "pids": {"limit": 0}
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
            hugepage_limits: None,
            network: None,
            pids: None,
        };

        assert_eq!(expected, resources);
    }

    fn resources_prototype() -> Resources {
        Resources {
            devices: Some(vec![]),
            memory: Some(Default::default()),
            cpu: Some(Default::default()),
            block_io: Some(Default::default()),
            hugepage_limits: Some(vec![]),
            network: Some(Default::default()),
            pids: Some(Default::default()),
        }
    }
}
