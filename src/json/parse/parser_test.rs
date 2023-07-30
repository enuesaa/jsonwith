#[cfg(test)]
mod tests {
    use crate::data::kv::Kv;
    use crate::data::kvs::Kvs;
    use crate::data::path::Path;
    use crate::data::tokens::Tokens;
    use crate::json::parse::parser::Parser;

    #[test]
    fn test_root_dict() {
        let text = "{\"a\": \"aaa\"}";
        let mut deserializer = Parser::new();
        let actual = deserializer.deserialize(text);

        assert_eq!(
            actual,
            Kvs {
                items: vec![
                    Kv::new(Path::from("$"), Tokens::MkDict),
                    Kv::new(Path::from("$.a"), Tokens::String("aaa".to_string())),
                    Kv::new(Path::from("$"), Tokens::EndDict),
                ],
            }
        );
    }

    #[test]
    fn test_root_array() {
        let text = "[\"aaa\"]";
        let mut deserializer = Parser::new();
        let actual = deserializer.deserialize(text);

        assert_eq!(
            actual,
            Kvs {
                items: vec![
                    Kv::new(Path::from("$"), Tokens::MkArray),
                    Kv::new(Path::from("$[0]"), Tokens::String("aaa".to_string())),
                    Kv::new(Path::from("$"), Tokens::EndArray),
                ],
            }
        );
    }

    #[test]
    fn test_root_string() {
        let text = "\"aaa\"";
        let mut deserializer = Parser::new();
        let actual = deserializer.deserialize(text);

        assert_eq!(
            actual,
            Kvs {
                items: vec![Kv::new(Path::from("$"), Tokens::String("aaa".to_string()))],
            }
        );
    }

    #[test]
    fn test_root_number() {
        let text = "107";
        let mut deserializer = Parser::new();
        let actual = deserializer.deserialize(text);
        println!("{:?}", actual);

        assert_eq!(
            actual,
            Kvs {
                items: vec![Kv::new(Path::from("$"), Tokens::Number(107))],
            }
        );
    }

    #[test]
    fn test_nested_dict() {
        let text = "{\"a\": \"aaa\", \"b\": {\"c\": \"ddd\"}, \"e\": 108}";
        let mut deserializer = Parser::new();
        let actual = deserializer.deserialize(text);

        assert_eq!(
            actual,
            Kvs {
                items: vec![
                    Kv::new(Path::from("$"), Tokens::MkDict),
                    Kv::new(Path::from("$.a"), Tokens::String("aaa".to_string())),
                    Kv::new(Path::from("$.b"), Tokens::MkDict),
                    Kv::new(Path::from("$.b.c"), Tokens::String("ddd".to_string())),
                    Kv::new(Path::from("$.b"), Tokens::EndDict),
                    Kv::new(Path::from("$.e"), Tokens::Number(108)),
                    Kv::new(Path::from("$"), Tokens::EndDict),
                ],
            }
        );
    }
}
