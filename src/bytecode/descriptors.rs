use super::BytecodeError;
use crate::bytecode::{BaseType, Descriptor, DescriptorKind, FieldType};

impl Descriptor {
    /// ```text
    /// MethodDescriptor:
    ///     ({ParameterDescriptor}) ReturnDescriptor
    ///
    /// ParameterDescriptor:
    ///     FieldType
    ///
    /// ReturnDescriptor:
    ///     FieldType
    ///     VoidDescriptor
    ///
    /// VoidDescriptor:
    ///     'V'
    /// ```
    pub fn parse_from_method(descriptor: String) -> Vec<Descriptor> {
        let mut chars = descriptor.chars();
        let mut descriptors: Vec<Descriptor> = vec![];

        // check if the method has parameters by checking if the first character is '('
        // if it is, then we have at least one parameter to parse.
        // see: https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.3.3
        let mut param_chars: Vec<char> = vec![];
        if chars.next() == Some('(') {
            param_chars = chars.by_ref().take_while(|c| *c != ')').collect();
            param_chars.reverse();
        }

        while !param_chars.is_empty() {
            if let Some(ty) = parse_field_type(&mut param_chars) {
                descriptors.push(Descriptor {
                    kind: DescriptorKind::Parameter,
                    ty,
                });
            }
        }

        // at this point we should have parsed all parameters and the next element is the return
        // type
        let mut ret_chars: Vec<char> = chars.collect();
        ret_chars.reverse();
        if let Some(ty) = parse_field_type(&mut ret_chars) {
            descriptors.push(Descriptor {
                kind: DescriptorKind::Return,
                ty,
            });
        }

        descriptors
    }

    /// ```text
    /// FieldDescriptor:
    ///     FieldType
    ///
    /// FieldType:
    ///     BaseType
    ///     ObjectType
    ///     ArrayType
    ///
    /// BaseType:
    ///     'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z'
    ///
    /// ObjectType:
    ///     'L' ClassName ';'
    ///
    /// ArrayType:
    ///     '['ComponentType
    ///
    /// ComponentType:
    ///     FieldType
    /// ```
    pub fn parse_from_field(descriptor: String) -> Result<Descriptor, BytecodeError> {
        let mut chars = descriptor.chars().collect::<Vec<char>>();
        chars.reverse();
        let ty = match parse_field_type(&mut chars) {
            Some(ty) => ty,
            None => return Err(BytecodeError::InvalidDescriptor),
        };
        Ok(Descriptor {
            kind: DescriptorKind::Type,
            ty,
        })
    }
}

pub(crate) fn parse_field_type(chars: &mut Vec<char>) -> Option<FieldType> {
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
            let child = parse_field_type(chars);
            child.map(|ty| FieldType::Array(Box::new(ty)))
        }
        _ => Some(FieldType::Base(BaseType::Void)),
    }
}

#[cfg(test)]
pub mod tests {
    use crate::bytecode::descriptors::{BaseType, Descriptor, DescriptorKind, FieldType};

    #[test]
    fn test_parse_field_descriptor() {
        let input = [
            "I",
            "D",
            "V",
            "Ljava/lang/String",
            "[D",
            "[Ljava/lang/String",
            "[[D",
            "[[Ljava/lang/String",
        ];

        let expected = [
            FieldType::Base(BaseType::Int),
            FieldType::Base(BaseType::Double),
            FieldType::Base(BaseType::Void),
            FieldType::Object("java/lang/String".to_string()),
            FieldType::Array(Box::new(FieldType::Base(BaseType::Double))),
            FieldType::Array(Box::new(FieldType::Object("java/lang/String".to_string()))),
            FieldType::Array(Box::new(FieldType::Array(Box::new(FieldType::Base(
                BaseType::Double,
            ))))),
            FieldType::Array(Box::new(FieldType::Array(Box::new(FieldType::Object(
                "java/lang/String".to_string(),
            ))))),
        ];

        for (idx, t) in input.iter().enumerate() {
            let ret = Descriptor::parse_from_field(t.to_string());
            assert!(ret.is_ok());
            assert_eq!(ret.unwrap().ty, expected[idx]);
        }
    }

    #[test]
    fn test_parse_method_descriptors() {
        let input = "(IDLjava/lang/String;)V";
        let expected = vec![
            Descriptor {
                kind: DescriptorKind::Parameter,
                ty: FieldType::Base(BaseType::Int),
            },
            Descriptor {
                kind: DescriptorKind::Parameter,
                ty: FieldType::Base(BaseType::Double),
            },
            Descriptor {
                kind: DescriptorKind::Parameter,
                ty: FieldType::Object("java/lang/String".to_string()),
            },
            Descriptor {
                kind: DescriptorKind::Return,
                ty: FieldType::Base(BaseType::Void),
            },
        ];

        let ret = Descriptor::parse_from_method(input.to_string());
        assert_eq!(ret, expected);
    }
}
