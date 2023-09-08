use crate::data::kvs::Kvs;
use crate::json::render::line::Line;
use crate::json::render::processor::Processor;
use crate::json::render::process_mapping::MappingProcessor;

use super::process_indent::IndentProcessor;

pub struct Renderer {
    lines: Vec<Line>,
    indent: usize,
}
impl Renderer {
    pub fn new(kvs: Kvs) -> Self {
        let lines: Vec<Line> = kvs.list().iter().map(|kv| Line::from(kv.clone())).collect();
        Renderer { lines, indent: 2 }
    }

    pub fn with_indent(&mut self, indent: usize) -> &mut Self {
        self.indent = indent;
        self
    }

    pub fn render(&mut self) -> String {
        self.process(&mut MappingProcessor::new());
        self.process(&mut IndentProcessor::new(self.indent));
        self.get_raw()
    }

    fn process<T: Processor>(&mut self, processor: &mut T) -> &mut Self {
        for line in self.lines.clone() {
            processor.push(&line);
        }
        self.lines = processor.process();
        self
    }

    fn get_raw(&self) -> String {
        self.lines
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}
