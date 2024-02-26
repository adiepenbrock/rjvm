use crate::types::elements::ClassFileVersion;

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantPool {
    entries: Vec<ConstantPoolEntry>,
}

impl Default for ConstantPool {
    fn default() -> Self {
        Self::new()
    }
}

impl ConstantPool {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
        }
    }

    pub fn add(&mut self, entry: ConstantPoolEntry) {
        self.entries.push(entry);
    }

    pub fn get_by_index(&self, idx: usize) -> Option<&ConstantPoolEntry> {
        if idx == 0 || idx >= self.entries.len() {
            None
        } else {
            let idx = idx - 1;
            let entry = &self.entries[idx];
            Some(entry)
        }
    }

    pub fn text_of_value(&self, index: usize) -> Option<String> {
        let entry = self.get_by_index(index)?;
        match entry {
            ConstantPoolEntry::Utf8 { bytes, .. } => {
                Some(String::from_utf8(bytes.clone()).unwrap())
            }
            ConstantPoolEntry::String { string_index } => {
                self.text_of_value(*string_index as usize)
            }
            _ => None,
        }
    }
}

impl IntoIterator for ConstantPool {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = ConstantPoolEntry;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstantPoolEntry {
    /// The `CONSTANT_Class_info` constnat is used to represent a class or an interface.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.1>
    Class { name_index: u16 },
    /// The `CONSTANT_Fieldref_info` constant is used to represent a reference to a field.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.2>
    FieldRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    /// The `CONSTANT_Methodref_info` constant is used to represent a reference to a method.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.2>
    MethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    /// The `CONSTANT_InterfaceMethodref_info` constant is used to represent a reference to an
    /// interface method.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.2>
    InterfaceMethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    /// The `CONSTANT_String_info` constant is used to represent constant objects of the
    /// type `String`
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.3>
    String { string_index: u16 },
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
        name_index: u16,
        descriptor_index: u16,
    },
    /// The `CONSTANT_Utf8_info` constant is used to represent constant string values.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.7>
    Utf8 { length: u16, bytes: Vec<u8> },
    /// The `CONSTANT_MethodHandle_info` constant is used to represent a method handle.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.8>
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
    /// The `CONSTANT_MethodType_info` constant is used to represent a method type.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.9>
    MethodType { descriptor_index: u16 },
    /// The `CONSTANT_Dynamic_info` constant is used to represent a dynamically-computed constant.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.10>
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    /// The `CONSTANT_InvokeDynamic_info` constant is used to represent an invokedynamic call site.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.10>
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    /// The `CONSTANT_Module_info` constant is used to represent a module.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.11>
    Module { name_index: u16 },
    /// The `CONSTANT_Package_info` constant is used to represent a package exported or opened
    /// by a module.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.12>
    Package { name_index: u16 },
}

