#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Network {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_list: Option<Vec<String>>,

    #[serde(rename = "allowUnqualifiedDNSQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_unqualified_dns_query: Option<bool>,

    #[serde(rename = "DNSSearchList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_search_list: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    network_shared_container_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    network_namespace: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::Network;
    use serde_json;

    #[test]
    fn serialize_network() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&network_prototype()).unwrap()).unwrap();

        let expected = json!({
            "endpointList": ["7a010682-17e0-4455-a838-02e5d9655fe6"],
            "allowUnqualifiedDNSQuery": true,
            "DNSSearchList": ["a.com", "b.com"],
            "networkSharedContainerName": "foo",
            "networkNamespace": "bar"
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_network() {
        let json = r#"{
            "endpointList": ["7a010682-17e0-4455-a838-02e5d9655fe6"],
            "allowUnqualifiedDNSQuery": true,
            "DNSSearchList": ["a.com", "b.com"],
            "networkSharedContainerName": "foo",
            "networkNamespace": "bar"
        }"#;

        let network: Network = serde_json::from_str(json).unwrap();

        assert_eq!(network_prototype(), network);
    }

    #[test]
    fn deserialize_network_optional_fields() {
        let network: Network = serde_json::from_str("{}").unwrap();
        let expected = Network{
            endpoint_list: None,
            allow_unqualified_dns_query: None,
            dns_search_list: None,
            network_shared_container_name: None,
            network_namespace: None,
        };

        assert_eq!(expected, network);
    }

    fn network_prototype() -> Network {
        Network {
            endpoint_list: Some(vec![String::from("7a010682-17e0-4455-a838-02e5d9655fe6")]),
            allow_unqualified_dns_query: Some(true),
            dns_search_list: Some(vec![String::from("a.com"), String::from("b.com")]),
            network_shared_container_name: Some(String::from("foo")),
            network_namespace: Some(String::from("bar")),
        }
    }
}
