pub mod core;

use crate::core::serializer::indent_processor::IndentProcessor;
use crate::core::deserializer::deserializer::Deserializer as Deserializerv2;
use crate::core::serializer::serializer::Serializer as Serializerv2;
use crate::core::yaml_serializer::serializer::Serializer as YamlSerializerv2;

pub fn json2jsonv2(value: &str) -> String {
    let kvs = Deserializerv2::new().deserialize(value);
    let raw = Serializerv2::new(kvs)
        .serialize()
        .process(&mut IndentProcessor::new(2))
        .get_raw();
    raw
}

pub fn json2yamlv2(value: &str) -> String {
    let kvs = Deserializerv2::new().deserialize(value);
    let raw = YamlSerializerv2::new(kvs)
        .serialize()
        .get_raw();
    raw
}
