#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct User{
    pub uid: isize,
    pub gid: isize,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_gids: Option<Vec<isize>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::User;
    use serde_json;

    #[test]
    fn serialize_user() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&user_prototype()).unwrap()).unwrap();

        let expected = json!({
            "uid": 42,
            "gid": 43,
            "additionalGids": [1, 2],
            "username": "pikachu"
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_user() {
        let json = r#"{
            "uid": 42,
            "gid": 43,
            "additionalGids": [1, 2],
            "username": "pikachu"
        }"#;

        let user: User = serde_json::from_str(json).unwrap();

        assert_eq!(user_prototype(), user);
    }

    #[test]
    fn deserialize_user_optional_fields() {
        let user: User = serde_json::from_str(r#"{
            "uid": 42,
            "gid": 43
        }"#).unwrap();

        let expected = User{
            additional_gids: None,
            username: None,

            ..user_prototype()
        };

        assert_eq!(expected, user);
    }

    fn user_prototype() -> User {
        User{
            uid: 42,
            gid: 43,
            additional_gids: Some(vec![1, 2]),
            username: Some(String::from("pikachu")),
        }
    }
}
