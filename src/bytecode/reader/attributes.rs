use crate::bytecode::attributes::{
    Annotation, AnnotationDefaultInfo, AnyAttribute, AttributeFactory, BootstrapMethod,
    BootstrapMethodsInfo, CodeInfo, ConstantValueInfo, Container, DeprecatedInfo, ElementValue,
    ElementValuePair, EnclosingMethodInfo, ExceptionTableEntry, ExceptionsInfo, Exports,
    InnerClass, InnerClassesInfo, LineNumberTableEntry, LineNumberTableInfo,
    LocalVarTargetTableEntry, LocalVariableTableEntry, LocalVariableTableInfo,
    LocalVariableTypeTableEntry, LocalVariableTypeTableInfo, MethodParameter, MethodParametersInfo,
    ModuleInfo, ModuleMainClassInfo, ModulePackagesInfo, NestHostInfo, NestMembersInfo, Opens,
    ParameterAnnotation, PermittedSubtypesInfo, Provides, RecordInfo, Requires,
    RuntimeInvisibleAnnotationsInfo, RuntimeInvisibleParameterAnnotationsInfo,
    RuntimeInvisibleTypeAnnotationsInfo, RuntimeVisibleAnnotationsInfo,
    RuntimeVisibleParameterAnnotationsInfo, RuntimeVisibleTypeAnnotationsInfo, SignatureInfo,
    SourceDebugExtensionInfo, SourceFileInfo, StackMapFrame, StackMapTableInfo, SyntheticInfo,
    TypeAnnotation, TypeAnnotationTargetInfo, TypeAnnotationTargetInfoType, TypePath,
    TypePathEntry, VerificationTypeInfo,
};
use crate::bytecode::flags::InnerClassAccessFlags;
use crate::bytecode::pool::{ConstantPool, ConstantPoolIndex};
use crate::bytecode::reader::BufferedReader;
use crate::bytecode::BytecodeError;

pub fn read_attribute(
    reader: &mut BufferedReader,
    cp: &mut ConstantPool,
    container: &Container,
) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
    let attribute_name_index = reader.peek_bytes::<u16>()?;
    let Some(name) = cp.text_of(attribute_name_index.into()) else {
        return Err(BytecodeError::InvalidData);
    };

    let Some(attr) = container.get_by_name(&name) else {
        return Err(BytecodeError::UnsupportedAttributeName(name));
    };

    attr.make(reader, cp, container)
}

fn read_annotation(
    reader: &mut BufferedReader,
    pool: &mut ConstantPool,
) -> Result<Annotation, BytecodeError> {
    let type_index = reader.take::<u16>()?;
    let num_element_value_pairs = reader.take::<u16>()?;
    let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);
    for _ in 0..num_element_value_pairs {
        let element_name_index = reader.take::<u16>()?;
        let element_value = read_elementvalue(reader, pool)?;
        element_value_pairs.push(ElementValuePair {
            element_name_index: ConstantPoolIndex::new(element_name_index),
            value: element_value,
        });
    }

    Ok(Annotation {
        type_index: ConstantPoolIndex::new(type_index),
        num_element_value_pairs,
        element_value_pairs,
    })
}

fn read_typeannotation(
    reader: &mut BufferedReader,
    cp: &mut ConstantPool,
) -> Result<TypeAnnotation, BytecodeError> {
    let target_type = reader.take::<u8>()?;
    let target_info = read_typeannotationtarget_info(reader, cp)?;
    let target_path = read_typepath(reader, cp)?;
    let type_index = reader.take::<u16>()?;
    let num_element_value_pairs = reader.take::<u16>()?;
    let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);
    for _ in 0..num_element_value_pairs {
        let element_name_index = reader.take::<u16>()?;
        let element_value = read_elementvalue(reader, cp)?;
        element_value_pairs.push(ElementValuePair {
            element_name_index: ConstantPoolIndex::new(element_name_index),
            value: element_value,
        });
    }

    Ok(TypeAnnotation {
        target_type,
        target_info,
        target_path,
        type_index: ConstantPoolIndex::new(type_index),
        num_element_value_pairs,
        element_value_pairs,
    })
}

