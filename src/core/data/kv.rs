use crate::core::data::tokens::Tokens;

#[derive(Clone)]
pub struct Kv {
    key: String,
    value: Tokens,
}

impl Kv {
    // immutable
    pub fn new(key: &str, value: Tokens) -> Self {
        Kv { key: key.to_string(), value }
    }

    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn value(&self) -> Tokens {
        self.value.clone()
    }
}
