mod weight_device;
mod iops_device;

use self::weight_device::WeightDevice;
use self::iops_device::RateDevice;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct BlockIo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf_weight: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_device: Option<Vec<WeightDevice>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_read_bps_device: Option<Vec<RateDevice>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_write_bps_device: Option<Vec<RateDevice>>,

    #[serde(rename = "throttleReadIOPSDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_read_iops_device: Option<Vec<RateDevice>>,

    #[serde(rename = "throttleWriteIOPSDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_write_iops_device: Option<Vec<RateDevice>>,
}

#[cfg(test)]
mod tests {
    use super::BlockIo;
    use serde_json;

    #[test]
    fn serialize_block_io() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&block_io_prototype()).unwrap()).unwrap();

        let expected = json!({
            "weight": 10,
            "leafWeight": 10,
            "weightDevice": [],
            "throttleReadBpsDevice": [],
            "throttleWriteBpsDevice": [],
            "throttleReadIOPSDevice": [],
            "throttleWriteIOPSDevice": []
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_block_io() {
        let json = r#"{
            "weight": 10,
            "leafWeight": 10,
            "weightDevice": [],
            "throttleReadBpsDevice": [],
            "throttleWriteBpsDevice": [],
            "throttleReadIOPSDevice": [],
            "throttleWriteIOPSDevice": []
        }"#;

        let block_io: BlockIo = serde_json::from_str(json).unwrap();

        assert_eq!(block_io_prototype(), block_io);
    }

    #[test]
    fn deserialize_block_io_optional_fields() {
        let block_io: BlockIo = serde_json::from_str("{}").unwrap();

        let expected = BlockIo{
            weight: None,
            leaf_weight: None,
            weight_device: None,
            throttle_read_bps_device: None,
            throttle_write_bps_device: None,
            throttle_read_iops_device: None,
            throttle_write_iops_device: None,
        };

        assert_eq!(expected, block_io);
    }

    fn block_io_prototype() -> BlockIo {
        BlockIo {
            weight: Some(10),
            leaf_weight: Some(10),
            weight_device: Some(vec![]),
            throttle_read_bps_device: Some(vec![]),
            throttle_write_bps_device: Some(vec![]),
            throttle_read_iops_device: Some(vec![]),
            throttle_write_iops_device: Some(vec![]),
        }
    }
}
