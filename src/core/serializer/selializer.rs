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
                        self.kvs.push(Kv::new(carry.get_path(), Tokens::MkDict))
                    },
                    '}' => {
                        carry.end_dict();
                    },
                    '[' => {
                        carry.start_array();
                        self.kvs.push(Kv::new(carry.get_path(), Tokens::MkArray))
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
                    't'|'f'|'n'|'0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                        carry.start_parsing_value();
                        carry.push(i);
                    },
                    _ => {},
                };

            } else if carry.in_key() {
                match i {
                    '"' => {
                        if carry.should_escape() {
                            carry.push(i);
                        } else {
                            carry.resolve();
                        };
                    },
                    _ => {
                        carry.push(i);
                    },
                }

            } else if carry.in_value() {
                match i {
                    '"' => {
                        if carry.should_end_with_quotation() {
                            if carry.should_escape() {
                                carry.push(i);
                            } else {
                                carry.push(i);
                                let value = carry.get_buf();
                                // todo judge type
                                self.kvs.push(Kv::new(carry.get_path(), Tokens::String(value)));
                                carry.resolve();
                            }
                        } else {
                            carry.push(i);
                        }
                    },
                    ',' => {
                        if carry.should_end_with_quotation() {
                            carry.push(i);
                        } else {
                            let value = carry.get_buf();
                            // todo judge type
                            self.kvs.push(Kv::new(carry.get_path(), Tokens::String(value)));
                            carry.resolve();
                        }
                    },
                    _ => {
                        carry.push(i);
                    }
                }

            } else {
                println!("unknown.")
            }
        };

        self.kvs.clone()
    }
}
