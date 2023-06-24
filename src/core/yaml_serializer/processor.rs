use crate::core::yaml_serializer::line::Line;

pub trait Processor {
    fn push(&mut self, line: &Line);
    fn process(&self) -> Vec<Line>;
}
