use crate::core::yaml_serializer::line::Line;
use crate::core::yaml_serializer::processor::Processor;

pub struct MappingProcessor {
    lines: Vec<Line>,
}
impl MappingProcessor {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }
}

impl Processor for MappingProcessor {
    fn push(&mut self, line: &Line) {
        let mut converted = line.clone();

        self.lines.push(converted);
    }

    fn process(&self) -> Vec<Line> {
        self.lines.clone()
    }
}
