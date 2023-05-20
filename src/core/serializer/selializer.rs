
use crate::core::data::collection::Collection;

// todo: create serializer trait

pub struct Serializer {
    indent: usize,
}
impl Serializer {
    pub fn new() -> Self {
        Serializer {
            indent: 2,
        }
    }

    // append serialize options like this.
    pub fn set_indent(&mut self, indent: usize) {
        self.indent = indent;
    }

    // perform json2yaml... for development...
    pub fn serialize(&self, text: &str) -> Collection {
        todo!()

        // 1文字ずつ読み取って..
    }
}
