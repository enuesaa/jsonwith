#[cfg(test)]
mod tests {
    use crate::core::data::kv::Kv;
    use crate::core::data::path::Path;
    use crate::core::data::tokens::Tokens;
    use crate::core::serializer::serializer::Serializer;
    use crate::core::data::kvs::Kvs;

    #[test]
    fn test_root_dict() {
        let mut serializer = Serializer::new();
        let actual = serializer.serialize_withline(Kvs {
            items: vec![
                Kv { path: Path::from("$"), value: Tokens::MkDict },
                Kv { path: Path::from("$.a"), value: Tokens::String("aaa".to_string()) },
                Kv { path: Path::from("$"), value: Tokens::EndDict },
            ],
        });

        assert_eq!(actual, String::from("{\n  \"a\": \"aaa\"\n}\n"));
    }

    #[test]
    fn test_root_array() {
        let mut serializer = Serializer::new();
        let actual = serializer.serialize_withline(Kvs {
            items: vec![
                Kv { path: Path::from("$"), value: Tokens::MkArray },
                Kv { path: Path::from("$[0]"), value: Tokens::String("aaa".to_string()) },
                Kv { path: Path::from("$"), value: Tokens::EndArray },
            ],
        });

        assert_eq!(actual, String::from("[\n  \"aaa\"\n]\n"));
    }

    #[test]
    fn test_root_string() {
        let mut serializer = Serializer::new();
        let actual = serializer.serialize_withline(Kvs {
            items: vec![
                Kv { path: Path::from("$"), value: Tokens::String("aaa".to_string()) },
            ],
        });

        assert_eq!(actual, String::from("\"aaa\"\n"));
    }

    #[test]
    fn test_root_number() {
        let mut serializer = Serializer::new();
        let actual = serializer.serialize_withline(Kvs {
            items: vec![
                Kv { path: Path::from("$"), value: Tokens::Number(107) },
            ],
        });

        assert_eq!(actual, String::from("107\n"));
    }

    #[test]
    fn test_nested_dict() {
        let mut serializer = Serializer::new();
        let actual = serializer.serialize_withline(Kvs {
            items: vec![
                Kv { path: Path::from("$"), value: Tokens::MkDict },
                Kv { path: Path::from("$.a"), value: Tokens::String("aaa".to_string()) },
                Kv { path: Path::from("$.b"), value: Tokens::MkDict },
                Kv { path: Path::from("$.b.c"), value: Tokens::String("ddd".to_string()) },
                Kv { path: Path::from("$.b"), value: Tokens::EndDict },
                Kv { path: Path::from("$.e"), value: Tokens::Number(108) },
                Kv { path: Path::from("$"), value: Tokens::EndDict },
            ],
        });

        assert_eq!(actual, String::from("{\n  \"a\": \"aaa\",\n  \"b\": {\n    \"c\": \"ddd\"\n  },\n  \"e\": 108\n}\n"));
    }
}