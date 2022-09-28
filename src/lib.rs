use wasm_bindgen::prelude::*;

pub mod json;
pub mod yaml;
pub mod util;

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
