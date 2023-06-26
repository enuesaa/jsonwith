use crate::core::data::kvs::Kvs;
use crate::core::serializer::line::Line;
use crate::core::serializer::processor::Processor;

use crate::core::serializer::mapping_processor::MappingProcessor;

pub struct Serializer {
    lines: Vec<Line>,
}
impl Serializer {
    pub fn new(kvs: Kvs) -> Self {
        let lines: Vec<Line> = kvs.list().iter().map(|kv| Line::from(kv.clone())).collect();
        Serializer { lines }
    }

    pub fn serialize(&mut self) -> &mut Self {
        self.process(&mut MappingProcessor::new());
        self
    }

    pub fn process<T: Processor>(&mut self, processor: &mut T) -> &mut Self {
        for line in self.lines.clone() {
            processor.push(&line);
        }
        self.lines = processor.process();
        self
    }

    pub fn get_raw(&self) -> String {
        self.lines
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}
