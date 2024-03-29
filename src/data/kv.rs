use crate::data::path::Path;
use crate::data::tokens::Tokens;

#[derive(Clone, Debug, PartialEq)]
pub struct Kv {
    path: Path,
    value: Tokens,
}

impl Kv {
    pub fn with(path: Path, value: Tokens) -> Self {
        Self { path, value }
    }

    pub fn get_path(&self) -> Path {
        self.path.clone()
    }

    pub fn get_value(&self) -> Tokens {
        self.value.clone()
    }
}
