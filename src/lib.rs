pub mod core;
pub mod json;
pub mod yaml;
pub mod cli;

use crate::json::parse::parser::Deserializer;
use crate::json::render::indent::IndentProcessor;
use crate::json::render::renderer::Serializer;
use crate::yaml::render::renderer::Serializer as YamlSerializer;

pub fn json2json(value: &str) -> String {
    let kvs = Deserializer::new().deserialize(value);
    let raw = Serializer::new(kvs)
        .serialize()
        .process(&mut IndentProcessor::new(2))
        .get_raw();
    raw
}

pub fn json2yaml(value: &str) -> String {
    let kvs = Deserializer::new().deserialize(value);
    let raw = YamlSerializer::new(kvs).serialize().get_raw();
    raw
}
