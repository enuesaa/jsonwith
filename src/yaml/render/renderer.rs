use crate::data::kvs::Kvs;
use crate::yaml::render::line::Line;
use crate::yaml::render::processor::Processor;

use crate::yaml::render::process_mapping::MappingProcessor;
use crate::yaml::render::process_dictinarray::DictInArrayProcessor;

pub struct Renderer {
    lines: Vec<Line>,
}
impl Renderer {
    pub fn new(kvs: Kvs) -> Self {
        let lines: Vec<Line> = kvs.list().iter().map(|kv| Line::from(kv.clone())).collect();
        Renderer { lines }
    }

    pub fn render(&mut self) -> &mut Self {
        self.process(&mut MappingProcessor::new());
        self.process(&mut DictInArrayProcessor::new());
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