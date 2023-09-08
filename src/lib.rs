pub mod data;
pub mod json;
pub mod yaml;

use crate::json::parse::parser::Parser as JsonParser;
use crate::json::render::renderer::Renderer as JsonRenderer;
use crate::json::render::process_indent::IndentProcessor as JsonIndentProcessor;
use crate::yaml::render::renderer::Renderer as YamlRenderer;

pub fn jsonformat(value: &str, indent: usize) -> String {
    let kvs = JsonParser::new().parse(value);
    let raw = JsonRenderer::new(kvs)
        .render()
        .process(&mut JsonIndentProcessor::new(indent))
        .get_raw();
    raw
}

pub fn json2yaml(value: &str, indent: usize) -> String {
    let kvs = JsonParser::new().parse(value);
    let raw = YamlRenderer::new(kvs).configure_indent(indent).render().get_raw();
    raw
}
