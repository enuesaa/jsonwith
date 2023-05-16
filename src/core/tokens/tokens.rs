use super::{array::ArrayToken, string::StringToken, bool::BoolToken, dict::DictToken, null::NullToken, number::NumberToken};

pub enum Tokens {
    ArrayToken(ArrayToken),
    StringToken(StringToken),
    BoolToken(BoolToken),
    DictToken(DictToken),
    NullToken(NullToken),
    NumberToken(NumberToken),
}
