use serde_derive::{Serialize, Deserialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Anet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_link: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub configure_allowed_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub defrouter: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_protection: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::Anet;
    use serde_json;

    #[test]
    fn serialize_anet() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&anet_prototype()).unwrap()).unwrap();

        let expected = serde_json::json!({
            "linkname": "net0",
            "lowerLink": "net2",
            "allowedAddress": "172.17.0.2/16",
            "configureAllowedAddress": "true",
            "defrouter": "172.17.0.1/16",
            "macAddress": "02:42:f8:52:c7:16",
            "linkProtection": "mac-nospoof, ip-nospoof"
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_anet() {
        let json = r#"{
            "linkname": "net0",
            "lowerLink": "net2",
            "allowedAddress": "172.17.0.2/16",
            "configureAllowedAddress": "true",
            "defrouter": "172.17.0.1/16",
            "macAddress": "02:42:f8:52:c7:16",
            "linkProtection": "mac-nospoof, ip-nospoof"
        }"#;

        let anet: Anet = serde_json::from_str(json).unwrap();

        assert_eq!(anet_prototype(), anet);
    }

    #[test]
    fn deserialize_anet_optional_fields() {
        let anet: Anet = serde_json::from_str("{}").unwrap();

        let expected = Anet{
            linkname: None,
            lower_link: None,
            allowed_address: None,
            configure_allowed_address: None,
            defrouter: None,
            mac_address: None,
            link_protection: None,
        };

        assert_eq!(expected, anet);
    }

    fn anet_prototype() -> Anet {
        Anet {
            linkname: Some(String::from("net0")),
            lower_link: Some(String::from("net2")),
            allowed_address: Some(String::from("172.17.0.2/16")),
            configure_allowed_address: Some(String::from("true")),
            defrouter: Some(String::from("172.17.0.1/16")),
            mac_address: Some(String::from("02:42:f8:52:c7:16")),
            link_protection: Some(String::from("mac-nospoof, ip-nospoof")),
        }
    }
}