impl ConstantPoolEntry {
    /// Checks whether a certain class file version (`version`) supports a particular
    /// `ConstantKind`.
    pub fn is_supported_by(&self, version: &ClassFileVersion) -> bool {
        match *self {
            ConstantPoolEntry::Utf8 { .. }
            | ConstantPoolEntry::Integer { .. }
            | ConstantPoolEntry::Float { .. }
            | ConstantPoolEntry::Long { .. }
            | ConstantPoolEntry::Double { .. }
            | ConstantPoolEntry::Class { .. }
            | ConstantPoolEntry::String { .. }
            | ConstantPoolEntry::FieldRef { .. }
            | ConstantPoolEntry::MethodRef { .. }
            | ConstantPoolEntry::InterfaceMethodRef { .. }
            | ConstantPoolEntry::NameAndType { .. }
                if version.major >= 45 && version.minor >= 3 =>
            {
                true
            }
            ConstantPoolEntry::MethodHandle { .. }
            | ConstantPoolEntry::MethodType { .. }
            | ConstantPoolEntry::InvokeDynamic { .. }
                if version.major >= 51 =>
            {
                true
            }
            ConstantPoolEntry::Module { .. } | ConstantPoolEntry::Package { .. }
                if version.major >= 53 =>
            {
                true
            }
            ConstantPoolEntry::Dynamic { .. } if version.major >= 55 => true,
            // we keep this catch-all case to avoid bugs when adding new ConstantKind variants
            // in the future and forgetting to update this function
            _ => false,
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

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum ConstantKindTag {
    /// The `CONSTANT_Class_info` constnat is used to represent a class or an interface.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.1>
    Class = 7,
    /// The `CONSTANT_Fieldref_info` constant is used to represent a reference to a field.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.2>
    Fieldref = 9,
    /// The `CONSTANT_Methodref_info` constant is used to represent a reference to a method.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.2>
    Methodref = 10,
    /// The `CONSTANT_InterfaceMethodref_info` constant is used to represent a reference to an
    /// interface method.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.2>
    InterfaceMethodref = 11,
    /// The `CONSTANT_String_info` constant is used to represent constant objects of the
    /// type `String`
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.3>
    String = 8,
    /// The `CONSTANT_Integer_info` constant is used to represent 4-byte numeric (int) constants.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.4>
    Integer = 3,
    /// The `CONSTANT_Float_info` constant is used to represent 4-byte floating-point constants.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.4>
    Float = 4,
    /// The `CONSTANT_Long_info` constant is used to represent 8-byte numeric (long) constants.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.5>
    Long = 5,
    /// The `CONSTANT_Double_info` constant is used to represent 8-byte numeric (double) constants.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.5>
    Double = 6,
    /// The `CONSTANT_NameAndType_info` constant is used to represent a field or method, without
    /// indicating which class or interface type it belongs to.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.6>
    NameAndType = 12,
    /// The `CONSTANT_Utf8_info` constant is used to represent constant string values.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.7>
    Utf8 = 1,
    /// The `CONSTANT_MethodHandle_info` constant is used to represent a method handle.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.8>
    MethodHandle = 15,
    /// The `CONSTANT_MethodType_info` constant is used to represent a method type.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.9>
    MethodType = 16,
    /// The `CONSTANT_Dynamic_info` constant is used to represent a dynamically-computed constant.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.10>
    Dynamic = 17,
    /// The `CONSTANT_InvokeDynamic_info` constant is used to represent an invokedynamic call site.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.10>
    InvokeDynamic = 18,
    /// The `CONSTANT_Module_info` constant is used to represent a module.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.11>
    Module = 19,
    /// The `CONSTANT_Package_info` constant is used to represent a package exported or opened
    /// by a module.
    /// <https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.12>
    Package = 20,
}

impl std::fmt::Display for ConstantPoolEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstantPoolEntry::Utf8 { .. } => write!(f, "CONSTANT_Utf8"),
            ConstantPoolEntry::Integer { .. } => write!(f, "CONSTANT_Integer"),
            ConstantPoolEntry::Float { .. } => write!(f, "CONSTANT_Float"),
            ConstantPoolEntry::Long { .. } => write!(f, "CONSTANT_Long"),
            ConstantPoolEntry::Double { .. } => write!(f, "CONSTANT_Double"),
            ConstantPoolEntry::Class { .. } => write!(f, "CONSTANT_Class"),
            ConstantPoolEntry::String { .. } => write!(f, "CONSTANT_String"),
            ConstantPoolEntry::FieldRef { .. } => write!(f, "CONSTANT_FieldRef"),
            ConstantPoolEntry::MethodRef { .. } => write!(f, "CONSTANT_MethodRef"),
            ConstantPoolEntry::InterfaceMethodRef { .. } => {
                write!(f, "CONSTANT_InterfaceMethodRef")
            }
            ConstantPoolEntry::NameAndType { .. } => write!(f, "CONSTANT_NameAndType"),
            ConstantPoolEntry::MethodHandle { .. } => write!(f, "CONSTANT_MethodHandle"),
            ConstantPoolEntry::MethodType { .. } => write!(f, "CONSTANT_MethodType"),
            ConstantPoolEntry::Dynamic { .. } => write!(f, "CONSTANT_Dynamic"),
            ConstantPoolEntry::InvokeDynamic { .. } => write!(f, "CONSTANT_InvokeDynamic"),
            ConstantPoolEntry::Module { .. } => write!(f, "CONSTANT_Module"),
            ConstantPoolEntry::Package { .. } => write!(f, "CONSTANT_Package"),
        }
    }
}
