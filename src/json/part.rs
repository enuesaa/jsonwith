use std::fmt;

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

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Part::String(a) => write!(f, "{}", a),
            Part::Boolean(b) => write!(f, "{}", b),
            Part::Number(c) => write!(f, "{}", c),
            Part::Null => write!(f, "null"),
            _ => write!(f, ""),
        }
    }
}
