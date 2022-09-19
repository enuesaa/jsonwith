use crate::json::parts::Parts;

#[derive(Debug, Clone)]
pub struct ModelValue {
  parts: Vec<Parts>,
}
impl ModelValue {
  pub fn new () -> Self {
    ModelValue { parts: Vec::new() }
  }
  pub fn push(&mut self, part: Parts) {
    self.parts.push(part);
  }
}

#[derive(Debug, Clone)]
pub enum Model {
  DictModel(ModelValue),
  ListModel(ModelValue),
  StringModel(ModelValue),
  BoolModel(ModelValue),
  NullModel(ModelValue),
  NumberModel(ModelValue),
}
