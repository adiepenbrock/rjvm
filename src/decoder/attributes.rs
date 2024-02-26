use crate::{
    decoder::{buffer::Buffer, error::DecodingError, Decodable},
    types::{
        attributes::{
            Annotation, AnnotationDefaultInfo, Attribute, BootstrapMethod, BootstrapMethodsInfo,
            CodeInfo, ConstantValueInfo, DeprecatedInfo, ElementValue, ElementValuePair,
            EnclosingMethodInfo, ExceptionTableEntry, ExceptionsInfo, Exports, InnerClass,
            InnerClassesInfo, LineNumberTableEntry, LineNumberTableInfo, LocalVarTargetTableEntry,
            LocalVariableTableEntry, LocalVariableTableInfo, LocalVariableTypeTableEntry,
            LocalVariableTypeTableInfo, MethodParameter, MethodParametersInfo, ModuleInfo,
            ModuleMainClassInfo, ModulePackagesInfo, NestHostInfo, NestMembersInfo, Opens,
            ParameterAnnotation, PermittedSubtypesInfo, Provides, RecordInfo, Requires,
            RuntimeInvisibleAnnotationsInfo, RuntimeInvisibleParameterAnnotationsInfo,
            RuntimeInvisibleTypeAnnotationsInfo, RuntimeVisibleAnnotationsInfo,
            RuntimeVisibleParameterAnnotationsInfo, RuntimeVisibleTypeAnnotationsInfo,
            SignatureInfo, SourceDebugExtensionInfo, SourceFileInfo, StackMapFrame,
            StackMapTableInfo, SyntheticInfo, TypeAnnotation, TypeAnnotationTargetInfo,
            TypeAnnotationTargetInfoType, TypePath, TypePathEntry, VerificationTypeInfo,
        },
        constants::ConstantPool,
        flags::InnerClassAccessFlags,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantPoolValueRef(pub usize);

impl From<usize> for ConstantPoolValueRef {
    fn from(index: usize) -> Self {
        ConstantPoolValueRef(index)
    }
}

impl From<u16> for ConstantPoolValueRef {
    fn from(index: u16) -> Self {
        ConstantPoolValueRef(index as usize)
    }
}

impl Decodable<Annotation> for Annotation {
    fn decode(
        buffer: &mut Buffer,
        constant_pool: &ConstantPool,
    ) -> Result<Annotation, DecodingError> {
        let type_index = buffer.take::<u16>().expect("decode `type_index`");
        let num_element_value_pairs = buffer
            .take::<u16>()
            .expect("decode `num_element_value_pairs`");
        let element_value_pairs = (0..num_element_value_pairs)
            .map(|_| {
                let element_name_index = buffer.take::<u16>().expect("decode `element_name_index`");
                let element_value = ElementValue::decode(buffer, constant_pool).unwrap();
                ElementValuePair {
                    element_name_index,
                    value: element_value,
                }
            })
            .collect::<Vec<ElementValuePair>>();
        Ok(Annotation {
            element_value_pairs,
            type_index,
            num_element_value_pairs,
        })
    }
}

impl Decodable<ElementValue> for ElementValue {
    fn decode(
        buffer: &mut Buffer,
        constant_pool: &ConstantPool,
    ) -> Result<ElementValue, DecodingError> {
        let tag = buffer.take::<u8>().unwrap();
        let value = match tag {
            b'B' | b'C' | b'D' | b'F' | b'I' | b'J' | b'S' | b'Z' | b's' => {
                let const_value_index = buffer.take::<u16>().expect("decode `const_value_index`");
                Some(ElementValue::ConstValueIndex(const_value_index))
            }
            b'e' => {
                let type_name_index = buffer.take::<u16>().expect("decode `type_name_index`");
                let const_name_index = buffer.take::<u16>().expect("decode `const_name_index`");
                Some(ElementValue::EnumConstValue {
                    type_name_index,
                    const_name_index,
                })
            }
            b'c' => {
                let class_info_index = buffer.take::<u16>().expect("decode `class_info_index`");
                Some(ElementValue::ClassInfoIndex(class_info_index))
            }
            b'@' => {
                let type_index = buffer.take::<u16>().unwrap();
                let num_element_value_pairs = buffer.take::<u16>().unwrap();
                let element_value_pairs = (0..num_element_value_pairs)
                    .map(|_| {
                        let element_name_index = buffer.take::<u16>().unwrap();
                        let value = ElementValue::decode(buffer, constant_pool).unwrap();
                        ElementValuePair {
                            element_name_index,
                            value,
                        }
                    })
                    .collect::<Vec<ElementValuePair>>();
                Some(ElementValue::Annotation(Annotation {
                    type_index,
                    num_element_value_pairs,
                    element_value_pairs,
                }))
            }
            b'[' => {
                let num_values = buffer.take::<u16>().expect("decode `num_values`");
                let values = (0..num_values)
                    .map(|_| ElementValue::decode(buffer, constant_pool).unwrap())
                    .collect::<Vec<ElementValue>>();
                Some(ElementValue::Array { num_values, values })
            }
            _ => panic!("Invalid tag: {}", tag),
        };

        if let Some(element_value) = value {
            Ok(element_value)
        } else {
            Err(DecodingError::InvalidClassFile)
        }
    }
}

impl Decodable<TypeAnnotation> for TypeAnnotation {
    fn decode(
        buffer: &mut Buffer,
        constant_pool: &ConstantPool,
    ) -> Result<TypeAnnotation, DecodingError> {
        let target_type = buffer.take::<u8>().unwrap();
        let target_info = TypeAnnotationTargetInfo::decode(buffer, constant_pool).unwrap();
        let target_path = TypePath::decode(buffer, constant_pool).unwrap();
        let type_index = buffer.take::<u16>().unwrap();
        let num_element_value_pairs = buffer.take::<u16>().unwrap();
        let element_value_pairs = (0..num_element_value_pairs)
            .map(|_| {
                let element_name_index = buffer.take::<u16>().unwrap();
                let element_value = ElementValue::decode(buffer, constant_pool).unwrap();
                ElementValuePair {
                    element_name_index,
                    value: element_value,
                }
            })
            .collect::<Vec<ElementValuePair>>();
        Ok(TypeAnnotation {
            target_type,
            target_info: TypeAnnotationTargetInfo { target_info },
            target_path,
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        })
    }
}

impl Decodable<TypeAnnotationTargetInfoType> for TypeAnnotationTargetInfo {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<TypeAnnotationTargetInfoType, DecodingError> {
        let tag = buffer.take::<u8>().unwrap();
        let target_info = match tag {
            0x00 | 0x01 => {
                let type_parameter_index = buffer.take::<u8>().unwrap();
                Some(TypeAnnotationTargetInfoType::TypeParameter {
                    type_parameter_index,
                })
            }
            0x10 => {
                let super_type_index = buffer.take::<u16>().unwrap();
                Some(TypeAnnotationTargetInfoType::SuperType { super_type_index })
            }
            0x11 | 0x12 => {
                let type_parameter_index = buffer.take::<u8>().unwrap();
                let bound_index = buffer.take::<u8>().unwrap();
                Some(TypeAnnotationTargetInfoType::TypeParameterBound {
                    type_parameter_index,
                    bound_index,
                })
            }
            0x13..=0x15 => Some(TypeAnnotationTargetInfoType::Empty {}),
            0x16 => {
                let formal_parameter_index = buffer.take::<u8>().unwrap();
                Some(TypeAnnotationTargetInfoType::FormalParameter {
                    formal_parameter_index,
                })
            }
            0x17 => {
                let throws_type_index = buffer.take::<u16>().unwrap();
                Some(TypeAnnotationTargetInfoType::Throws { throws_type_index })
            }
            0x40 | 0x41 => {
                let table_length = buffer.take::<u16>().unwrap();
                let table = (0..table_length)
                    .map(|_| {
                        let start_pc = buffer.take::<u16>().unwrap();
                        let length = buffer.take::<u16>().unwrap();
                        let index = buffer.take::<u16>().unwrap();
                        Ok(LocalVarTargetTableEntry {
                            start_pc,
                            length,
                            index,
                        })
                    })
                    .collect::<Result<Vec<LocalVarTargetTableEntry>, DecodingError>>()?;
                Some(TypeAnnotationTargetInfoType::LocalVar { table })
            }
            0x42 => {
                let exception_table_index = buffer.take::<u16>().unwrap();
                Some(TypeAnnotationTargetInfoType::Catch {
                    exception_table_index,
                })
            }
            0x43..=0x46 => {
                let offset = buffer.take::<u16>().unwrap();
                Some(TypeAnnotationTargetInfoType::Offset { offset })
            }
            0x47..=0x4B => {
                let offset = buffer.take::<u16>().unwrap();
                let type_argument_index = buffer.take::<u8>().unwrap();
                Some(TypeAnnotationTargetInfoType::TypeArgument {
                    offset,
                    type_argument_index,
                })
            }
            _ => None,
        };

        if let Some(target_info) = target_info {
            Ok(target_info)
        } else {
            Err(DecodingError::InvalidClassFile)
        }
    }
}

impl Decodable<TypePath> for TypePath {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<TypePath, DecodingError> {
        let path_length = buffer.take::<u8>().unwrap();
        let path = (0..path_length)
            .map(|_| {
                let type_path_kind = buffer.take::<u8>().unwrap();
                let type_argument_index = buffer.take::<u8>().unwrap();
                Ok(TypePathEntry {
                    type_path_kind,
                    type_argument_index,
                })
            })
            .collect::<Result<Vec<TypePathEntry>, DecodingError>>()?;
        Ok(TypePath { path_length, path })
    }
}

impl Decodable<BootstrapMethod> for BootstrapMethod {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<BootstrapMethod, DecodingError> {
        let bootstrap_method_ref = buffer.take::<u16>().unwrap();
        let num_bootstrap_arguments = buffer.take::<u16>().unwrap();
        let bootstrap_arguments = (0..num_bootstrap_arguments)
            .map(|_| buffer.take::<u16>().unwrap())
            .collect::<Vec<u16>>();
        Ok(BootstrapMethod {
            bootstrap_method_ref,
            num_bootstrap_arguments,
            bootstrap_arguments,
        })
    }
}

impl Decodable<Requires> for Requires {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<Requires, DecodingError> {
        let requires_index = buffer.take::<u16>().unwrap();
        let requires_flags = buffer.take::<u16>().unwrap();
        let requires_version = buffer.take::<u16>().unwrap();
        Ok(Requires {
            requires_index,
            requires_flags,
            requires_version_index: requires_version,
        })
    }
}

impl Decodable<Exports> for Exports {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<Exports, DecodingError> {
        let exports_index = buffer.take::<u16>().unwrap();
        let exports_flags = buffer.take::<u16>().unwrap();
        let exports_to_count = buffer.take::<u16>().unwrap();
        let exports_to_indices = (0..exports_to_count)
            .map(|_| buffer.take::<u16>().unwrap())
            .collect::<Vec<u16>>();
        Ok(Exports {
            exports_index,
            exports_flags,
            exports_to_count,
            exports_to_index: exports_to_indices,
        })
    }
}

impl Decodable<Opens> for Opens {
    fn decode(buffer: &mut Buffer, _constant_pool: &ConstantPool) -> Result<Opens, DecodingError> {
        let opens_index = buffer.take::<u16>().unwrap();
        let opens_flags = buffer.take::<u16>().unwrap();
        let opens_to_count = buffer.take::<u16>().unwrap();
        let opens_to_indices = (0..opens_to_count)
            .map(|_| buffer.take::<u16>().unwrap())
            .collect::<Vec<u16>>();
        Ok(Opens {
            opens_index,
            opens_flags,
            opens_to_count,
            opens_to_index: opens_to_indices,
        })
    }
}

impl Decodable<Provides> for Provides {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<Provides, DecodingError> {
        let provides_index = buffer.take::<u16>().unwrap();
        let provides_with_count = buffer.take::<u16>().unwrap();
        let provides_with_indices = (0..provides_with_count)
            .map(|_| buffer.take::<u16>().unwrap())
            .collect::<Vec<u16>>();
        Ok(Provides {
            provides_index,
            provides_with_count,
            provides_with_index: provides_with_indices,
        })
    }
}

impl Attribute {
    pub fn decode(buffer: &mut Buffer, pool: &ConstantPool) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.peek_bytes::<u16>().unwrap();
        let attribute_name = pool.text_of_value(attribute_name_index as usize).unwrap();

        let attribute = match attribute_name.as_str() {
            "ConstantValue" => ConstantValueInfo::decode(buffer, pool)?,
            "Code" => CodeInfo::decode(buffer, pool)?,
            "StackMapTable" => StackMapTableInfo::decode(buffer, pool)?,
            "Exceptions" => ExceptionsInfo::decode(buffer, pool)?,
            "InnerClasses" => InnerClassesInfo::decode(buffer, pool)?,
            "EnclosingMethod" => EnclosingMethodInfo::decode(buffer, pool)?,
            "Synthetic" => SyntheticInfo::decode(buffer, pool)?,
            "Signature" => SignatureInfo::decode(buffer, pool)?,
            "SourceFile" => SourceFileInfo::decode(buffer, pool)?,
            "SourceDebugExtension" => SourceDebugExtensionInfo::decode(buffer, pool)?,
            "LineNumberTable" => LineNumberTableInfo::decode(buffer, pool)?,
            "LocalVariableTable" => LocalVariableTableInfo::decode(buffer, pool)?,
            "LocalVariableTypeTable" => LocalVariableTypeTableInfo::decode(buffer, pool)?,
            "Deprecated" => DeprecatedInfo::decode(buffer, pool)?,
            "RuntimeVisibleAnnotations" => RuntimeVisibleAnnotationsInfo::decode(buffer, pool)?,
            "RuntimeInvisibleAnnotations" => RuntimeInvisibleAnnotationsInfo::decode(buffer, pool)?,
            "RuntimeVisibleParameterAnnotations" => {
                RuntimeVisibleParameterAnnotationsInfo::decode(buffer, pool)?
            }
            "RuntimeInvisibleParameterAnnotations" => {
                RuntimeInvisibleParameterAnnotationsInfo::decode(buffer, pool)?
            }
            "RuntimeVisibleTypeAnnotations" => {
                RuntimeVisibleTypeAnnotationsInfo::decode(buffer, pool)?
            }
            "RuntimeInvisibleTypeAnnotations" => {
                RuntimeInvisibleTypeAnnotationsInfo::decode(buffer, pool)?
            }
            "AnnotationDefault" => AnnotationDefaultInfo::decode(buffer, pool)?,
            "BootstrapMethods" => BootstrapMethodsInfo::decode(buffer, pool)?,
            "MethodParameters" => MethodParametersInfo::decode(buffer, pool)?,
            "Module" => ModuleInfo::decode(buffer, pool)?,
            "ModulePackages" => ModulePackagesInfo::decode(buffer, pool)?,
            "ModuleMainClass" => ModuleMainClassInfo::decode(buffer, pool)?,
            "NestHost" => NestHostInfo::decode(buffer, pool)?,
            "NestMembers" => NestMembersInfo::decode(buffer, pool)?,
            "Record" => RecordInfo::decode(buffer, pool)?,
            "PermittedSubtypes" => PermittedSubtypesInfo::decode(buffer, pool)?,
            _ => return Err(DecodingError::UnsupportedAttributeName),
        };

        Ok(attribute)
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.info.downcast_ref()
    }
}

impl Decodable<Attribute> for ConstantValueInfo {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let constantvalue_index = buffer.take::<u16>().unwrap();

        let info = ConstantValueInfo {
            attribute_length,
            attribute_name_index,
            constantvalue_index,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for CodeInfo {
    fn decode(
        buffer: &mut Buffer,
        constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let max_stack = buffer.take::<u16>().unwrap();
        let max_locals = buffer.take::<u16>().unwrap();
        let code_length = buffer.take::<u32>().unwrap();
        let code = buffer.take_length(code_length as usize).unwrap();
        let exception_table_length = buffer.take::<u16>().unwrap();
        let exception_table = (0..exception_table_length)
            .map(|_| {
                let start_pc = buffer.take::<u16>().unwrap();
                let end_pc = buffer.take::<u16>().unwrap();
                let handler_pc = buffer.take::<u16>().unwrap();
                let catch_type = buffer.take::<u16>().unwrap();
                ExceptionTableEntry {
                    start_pc,
                    end_pc,
                    handler_pc,
                    catch_type,
                }
            })
            .collect();
        let attributes_count = buffer.take::<u16>().unwrap();
        let attributes = (0..attributes_count)
            .map(|_| Attribute::decode(buffer, constant_pool).unwrap())
            .collect();

        let info = CodeInfo {
            attribute_name_index,
            attribute_length,
            max_stack,
            max_locals,
            code_length,
            code: code.to_vec(),
            exception_table_length,
            exception_table,
            attributes_count,
            attributes,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for StackMapTableInfo {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let number_of_entries = buffer.take::<u16>().unwrap();
        let frame_type = buffer.take::<u8>().unwrap();
        let entries = (0..number_of_entries)
            .map(|_| {
                match frame_type {
                    0..=63 => Some(StackMapFrame::SameFrame { frame_type }),
                    64..=127 => {
                        let verification_type = buffer.take::<u8>().unwrap();
                        let verification_type = match verification_type {
                            0 => Some(VerificationTypeInfo::Top),
                            1 => Some(VerificationTypeInfo::Integer),
                            2 => Some(VerificationTypeInfo::Float),
                            3 => Some(VerificationTypeInfo::Double),
                            4 => Some(VerificationTypeInfo::Long),
                            5 => Some(VerificationTypeInfo::Null),
                            6 => Some(VerificationTypeInfo::UninitializedThis),
                            7 => {
                                let class = buffer.take::<u16>().unwrap();
                                Some(VerificationTypeInfo::Object { class })
                            }
                            8 => {
                                let offset = buffer.take::<u16>().unwrap();
                                Some(VerificationTypeInfo::Uninitialized { offset })
                            }
                            _ => None,
                        };
                        Some(StackMapFrame::SameLocals1StackItemFrame {
                            frame_type,
                            stack: verification_type.unwrap(),
                        })
                    }
                    247 => {
                        let offset_delta = buffer.take::<u16>().unwrap();
                        let verification_type = buffer.take::<u8>().unwrap();
                        let verification_type = match verification_type {
                            0 => Some(VerificationTypeInfo::Top),
                            1 => Some(VerificationTypeInfo::Integer),
                            2 => Some(VerificationTypeInfo::Float),
                            3 => Some(VerificationTypeInfo::Double),
                            4 => Some(VerificationTypeInfo::Long),
                            5 => Some(VerificationTypeInfo::Null),
                            6 => Some(VerificationTypeInfo::UninitializedThis),
                            7 => {
                                let class = buffer.take::<u16>().unwrap();
                                Some(VerificationTypeInfo::Object { class })
                            }
                            8 => {
                                let offset = buffer.take::<u16>().unwrap();
                                Some(VerificationTypeInfo::Uninitialized { offset })
                            }
                            _ => None,
                        };

                        Some(StackMapFrame::SameLocals1StackItemFrameExtended {
                            frame_type,
                            offset_delta,
                            stack: verification_type.unwrap(),
                        })
                    }
                    248..=250 => {
                        let offset_delta = buffer.take::<u16>().unwrap();
                        Some(StackMapFrame::ChopFrame {
                            frame_type,
                            offset_delta,
                        })
                    }
                    251 => {
                        let offset_delta = buffer.take::<u16>().unwrap();
                        Some(StackMapFrame::SameFrameExtended {
                            frame_type,
                            offset_delta,
                        })
                    }
                    252..=254 => {
                        let offset_delta = buffer.take::<u16>().unwrap();
                        let locals = (0..frame_type - 251)
                            .map(|_| {
                                let verification_type = buffer.take::<u8>().unwrap();
                                let verification_type = match verification_type {
                                    0 => Some(VerificationTypeInfo::Top),
                                    1 => Some(VerificationTypeInfo::Integer),
                                    2 => Some(VerificationTypeInfo::Float),
                                    3 => Some(VerificationTypeInfo::Double),
                                    4 => Some(VerificationTypeInfo::Long),
                                    5 => Some(VerificationTypeInfo::Null),
                                    6 => Some(VerificationTypeInfo::UninitializedThis),
                                    7 => {
                                        let class = buffer.take::<u16>().unwrap();
                                        Some(VerificationTypeInfo::Object { class })
                                    }
                                    8 => {
                                        let offset = buffer.take::<u16>().unwrap();
                                        Some(VerificationTypeInfo::Uninitialized { offset })
                                    }
                                    _ => None,
                                };
                                verification_type.unwrap()
                            })
                            .collect::<Vec<VerificationTypeInfo>>();
                        Some(StackMapFrame::AppendFrame {
                            frame_type,
                            offset_delta,
                            locals,
                        })
                    }
                    255 => {
                        let offset_delta = buffer.take::<u16>().unwrap();
                        let number_of_locals = buffer.take::<u16>().unwrap();
                        let locals = (0..number_of_locals)
                            .map(|_| {
                                let verification_type = buffer.take::<u8>().unwrap();
                                let verification_type = match verification_type {
                                    0 => Some(VerificationTypeInfo::Top),
                                    1 => Some(VerificationTypeInfo::Integer),
                                    2 => Some(VerificationTypeInfo::Float),
                                    3 => Some(VerificationTypeInfo::Double),
                                    4 => Some(VerificationTypeInfo::Long),
                                    5 => Some(VerificationTypeInfo::Null),
                                    6 => Some(VerificationTypeInfo::UninitializedThis),
                                    7 => {
                                        let class = buffer.take::<u16>().unwrap();
                                        Some(VerificationTypeInfo::Object { class })
                                    }
                                    8 => {
                                        let offset = buffer.take::<u16>().unwrap();
                                        Some(VerificationTypeInfo::Uninitialized { offset })
                                    }
                                    _ => None,
                                };
                                verification_type.unwrap()
                            })
                            .collect::<Vec<VerificationTypeInfo>>();
                        let number_of_stack_items = buffer.take::<u16>().unwrap();
                        let stack = (0..number_of_stack_items)
                            .map(|_| {
                                let verification_type = buffer.take::<u8>().unwrap();
                                let verification_type = match verification_type {
                                    0 => Some(VerificationTypeInfo::Top),
                                    1 => Some(VerificationTypeInfo::Integer),
                                    2 => Some(VerificationTypeInfo::Float),
                                    3 => Some(VerificationTypeInfo::Double),
                                    4 => Some(VerificationTypeInfo::Long),
                                    5 => Some(VerificationTypeInfo::Null),
                                    6 => Some(VerificationTypeInfo::UninitializedThis),
                                    7 => {
                                        let class = buffer.take::<u16>().unwrap();
                                        Some(VerificationTypeInfo::Object { class })
                                    }
                                    8 => {
                                        let offset = buffer.take::<u16>().unwrap();
                                        Some(VerificationTypeInfo::Uninitialized { offset })
                                    }
                                    _ => None,
                                };
                                verification_type.unwrap()
                            })
                            .collect::<Vec<VerificationTypeInfo>>();
                        Some(StackMapFrame::FullFrame {
                            frame_type,
                            number_of_locals,
                            number_of_stack_items,
                            offset_delta,
                            locals,
                            stack,
                        })
                    }
                    _ => None,
                }
                .unwrap()
            })
            .collect();

        let info = StackMapTableInfo {
            attribute_name_index,
            attribute_length,
            number_of_entries,
            entries,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for ExceptionsInfo {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let number_of_exceptions = buffer.take::<u16>().unwrap();
        let exception_index_table = (0..number_of_exceptions)
            .map(|_| buffer.take::<u16>().unwrap())
            .collect();

        let info = ExceptionsInfo {
            attribute_name_index,
            attribute_length,
            number_of_exceptions,
            exception_index_table,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for InnerClassesInfo {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let number_of_classes = buffer.take::<u16>().unwrap();
        let classes = (0..number_of_classes)
            .map(|_| {
                let inner_class_info_index = buffer.take::<u16>().unwrap();
                let outer_class_info_index = buffer.take::<u16>().unwrap();
                let inner_name_index = buffer.take::<u16>().unwrap();
                let inner_class_access_flags = buffer.take::<u16>().unwrap();
                let inner_class_access_flags =
                    InnerClassAccessFlags::from_bits(inner_class_access_flags).unwrap();

                InnerClass {
                    inner_class_info_index,
                    outer_class_info_index,
                    inner_name_index,
                    inner_class_access_flags,
                }
            })
            .collect();

        let info = InnerClassesInfo {
            attribute_name_index,
            attribute_length,
            number_of_classes,
            classes,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for EnclosingMethodInfo {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let class_index = buffer.take::<u16>().unwrap();
        let method_index = buffer.take::<u16>().unwrap();

        let info = EnclosingMethodInfo {
            attribute_name_index,
            attribute_length,
            class_index,
            method_index,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for SyntheticInfo {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();

        let info = SyntheticInfo {
            attribute_name_index,
            attribute_length,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for SignatureInfo {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let signature_index = buffer.take::<u16>().unwrap();

        let info = SignatureInfo {
            attribute_name_index,
            attribute_length,
            signature_index,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for SourceFileInfo {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let sourcefile_index = buffer.take::<u16>().unwrap();

        let info = SourceFileInfo {
            attribute_name_index,
            attribute_length,
            sourcefile_index,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for SourceDebugExtensionInfo {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let debug_extension = buffer.take::<Vec<u8>>().unwrap();

        let info = SourceDebugExtensionInfo {
            attribute_name_index,
            attribute_length,
            debug_extension,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for LineNumberTableInfo {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let line_number_table_length = buffer.take::<u16>().unwrap();
        let line_number_table = (0..line_number_table_length)
            .map(|_| {
                let start_pc = buffer.take::<u16>().unwrap();
                let line_number = buffer.take::<u16>().unwrap();
                LineNumberTableEntry {
                    start_pc,
                    line_number,
                }
            })
            .collect();

        let info = LineNumberTableInfo {
            attribute_name_index,
            attribute_length,
            line_number_table_length,
            line_number_table,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for LocalVariableTableInfo {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let local_variable_table_length = buffer.take::<u16>().unwrap();
        let local_variable_table = (0..local_variable_table_length)
            .map(|_| {
                let start_pc = buffer.take::<u16>().unwrap();
                let length = buffer.take::<u16>().unwrap();
                let name_index = buffer.take::<u16>().unwrap();
                let descriptor_index = buffer.take::<u16>().unwrap();
                let index = buffer.take::<u16>().unwrap();
                LocalVariableTableEntry {
                    start_pc,
                    length,
                    name_index,
                    descriptor_index,
                    index,
                }
            })
            .collect();

        let info = LocalVariableTableInfo {
            attribute_name_index,
            attribute_length,
            local_variable_table_length,
            local_variable_table,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for LocalVariableTypeTableInfo {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let local_variable_type_table_length = buffer.take::<u16>().unwrap();
        let local_variable_type_table = (0..local_variable_type_table_length)
            .map(|_| {
                let start_pc = buffer.take::<u16>().unwrap();
                let length = buffer.take::<u16>().unwrap();
                let name_index = buffer.take::<u16>().unwrap();
                let signature_index = buffer.take::<u16>().unwrap();
                let index = buffer.take::<u16>().unwrap();
                LocalVariableTypeTableEntry {
                    start_pc,
                    length,
                    name_index,
                    signature_index,
                    index,
                }
            })
            .collect();

        let info = LocalVariableTypeTableInfo {
            attribute_name_index,
            attribute_length,
            local_variable_type_table_length,
            local_variable_type_table,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for DeprecatedInfo {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();

        let info = DeprecatedInfo {
            attribute_name_index,
            attribute_length,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for RuntimeVisibleAnnotationsInfo {
    fn decode(
        buffer: &mut Buffer,
        constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let num_annotations = buffer.take::<u16>().unwrap();
        let annotations = (0..num_annotations)
            .map(|_| Annotation::decode(buffer, constant_pool))
            .collect::<Result<Vec<Annotation>, DecodingError>>()?;

        let info = RuntimeVisibleAnnotationsInfo {
            attribute_name_index,
            attribute_length,
            num_annotations,
            annotations,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for RuntimeInvisibleAnnotationsInfo {
    fn decode(
        buffer: &mut Buffer,
        constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let num_annotations = buffer.take::<u16>().unwrap();
        let annotations = (0..num_annotations)
            .map(|_| Annotation::decode(buffer, constant_pool))
            .collect::<Result<Vec<Annotation>, DecodingError>>()?;

        let info = RuntimeInvisibleAnnotationsInfo {
            attribute_name_index,
            attribute_length,
            num_annotations,
            annotations,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for RuntimeVisibleParameterAnnotationsInfo {
    fn decode(
        buffer: &mut Buffer,
        constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let num_parameters = buffer.take::<u8>().unwrap();
        let parameter_annotations = (0..num_parameters)
            .map(|_| {
                let num_annotations = buffer.take::<u16>().unwrap();
                let annotations = (0..num_annotations)
                    .map(|_| Annotation::decode(buffer, constant_pool))
                    .collect::<Result<Vec<Annotation>, DecodingError>>()?;
                Ok(ParameterAnnotation {
                    num_annotations,
                    annotations,
                })
            })
            .collect::<Result<Vec<ParameterAnnotation>, DecodingError>>()?;

        let info = RuntimeVisibleParameterAnnotationsInfo {
            attribute_name_index,
            attribute_length,
            num_parameters,
            parameter_annotations,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for RuntimeInvisibleParameterAnnotationsInfo {
    fn decode(
        buffer: &mut Buffer,
        constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let num_parameters = buffer.take::<u8>().unwrap();
        let parameter_annotations = (0..num_parameters)
            .map(|_| {
                let num_annotations = buffer.take::<u16>().unwrap();
                let annotations = (0..num_annotations)
                    .map(|_| Annotation::decode(buffer, constant_pool))
                    .collect::<Result<Vec<Annotation>, DecodingError>>()?;
                Ok(ParameterAnnotation {
                    num_annotations,
                    annotations,
                })
            })
            .collect::<Result<Vec<ParameterAnnotation>, DecodingError>>()?;

        let info = RuntimeInvisibleParameterAnnotationsInfo {
            attribute_name_index,
            attribute_length,
            num_parameters,
            parameter_annotations,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for RuntimeVisibleTypeAnnotationsInfo {
    fn decode(
        buffer: &mut Buffer,
        constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let num_annotations = buffer.take::<u16>().unwrap();
        let annotations = (0..num_annotations)
            .map(|_| TypeAnnotation::decode(buffer, constant_pool))
            .collect::<Result<Vec<TypeAnnotation>, DecodingError>>()?;

        let info = RuntimeVisibleTypeAnnotationsInfo {
            attribute_name_index,
            attribute_length,
            num_annotations,
            annotations,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for RuntimeInvisibleTypeAnnotationsInfo {
    fn decode(
        buffer: &mut Buffer,
        constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let num_annotations = buffer.take::<u16>().unwrap();
        let annotations = (0..num_annotations)
            .map(|_| TypeAnnotation::decode(buffer, constant_pool))
            .collect::<Result<Vec<TypeAnnotation>, DecodingError>>()?;

        let info = RuntimeInvisibleTypeAnnotationsInfo {
            attribute_name_index,
            attribute_length,
            num_annotations,
            annotations,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for AnnotationDefaultInfo {
    fn decode(
        buffer: &mut Buffer,
        constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let default_value = ElementValue::decode(buffer, constant_pool)?;

        let info = AnnotationDefaultInfo {
            attribute_name_index,
            attribute_length,
            default_value,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for BootstrapMethodsInfo {
    fn decode(
        buffer: &mut Buffer,
        constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let num_bootstrap_methods = buffer.take::<u16>().unwrap();
        let bootstrap_methods = (0..num_bootstrap_methods)
            .map(|_| BootstrapMethod::decode(buffer, constant_pool))
            .collect::<Result<Vec<BootstrapMethod>, DecodingError>>()?;

        let info = BootstrapMethodsInfo {
            attribute_name_index,
            attribute_length,
            num_bootstrap_methods,
            bootstrap_methods,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for MethodParametersInfo {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let num_parameters = buffer.take::<u8>().unwrap();
        let parameters = (0..num_parameters)
            .map(|_| {
                let name_index = buffer.take::<u16>().unwrap();
                let access_flags = buffer.take::<u16>().unwrap();
                Ok(MethodParameter {
                    name_index,
                    access_flags,
                })
            })
            .collect::<Result<Vec<MethodParameter>, DecodingError>>()?;

        let info = MethodParametersInfo {
            attribute_name_index,
            attribute_length,
            parameters_count: num_parameters,
            parameters,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for ModuleInfo {
    fn decode(
        buffer: &mut Buffer,
        constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let name_index = buffer.take::<u16>().unwrap();
        let access_flags = buffer.take::<u16>().unwrap();
        let version_index = buffer.take::<u16>().unwrap();
        let requires_count = buffer.take::<u16>().unwrap();
        let requires = (0..requires_count)
            .map(|_| Requires::decode(buffer, constant_pool))
            .collect::<Result<Vec<Requires>, DecodingError>>()?;
        let exports_count = buffer.take::<u16>().unwrap();
        let exports = (0..exports_count)
            .map(|_| Exports::decode(buffer, constant_pool))
            .collect::<Result<Vec<Exports>, DecodingError>>()?;
        let opens_count = buffer.take::<u16>().unwrap();
        let opens = (0..opens_count)
            .map(|_| Opens::decode(buffer, constant_pool))
            .collect::<Result<Vec<Opens>, DecodingError>>()?;
        let uses_count = buffer.take::<u16>().unwrap();
        let uses_index = (0..uses_count)
            .map(|_| buffer.take::<u16>().unwrap())
            .collect::<Vec<u16>>();
        let provides_count = buffer.take::<u16>().unwrap();
        let provides = (0..provides_count)
            .map(|_| Provides::decode(buffer, constant_pool))
            .collect::<Result<Vec<Provides>, DecodingError>>()?;

        let info = ModuleInfo {
            attribute_name_index,
            attribute_length,
            module_flags: access_flags,
            module_name_index: name_index,
            module_version_index: version_index,
            requires_count,
            requires,
            exports_count,
            exports,
            opens_count,
            opens,
            uses_count,
            uses_index,
            provides_count,
            provides,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for ModulePackagesInfo {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let package_count = buffer.take::<u16>().unwrap();
        let package_index = (0..package_count)
            .map(|_| buffer.take::<u16>().unwrap())
            .collect::<Vec<u16>>();

        let info = ModulePackagesInfo {
            attribute_name_index,
            attribute_length,
            package_count,
            package_index,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for ModuleMainClassInfo {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let main_class_index = buffer.take::<u16>().unwrap();

        let info = ModuleMainClassInfo {
            attribute_name_index,
            attribute_length,
            main_class_index,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for NestHostInfo {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let host_class_index = buffer.take::<u16>().unwrap();

        let info = NestHostInfo {
            attribute_name_index,
            attribute_length,
            host_class_index,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for NestMembersInfo {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let member_count = buffer.take::<u16>().unwrap();
        let member_index = (0..member_count)
            .map(|_| buffer.take::<u16>().unwrap())
            .collect::<Vec<u16>>();

        let info = NestMembersInfo {
            attribute_name_index,
            attribute_length,
            classes: member_index,
            number_of_classes: member_count,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for RecordInfo {
    fn decode(
        buffer: &mut Buffer,
        constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let component_count = buffer.take::<u16>().unwrap();
        let component_index = (0..component_count)
            .map(|_| Attribute::decode(buffer, constant_pool).unwrap())
            .collect::<Vec<Attribute>>();

        let info = RecordInfo {
            attribute_name_index,
            component_count,
            attribute_length,
            components: component_index,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}

impl Decodable<Attribute> for PermittedSubtypesInfo {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<Attribute, DecodingError> {
        let attribute_name_index = buffer.take::<u16>().unwrap();
        let attribute_length = buffer.take::<u32>().unwrap();
        let subtype_count = buffer.take::<u16>().unwrap();
        let subtype_index = (0..subtype_count)
            .map(|_| buffer.take::<u16>().unwrap())
            .collect::<Vec<u16>>();

        let info = PermittedSubtypesInfo {
            attribute_name_index,
            attribute_length,
            number_of_classes: subtype_count,
            classes: subtype_index,
        };

        Ok(Attribute {
            info: Box::new(info),
        })
    }
}
