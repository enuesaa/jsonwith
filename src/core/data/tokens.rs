#[derive(Clone, Debug, PartialEq)]
pub enum Tokens {
    MkArray,
    EndArray,
    MkDict,
    EndDict,
    String(String),
    Number(usize),
    Bool(bool),
    Null,
}
