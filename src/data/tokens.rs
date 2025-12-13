#[derive(Clone, Debug, PartialEq)]
pub enum Tokens {
    String(String),
    Number(f64),
    Bool(bool),
    Null,
    MkArray,
    EndArray,
    MkDict,
    EndDict,
}