fn read_typeannotationtarget_info(
    reader: &mut BufferedReader,
    _cp: &mut ConstantPool,
) -> Result<TypeAnnotationTargetInfo, BytecodeError> {
    let target_type = reader.take::<u8>()?;
    let target_info = match target_type {
        0x00 | 0x01 => {
            let offset = reader.take::<u16>()?;
            TypeAnnotationTargetInfoType::TypeParameter {
                type_parameter_index: ConstantPoolIndex::new(offset),
            }
        }
        0x10 => {
            let offset = reader.take::<u16>()?;
            TypeAnnotationTargetInfoType::SuperType {
                super_type_index: ConstantPoolIndex::new(offset),
            }
        }
        0x11 | 0x12 => {
            let offset = reader.take::<u16>()?;
            let type_argument_index = reader.take::<u8>()?;
            TypeAnnotationTargetInfoType::TypeParameterBound {
                type_parameter_index: ConstantPoolIndex::new(offset),
                bound_index: ConstantPoolIndex::new(type_argument_index),
            }
        }
        0x13..=0x15 => TypeAnnotationTargetInfoType::Empty,
        0x16 => {
            let parameter_index = reader.take::<u16>()?;
            TypeAnnotationTargetInfoType::FormalParameter {
                formal_parameter_index: ConstantPoolIndex::new(parameter_index),
            }
        }
        0x17 => {
            let type_index = reader.take::<u16>()?;
            TypeAnnotationTargetInfoType::Throws {
                throws_type_index: ConstantPoolIndex::new(type_index),
            }
        }
        0x40 | 0x41 => {
            let table_length = reader.take::<u16>()?;
            let mut table = Vec::with_capacity(table_length as usize);
            for _ in 0..table_length {
                let start_pc = reader.take::<u16>()?;
                let length = reader.take::<u16>()?;
                let index = reader.take::<u16>()?;
                table.push(LocalVarTargetTableEntry {
                    start_pc,
                    length,
                    index: ConstantPoolIndex::new(index),
                });
            }
            TypeAnnotationTargetInfoType::LocalVar { table }
        }
        0x42 => {
            let exception_table_index = reader.take::<u16>()?;
            TypeAnnotationTargetInfoType::Catch {
                exception_table_index: ConstantPoolIndex::new(exception_table_index),
            }
        }
        0x43..=0x46 => {
            let offset = reader.take::<u16>()?;
            TypeAnnotationTargetInfoType::Offset { offset }
        }
        0x47..=0x4B => {
            let offset = reader.take::<u16>()?;
            let type_argument_index = reader.take::<u8>()?;
            TypeAnnotationTargetInfoType::TypeArgument {
                offset,
                type_argument_index: ConstantPoolIndex::new(type_argument_index),
            }
        }
        _ => return Err(BytecodeError::InvalidData),
    };

    Ok(TypeAnnotationTargetInfo { target_info })
}

#[derive(Debug)]
pub struct ConstantValueAttributeFactory;

#[derive(Debug)]
pub struct CodeAttributeFactory;

#[derive(Debug)]
pub struct LineNumberTableAttributeFactory;

impl AttributeFactory for ConstantValueAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        _pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let constantvalue_index = reader.take::<u16>()?;

        Ok(Box::new(ConstantValueInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            constantvalue_index: ConstantPoolIndex::new(constantvalue_index),
        }))
    }
}

impl AttributeFactory for CodeAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        pool: &mut ConstantPool,
        container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let max_stack = reader.take::<u16>()?;
        let max_locals = reader.take::<u16>()?;
        let code_length = reader.take::<u32>()?;
        let code = reader.take_bytes(code_length as usize)?;
        let exception_table_length = reader.take::<u16>()?;
        let mut exception_table = Vec::with_capacity(exception_table_length as usize);
        for _ in 0..exception_table_length {
            let start_pc = reader.take::<u16>()?;
            let end_pc = reader.take::<u16>()?;
            let handler_pc = reader.take::<u16>()?;
            let catch_type = reader.take::<u16>()?;
            exception_table.push(ExceptionTableEntry {
                start_pc,
                end_pc,
                handler_pc,
                catch_type: ConstantPoolIndex::new(catch_type),
            });
        }
        let attributes_count = reader.take::<u16>()?;
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            let attribute = read_attribute(reader, pool, container)?;
            attributes.push(attribute);
        }

        Ok(Box::new(CodeInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            max_stack,
            max_locals,
            code_length,
            code: code.to_vec(),
            exception_table_length,
            exception_table,
            attributes_count,
            attributes,
        }))
    }
}

#[derive(Debug)]
pub struct StackMapTableAttributeFactory;

