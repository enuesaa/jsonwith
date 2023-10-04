use jsonwith::data::kv::Kv;
use jsonwith::data::kvs::Kvs;
use jsonwith::data::path::Path;
use jsonwith::data::tokens::Tokens;
use jsonwith::json::render::renderer::Renderer;

#[test]
fn test_root_dict() {
    let mut renderer = Renderer::new(Kvs::from(vec![
        Kv::with(Path::from("$"), Tokens::MkDict),
        Kv::with(Path::from("$.a"), Tokens::String("aaa".to_string())),
        Kv::with(Path::from("$"), Tokens::EndDict),
    ]));
    let actual = renderer.render();

    assert_eq!(actual, String::from("{\n  \"a\": \"aaa\"\n}\n"));
}

#[test]
fn test_root_array() {
    let mut renderer = Renderer::new(Kvs::from(vec![
        Kv::with(Path::from("$"), Tokens::MkArray),
        Kv::with(Path::from("$[0]"), Tokens::String("aaa".to_string())),
        Kv::with(Path::from("$"), Tokens::EndArray),
    ]));
    let actual = renderer.render();

    assert_eq!(actual, String::from("[\n  \"aaa\"\n]\n"));
}

#[test]
fn test_root_string() {
    let mut renderer = Renderer::new(Kvs::from(vec![Kv::with(
        Path::from("$"),
        Tokens::String("aaa".to_string()),
    )]));
    let actual = renderer.render();

    assert_eq!(actual, String::from("\"aaa\"\n"));
}

#[test]
fn test_root_number() {
    let mut renderer = Renderer::new(Kvs::from(vec![Kv::with(
        Path::from("$"),
        Tokens::Number(107),
    )]));
    let actual = renderer.render();

    assert_eq!(actual, String::from("107\n"));
}

#[test]
fn test_nested_dict() {
    let mut renderer = Renderer::new(Kvs::from(vec![
        Kv::with(Path::from("$"), Tokens::MkDict),
        Kv::with(Path::from("$.a"), Tokens::String("aaa".to_string())),
        Kv::with(Path::from("$.b"), Tokens::MkDict),
        Kv::with(Path::from("$.b.c"), Tokens::String("ddd".to_string())),
        Kv::with(Path::from("$.b"), Tokens::EndDict),
        Kv::with(Path::from("$.e"), Tokens::Number(108)),
        Kv::with(Path::from("$"), Tokens::EndDict),
    ]));
    let actual = renderer.render();

    assert_eq!(
        actual,
        String::from(
            "{\n  \"a\": \"aaa\",\n  \"b\": {\n    \"c\": \"ddd\"\n  },\n  \"e\": 108\n}\n"
        )
    );
}

#[test]
fn test_need_comma_after_end_dict() {
    let mut renderer = Renderer::new(Kvs::from(vec![
        Kv::with(Path::from("$"), Tokens::MkDict),
        Kv::with(Path::from("$.a"), Tokens::MkDict),
        Kv::with(Path::from("$.a"), Tokens::EndDict),
        Kv::with(Path::from("$.b"), Tokens::String("bbb".to_string())),
        Kv::with(Path::from("$"), Tokens::EndDict),
    ]));
    let actual = renderer.render();

    assert_eq!(
        actual,
        String::from(
            "{\n  \"a\": {},\n  \"b\": \"bbb\"\n}\n"
        )
    );
}

#[test]
fn test_donot_need_comma_last_end_dict() {
    let mut renderer = Renderer::new(Kvs::from(vec![
        Kv::with(Path::from("$"), Tokens::MkDict),
        Kv::with(Path::from("$.a"), Tokens::MkDict),
        Kv::with(Path::from("$.a"), Tokens::EndDict),
        Kv::with(Path::from("$"), Tokens::EndDict),
    ]));
    let actual = renderer.render();

    assert_eq!(
        actual,
        String::from(
            "{\n  \"a\": {}\n}\n"
        )
    );
}
