use crate::json::parts::{Parts, ScalarTypes, ScalarJudger};

pub struct Serializer {
    pub buff: Vec<Parts>,
    scalar_judger: ScalarJudger,
}
impl Serializer {
    pub fn new () -> Self {
        Serializer {buff: Vec::new(), scalar_judger: ScalarJudger::new()}
    }

    pub fn serialize (&mut self, val: &str) {
        for i in val.chars() {
            if ! self.scalar_judger.is_empty() {
                self.append_to_judger(i);
            } else {
                match i {
                    '{' => self.buff.push(Parts::StartDict),
                    '}' => self.buff.push(Parts::EndDict),
                    '[' => self.buff.push(Parts::StartList),
                    ']' => self.buff.push(Parts::EndList),
                    ',' => self.buff.push(Parts::Comma),
                    '\n' => {},
                    _ => {self.append_to_judger(i)}
                };
            }
        };
    }

    fn append_to_judger(&mut self, i: char) {
        let scalar_type = self.scalar_judger.append(&i);
        match scalar_type {
            ScalarTypes::NotDefined => {},
            _ => { self.buff.push(Parts::Scalar(scalar_type)) },
        }
    }
}
