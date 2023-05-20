use crate::core::data::kvs::Kvs;
use crate::core::data::kv::Kv;
use crate::core::data::tokens::Tokens;
use crate::core::serializer::carry::Carry;

// todo: create serializer trait

pub struct Serializer {
    indent: usize,
    kvs: Kvs,
}
impl Serializer {
    pub fn new() -> Self {
        Serializer {
            indent: 2,
            kvs: Kvs::new(),
        }
    }

    // append serialize options like this.
    pub fn set_indent(&mut self, indent: usize) {
        self.indent = indent;
    }

    // perform json2yaml... for development...
    pub fn serialize(&mut self, text: &str) -> Kvs {

        let mut carry = Carry::new();

        for i in text.chars() {
            if carry.in_space() {
                match i {
                    '{' => {
                        self.kvs.push(Kv::new("key", Tokens::MkDict))
                    },
                    '[' => {
                        self.kvs.push(Kv::new("key", Tokens::MkArray))
                    },
                    _ => {}
                };
            }
        }

        todo!()

        // 1文字ずつ読み取って..
    }
}
