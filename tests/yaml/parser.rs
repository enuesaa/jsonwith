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
