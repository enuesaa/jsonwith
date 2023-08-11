use jsonwith::data::kv::Kv;
use jsonwith::data::kvs::Kvs;
use jsonwith::data::path::Path;
use jsonwith::data::tokens::Tokens;
use jsonwith::json::render::renderer::Renderer;
use jsonwith::json::render::process_indent::IndentProcessor;

#[test]
fn test_root_dict() {
    let mut serializer = Renderer::new(
        Kvs::from(vec![
            Kv::new(Path::from("$"), Tokens::MkDict),
            Kv::new(Path::from("$.a"), Tokens::String("aaa".to_string())),
            Kv::new(Path::from("$"), Tokens::EndDict),
        ]),
    );
    serializer.serialize();
    serializer.process(&mut IndentProcessor::new(2));
    let actual = serializer.get_raw();

    assert_eq!(actual, String::from("{\n  \"a\": \"aaa\"\n}\n"));
}

#[test]
fn test_root_array() {
    let mut serializer = Renderer::new(
        Kvs::from(vec![
            Kv::new(Path::from("$"), Tokens::MkArray),
            Kv::new(Path::from("$[0]"), Tokens::String("aaa".to_string())),
            Kv::new(Path::from("$"), Tokens::EndArray),
        ]),
    );
    serializer.serialize();
    serializer.process(&mut IndentProcessor::new(2));
    let actual = serializer.get_raw();

    assert_eq!(actual, String::from("[\n  \"aaa\"\n]\n"));
}

#[test]
fn test_root_string() {
    let mut serializer = Renderer::new(
        Kvs::from(vec![
            Kv::new(Path::from("$"), Tokens::String("aaa".to_string())),
        ]),
    );
    serializer.serialize();
    serializer.process(&mut IndentProcessor::new(2));
    let actual = serializer.get_raw();

    assert_eq!(actual, String::from("\"aaa\"\n"));
}

#[test]
fn test_root_number() {
    let mut serializer = Renderer::new(
        Kvs::from(vec![
            Kv::new(Path::from("$"), Tokens::Number(107)),
        ]),
    );
    serializer.serialize();
    serializer.process(&mut IndentProcessor::new(2));
    let actual = serializer.get_raw();

    assert_eq!(actual, String::from("107\n"));
}

#[test]
fn test_nested_dict() {
    let mut serializer = Renderer::new(
        Kvs::from(vec![
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
    serializer.process(&mut IndentProcessor::new(2));
    let actual = serializer.get_raw();

    assert_eq!(
        actual,
        String::from(
            "{\n  \"a\": \"aaa\",\n  \"b\": {\n    \"c\": \"ddd\"\n  },\n  \"e\": 108\n}\n"
        )
    );
}

