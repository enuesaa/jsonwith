use crate::core::data::tokens::Tokens;
use crate::core::data::path::Path;

#[derive(Clone, Debug)]
pub struct Kv {
    path: Path,
    value: Tokens,
}

impl Kv {
    // immutable
    pub fn new(path: Path, value: Tokens) -> Self {
        Kv { path, value }
    }

    pub fn path(&self) -> Path {
        self.path.clone()
    }

    pub fn value(&self) -> Tokens {
        self.value.clone()
    }
}