impl AttributeFactory for StackMapTableAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        _pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let number_of_entries = reader.take::<u16>()?;
        let frame_type = reader.take::<u8>()?;
        let mut entries = Vec::with_capacity(number_of_entries as usize);
        for _ in 0..number_of_entries {
            let entry = match frame_type {
                0..=63 => StackMapFrame::SameFrame { frame_type },
                64..=127 => {
                    let verification_type = reader.take::<u8>()?;
                    let verification_type = match verification_type {
                        0 => VerificationTypeInfo::Top,
                        1 => VerificationTypeInfo::Integer,
                        2 => VerificationTypeInfo::Float,
                        3 => VerificationTypeInfo::Double,
                        4 => VerificationTypeInfo::Long,
                        5 => VerificationTypeInfo::Null,
                        6 => VerificationTypeInfo::UninitializedThis,
                        7 => {
                            let class = reader.take::<u16>()?;
                            VerificationTypeInfo::Object {
                                class: class.into(),
                            }
                        }
                        8 => {
                            let offset = reader.take::<u16>()?;
                            VerificationTypeInfo::Uninitialized { offset }
                        }
                        _ => return Err(BytecodeError::UnsupportedVerificationType),
                    };
                    StackMapFrame::SameLocals1StackItemFrame {
                        frame_type,
                        stack: verification_type,
                    }
                }
                247 => {
                    let offset_delta = reader.take::<u16>()?;
                    let verification_type = reader.take::<u8>()?;
                    let verification_type = match verification_type {
                        0 => VerificationTypeInfo::Top,
                        1 => VerificationTypeInfo::Integer,
                        2 => VerificationTypeInfo::Float,
                        3 => VerificationTypeInfo::Double,
                        4 => VerificationTypeInfo::Long,
                        5 => VerificationTypeInfo::Null,
                        6 => VerificationTypeInfo::UninitializedThis,
                        7 => {
                            let class = reader.take::<u16>()?;
                            VerificationTypeInfo::Object {
                                class: class.into(),
                            }
                        }
                        8 => {
                            let offset = reader.take::<u16>()?;
                            VerificationTypeInfo::Uninitialized { offset }
                        }
                        _ => return Err(BytecodeError::UnsupportedVerificationType),
                    };

                    StackMapFrame::SameLocals1StackItemFrameExtended {
                        frame_type,
                        offset_delta,
                        stack: verification_type,
                    }
                }
                248..=250 => {
                    let offset_delta = reader.take::<u16>()?;
                    StackMapFrame::ChopFrame {
                        frame_type,
                        offset_delta,
                    }
                }
                251 => {
                    let offset_delta = reader.take::<u16>()?;
                    StackMapFrame::SameFrameExtended {
                        frame_type,
                        offset_delta,
                    }
                }
                252..=254 => {
                    let offset_delta = reader.take::<u16>()?;
                    let mut locals = Vec::with_capacity(frame_type as usize - 251);
                    for _ in 0..frame_type - 251 {
                        let verification_type = reader.take::<u8>()?;
                        let verification_type = match verification_type {
                            0 => VerificationTypeInfo::Top,
                            1 => VerificationTypeInfo::Integer,
                            2 => VerificationTypeInfo::Float,
                            3 => VerificationTypeInfo::Double,
                            4 => VerificationTypeInfo::Long,
                            5 => VerificationTypeInfo::Null,
                            6 => VerificationTypeInfo::UninitializedThis,
                            7 => {
                                let class = reader.take::<u16>()?;
                                VerificationTypeInfo::Object {
                                    class: class.into(),
                                }
                            }
                            8 => {
                                let offset = reader.take::<u16>()?;
                                VerificationTypeInfo::Uninitialized { offset }
                            }
                            _ => return Err(BytecodeError::UnsupportedVerificationType),
                        };
                        locals.push(verification_type);
                    }
                    StackMapFrame::AppendFrame {
                        frame_type,
                        offset_delta,
                        locals,
                    }
                }
                255 => {
                    let offset_delta = reader.take::<u16>()?;
                    let number_of_locals = reader.take::<u16>()?;
                    let mut locals = Vec::with_capacity(number_of_locals as usize);
                    for _ in 0..number_of_locals {
                        let verification_type = reader.take::<u8>()?;
                        let verification_type = match verification_type {
                            0 => VerificationTypeInfo::Top,
                            1 => VerificationTypeInfo::Integer,
                            2 => VerificationTypeInfo::Float,
                            3 => VerificationTypeInfo::Double,
                            4 => VerificationTypeInfo::Long,
                            5 => VerificationTypeInfo::Null,
                            6 => VerificationTypeInfo::UninitializedThis,
                            7 => {
                                let class = reader.take::<u16>()?;
                                VerificationTypeInfo::Object {
                                    class: class.into(),
                                }
                            }
                            8 => {
                                let offset = reader.take::<u16>()?;
                                VerificationTypeInfo::Uninitialized { offset }
                            }
                            _ => return Err(BytecodeError::UnsupportedVerificationType),
                        };
                        locals.push(verification_type);
                    }
                    let number_of_stack_items = reader.take::<u16>()?;
                    let mut stack = Vec::with_capacity(number_of_stack_items as usize);
                    for _ in 0..number_of_stack_items {
                        let verification_type = reader.take::<u8>()?;
                        let verification_type = match verification_type {
                            0 => VerificationTypeInfo::Top,
                            1 => VerificationTypeInfo::Integer,
                            2 => VerificationTypeInfo::Float,
                            3 => VerificationTypeInfo::Double,
                            4 => VerificationTypeInfo::Long,
                            5 => VerificationTypeInfo::Null,
                            6 => VerificationTypeInfo::UninitializedThis,
                            7 => {
                                let class = reader.take::<u16>()?;
                                VerificationTypeInfo::Object {
                                    class: class.into(),
                                }
                            }
                            8 => {
                                let offset = reader.take::<u16>()?;
                                VerificationTypeInfo::Uninitialized { offset }
                            }
                            _ => return Err(BytecodeError::UnsupportedVerificationType),
                        };
                        stack.push(verification_type);
                    }
                    StackMapFrame::FullFrame {
                        frame_type,
                        number_of_locals,
                        number_of_stack_items,
                        offset_delta,
                        locals,
                        stack,
                    }
                }
                _ => return Err(BytecodeError::InvalidData),
            };

            entries.push(entry);
        }

        Ok(Box::new(StackMapTableInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            number_of_entries,
            entries,
        }))
    }
}

