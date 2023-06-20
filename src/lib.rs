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

pub fn json2jsonv2(value: &str) -> String {
    let kvs = Deserializerv2::new().deserialize(value);
    let raw = Serializerv2::new().serialize(kvs);
    raw

    // to_string の命名が綺麗だが serializer だと違和感があるので命名を変えてもいいかも

    // let raw = Serializerv2::with(kvs)
    //     .process(IndentProcessor::new())
    //     .process(OtherProcessor::new())
    //     .get_raw()
}

pub fn json2json(value: &str, indent: usize) -> String {
    let mut serializer = JsonSerializer::new();
    let values = serializer.serialize(value);

    let mut deserializer = JsonDeserializer::new();
    deserializer.indent = indent;
    deserializer.deserialize(values)
}
