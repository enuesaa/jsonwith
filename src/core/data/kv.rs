use crate::core::data::tokens::Tokens;

pub struct Kv {
    pub key: String,
    pub value: Tokens,
}

pub struct KvCollection {
    pub items: Vec<Kv>,
}