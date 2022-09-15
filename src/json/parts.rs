
#[derive(Debug, Clone)]
pub enum ScalarTypes {
    NotDefined,
    String,
    Boolean,
    Null,
    Number,
}

/** https://qiita.com/togatoga/items/9d600e20325775f09547 */
#[derive(Debug)]
pub enum Parts {
    StartDict, // {
    EndDict,   // }
    StartList, // [
    EndList,   // ]
    Comma,     // ,
    Scalar(ScalarTypes),    // "aa", 99, null, true, false
    Others,    // temporary. for development.
}
