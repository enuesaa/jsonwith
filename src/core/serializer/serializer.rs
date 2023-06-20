use crate::core::data::kvs::Kvs;
use crate::core::serializer::line::Line;
use crate::core::serializer::processor::Processor;

pub struct Serializer {
    lines: Vec<Line>
}
impl Serializer {
    pub fn with(kvs: Kvs) -> Self {
        let lines: Vec<Line> = kvs.list().iter()
            .map(|kv| Line::from(kv.clone()))
            .collect();
        Serializer { lines }
    }

    pub fn process<T: Processor>(&mut self, processor: T) -> &mut Self {
        self.lines = processor.process(&mut self.lines);
        self
    }

    pub fn get_raw(&self) -> String {
        self.lines.iter().map(|l| l.to_string()).collect::<Vec<String>>().join("")   
    }
}
