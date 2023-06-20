use crate::core::serializer::line::Line;

pub trait Processor {
    fn process(&self, lines: Vec<Line>) -> Vec<Line>;
}

pub struct MappingProcessor {}
impl Processor for MappingProcessor {
    fn process(&self, lines: Vec<Line>) -> Vec<Line> {
        lines
    }
}

pub struct IndentProcessor {
    indent: usize,
}
impl Processor for IndentProcessor {
    fn process(&self, lines: Vec<Line>) -> Vec<Line> {
        lines
    }
}
