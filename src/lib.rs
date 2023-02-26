pub mod json;
pub mod util;
pub mod yaml;

use crate::json::deserializer::Deserializer as JsonDeserializer;
use crate::json::serializer::Serializer as JsonSerializer;
use crate::yaml::deserializer::Deserializer as YamlDeserializer;

pub fn json2yaml(value: &str, indent: usize) -> String {
    let mut serializer = JsonSerializer::new();
    let values = serializer.serialize(value);

    let mut deserializer = YamlDeserializer::new();
    deserializer.indent = indent;
    deserializer.deserialize(values)
}

pub fn json2json(value: &str, indent: usize) -> String {
    let mut serializer = JsonSerializer::new();
    let values = serializer.serialize(value);

    let mut deserializer = JsonDeserializer::new();
    deserializer.indent = indent;
    deserializer.deserialize(values)
}
