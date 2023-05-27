use crate::core::data::kvs::Kvs;
use crate::core::data::kv::Kv;
use crate::core::data::tokens::Tokens;
use crate::core::serializer::context::Context;

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

    pub fn serialize(&mut self, text: &str) -> Kvs {

        let mut context = Context::new();
        for i in text.chars() {
            if context.in_space() {
                match i {
                    '{' => context.start_dict(),
                    '}' => context.end_dict(),
                    '[' => context.start_array(),
                    ']' => context.end_array(),
                    '"' => context.found_quotation(),
                    't'|'f'|'n'|'0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                        context.declare_value();
                        context.push(i);
                    },
                    _ => {},
                };

            } else if context.in_key() {
                match i {
                    '"' => {
                        if context.should_escape() {
                            context.push(i);
                        } else {
                            context.resolve();
                        };
                    },
                    _ => {
                        context.push(i);
                    },
                }

            } else if context.in_value() {
                match i {
                    '"' => {
                        if context.should_end_with_quotation() {
                            if context.should_escape() {
                                context.push(i);
                            } else {
                                context.push(i);
                                let value = context.get_buf();
                                // todo judge type
                                self.kvs.push(Kv::new(context.get_path(), Tokens::String(value)));
                                context.resolve();
                            }
                        } else {
                            context.push(i);
                        }
                    },
                    ',' => {
                        if context.should_end_with_quotation() {
                            context.push(i);
                        } else {
                            let value = context.get_buf();
                            // todo judge type
                            self.kvs.push(Kv::new(context.get_path(), Tokens::String(value)));
                            context.resolve();
                        }
                    },
                    _ => {
                        context.push(i);
                    }
                }

            } else {
                println!("unknown.")
            }
        };

        self.kvs.clone()
    }
}
