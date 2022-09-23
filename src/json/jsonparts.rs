#[derive(Debug, Clone, PartialEq)]
pub enum JsonParts {
    StartDict,
    StartList,
    String(String),
    Boolean(String),
    Null,
    Number(String),
    NotDefined,
}
