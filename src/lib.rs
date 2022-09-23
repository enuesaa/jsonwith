use wasm_bindgen::prelude::*;

pub mod json;
pub mod yaml;

#[wasm_bindgen]
pub fn greet(value: &str) -> String {
  // let mut serializer = Serializer::new();
  // serializer.serialize(&value);

  // let values = serializer.values.clone();
  // let mut deserializer = Deserializer::new(values);
  // deserializer.print();
  String::from("a") + value
}
