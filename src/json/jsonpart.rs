#[derive(Debug, Clone, PartialEq)]
pub enum JsonPart {
    StartDict,
    StartList,
    String(String),
    Boolean(String),
    Null,
    Number(String),
    NotDefined,
}
