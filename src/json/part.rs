#[derive(Debug, Clone, PartialEq)]
pub enum Part {
    StartDict,
    StartList,
    String(String),
    Boolean(bool),
    Null,
    Number(usize),
    NotDefined,
}
