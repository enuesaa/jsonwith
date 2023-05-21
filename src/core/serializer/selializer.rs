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
                        carry.start_dict();
                        self.kvs.push(Kv::new("key", Tokens::MkDict))
                    },
                    '}' => {
                        carry.end_dict();
                    },
                    '[' => {
                        carry.start_array();
                        self.kvs.push(Kv::new("key", Tokens::MkArray))
                    },
                    ']' => {
                        carry.end_array();
                    },
                    '"' => {
                        if carry.should_start_parsing_key() {
                            carry.start_parsing_key();
                        } else {
                            carry.start_parsing_value();
                            carry.push(i);
                        }
                    },
                    // t, f, n, 0~9
                    _ => {}
                };

            } else if carry.in_key() {
                match i {
                    '"' => {
                        if carry.should_escape() {
                            carry.push(i);
                        } else {
                            carry.resolve_as_key();
                        }
                    },
                    _ => {
                        carry.push(i);
                    },
                }
            } else if carry.in_value() {

            } else {
                println!("unknown.")
            }
        }

        todo!()
    }
}