#[derive(Debug)]
pub struct ExceptionsAttributeFactory;

impl AttributeFactory for ExceptionsAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        _pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let number_of_exceptions = reader.take::<u16>()?;
        let mut exception_index_table = Vec::with_capacity(number_of_exceptions as usize);
        for _ in 0..number_of_exceptions {
            let exception_index = reader.take::<u16>()?;
            exception_index_table.push(ConstantPoolIndex::new(exception_index));
        }

        Ok(Box::new(ExceptionsInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            number_of_exceptions,
            exception_index_table,
        }))
    }
}

#[derive(Debug)]
pub struct InnerClassesAttributeFactory;

impl AttributeFactory for InnerClassesAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        _pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let number_of_classes = reader.take::<u16>()?;
        let mut classes = Vec::with_capacity(number_of_classes as usize);
        for _ in 0..number_of_classes {
            let inner_class_info_index = reader.take::<u16>()?;
            let outer_class_info_index = reader.take::<u16>()?;
            let inner_name_index = reader.take::<u16>()?;
            let inner_class_access_flags = reader.take::<u16>()?;
            let Some(inner_class_access_flags) =
                InnerClassAccessFlags::from_bits(inner_class_access_flags)
            else {
                return Err(BytecodeError::InvalidData);
            };
            classes.push(InnerClass {
                inner_class_info_index: ConstantPoolIndex::new(inner_class_info_index),
                outer_class_info_index: ConstantPoolIndex::new(outer_class_info_index),
                inner_name_index: ConstantPoolIndex::new(inner_name_index),
                inner_class_access_flags,
            });
        }

        Ok(Box::new(InnerClassesInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            number_of_classes,
            classes,
        }))
    }
}

#[derive(Debug)]
pub struct EnclosingMethodAttributeFactory;

impl AttributeFactory for EnclosingMethodAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        _pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let class_index = reader.take::<u16>()?;
        let method_index = reader.take::<u16>()?;

        Ok(Box::new(EnclosingMethodInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            class_index: ConstantPoolIndex::new(class_index),
            method_index: ConstantPoolIndex::new(method_index),
        }))
    }
}

#[derive(Debug)]
pub struct SyntheticAttributeFactory;

impl AttributeFactory for SyntheticAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        _pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;

        Ok(Box::new(SyntheticInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
        }))
    }
}

#[derive(Debug)]
pub struct SignatureAttributeFactory;

impl AttributeFactory for SignatureAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        _pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let signature_index = reader.take::<u16>()?;

        Ok(Box::new(SignatureInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            signature_index: ConstantPoolIndex::new(signature_index),
        }))
    }
}

#[derive(Debug)]
pub struct SourceFileAttributeFactory;

impl AttributeFactory for SourceFileAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        _pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let sourcefile_index = reader.take::<u16>()?;

        Ok(Box::new(SourceFileInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            sourcefile_index: ConstantPoolIndex::new(sourcefile_index),
        }))
    }
}

