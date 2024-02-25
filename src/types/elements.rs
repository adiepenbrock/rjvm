use crate::types::{
    attributes::Attribute,
    constants::ConstantPool,
    flags::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags},
};

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
    pub this_class: u16,
    pub super_class: u16,
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
    pub descriptor: String,
    pub access_flags: FieldAccessFlags,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct Method {
    pub access_flags: MethodAccessFlags,
    pub name: String,
    pub descriptor: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct Interface {
    pub name_index: u16,
}
