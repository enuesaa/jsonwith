use jsonwith::data::kv::Kv;
use jsonwith::data::kvs::Kvs;
use jsonwith::data::path::Path;
use jsonwith::data::tokens::Tokens;
use jsonwith::yaml::parse::parser::Parser;

#[test]
fn test_normal() {
    let text = "a: aaa";
    let mut parser = Parser::new();
    let actual = parser.parse(text);

    assert_eq!(
        actual,
        Kvs::from(vec![
            Kv::with(Path::from("$"), Tokens::MkDict),
            Kv::with(Path::from("$.a"), Tokens::String("aaa".to_string())),
            Kv::with(Path::from("$"), Tokens::EndDict),
        ]),
    );
}

#[test]
fn test_number() {
    let text = "a: 99";
    let mut parser = Parser::new();
    let actual = parser.parse(text);

    assert_eq!(
        actual,
        Kvs::from(vec![
            Kv::with(Path::from("$"), Tokens::MkDict),
            Kv::with(Path::from("$.a"), Tokens::Number(99)),
            Kv::with(Path::from("$"), Tokens::EndDict),
        ]),
    );
}

#[test]
fn test_bool() {
    let text = "a: true";
    let mut parser = Parser::new();
    let actual = parser.parse(text);

    assert_eq!(
        actual,
        Kvs::from(vec![
            Kv::with(Path::from("$"), Tokens::MkDict),
            Kv::with(Path::from("$.a"), Tokens::Bool(true)),
            Kv::with(Path::from("$"), Tokens::EndDict),
        ]),
    );
}

#[test]
fn test_null() {
    let text = "a: null";
    let mut parser = Parser::new();
    let actual = parser.parse(text);

    assert_eq!(
        actual,
        Kvs::from(vec![
            Kv::with(Path::from("$"), Tokens::MkDict),
            Kv::with(Path::from("$.a"), Tokens::Null),
            Kv::with(Path::from("$"), Tokens::EndDict),
        ]),
    );
}
