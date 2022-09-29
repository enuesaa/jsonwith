use wasm_bindgen::prelude::*;

pub mod json;
pub mod util;
pub mod yaml;

use crate::json::deserializer::Deserializer as JsonDeserializer;
use crate::json::serializer::Serializer as JsonSerializer;
use crate::yaml::deserializer::Deserializer as YamlDeserializer;

/* @see https://github.com/rustwasm/wasm-bindgen/issues/2882 */
#[wasm_bindgen]
pub fn json2yaml(value: &str) -> String {
    let mut serializer = JsonSerializer::new();
    let values = serializer.serialize(&value);

    let mut deserializer = YamlDeserializer::new();
    let yaml_string = deserializer.deserialize(values);
    yaml_string
}

pub fn json2json(value: &str) -> String {
    let mut serializer = JsonSerializer::new();
    let values = serializer.serialize(&value);

    let mut deserializer = JsonDeserializer::new();
    let formatted = deserializer.deserialize(values);
    formatted
}
