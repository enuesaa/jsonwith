#[cfg(test)]
mod tests {
    use crate::core::data::kv::Kv;
    use crate::core::data::kvs::Kvs;
    use crate::core::data::path::Path;
    use crate::core::data::tokens::Tokens;
    use crate::core::deserializer::deserializer::Deserializer;

    #[test]
    fn test_root_dict() {
        let text = "{\"a\": \"aaa\"}";
        let mut deserializer = Deserializer::new();
        let actual = deserializer.deserialize(text);

        assert_eq!(
            actual,
            Kvs {
                items: vec![
                    Kv {
                        path: Path::from("$"),
                        value: Tokens::MkDict
                    },
                    Kv {
                        path: Path::from("$.a"),
                        value: Tokens::String("aaa".to_string())
                    },
                    Kv {
                        path: Path::from("$"),
                        value: Tokens::EndDict
                    },
                ],
            }
        );
    }

    #[test]
    fn test_root_array() {
        let text = "[\"aaa\"]";
        let mut deserializer = Deserializer::new();
        let actual = deserializer.deserialize(text);

        assert_eq!(
            actual,
            Kvs {
                items: vec![
                    Kv {
                        path: Path::from("$"),
                        value: Tokens::MkArray
                    },
                    Kv {
                        path: Path::from("$[0]"),
                        value: Tokens::String("aaa".to_string())
                    },
                    Kv {
                        path: Path::from("$"),
                        value: Tokens::EndArray
                    },
                ],
            }
        );
    }

    #[test]
    fn test_root_string() {
        let text = "\"aaa\"";
        let mut deserializer = Deserializer::new();
        let actual = deserializer.deserialize(text);

        assert_eq!(
            actual,
            Kvs {
                items: vec![Kv {
                    path: Path::from("$"),
                    value: Tokens::String("aaa".to_string())
                },],
            }
        );
    }

    #[test]
    fn test_root_number() {
        let text = "107";
        let mut deserializer = Deserializer::new();
        let actual = deserializer.deserialize(text);
        println!("{:?}", actual);

        assert_eq!(
            actual,
            Kvs {
                items: vec![Kv {
                    path: Path::from("$"),
                    value: Tokens::Number(107)
                },],
            }
        );
    }

    #[test]
    fn test_nested_dict() {
        let text = "{\"a\": \"aaa\", \"b\": {\"c\": \"ddd\"}, \"e\": 108}";
        let mut deserializer = Deserializer::new();
        let actual = deserializer.deserialize(text);

        assert_eq!(
            actual,
            Kvs {
                items: vec![
                    Kv {
                        path: Path::from("$"),
                        value: Tokens::MkDict
                    },
                    Kv {
                        path: Path::from("$.a"),
                        value: Tokens::String("aaa".to_string())
                    },
                    Kv {
                        path: Path::from("$.b"),
                        value: Tokens::MkDict
                    },
                    Kv {
                        path: Path::from("$.b.c"),
                        value: Tokens::String("ddd".to_string())
                    },
                    Kv {
                        path: Path::from("$.b"),
                        value: Tokens::EndDict
                    },
                    Kv {
                        path: Path::from("$.e"),
                        value: Tokens::Number(108)
                    },
                    Kv {
                        path: Path::from("$"),
                        value: Tokens::EndDict
                    },
                ],
            }
        );
    }
}
