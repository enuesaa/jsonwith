use crate::json::value::Value;

pub struct Deserializer {
    pub values: Vec<Value>,
}
impl Deserializer {
    pub fn new(values: Vec<Value>) -> Self {
        Deserializer{values}
    }

    pub fn print(&mut self) {
        let values = self.values.clone();
        for mut i in values {
            println!("{}\t {:?}", i.path.to_string(), i.part);
        }
    }
}