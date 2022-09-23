use crate::json::part::Part;
use crate::json::path::Path;
use crate::json::scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub path: Path,
    pub part: Part,
}
impl Value {
    pub fn new(path: &Path, part: Part) -> Self {
        Value { path: path.clone(), part }
    }

    pub fn start_dict(path: &Path) -> Self {
        Value { path: path.clone(), part: Part::StartDict }
    }

    pub fn start_list(path: &Path) -> Self {
        Value { path: path.clone(), part: Part::StartList }
    }

    pub fn scalar(path: &Path, scalar: Scalar) -> Self {
        Value { path: path.clone(), part: scalar.part }
    }
}