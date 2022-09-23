use crate::json::parts::{Parts, ScalarJudger, Scalar};

#[derive(Clone)]
pub struct Serializer {
    pub buff: Vec<Parts>
}
impl Serializer {
    pub fn new(json_string: &str) -> Self {
        let mut serializer = Serializer {buff: Vec::new()};
        serializer.parse(json_string);
        serializer
    }

    fn parse(&mut self, json_string: &str) {
        let mut scalar_judger = ScalarJudger::new();
        for i in json_string.chars() {
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
}
