mod hypervisor;
mod kernel;
mod image;

use self::hypervisor::Hypervisor;
use self::image::Image;
use self::kernel::Kernel;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Vm {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypervisor: Option<Hypervisor>,

    pub kernel: Kernel,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
}

#[cfg(test)]
mod tests {
    use super::Vm;
    use serde_json;

    #[test]
    fn serialize_vm() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&vm_prototype()).unwrap()).unwrap();

        let expected = serde_json::json!({
            "hypervisor": {"path": ""},
            "kernel": {"path": ""},
            "image": {"path": "", "format": ""}
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_vm() {
        let json = r#"{
            "hypervisor": {"path": ""},
            "kernel": {"path": ""},
            "image": {"path": "", "format": ""}
        }"#;

        let vm: Vm = serde_json::from_str(json).unwrap();

        assert_eq!(vm_prototype(), vm);
    }

    #[test]
    fn deserialize_vm_optional_fields() {
        let vm: Vm = serde_json::from_str(r#"{"kernel": {"path": ""}}"#).unwrap();
        let expected = Vm{
            hypervisor: None,
            image: None,

            ..vm_prototype()
        };

        assert_eq!(expected, vm);
    }

    fn vm_prototype() -> Vm {
        Vm {
            hypervisor: Some(Default::default()),
            kernel: Default::default(),
            image: Some(Default::default()),
        }
    }
}

