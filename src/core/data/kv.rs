use crate::core::data::tokens::Tokens;
use crate::core::data::path::Path;

// immutable
// generics にするか迷う
#[derive(Clone, Debug)]
pub struct Kv {
    path: Path,
    value: Tokens,
}

impl Kv {
    pub fn new(path: Path, value: Tokens) -> Self {
        Kv { path, value: value }
    }

    pub fn mkarray(path: Path) -> Self {
        Kv { path, value: Tokens::MkArray }
    }

    pub fn mkdict(path: Path) -> Self {
        Kv { path, value: Tokens::MkDict }
    }

    pub fn get_path(&self) -> Path {
        self.path.clone()
    }

    pub fn get_value(&self) -> Tokens {
        self.value.clone()
    }
}
