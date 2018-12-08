use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Image {
    pub path: String,
    pub format: String,
}

#[cfg(test)]
mod tests {
    use super::Image;
    use serde_json;

    #[test]
    fn serialize_image() {
        let json: serde_json::Value = serde_json::from_str(&serde_json::to_string(&image_prototype()).unwrap()).unwrap();

        let expected = serde_json::json!({
            "path": "/foo/bar",
            "format": "dummy"
        });

        assert_eq!(expected, json);
    }

    #[test]
    fn deserialize_image() {
        let json = r#"{
            "path": "/foo/bar",
            "format": "dummy"
        }"#;

        let image: Image = serde_json::from_str(json).unwrap();

        assert_eq!(image_prototype(), image);
    }

    fn image_prototype() -> Image {
        Image {
            path: String::from("/foo/bar"),
            format: String::from("dummy"),
        }
    }
}
