#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Config{
    pub oci_version: String,
}

#[cfg(test)]
mod tests {
    use super::Config;
    use serde_json;

    #[test]
    fn serialize() {
        let config = Config{oci_version: String::from("foo")};
        let expected = json!({
            "ociVersion": "foo"
        });

        assert_eq!(expected.to_string(), serde_json::to_string(&config).unwrap());
    }

    #[test]
    fn deserialize() {
        let config: Config = serde_json::from_str(r#"{
            "ociVersion": "foo"
        }"#).unwrap();
        let expected = Config{oci_version: String::from("foo")};

        assert_eq!(expected, config);
    }
}
