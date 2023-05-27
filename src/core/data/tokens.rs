
#[derive(Clone, Debug)]
pub enum Tokens {
    MkArray,
    MkDict,
    String(String),
    Number(usize),
    Bool(bool),
    Null,
}