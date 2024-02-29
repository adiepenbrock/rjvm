use crate::{
    decoder::{buffer::BufferedReader, error::DecodingError, Decodable},
    types::{
        attributes::Attribute,
        constants::ConstantPool,
        descriptors::{BaseType, FieldType, MethodDescriptor},
        elements::Method,
        flags::MethodAccessFlags,
    },
};

impl Decodable<Method> for Method {
    fn decode(
        buffer: &mut BufferedReader,
        constant_pool: &ConstantPool,
    ) -> Result<Method, DecodingError> {
        let access_flags = buffer.take::<u16>().unwrap();
        let access_flags = MethodAccessFlags::from_bits(access_flags).unwrap();

        let name_index = buffer.take::<u16>().unwrap();
        let name = constant_pool.text_of_value(name_index as usize).unwrap();

        let descriptor_index = buffer.take::<u16>().unwrap();
        let descriptor = constant_pool
            .text_of_value(descriptor_index as usize)
            .unwrap();

        let attributes_count = buffer.take::<u16>().unwrap();
        let attributes = (0..attributes_count)
            .map(|_| Attribute::decode(buffer, constant_pool).unwrap())
            .collect();

        Ok(Method {
            access_flags,
            name,
            descriptor,
            attributes,
        })
    }
}

pub fn parse_method_descriptor(descriptor: String) -> Result<MethodDescriptor, DecodingError> {
    let mut chars = descriptor.chars();

    // check if the method has parameters by checking if the first character is '('
    // if it is, then we have at least one parameter to parse.
    // see: https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.3.3
    let mut parameters = vec![];
    let mut param_chars: Vec<char> = vec![];
    if chars.next() == Some('(') {
        param_chars = chars.by_ref().take_while(|c| *c != ')').collect();
        param_chars.reverse();
    }

    while !param_chars.is_empty() {
        if let Some(ty) = parse_descriptor(&mut param_chars) {
            parameters.push(ty);
        }
    }

    // at this point we should have parsed all parameters and the next element is the return type
    let mut ret_chars: Vec<char> = chars.collect();
    ret_chars.reverse();
    let ret_ty = parse_descriptor(&mut ret_chars);

    Ok(MethodDescriptor {
        parameters,
        return_ty: ret_ty,
    })
}

pub fn parse_descriptor(chars: &mut Vec<char>) -> Option<FieldType> {
    match chars.pop() {
        Some('B') => Some(FieldType::Base(BaseType::Byte)),
        Some('C') => Some(FieldType::Base(BaseType::Char)),
        Some('D') => Some(FieldType::Base(BaseType::Double)),
        Some('F') => Some(FieldType::Base(BaseType::Float)),
        Some('I') => Some(FieldType::Base(BaseType::Int)),
        Some('J') => Some(FieldType::Base(BaseType::Long)),
        Some('S') => Some(FieldType::Base(BaseType::Short)),
        Some('Z') => Some(FieldType::Base(BaseType::Boolean)),
        Some('L') => {
            let mut class_name = String::new();
            while let Some(c) = chars.pop() {
                if c == ';' {
                    break;
                }
                class_name.push(c);
            }
            Some(FieldType::Object(class_name))
        }
        Some('[') => {
            let child = parse_descriptor(chars);
            child.map(|ty| FieldType::Array(Box::new(ty)))
        }
        _ => None,
    }
}
