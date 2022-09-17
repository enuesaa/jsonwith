use crate::json::parts::{Parts, ScalarJudger};

pub struct Serializer {
    pub buff: Vec<Parts>,
}
impl Serializer {
    pub fn new (val: &str) -> Self {
        let mut serializer = Serializer {buff: Vec::new()};
        serializer.serialize(val);
        serializer
    }

    fn serialize (&mut self, val: &str) {
        let mut scalar_judger = ScalarJudger::new();
        for i in val.chars() {
            if scalar_judger.resolved == false {
                scalar_judger.append(&i);
                if scalar_judger.resolved == true {
                    self.buff.push(Parts::Scalar(scalar_judger.scalar_type));
                    scalar_judger = ScalarJudger::new();
                }
            } else {
                // このへんで sclar値 として成り立つか判断する処理 が必要
                match i {
                    '{' => self.buff.push(Parts::StartDict),
                    '}' => self.buff.push(Parts::EndDict),
                    '[' => self.buff.push(Parts::StartList),
                    ']' => self.buff.push(Parts::EndList),
                    ',' => self.buff.push(Parts::Comma),
                    ':' => self.buff.push(Parts::Colon),
                    '\n' => {},
                    ' ' => {},
                    _ => {
                        scalar_judger.append(&i);
                        if scalar_judger.resolved == true {
                            self.buff.push(Parts::Scalar(scalar_judger.scalar_type));
                            scalar_judger = ScalarJudger::new();
                        }
                    },
                };
            }
        }
    }
}
