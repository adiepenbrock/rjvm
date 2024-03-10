use crate::bytecode::{BytecodeError, pool::{ConstantPool, ConstantPoolEntry, ConstantPoolIndex, ConstantTag}, reader::BufferedReader};

pub fn read_constant_pool_entry(
    reader: &mut BufferedReader,
    _cp: &mut ConstantPool,
) -> Result<ConstantPoolEntry, BytecodeError> {
    let tag = reader.take::<u8>()?;
    let Some(tag) = ConstantTag::from_tag(tag) else {
        return Err(BytecodeError::InvalidData);
    };

    let entry = match tag {
        ConstantTag::Class => {
            let name_index = reader.take::<u16>()?;

            ConstantPoolEntry::Class {
                name_index: ConstantPoolIndex::new(name_index),
            }
        }
        ConstantTag::FieldRef => {
            let class_index = reader.take::<u16>()?;
            let name_and_type_index = reader.take::<u16>()?;

            ConstantPoolEntry::FieldRef {
                class_index: ConstantPoolIndex::new(class_index),
                name_and_type_index: ConstantPoolIndex::new(name_and_type_index),
            }
        }
        ConstantTag::MethodRef => {
            let class_index = reader.take::<u16>()?;
            let name_and_type_index = reader.take::<u16>()?;

            ConstantPoolEntry::MethodRef {
                class_index: ConstantPoolIndex::new(class_index),
                name_and_type_index: ConstantPoolIndex::new(name_and_type_index),
            }
        }
        ConstantTag::InterfaceMethodRef => {
            let class_index = reader.take::<u16>()?;
            let name_and_type_index = reader.take::<u16>()?;

            ConstantPoolEntry::InterfaceMethodRef {
                class_index: ConstantPoolIndex::new(class_index),
                name_and_type_index: ConstantPoolIndex::new(name_and_type_index),
            }
        }
        ConstantTag::String => {
            let string_index = reader.take::<u16>()?;

            ConstantPoolEntry::String {
                string_index: ConstantPoolIndex::new(string_index),
            }
        }
        ConstantTag::Integer => {
            let bytes = reader.take::<i32>()?;

            ConstantPoolEntry::Integer { bytes }
        }
        ConstantTag::Float => {
            let bytes = reader.take::<f32>().expect("msg");

            ConstantPoolEntry::Float { bytes }
        }
        ConstantTag::Long => {
            let high_bytes = reader.take::<u32>()?;
            let low_bytes = reader.take::<u32>()?;

            ConstantPoolEntry::Long {
                high_bytes,
                low_bytes,
            }
        }
        ConstantTag::Double => {
            let high_bytes = reader.take::<u32>()?;
            let low_bytes = reader.take::<u32>()?;

            ConstantPoolEntry::Double {
                high_bytes,
                low_bytes,
            }
        }
        ConstantTag::NameAndType => {
            let name_index = reader.take::<u16>()?;
            let descriptor_index = reader.take::<u16>()?;

            ConstantPoolEntry::NameAndType {
                name_index: ConstantPoolIndex::new(name_index),
                descriptor_index: ConstantPoolIndex::new(descriptor_index),
            }
        }
        ConstantTag::Utf8 => {
            let length = reader.take::<u16>()?;
            let bytes = reader.take_bytes(length as usize)?;

            ConstantPoolEntry::Utf8 {
                length,
                bytes: bytes.to_vec(),
            }
        }
        ConstantTag::MethodHandle => {
            let reference_kind = reader.take::<u8>()?;
            let reference_index = reader.take::<u16>()?;

            ConstantPoolEntry::MethodHandle {
                reference_kind,
                reference_index: ConstantPoolIndex::new(reference_index),
            }
        }
        ConstantTag::MethodType => {
            let descriptor_index = reader.take::<u16>()?;

            ConstantPoolEntry::MethodType {
                descriptor_index: ConstantPoolIndex::new(descriptor_index),
            }
        }
        ConstantTag::Dynamic => {
            let bootstrap_method_attr_index = reader.take::<u16>()?;
            let name_and_type_index = reader.take::<u16>()?;

            ConstantPoolEntry::Dynamic {
                bootstrap_method_attr_index: ConstantPoolIndex::new(bootstrap_method_attr_index),
                name_and_type_index: ConstantPoolIndex::new(name_and_type_index),
            }
        }
        ConstantTag::InvokeDynamic => {
            let bootstrap_method_attr_index = reader.take::<u16>()?;
            let name_and_type_index = reader.take::<u16>()?;

            ConstantPoolEntry::InvokeDynamic {
                bootstrap_method_attr_index: ConstantPoolIndex::new(bootstrap_method_attr_index),
                name_and_type_index: ConstantPoolIndex::new(name_and_type_index),
            }
        }
        ConstantTag::Module => {
            let name_index = reader.take::<u16>()?;

            ConstantPoolEntry::Module {
                name_index: ConstantPoolIndex::new(name_index),
            }
        }
        ConstantTag::Package => {
            let name_index = reader.take::<u16>()?;

            ConstantPoolEntry::Package {
                name_index: ConstantPoolIndex::new(name_index),
            }
        }
    };

    Ok(entry)
}
