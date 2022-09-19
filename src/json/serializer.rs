use crate::json::parts::{Parts, ScalarJudger};
use crate::json::model::{Model, ModelValue};

#[derive(Clone)]
pub struct Serializer {
    pub buff: Vec<Parts>,
    model: Option<Model>,
}
impl Serializer {
    pub fn new (val: &str) -> Self {
        let mut serializer = Serializer {buff: Vec::new(), model: None};
        serializer.parse_val(val);
        serializer.to_model();
        serializer
    }

    fn parse_val (&mut self, val: &str) {
        let mut scalar_judger = ScalarJudger::new();
        for i in val.chars() {
            // @todo fix. this code checks `scalar judger is initialized`.
            if scalar_judger.resolved == false {
                scalar_judger.resolve_next(&i);
                if scalar_judger.resolved == true {
                    self.buff.push(Parts::Scalar(scalar_judger.scalar_type));
                    scalar_judger = ScalarJudger::new();
                }
            } else {
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
                        scalar_judger.resolve_next(&i);
                        if scalar_judger.resolved == true {
                            self.buff.push(Parts::Scalar(scalar_judger.scalar_type));
                            scalar_judger = ScalarJudger::new();
                        }
                    },
                };
            }
        }
    }

    fn to_model (&mut self) {
        let buff = self.buff.clone();
        let mut model :Option<Model> = None;
        for buf in buff {
            println!("{:?}", buf);
            if buf == Parts::StartDict {
                model = Some(Model::DictModel(ModelValue::new()));
            }
            if buf == Parts::StartList {
                model = Some(Model::ListModel(ModelValue::new()));
            }
        }
        println!("{:?}", self.model);
    }
}
