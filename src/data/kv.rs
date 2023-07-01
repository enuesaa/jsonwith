use crate::data::path::Path;
use crate::data::tokens::Tokens;

// immutable
#[derive(Clone, Debug, PartialEq)]
pub struct Kv {
    pub path: Path,
    pub value: Tokens,
}

impl Kv {
    pub fn get_path(&self) -> Path {
        self.path.clone()
    }

    pub fn get_value(&self) -> Tokens {
        self.value.clone()
    }
}
