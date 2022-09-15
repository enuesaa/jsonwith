pub trait ValueTrait {
    fn get_name(&self) -> &str {
        "others"
    }
}

pub struct DictModel {}
impl ValueTrait for DictModel {
    fn get_name(&self) -> &str {
        "dict"
    }
}

pub struct ArrayModel {}
impl ValueTrait for ArrayModel {
    fn get_name(&self) -> &str {
        "array"
    }
}

pub struct StringModel {}
impl ValueTrait for StringModel {
    fn get_name(&self) -> &str {
        "string"
    }
}

pub struct NumberModel {}
impl ValueTrait for NumberModel {
    fn get_name(&self) -> &str {
        "number"
    }
}

pub struct NullModel {}
impl ValueTrait for NullModel {
    fn get_name(&self) -> &str {
        "null"
    }
}

pub struct BooleanModel {}
impl ValueTrait for BooleanModel {
    fn get_name(&self) -> &str {
        "boolean"
    }
}
