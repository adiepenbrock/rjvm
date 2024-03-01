#[derive(Debug, Clone, PartialEq)]
pub enum DescriptorKind {
    Parameter,
    Return,
    Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Descriptor {
    pub kind: DescriptorKind,
    pub ty: FieldType,
}

impl std::fmt::Display for Descriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ty)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    Base(BaseType),
    Object(String),
    Array(Box<FieldType>),
}

impl std::fmt::Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldType::Base(ty) => write!(f, "{}", ty),
            FieldType::Object(ty) => write!(f, "{}", ty),
            FieldType::Array(ty) => write!(f, "[{}]", ty),
        }
    }
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
    Void,
}

impl std::fmt::Display for BaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BaseType::Byte => write!(f, "byte"),
            BaseType::Char => write!(f, "char"),
            BaseType::Double => write!(f, "double"),
            BaseType::Float => write!(f, "float"),
            BaseType::Int => write!(f, "int"),
            BaseType::Long => write!(f, "long"),
            BaseType::Short => write!(f, "short"),
            BaseType::Boolean => write!(f, "boolean"),
            BaseType::Void => write!(f, "void"),
        }
    }
}
