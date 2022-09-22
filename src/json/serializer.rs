use crate::json::model::{Model, ModelValue};
use crate::json::parts::{Parts, ScalarJudger, ScalarTypes, Scalar};

#[derive(Clone)]
pub struct Serializer {
    pub buff: Vec<Parts>,
    model: Option<Model>,
}
impl Serializer {
    pub fn new(val: &str) -> Self {
        let mut serializer = Serializer {
            buff: Vec::new(),
            model: None,
        };
        serializer.parse_val(val);
        serializer.print_yaml();
        serializer
    }

    fn parse_val(&mut self, val: &str) {
        let mut scalar_judger = ScalarJudger::new();
        for i in val.chars() {
            // @todo fix. this code checks `scalar judger is initialized`.
            if !scalar_judger.resolved {
                scalar_judger.resolve_next(&i);
                if scalar_judger.resolved {
                    self.buff.push(Parts::Scalar(Scalar{scalar_type: scalar_judger.clone().scalar_type, value: scalar_judger.clone().get_value()}));
                    scalar_judger = ScalarJudger::new();
                    match i {
                        '{' => self.buff.push(Parts::StartDict),
                        '}' => self.buff.push(Parts::EndDict),
                        '[' => self.buff.push(Parts::StartList),
                        ']' => self.buff.push(Parts::EndList),
                        ',' => self.buff.push(Parts::Comma),
                        ':' => self.buff.push(Parts::Colon),
                        '\n' => {}
                        ' ' => {}
                        _ => {}
                    };
                }
            } else {
                match i {
                    '{' => self.buff.push(Parts::StartDict),
                    '}' => self.buff.push(Parts::EndDict),
                    '[' => self.buff.push(Parts::StartList),
                    ']' => self.buff.push(Parts::EndList),
                    ',' => self.buff.push(Parts::Comma),
                    ':' => self.buff.push(Parts::Colon),
                    '\n' => {}
                    ' ' => {}
                    _ => {
                        scalar_judger.resolve_next(&i);
                        if scalar_judger.resolved {
                            self.buff.push(Parts::Scalar(Scalar{scalar_type: scalar_judger.clone().scalar_type, value: scalar_judger.clone().get_value()}));
                            scalar_judger = ScalarJudger::new();
                        }
                    }
                };
            }
        }
    }

    #[allow(dead_code)]
    fn convert_model(&mut self) {
        let buff = self.buff.clone();
        for buf in buff {
            println!("{:?}", buf);
            if buf == Parts::StartDict {
                self.model = Some(Model::DictModel(ModelValue::new()));
            }
        }
        println!("{:?}", self.model);
    }

    fn print_yaml(&mut self) {
        let buff = self.buff.clone();
        println!("{:?}", buff);
        let mut space_count: usize = 0;
        let mut need_colon: bool = false;
        for buf in buff {
            if buf == Parts::StartDict {
                println!();
                space_count += 2;
                need_colon = false;
            }
            if buf == Parts::EndDict {
                space_count -= 2;
            }
            if buf == Parts::StartList {
                println!();
                space_count += 2;
                need_colon = false;
            }
            if buf == Parts::EndList {
                space_count -= 2;
            }
            // if buf == Parts::Scalar(ScalarTypes::String) {
            //     if !need_colon {
            //         print!("{}", " ".repeat(space_count));
            //     }
            //     print!("string");
            //     need_colon = !need_colon;
            // }
            // if buf == Parts::Scalar(ScalarTypes::Number) {
            //     print!("number");
            //     need_colon = false;
            // }
            // if buf == Parts::Scalar(ScalarTypes::Boolean) {
            //     print!("bool");
            //     need_colon = false;
            // }
            // if buf == Parts::Scalar(ScalarTypes::Null) {
            //     print!("null");
            //     need_colon = false;
            // }
            if buf == Parts::Colon {
                print!(": ");
            }
            if buf == Parts::Comma {
                println!();
            }
        }
        println!("\n");
    }
}
