#[cfg(test)]
mod tests {
    use crate::data::kv::Kv;
    use crate::data::kvs::Kvs;
    use crate::data::path::Path;
    use crate::data::tokens::Tokens;
    use crate::yaml::render::renderer::Renderer;

    #[test]
    fn test_root_dict() {
        let mut serializer = Renderer::new(
            Kvs::new(vec![
                Kv::new(Path::from("$"), Tokens::MkDict),
                Kv::new(Path::from("$.a"), Tokens::String("aaa".to_string())),
                Kv::new(Path::from("$"), Tokens::EndDict),
            ]),
        );
        serializer.serialize();
        let actual = serializer.get_raw();

        assert_eq!(actual, String::from("a: aaa\n"));
    }

    #[test]
    fn test_root_array() {
        let mut serializer = Renderer::new(
            Kvs::new(vec![
                Kv::new(Path::from("$"), Tokens::MkArray),
                Kv::new(Path::from("$[0]"), Tokens::String("aaa".to_string())),
                Kv::new(Path::from("$"), Tokens::EndArray),
            ]),
        );
        serializer.serialize();
        let actual = serializer.get_raw();

        assert_eq!(actual, String::from("- aaa\n"));
    }

    #[test]
    fn test_array_in_dict() {
        let mut serializer = Renderer::new(
            Kvs::new(vec![
                Kv::new(Path::from("$"), Tokens::MkDict),
                Kv::new(Path::from("$.a"), Tokens::MkArray),
                Kv::new(Path::from("$.a[0]"), Tokens::String("aaa".to_string())),
                Kv::new(Path::from("$"), Tokens::EndDict),
            ]),
        );
        serializer.serialize();
        let actual = serializer.get_raw();

        assert_eq!(actual, String::from("a: \n- aaa\n"));
    }

    #[test]
    fn test_nested_dict() {
        let mut serializer = Renderer::new(
            Kvs::new(vec![
                Kv::new(Path::from("$"), Tokens::MkDict),
                Kv::new(Path::from("$.a"), Tokens::String("aaa".to_string())),
                Kv::new(Path::from("$.b"), Tokens::MkDict),
                Kv::new(Path::from("$.b.c"), Tokens::String("ddd".to_string())),
                Kv::new(Path::from("$.b"), Tokens::EndDict),
                Kv::new(Path::from("$.e"), Tokens::Number(108)),
                Kv::new(Path::from("$"), Tokens::EndDict),
            ]),
        );
        serializer.serialize();
        let actual = serializer.get_raw();

        assert_eq!(
            actual,
            String::from(
                "a: aaa
b: 
  c: ddd
e: 108
"
            )
        );
    }

//     #[test]
//     fn test_array_in_array() {
//         let mut serializer = Serializer::new(Kvs {
//             items: vec![
//                 Kv {
//                     path: Path::from("$"),
//                     value: Tokens::MkDict,
//                 },
//                 Kv {
//                     path: Path::from("$.a"),
//                     value: Tokens::MkArray,
//                 },
//                 Kv {
//                     path: Path::from("$.a[0]"),
//                     value: Tokens::MkArray,
//                 },
//                 Kv {
//                     path: Path::from("$.a[0][0]"),
//                     value: Tokens::String("aa".to_string()),
//                 },
//                 Kv {
//                     path: Path::from("$.a[0]"),
//                     value: Tokens::EndArray,
//                 },
//                 Kv {
//                     path: Path::from("$.a"),
//                     value: Tokens::EndArray,
//                 },
//                 Kv {
//                     path: Path::from("$"),
//                     value: Tokens::EndDict,
//                 },
//             ],
//         });
//         serializer.serialize();
//         let actual = serializer.get_raw();

//         assert_eq!(
//             actual,
//             String::from(
//                 "a: 
// - - aa
// "
//             )
//         );
//     }
}
