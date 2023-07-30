use crate::data::path::Path;
use crate::data::tokens::Tokens;

// immutable
#[derive(Clone, Debug, PartialEq)]
pub struct Kv {
    path: Path,
    value: Tokens,
}

impl Kv {
    pub fn new(path: Path, value: Tokens) -> Self {
        Self {
            path,
            value,
        }
    }

    pub fn get_path(&self) -> Path {
        self.path.clone()
    }

    pub fn get_value(&self) -> Tokens {
        self.value.clone()
    }
}
