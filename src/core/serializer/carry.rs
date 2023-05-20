use crate::core::data::path::Path;

#[derive(PartialEq)]
enum ParseStatus {
    ParsingSpace,
    ParsingKey,
    ParsingInValue,
}

pub struct Carry {
    status: ParseStatus,
    // path: Path,
}
impl Carry {
    pub fn new() -> Self {
        Carry {
            status: ParseStatus::ParsingSpace,
        }
    }

    pub fn in_space(&self) -> bool {
        self.status == ParseStatus::ParsingSpace
    }
}