#[derive(Debug)]
pub struct SourceDebugExtensionAttributeFactory;

impl AttributeFactory for SourceDebugExtensionAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        _pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let debug_extension = reader.take::<Vec<u8>>()?;

        Ok(Box::new(SourceDebugExtensionInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            debug_extension,
        }))
    }
}

impl AttributeFactory for LineNumberTableAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        _pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let line_number_table_length = reader.take::<u16>()?;
        let mut line_number_table = Vec::with_capacity(line_number_table_length as usize);
        for _ in 0..line_number_table_length {
            let start_pc = reader.take::<u16>()?;
            let line_number = reader.take::<u16>()?;
            line_number_table.push(LineNumberTableEntry {
                start_pc,
                line_number,
            });
        }

        Ok(Box::new(LineNumberTableInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            line_number_table_length,
            line_number_table,
        }))
    }
}

#[derive(Debug)]
pub struct LocalVariableTableAttributeFactory;

impl AttributeFactory for LocalVariableTableAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        _pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let local_variable_table_length = reader.take::<u16>()?;
        let mut local_variable_table = Vec::with_capacity(local_variable_table_length as usize);
        for _ in 0..local_variable_table_length {
            let start_pc = reader.take::<u16>()?;
            let length = reader.take::<u16>()?;
            let name_index = reader.take::<u16>()?;
            let descriptor_index = reader.take::<u16>()?;
            let index = reader.take::<u16>()?;
            local_variable_table.push(LocalVariableTableEntry {
                start_pc,
                length,
                name_index: ConstantPoolIndex::new(name_index),
                descriptor_index: ConstantPoolIndex::new(descriptor_index),
                index: ConstantPoolIndex::new(index),
            });
        }

        Ok(Box::new(LocalVariableTableInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            local_variable_table_length,
            local_variable_table,
        }))
    }
}

#[derive(Debug)]
pub struct LocalVariableTypeTableAttributeFactory;

impl AttributeFactory for LocalVariableTypeTableAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        _pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let local_variable_type_table_length = reader.take::<u16>()?;
        let mut local_variable_type_table =
            Vec::with_capacity(local_variable_type_table_length as usize);
        for _ in 0..local_variable_type_table_length {
            let start_pc = reader.take::<u16>()?;
            let length = reader.take::<u16>()?;
            let name_index = reader.take::<u16>()?;
            let signature_index = reader.take::<u16>()?;
            let index = reader.take::<u16>()?;
            local_variable_type_table.push(LocalVariableTypeTableEntry {
                start_pc,
                length,
                name_index: ConstantPoolIndex::new(name_index),
                signature_index: ConstantPoolIndex::new(signature_index),
                index: ConstantPoolIndex::new(index),
            });
        }

        Ok(Box::new(LocalVariableTypeTableInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            local_variable_type_table_length,
            local_variable_type_table,
        }))
    }
}

#[derive(Debug)]
pub struct DeprecatedAttributeFactory;

impl AttributeFactory for DeprecatedAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        _pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;

        Ok(Box::new(DeprecatedInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
        }))
    }
}

#[derive(Debug)]
pub struct RuntimeVisibleAnnotationsAttributeFactory;

impl AttributeFactory for RuntimeVisibleAnnotationsAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let num_annotations = reader.take::<u16>()?;
        let mut annotations = Vec::with_capacity(num_annotations as usize);
        for _ in 0..num_annotations {
            let annotation = read_annotation(reader, pool)?;
            annotations.push(annotation);
        }

        Ok(Box::new(RuntimeVisibleAnnotationsInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            num_annotations,
            annotations,
        }))
    }
}

#[derive(Debug)]
pub struct RuntimeInvisibleAnnotationsAttributeFactory;

impl AttributeFactory for RuntimeInvisibleAnnotationsAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let num_annotations = reader.take::<u16>()?;
        let mut annotations = Vec::with_capacity(num_annotations as usize);
        for _ in 0..num_annotations {
            let annotation = read_annotation(reader, pool)?;
            annotations.push(annotation);
        }

        Ok(Box::new(RuntimeInvisibleAnnotationsInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            num_annotations,
            annotations,
        }))
    }
}

#[derive(Debug)]
pub struct RuntimeVisibleParameterAnnotationsAttributeFactory;

