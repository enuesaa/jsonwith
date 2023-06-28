use crate::json::render::line::Line;

pub trait Processor {
    fn push(&mut self, line: &Line);
    fn process(&self) -> Vec<Line>;
}
