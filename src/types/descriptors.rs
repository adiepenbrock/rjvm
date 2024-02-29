#[derive(Debug, Clone, PartialEq)]
pub struct MethodDescriptor {
    pub parameters: Vec<FieldType>,
    pub return_ty: Option<FieldType>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    Base(BaseType),
    Object(String),
    Array(Box<FieldType>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BaseType {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Boolean,
}