impl AttributeFactory for RuntimeVisibleParameterAnnotationsAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let num_parameters = reader.take::<u8>()?;
        let mut annotations = Vec::with_capacity(num_parameters as usize);
        for _ in 0..num_parameters {
            let num_annotations = reader.take::<u16>()?;
            let mut parameter_annotations = Vec::with_capacity(num_annotations as usize);
            for _ in 0..num_annotations {
                let annotation = read_annotation(reader, pool)?;
                parameter_annotations.push(annotation);
            }
            annotations.push(ParameterAnnotation {
                num_annotations,
                annotations: parameter_annotations,
            });
        }

        Ok(Box::new(RuntimeVisibleParameterAnnotationsInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            num_parameters,
            parameter_annotations: annotations,
        }))
    }
}

#[derive(Debug)]
pub struct RuntimeInvisibleParameterAnnotationsAttributeFactory;

impl AttributeFactory for RuntimeInvisibleParameterAnnotationsAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let num_parameters = reader.take::<u8>()?;
        let mut annotations = Vec::with_capacity(num_parameters as usize);
        for _ in 0..num_parameters {
            let num_annotations = reader.take::<u16>()?;
            let mut parameter_annotations = Vec::with_capacity(num_annotations as usize);
            for _ in 0..num_annotations {
                let annotation = read_annotation(reader, pool)?;
                parameter_annotations.push(annotation);
            }
            annotations.push(ParameterAnnotation {
                num_annotations,
                annotations: parameter_annotations,
            });
        }

        Ok(Box::new(RuntimeInvisibleParameterAnnotationsInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            num_parameters,
            parameter_annotations: annotations,
        }))
    }
}

#[derive(Debug)]
pub struct RuntimeVisibleTypeAnnotationsAttributeFactory;

impl AttributeFactory for RuntimeVisibleTypeAnnotationsAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let num_annotations = reader.take::<u16>()?;
        let mut annotations = Vec::with_capacity(num_annotations as usize);
        for _ in 0..num_annotations {
            let annotation = read_typeannotation(reader, pool)?;
            annotations.push(annotation);
        }

        Ok(Box::new(RuntimeVisibleTypeAnnotationsInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            num_annotations,
            annotations,
        }))
    }
}

#[derive(Debug)]
pub struct RuntimeInvisibleTypeAnnotationsAttributeFactory;

impl AttributeFactory for RuntimeInvisibleTypeAnnotationsAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let num_annotations = reader.take::<u16>()?;
        let mut annotations = Vec::with_capacity(num_annotations as usize);
        for _ in 0..num_annotations {
            let annotation = read_typeannotation(reader, pool)?;
            annotations.push(annotation);
        }

        Ok(Box::new(RuntimeInvisibleTypeAnnotationsInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            num_annotations,
            annotations,
        }))
    }
}

#[derive(Debug)]
pub struct BootstrapMethodsAttributeFactory;

impl AttributeFactory for BootstrapMethodsAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        _pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let num_bootstrap_methods = reader.take::<u16>()?;
        let mut bootstrap_methods = Vec::with_capacity(num_bootstrap_methods as usize);
        for _ in 0..num_bootstrap_methods {
            let bootstrap_method_ref = reader.take::<u16>()?;
            let num_bootstrap_arguments = reader.take::<u16>()?;
            let mut bootstrap_arguments = Vec::with_capacity(num_bootstrap_arguments as usize);
            for _ in 0..num_bootstrap_arguments {
                let bootstrap_argument = reader.take::<u16>()?;
                bootstrap_arguments.push(ConstantPoolIndex::new(bootstrap_argument));
            }
            bootstrap_methods.push(BootstrapMethod {
                bootstrap_method_ref: ConstantPoolIndex::new(bootstrap_method_ref),
                num_bootstrap_arguments,
                bootstrap_arguments,
            });
        }

        Ok(Box::new(BootstrapMethodsInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            num_bootstrap_methods,
            bootstrap_methods,
        }))
    }
}

#[derive(Debug)]
pub struct AnnotationDefaultAttributeFactory;

impl AttributeFactory for AnnotationDefaultAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let default_value = read_elementvalue(reader, pool)?;

        Ok(Box::new(AnnotationDefaultInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            default_value,
        }))
    }
}

#[derive(Debug)]
pub struct MethodParametersAttributeFactory;

impl AttributeFactory for MethodParametersAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        _pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let parameters_count = reader.take::<u8>()?;
        let mut parameters = Vec::with_capacity(parameters_count as usize);
        for _ in 0..parameters_count {
            let name_index = reader.take::<u16>()?;
            let access_flags = reader.take::<u16>()?;
            parameters.push(MethodParameter {
                name_index: ConstantPoolIndex::new(name_index),
                access_flags,
            });
        }

        Ok(Box::new(MethodParametersInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            parameters_count,
            parameters,
        }))
    }
}

