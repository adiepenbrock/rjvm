#[derive(Debug, Clone, PartialEq)]
pub enum DecodingError {
    InvalidClassFile,
    InvalidAccessFlags,
    InvalidConstantPoolIndex,
    UnsupportedAttributeName,
}
