pub mod data;
pub mod json;
pub mod yaml;

use crate::json::parse::parser::Parser as JsonParser;
use crate::json::render::renderer::Renderer as JsonRenderer;
use crate::yaml::parse::parser::Parser as YamlParser;
use crate::yaml::render::renderer::Renderer as YamlRenderer;

pub fn jsonformat(value: &str, indent: usize) -> String {
    let kvs = JsonParser::new().parse(value);
    let raw = JsonRenderer::new(kvs).with_indent(indent).render();
    raw
}

pub fn json2yaml(value: &str, indent: usize) -> String {
    let kvs = JsonParser::new().parse(value);
    let raw = YamlRenderer::new(kvs).with_indent(indent).render();
    raw
}

pub fn yaml2json(value: &str, indent: usize) -> String {
    let kvs = YamlParser::new().parse(value);
    let raw = JsonRenderer::new(kvs).with_indent(indent).render();
    raw
}
