pub mod json;
pub mod yaml;
pub mod core;

use crate::json::deserializer::Deserializer as JsonDeserializer;
use crate::json::serializer::Serializer as JsonSerializer;
use crate::yaml::deserializer::Deserializer as YamlDeserializer;

use crate::core::deserializer::deserializer::Deserializer as Deserializerv2;
use crate::core::serializer::serializer::Serializer as Serializerv2;

pub fn json2yaml(value: &str, indent: usize) -> String {
    let mut serializer = JsonSerializer::new();
    let values = serializer.serialize(value);

    let mut deserializer = YamlDeserializer::new();
    deserializer.indent = indent;
    deserializer.deserialize(values)
}

pub fn json2yamlv2(value: &str) {
    let kvs = Deserializerv2::new().deserialize(value);
    let raw = Serializerv2::new().serialize(kvs);
    println!("{}", raw);
}

pub fn json2json(value: &str, indent: usize) -> String {
    let mut serializer = JsonSerializer::new();
    let values = serializer.serialize(value);

    let mut deserializer = JsonDeserializer::new();
    deserializer.indent = indent;
    deserializer.deserialize(values)
}
