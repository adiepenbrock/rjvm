use crate::bytecode::attributes::Attribute;
use crate::bytecode::flags::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use crate::bytecode::pool::{ConstantPool, ConstantPoolIndex};

pub mod attributes;
pub mod descriptors;
pub mod flags;
pub mod pool;
pub mod reader;

#[derive(Debug, Clone, PartialEq)]
pub enum BytecodeError {
    ConstantPoolEntryAlreadyExists,
    ConstantPoolEntryNotFound,
    UnsupportedAttributeName,
    InvalidClassFile,
    UnexpectedEndOfData,
    InvalidData,
    UnsupportedInstruction,
    InvalidDescriptor,
    UnsupportedVerificationType,
}

#[derive(Debug)]
pub struct ClassFileVersion {
    pub minor: u16,
    pub major: u16,
}

#[derive(Debug)]
pub struct ClassFile {
    pub magic_number: u32,
    pub version: ClassFileVersion,
    pub constant_pool_count: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: ClassAccessFlags,
    pub this_class: ConstantPoolIndex,
    pub super_class: ConstantPoolIndex,
    pub interfaces_count: u16,
    pub interfaces: Vec<Interface>,
    pub fields_count: u16,
    pub fields: Vec<Field>,
    pub methods_count: u16,
    pub methods: Vec<Method>,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub descriptor: Descriptor,
    pub access_flags: FieldAccessFlags,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct Method {
    pub access_flags: MethodAccessFlags,
    pub name: String,
    pub descriptor: Vec<Descriptor>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct Interface {
    pub name_index: ConstantPoolIndex,
}

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
