use crate::core::data::path::Path;

#[derive(PartialEq)]
enum ParseStatus {
    ParsingSpace,
    ParsingKey,
    ParsingInValue,
}

pub struct Carry {
    status: ParseStatus,
    path: Path,
}
impl Carry {
    pub fn new() -> Self {
        Carry {
            status: ParseStatus::ParsingSpace,
            path: Path::new(),
        }
    }

    pub fn in_space(&self) -> bool {
        self.status == ParseStatus::ParsingSpace
    }

    pub fn in_key(&self) -> bool {
        self.status == ParseStatus::ParsingKey
    }

    pub fn in_value(&self) -> bool {
        self.status == ParseStatus::ParsingInValue
    }

    pub fn deeper(&self, key: &str) {
        todo!()
    }

    pub fn upper(&self) {
        todo!()
    }
}