#[derive(Debug)]
pub struct ModuleAttributeFactory;

impl AttributeFactory for ModuleAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        _pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let name_index = reader.take::<u16>()?;
        let access_flags = reader.take::<u16>()?;
        let version_index = reader.take::<u16>()?;
        let requires_count = reader.take::<u16>()?;
        let mut requires = Vec::with_capacity(requires_count as usize);
        for _ in 0..requires_count {
            let requires_index = reader.take::<u16>()?;
            let requires_flags = reader.take::<u16>()?;
            let requires_version_index = reader.take::<u16>()?;
            requires.push(Requires {
                requires_index: ConstantPoolIndex::new(requires_index),
                requires_flags,
                requires_version_index: ConstantPoolIndex::new(requires_version_index),
            });
        }

        let exports_count = reader.take::<u16>()?;
        let mut exports = Vec::with_capacity(exports_count as usize);
        for _ in 0..exports_count {
            let exports_index = reader.take::<u16>()?;
            let exports_flags = reader.take::<u16>()?;
            let exports_to_count = reader.take::<u16>()?;
            let mut indices = Vec::with_capacity(exports_to_count as usize);
            for _ in 0..exports_to_count {
                let result = reader.take::<u16>()?;
                indices.push(ConstantPoolIndex::new(result));
            }
            exports.push(Exports {
                exports_index: ConstantPoolIndex::new(exports_index),
                exports_flags,
                exports_to_count,
                exports_to_index: indices,
            });
        }

        let opens_count = reader.take::<u16>()?;
        let mut opens = Vec::with_capacity(opens_count as usize);
        for _ in 0..opens_count {
            let opens_index = reader.take::<u16>()?;
            let opens_flags = reader.take::<u16>()?;
            let opens_to_count = reader.take::<u16>()?;
            let mut indices = Vec::with_capacity(opens_to_count as usize);
            for _ in 0..opens_to_count {
                let result = reader.take::<u16>()?;
                indices.push(ConstantPoolIndex::new(result));
            }
            opens.push(Opens {
                opens_index: ConstantPoolIndex::new(opens_index),
                opens_flags,
                opens_to_count,
                opens_to_index: indices,
            });
        }

        let uses_count = reader.take::<u16>()?;
        let mut uses_index = Vec::with_capacity(uses_count as usize);
        for _ in 0..uses_count {
            let result = reader.take::<u16>()?;
            uses_index.push(ConstantPoolIndex::new(result));
        }

        let provides_count = reader.take::<u16>()?;
        let mut provides = Vec::with_capacity(provides_count as usize);
        for _ in 0..provides_count {
            let provides_index = reader.take::<u16>()?;
            let provides_with_count = reader.take::<u16>()?;
            let mut indices = Vec::with_capacity(provides_with_count as usize);
            for _ in 0..provides_with_count {
                let result = reader.take::<u16>()?;
                indices.push(ConstantPoolIndex::new(result));
            }
            provides.push(Provides {
                provides_index: ConstantPoolIndex::new(provides_index),
                provides_with_count,
                provides_with_index: indices,
            });
        }

        Ok(Box::new(ModuleInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            module_flags: access_flags,
            module_name_index: ConstantPoolIndex::new(name_index),
            module_version_index: ConstantPoolIndex::new(version_index),
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
        }))
    }
}

#[derive(Debug)]
pub struct ModulePackagesAttributeFactory;

impl AttributeFactory for ModulePackagesAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        _pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let package_count = reader.take::<u16>()?;
        let mut package_index = Vec::with_capacity(package_count as usize);
        for _ in 0..package_count {
            let result = reader.take::<u16>()?;
            package_index.push(ConstantPoolIndex::new(result));
        }

        Ok(Box::new(ModulePackagesInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            package_count,
            package_index,
        }))
    }
}

#[derive(Debug)]
pub struct ModuleMainClassAttributeFactory;

impl AttributeFactory for ModuleMainClassAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        _pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let main_class_index = reader.take::<u16>()?;

        Ok(Box::new(ModuleMainClassInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            main_class_index: ConstantPoolIndex::new(main_class_index),
        }))
    }
}

#[derive(Debug)]
pub struct NestHostAttributeFactory;

impl AttributeFactory for NestHostAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        _pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let host_class_index = reader.take::<u16>()?;

        Ok(Box::new(NestHostInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            host_class_index: ConstantPoolIndex::new(host_class_index),
        }))
    }
}

#[derive(Debug)]
pub struct NestMembersAttributeFactory;

