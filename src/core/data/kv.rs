use crate::core::data::tokens::Tokens;

#[derive(Clone)]
pub struct Kv {
    path: String,
    value: Tokens,
}

impl Kv {
    // immutable
    pub fn new(key: &str, value: Tokens) -> Self {
        Kv { path: key.to_string(), value }
    }

    pub fn key(&self) -> String {
        self.path.clone()
    }

    pub fn value(&self) -> Tokens {
        self.value.clone()
    }
}
