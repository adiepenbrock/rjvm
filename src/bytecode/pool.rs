use std::collections::HashMap;

use crate::bytecode::BytecodeError;

/// The constant pool index is a 1-based index used to reference items in the [`ConstantPool`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConstantPoolIndex(usize);

impl From<usize> for ConstantPoolIndex {
    fn from(value: usize) -> Self {
        ConstantPoolIndex(value)
    }
}

impl From<u16> for ConstantPoolIndex {
    fn from(value: u16) -> Self {
        ConstantPoolIndex(value as usize)
    }
}

impl ConstantPoolIndex {
    pub fn new<T: Into<usize>>(value: T) -> Self {
        Self(value.into())
    }

    /// Returns the value of the index.
    pub fn index(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstantPoolEntry {
    /// The `CONSTANT_Class_info` constant is used to represent a class or an interface.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.1>
    Class { name_index: ConstantPoolIndex },
    /// The `CONSTANT_Fieldref_info` constant is used to represent a reference to a field.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.2>
    FieldRef {
        class_index: ConstantPoolIndex,
        name_and_type_index: ConstantPoolIndex,
    },
    /// The `CONSTANT_Methodref_info` constant is used to represent a reference to a method.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.2>
    MethodRef {
        class_index: ConstantPoolIndex,
        name_and_type_index: ConstantPoolIndex,
    },
    /// The `CONSTANT_InterfaceMethodref_info` constant is used to represent a reference to an
    /// interface method.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.2>
    InterfaceMethodRef {
        class_index: ConstantPoolIndex,
        name_and_type_index: ConstantPoolIndex,
    },
    /// The `CONSTANT_String_info` constant is used to represent constant objects of the
    /// type `String`
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.3>
    String { string_index: ConstantPoolIndex },
    /// The `CONSTANT_Integer_info` constant is used to represent 4-byte numeric (int) constants.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.4>
    Integer { bytes: i32 },
    /// The `CONSTANT_Float_info` constant is used to represent 4-byte floating-point constants.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.4>
    Float { bytes: f32 },
    /// The `CONSTANT_Long_info` constant is used to represent 8-byte numeric (long) constants.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.5>
    Long { high_bytes: u32, low_bytes: u32 },
    /// The `CONSTANT_Double_info` constant is used to represent 8-byte numeric (double) constants.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.5>
    Double { high_bytes: u32, low_bytes: u32 },
    /// The `CONSTANT_NameAndType_info` constant is used to represent a field or method, without
    /// indicating which class or interface type it belongs to.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.6>
    NameAndType {
        name_index: ConstantPoolIndex,
        descriptor_index: ConstantPoolIndex,
    },
    /// The `CONSTANT_Utf8_info` constant is used to represent constant string values.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.7>
    Utf8 { length: u16, bytes: Vec<u8> },
    /// The `CONSTANT_MethodHandle_info` constant is used to represent a method handle.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.8>
    MethodHandle {
        reference_kind: u8,
        reference_index: ConstantPoolIndex,
    },
    /// The `CONSTANT_MethodType_info` constant is used to represent a method type.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.9>
    MethodType { descriptor_index: ConstantPoolIndex },
    /// The `CONSTANT_Dynamic_info` constant is used to represent a dynamically-computed constant.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.10>
    Dynamic {
        bootstrap_method_attr_index: ConstantPoolIndex,
        name_and_type_index: ConstantPoolIndex,
    },
    /// The `CONSTANT_InvokeDynamic_info` constant is used to represent an invokedynamic call site.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.10>
    InvokeDynamic {
        bootstrap_method_attr_index: ConstantPoolIndex,
        name_and_type_index: ConstantPoolIndex,
    },
    /// The `CONSTANT_Module_info` constant is used to represent a module.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.11>
    Module { name_index: ConstantPoolIndex },
    /// The `CONSTANT_Package_info` constant is used to represent a package exported or opened
    /// by a module.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.12>
    Package { name_index: ConstantPoolIndex },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantPool {
    entries: HashMap<ConstantPoolIndex, ConstantPoolEntry>,
}

impl ConstantPool {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Insert a new entry into the [ConstantPool] at the given index. If the index is already
    /// present in the [ConstantPool], this function will return an error.
    pub fn insert(
        &mut self,
        index: ConstantPoolIndex,
        value: ConstantPoolEntry,
    ) -> Result<(), BytecodeError> {
        if self.entries.contains_key(&index) {
            return Err(BytecodeError::ConstantPoolEntryAlreadyExists);
        }

        self.entries.insert(index, value);
        Ok(())
    }

    /// Get a reference to the [entry](ConstantPoolEntry) at the given index in the [ConstantPool].
    /// If the index is not present in the [ConstantPool], this function will return `None`.
    pub fn get(&self, index: ConstantPoolIndex) -> Option<&ConstantPoolEntry> {
        self.entries.get(&index)
    }

    /// Removes the entry at the given index from the [ConstantPool] and returns it. If the index is
    /// not present in the [ConstantPool], this function will return `None`.
    pub fn remove(&mut self, index: ConstantPoolIndex) -> Option<ConstantPoolEntry> {
        self.entries.remove(&index)
    }

    /// Returns the number of entries in the [ConstantPool].
    pub fn size(&self) -> usize {
        self.entries.len()
    }

    /// Convenience function to check if the [ConstantPool] is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the text representation of the entry at the given index in the [ConstantPool]. If
    /// the index is not present in the [ConstantPool], this function will return `None`.
    pub fn text_of(&self, index: ConstantPoolIndex) -> Option<String> {
        let entry = match self.get(index) {
            Some(entry) => entry,
            None => return None,
        };

        match entry {
            ConstantPoolEntry::Utf8 { bytes, .. } => {
                Some(String::from_utf8(bytes.clone()).unwrap())
            }
            ConstantPoolEntry::String { string_index } => {
                self.text_of(ConstantPoolIndex::from(*string_index))
            }
            ConstantPoolEntry::Integer { bytes } => Some(bytes.to_string()),
            ConstantPoolEntry::Float { bytes } => Some(bytes.to_string()),
            ConstantPoolEntry::MethodRef {
                class_index,
                name_and_type_index,
            } => Some(format!(
                "{}.{}",
                self.text_of(ConstantPoolIndex::from(*class_index))?,
                self.text_of(ConstantPoolIndex::from(*name_and_type_index))?
            )),
            ConstantPoolEntry::InterfaceMethodRef {
                class_index,
                name_and_type_index,
            } => Some(format!(
                "{}.{}",
                self.text_of(ConstantPoolIndex::from(*class_index))?,
                self.text_of(ConstantPoolIndex::from(*name_and_type_index))?
            )),
            ConstantPoolEntry::NameAndType {
                name_index,
                descriptor_index,
            } => Some(format!(
                "{}: {}",
                self.text_of(ConstantPoolIndex::from(*name_index))?,
                self.text_of(ConstantPoolIndex::from(*descriptor_index))?
            )),
            ConstantPoolEntry::Class { name_index } => {
                self.text_of(ConstantPoolIndex::from(*name_index))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstantTag {
    /// The `CONSTANT_Class_info` constnat is used to represent a class or an interface.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.1>
    Class,
    /// The `CONSTANT_Fieldref_info` constant is used to represent a reference to a field.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.2>
    FieldRef,
    /// The `CONSTANT_Methodref_info` constant is used to represent a reference to a method.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.2>
    MethodRef,
    /// The `CONSTANT_InterfaceMethodref_info` constant is used to represent a reference to an
    /// interface method.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.2>
    InterfaceMethodRef,
    /// The `CONSTANT_String_info` constant is used to represent constant objects of the
    /// type `String`
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.3>
    String,
    /// The `CONSTANT_Integer_info` constant is used to represent 4-byte numeric (int) constants.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.4>
    Integer,
    /// The `CONSTANT_Float_info` constant is used to represent 4-byte floating-point constants.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.4>
    Float,
    /// The `CONSTANT_Long_info` constant is used to represent 8-byte numeric (long) constants.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.5>
    Long,
    /// The `CONSTANT_Double_info` constant is used to represent 8-byte numeric (double) constants.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.5>
    Double,
    /// The `CONSTANT_NameAndType_info` constant is used to represent a field or method, without
    /// indicating which class or interface type it belongs to.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.6>
    NameAndType,
    /// The `CONSTANT_Utf8_info` constant is used to represent constant string values.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.7>
    Utf8,
    /// The `CONSTANT_MethodHandle_info` constant is used to represent a method handle.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.8>
    MethodHandle,
    /// The `CONSTANT_MethodType_info` constant is used to represent a method type.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.9>
    MethodType,
    /// The `CONSTANT_Dynamic_info` constant is used to represent a dynamically-computed constant.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.10>
    Dynamic,
    /// The `CONSTANT_InvokeDynamic_info` constant is used to represent an invokedynamic call site.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.10>
    InvokeDynamic,
    /// The `CONSTANT_Module_info` constant is used to represent a module.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.11>
    Module,
    /// The `CONSTANT_Package_info` constant is used to represent a package exported or opened
    /// by a module.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.12>
    Package,
}

impl ConstantTag {
    pub fn from_tag(tag: u8) -> Option<ConstantTag> {
        match tag {
            // <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.1>
            1 => Some(ConstantTag::Utf8),
            // <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.4>
            3 => Some(ConstantTag::Integer),
            // <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.4>
            4 => Some(ConstantTag::Float),
            // <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.5>
            5 => Some(ConstantTag::Long),
            // <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.5>
            6 => Some(ConstantTag::Double),
            // <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.1>
            7 => Some(ConstantTag::Class),
            // <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.3>
            8 => Some(ConstantTag::String),
            // <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.2>
            9 => Some(ConstantTag::FieldRef),
            // <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.2>
            10 => Some(ConstantTag::MethodRef),
            // <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.2>
            11 => Some(ConstantTag::InterfaceMethodRef),
            // <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.6>
            12 => Some(ConstantTag::NameAndType),
            // <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.8>
            15 => Some(ConstantTag::MethodHandle),
            // <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.9>
            16 => Some(ConstantTag::MethodType),
            // <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.10>
            17 => Some(ConstantTag::Dynamic),
            // <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.10>
            18 => Some(ConstantTag::InvokeDynamic),
            // <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.11>
            19 => Some(ConstantTag::Module),
            // <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.12>
            20 => Some(ConstantTag::Package),
            _ => None,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::bytecode::pool::ConstantPoolIndex;

    #[test]
    fn constant_pool_index_from_impl() {
        let cpi = ConstantPoolIndex::from(42u16);
        assert_eq!(cpi, ConstantPoolIndex(42));
        let cpi = ConstantPoolIndex::from(42usize);
        assert_eq!(cpi, ConstantPoolIndex(42));
    }
}
