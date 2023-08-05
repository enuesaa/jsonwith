#[derive(Clone, Debug, PartialEq)]
pub enum Tokens {
    String(String),
    Number(usize),
    Bool(bool),
    Null,
    MkArray,
    EndArray,
    MkDict,
    EndDict,
}
