use jsonwith::data::kv::Kv;
use jsonwith::data::kvs::Kvs;
use jsonwith::data::path::Path;
use jsonwith::data::tokens::Tokens;
use jsonwith::yaml::render::renderer::Renderer;

#[test]
fn test_root_dict() {
    let mut renderer = Renderer::new(
        Kvs::from(vec![
            Kv::with(Path::from("$"), Tokens::MkDict),
            Kv::with(Path::from("$.a"), Tokens::String("aaa".to_string())),
            Kv::with(Path::from("$"), Tokens::EndDict),
        ]),
    );
    renderer.render();
    let actual = renderer.get_raw();

    assert_eq!(actual, String::from("a: aaa\n"));
}

#[test]
fn test_root_array() {
    let mut renderer = Renderer::new(
        Kvs::from(vec![
            Kv::with(Path::from("$"), Tokens::MkArray),
            Kv::with(Path::from("$[0]"), Tokens::String("aaa".to_string())),
            Kv::with(Path::from("$"), Tokens::EndArray),
        ]),
    );
    renderer.render();
    let actual = renderer.get_raw();

    assert_eq!(actual, String::from("- aaa\n"));
}

#[test]
fn test_array_in_dict() {
    let mut renderer = Renderer::new(
        Kvs::from(vec![
            Kv::with(Path::from("$"), Tokens::MkDict),
            Kv::with(Path::from("$.a"), Tokens::MkArray),
            Kv::with(Path::from("$.a[0]"), Tokens::String("aaa".to_string())),
            Kv::with(Path::from("$"), Tokens::EndDict),
        ]),
    );
    renderer.render();
    let actual = renderer.get_raw();

    assert_eq!(actual, String::from("a: \n- aaa\n"));
}

#[test]
fn test_nested_dict() {
    let mut renderer = Renderer::new(
        Kvs::from(vec![
            Kv::with(Path::from("$"), Tokens::MkDict),
            Kv::with(Path::from("$.a"), Tokens::String("aaa".to_string())),
            Kv::with(Path::from("$.b"), Tokens::MkDict),
            Kv::with(Path::from("$.b.c"), Tokens::String("ddd".to_string())),
            Kv::with(Path::from("$.b"), Tokens::EndDict),
            Kv::with(Path::from("$.e"), Tokens::Number(108)),
            Kv::with(Path::from("$"), Tokens::EndDict),
        ]),
    );
    renderer.render();
    let actual = renderer.get_raw();

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
