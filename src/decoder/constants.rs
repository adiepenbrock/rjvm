use crate::{
    decoder::{buffer::Buffer, error::DecodingError, Decodable},
    types::constants::{ConstantPool, ConstantPoolEntry, ConstantTag},
};

impl Decodable<ConstantPoolEntry> for ConstantPoolEntry {
    fn decode(
        buffer: &mut Buffer,
        _constant_pool: &ConstantPool,
    ) -> Result<ConstantPoolEntry, DecodingError> {
        let tag = buffer.take::<u8>().unwrap();
        let tag = ConstantTag::from_tag(tag).unwrap();

        let entry = match tag {
            ConstantTag::Class => {
                let name_index = buffer.take::<u16>().expect("msg");

                ConstantPoolEntry::Class { name_index }
            }
            ConstantTag::FieldRef => {
                let class_index = buffer.take::<u16>().expect("msg");
                let name_and_type_index = buffer.take::<u16>().expect("msg");

                ConstantPoolEntry::FieldRef {
                    class_index,
                    name_and_type_index,
                }
            }
            ConstantTag::MethodRef => {
                let class_index = buffer.take::<u16>().expect("msg");
                let name_and_type_index = buffer.take::<u16>().expect("msg");

                ConstantPoolEntry::MethodRef {
                    class_index,
                    name_and_type_index,
                }
            }
            ConstantTag::InterfaceMethodRef => {
                let class_index = buffer.take::<u16>().expect("msg");
                let name_and_type_index = buffer.take::<u16>().expect("msg");

                ConstantPoolEntry::InterfaceMethodRef {
                    class_index,
                    name_and_type_index,
                }
            }
            ConstantTag::String => {
                let string_index = buffer.take::<u16>().expect("msg");

                ConstantPoolEntry::String { string_index }
            }
            ConstantTag::Integer => {
                let bytes = buffer.take::<i32>().expect("msg");

                ConstantPoolEntry::Integer { bytes }
            }
            ConstantTag::Float => {
                let bytes = buffer.take::<f32>().expect("msg");

                ConstantPoolEntry::Float { bytes }
            }
            ConstantTag::Long => {
                let high_bytes = buffer.take::<u32>().expect("msg");
                let low_bytes = buffer.take::<u32>().expect("msg");

                ConstantPoolEntry::Long {
                    high_bytes,
                    low_bytes,
                }
            }
            ConstantTag::Double => {
                let high_bytes = buffer.take::<u32>().expect("msg");
                let low_bytes = buffer.take::<u32>().expect("msg");

                ConstantPoolEntry::Double {
                    high_bytes,
                    low_bytes,
                }
            }
            ConstantTag::NameAndType => {
                let name_index = buffer.take::<u16>().expect("msg");
                let descriptor_index = buffer.take::<u16>().expect("msg");

                ConstantPoolEntry::NameAndType {
                    name_index,
                    descriptor_index,
                }
            }
            ConstantTag::Utf8 => {
                let length = buffer.take::<u16>().expect("msg");
                let bytes = buffer.take_length(length as usize).expect("msg");

                ConstantPoolEntry::Utf8 {
                    length,
                    bytes: bytes.to_vec(),
                }
            }
            ConstantTag::MethodHandle => {
                let reference_kind = buffer.take::<u8>().expect("msg");
                let reference_index = buffer.take::<u16>().expect("msg");

                ConstantPoolEntry::MethodHandle {
                    reference_kind,
                    reference_index,
                }
            }
            ConstantTag::MethodType => {
                let descriptor_index = buffer.take::<u16>().expect("msg");

                ConstantPoolEntry::MethodType { descriptor_index }
            }
            ConstantTag::Dynamic => {
                let bootstrap_method_attr_index = buffer.take::<u16>().expect("msg");
                let name_and_type_index = buffer.take::<u16>().expect("msg");

                ConstantPoolEntry::Dynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                }
            }
            ConstantTag::InvokeDynamic => {
                let bootstrap_method_attr_index = buffer.take::<u16>().expect("msg");
                let name_and_type_index = buffer.take::<u16>().expect("msg");

                ConstantPoolEntry::InvokeDynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                }
            }
            ConstantTag::Module => {
                let name_index = buffer.take::<u16>().expect("msg");

                ConstantPoolEntry::Module { name_index }
            }
            ConstantTag::Package => {
                let name_index = buffer.take::<u16>().expect("msg");

                ConstantPoolEntry::Package { name_index }
            }
        };

        Ok(entry)
    }
}