impl AttributeFactory for NestMembersAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        _pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let number_of_classes = reader.take::<u16>()?;
        let mut classes = Vec::with_capacity(number_of_classes as usize);
        for _ in 0..number_of_classes {
            let idx = reader.take::<u16>()?;
            classes.push(ConstantPoolIndex::new(idx));
        }

        Ok(Box::new(NestMembersInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            number_of_classes,
            classes,
        }))
    }
}

#[derive(Debug)]
pub struct RecordAttributeFactory;

impl AttributeFactory for RecordAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        pool: &mut ConstantPool,
        container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let component_count = reader.take::<u16>()?;
        let mut attributes = Vec::with_capacity(component_count as usize);
        for _ in 0..component_count {
            let attr = read_attribute(reader, pool, container)?;
            attributes.push(attr);
        }

        Ok(Box::new(RecordInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            component_count,
            components: attributes,
        }))
    }
}

#[derive(Debug)]
pub struct PermittedSubtypesAttributeFactory;

impl AttributeFactory for PermittedSubtypesAttributeFactory {
    fn make(
        &self,
        reader: &mut BufferedReader,
        pool: &mut ConstantPool,
        _container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError> {
        let attribute_name_index = reader.take::<u16>()?;
        let attribute_length = reader.take::<u32>()?;
        let number_of_classes = reader.take::<u16>()?;
        let mut classes = Vec::with_capacity(number_of_classes as usize);
        for _ in 0..number_of_classes {
            let class_index = reader.take::<u16>()?;
            let Some(class) = pool.text_of(class_index.into()) else {
                return Err(BytecodeError::InvalidData);
            };
            classes.push(class);
        }

        Ok(Box::new(PermittedSubtypesInfo {
            attribute_name_index: ConstantPoolIndex::new(attribute_name_index),
            attribute_length,
            number_of_classes,
            classes,
        }))
    }
}

fn read_typepath(
    reader: &mut BufferedReader,
    _cp: &mut ConstantPool,
) -> Result<TypePath, BytecodeError> {
    let path_length = reader.take::<u8>()?;
    let mut path = Vec::with_capacity(path_length as usize);
    for _ in 0..path_length {
        let type_path_kind = reader.take::<u8>()?;
        let type_argument_index = reader.take::<u8>()?;
        path.push(TypePathEntry {
            type_path_kind,
            type_argument_index: ConstantPoolIndex::new(type_argument_index),
        });
    }

    Ok(TypePath { path_length, path })
}

#[allow(clippy::only_used_in_recursion)]
fn read_elementvalue(
    reader: &mut BufferedReader,
    cp: &mut ConstantPool,
) -> Result<ElementValue, BytecodeError> {
    let tag = reader.take::<u8>()?;
    let value = match tag {
        b'B' | b'C' | b'D' | b'F' | b'I' | b'J' | b'S' | b'Z' | b's' => {
            let const_value_index = reader.take::<u16>()?;
            ElementValue::ConstValueIndex(ConstantPoolIndex::new(const_value_index))
        }
        b'e' => {
            let type_name_index = reader.take::<u16>()?;
            let const_name_index = reader.take::<u16>()?;
            ElementValue::EnumConstValue {
                type_name_index: ConstantPoolIndex::new(type_name_index),
                const_name_index: ConstantPoolIndex::new(const_name_index),
            }
        }
        b'c' => {
            let class_info_index = reader.take::<u16>()?;
            ElementValue::ClassInfoIndex(ConstantPoolIndex::new(class_info_index))
        }
        b'@' => {
            let type_index = reader.take::<u16>()?;
            let num_element_value_pairs = reader.take::<u16>()?;
            let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);
            for _ in 0..num_element_value_pairs {
                let element_name_index = reader.take::<u16>()?;
                let element_value = read_elementvalue(reader, cp)?;
                element_value_pairs.push(ElementValuePair {
                    element_name_index: ConstantPoolIndex::new(element_name_index),
                    value: element_value,
                });
            }
            ElementValue::Annotation(Annotation {
                type_index: ConstantPoolIndex::new(type_index),
                num_element_value_pairs,
                element_value_pairs,
            })
        }
        b'[' => {
            let num_values = reader.take::<u16>()?;
            let mut values = Vec::with_capacity(num_values as usize);
            for _ in 0..num_values {
                let value = read_elementvalue(reader, cp)?;
                values.push(value);
            }
            ElementValue::Array { num_values, values }
        }
        _ => return Err(BytecodeError::InvalidData),
    };

    Ok(value)
}
