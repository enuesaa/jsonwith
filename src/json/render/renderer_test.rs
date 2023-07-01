#[cfg(test)]
mod tests {
    use crate::data::kv::Kv;
    use crate::data::kvs::Kvs;
    use crate::data::path::Path;
    use crate::data::tokens::Tokens;
    use crate::json::render::renderer::Renderer;
    use crate::json::render::process_indent::IndentProcessor;

    #[test]
    fn test_root_dict() {
        let mut serializer = Renderer::new(Kvs {
            items: vec![
                Kv {
                    path: Path::from("$"),
                    value: Tokens::MkDict,
                },
                Kv {
                    path: Path::from("$.a"),
                    value: Tokens::String("aaa".to_string()),
                },
                Kv {
                    path: Path::from("$"),
                    value: Tokens::EndDict,
                },
            ],
        });
        serializer.serialize();
        serializer.process(&mut IndentProcessor::new(2));
        let actual = serializer.get_raw();

        assert_eq!(actual, String::from("{\n  \"a\": \"aaa\"\n}\n"));
    }

    #[test]
    fn test_root_array() {
        let mut serializer = Renderer::new(Kvs {
            items: vec![
                Kv {
                    path: Path::from("$"),
                    value: Tokens::MkArray,
                },
                Kv {
                    path: Path::from("$[0]"),
                    value: Tokens::String("aaa".to_string()),
                },
                Kv {
                    path: Path::from("$"),
                    value: Tokens::EndArray,
                },
            ],
        });
        serializer.serialize();
        serializer.process(&mut IndentProcessor::new(2));
        let actual = serializer.get_raw();

        assert_eq!(actual, String::from("[\n  \"aaa\"\n]\n"));
    }

    #[test]
    fn test_root_string() {
        let mut serializer = Renderer::new(Kvs {
            items: vec![Kv {
                path: Path::from("$"),
                value: Tokens::String("aaa".to_string()),
            }],
        });
        serializer.serialize();
        serializer.process(&mut IndentProcessor::new(2));
        let actual = serializer.get_raw();

        assert_eq!(actual, String::from("\"aaa\"\n"));
    }

    #[test]
    fn test_root_number() {
        let mut serializer = Renderer::new(Kvs {
            items: vec![Kv {
                path: Path::from("$"),
                value: Tokens::Number(107),
            }],
        });
        serializer.serialize();
        serializer.process(&mut IndentProcessor::new(2));
        let actual = serializer.get_raw();

        assert_eq!(actual, String::from("107\n"));
    }

    #[test]
    fn test_nested_dict() {
        let mut serializer = Renderer::new(Kvs {
            items: vec![
                Kv {
                    path: Path::from("$"),
                    value: Tokens::MkDict,
                },
                Kv {
                    path: Path::from("$.a"),
                    value: Tokens::String("aaa".to_string()),
                },
                Kv {
                    path: Path::from("$.b"),
                    value: Tokens::MkDict,
                },
                Kv {
                    path: Path::from("$.b.c"),
                    value: Tokens::String("ddd".to_string()),
                },
                Kv {
                    path: Path::from("$.b"),
                    value: Tokens::EndDict,
                },
                Kv {
                    path: Path::from("$.e"),
                    value: Tokens::Number(108),
                },
                Kv {
                    path: Path::from("$"),
                    value: Tokens::EndDict,
                },
            ],
        });
        serializer.serialize();
        serializer.process(&mut IndentProcessor::new(2));
        let actual = serializer.get_raw();

        assert_eq!(
            actual,
            String::from(
                "{\n  \"a\": \"aaa\",\n  \"b\": {\n    \"c\": \"ddd\"\n  },\n  \"e\": 108\n}\n"
            )
        );
    }
}
