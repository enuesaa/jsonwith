use crate::json::part::Part;
use crate::json::path::Path;
use crate::json::scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub path: Path,
    pub part: Part,
}
impl Value {
    pub fn from_scalar(path: &Path, scalar: Scalar) -> Self {
        Value { path: path.clone(), part: scalar.part }
    }
}