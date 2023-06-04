use crate::core::data::kvs::Kvs;

// json serializer for now.
pub struct Serializer {
    indent: usize,
}
impl Serializer {
    pub fn new() -> Self {
        Serializer { indent: 2 }
    }

    // configure options like this.
    pub fn set_indent(&mut self, indent: usize) {
        self.indent = indent;
    }

    pub fn serialize(&mut self, kvs: Kvs) -> String {
        for kv in kvs.list() {
            println!("{:?}", kv);
        };

        String::from("")
    }
}
