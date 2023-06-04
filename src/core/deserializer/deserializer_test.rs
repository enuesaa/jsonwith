#[cfg(test)]
mod tests {
    use crate::core::data::kv::Kv;
    use crate::core::data::path::{Path, PathItem};
    use crate::core::data::tokens::Tokens;
    use crate::core::deserializer::deserializer::Deserializer;
    use crate::core::data::kvs::Kvs;

    #[test]
    fn test_root_dict() {
        let text = "{\"a\": \"aaa\"}";
        let mut serializer = Deserializer::new();
        let actual = serializer.deserialize(text);

        assert_eq!(actual, Kvs {
            items: vec![
                Kv { path: Path { route: vec![], }, value: Tokens::MkDict },
                Kv { path: Path { route: vec![ PathItem::Key("a".to_string()) ], }, value: Tokens::String("aaa".to_string()) },
            ],
        });
    }

    #[test]
    fn test_root_array() {
        let text = "[\"aaa\"]";
        let mut serializer = Deserializer::new();
        let actual = serializer.deserialize(text);

        assert_eq!(actual, Kvs {
            items: vec![
                Kv { path: Path { route: vec![], }, value: Tokens::MkArray },
                Kv { path: Path { route: vec![ PathItem::Index(0) ], }, value: Tokens::String("aaa".to_string()) },
            ],
        });
    }

    #[test]
    fn test_root_string() {
        let text = "\"aaa\"";
        let mut serializer = Deserializer::new();
        let actual = serializer.deserialize(text);

        assert_eq!(actual, Kvs {
            items: vec![
                Kv { path: Path { route: vec![], }, value: Tokens::String("aaa".to_string()) },
            ],
        });
    }


    #[test]
    fn test_nested_dict() {
        let text = "{\"a\": \"aaa\", \"b\": {\"c\": \"ddd\"}}";
        let mut serializer = Deserializer::new();
        let actual = serializer.deserialize(text);

        assert_eq!(actual, Kvs {
            items: vec![
                Kv { path: Path { route: vec![], }, value: Tokens::MkDict },
                Kv { path: Path { route: vec![ PathItem::Key("a".to_string()) ], }, value: Tokens::String("aaa".to_string()) },
                Kv { path: Path { route: vec![ PathItem::Key("b".to_string()) ], }, value: Tokens::MkDict },
                Kv { path: Path { route: vec![ PathItem::Key("b".to_string()), PathItem::Key("c".to_string()) ], }, value: Tokens::String("ddd".to_string()) },
            ],
        });
    }
}