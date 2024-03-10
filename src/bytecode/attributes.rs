use crate::bytecode::{flags::InnerClassAccessFlags, pool::ConstantPoolIndex};

#[derive(Debug)]
pub enum Attribute {
    ConstantValue(ConstantValueInfo),
    Code(CodeInfo),
    StackMapTable(StackMapTableInfo),
    Exceptions(ExceptionsInfo),
    InnerClasses(InnerClassesInfo),
    EnclosingMethod(EnclosingMethodInfo),
    Synthetic(SyntheticInfo),
    Signature(SignatureInfo),
    SourceFile(SourceFileInfo),
    SourceDebugExtension(SourceDebugExtensionInfo),
    LineNumberTable(LineNumberTableInfo),
    LocalVariableTable(LocalVariableTableInfo),
    LocalVariableTypeTable(LocalVariableTypeTableInfo),
    Deprecated(DeprecatedInfo),
    RuntimeVisibleAnnotations(RuntimeVisibleAnnotationsInfo),
    RuntimeInvisibleAnnotations(RuntimeInvisibleAnnotationsInfo),
    RuntimeVisibleParameterAnnotations(RuntimeVisibleParameterAnnotationsInfo),
    RuntimeInvisibleParameterAnnotations(RuntimeInvisibleParameterAnnotationsInfo),
    RuntimeVisibleTypeAnnotations(RuntimeVisibleTypeAnnotationsInfo),
    RuntimeInvisibleTypeAnnotations(RuntimeInvisibleTypeAnnotationsInfo),
    AnnotationDefault(AnnotationDefaultInfo),
    BootstrapMethods(BootstrapMethodsInfo),
    MethodParameters(MethodParametersInfo),
    Module(ModuleInfo),
    ModulePackages(ModulePackagesInfo),
    ModuleMainClass(ModuleMainClassInfo),
    NestHost(NestHostInfo),
    NestMembers(NestMembersInfo),
    Record(RecordInfo),
    PermittedSubtypes(PermittedSubtypesInfo),
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConstantValue(_) => write!(f, "ConstantValue"),
            Self::Code(_) => write!(f, "Code"),
            Self::StackMapTable(_) => write!(f, "StackMapTable"),
            Self::Exceptions(_) => write!(f, "Exceptions"),
            Self::InnerClasses(_) => write!(f, "InnerClasses"),
            Self::EnclosingMethod(_) => write!(f, "EnclosingMethod"),
            Self::Synthetic(_) => write!(f, "Synthetic"),
            Self::Signature(_) => write!(f, "Signature"),
            Self::SourceFile(_) => write!(f, "SourceFile"),
            Self::SourceDebugExtension(_) => write!(f, "SourceDebugExtension"),
            Self::LineNumberTable(_) => write!(f, "LineNumberTable"),
            Self::LocalVariableTable(_) => write!(f, "LocalVariableTable"),
            Self::LocalVariableTypeTable(_) => write!(f, "LocalVariableTypeTable"),
            Self::Deprecated(_) => write!(f, "Deprecated"),
            Self::RuntimeVisibleAnnotations(_) => write!(f, "RuntimeVisibleAnnotations"),
            Self::RuntimeInvisibleAnnotations(_) => write!(f, "RuntimeInvisibleAnnotations"),
            Self::RuntimeVisibleParameterAnnotations(_) => {
                write!(f, "RuntimeVisibleParameterAnnotations")
            }
            Self::RuntimeInvisibleParameterAnnotations(_) => {
                write!(f, "RuntimeInvisibleParameterAnnotations")
            }
            Self::RuntimeVisibleTypeAnnotations(_) => write!(f, "RuntimeVisibleTypeAnnotations"),
            Self::RuntimeInvisibleTypeAnnotations(_) => {
                write!(f, "RuntimeInvisibleTypeAnnotations")
            }
            Self::AnnotationDefault(_) => write!(f, "AnnotationDefault"),
            Self::BootstrapMethods(_) => write!(f, "BootstrapMethods"),
            Self::MethodParameters(_) => write!(f, "MethodParameters"),
            Self::Module(_) => write!(f, "Module"),
            Self::ModulePackages(_) => write!(f, "ModulePackages"),
            Self::ModuleMainClass(_) => write!(f, "ModuleMainClass"),
            Self::NestHost(_) => write!(f, "NestHost"),
            Self::NestMembers(_) => write!(f, "NestMembers"),
            Self::Record(_) => write!(f, "Record"),
            Self::PermittedSubtypes(_) => write!(f, "PermittedSubtypes"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExceptionTableEntry {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: ConstantPoolIndex,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VerificationTypeInfo {
    Top,
    Integer,
    Float,
    Double,
    Long,
    Null,
    UninitializedThis,
    Object {
        /// An index into the constant pool for the class of the object
        class: u16,
    },
    Uninitialized {
        /// Offset into associated code array of a new instruction
        /// that created the object being stored here.
        offset: u16,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct LineNumberTableEntry {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalVariableTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: ConstantPoolIndex,
    pub descriptor_index: ConstantPoolIndex,
    pub index: ConstantPoolIndex,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalVariableTypeTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: ConstantPoolIndex,
    pub signature_index: ConstantPoolIndex,
    pub index: ConstantPoolIndex,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    pub type_index: ConstantPoolIndex,
    pub num_element_value_pairs: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElementValuePair {
    pub element_name_index: ConstantPoolIndex,
    pub value: ElementValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElementTag {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Boolean,
    String,
    Enum {
        type_name_index: ConstantPoolIndex,
        const_name_index: ConstantPoolIndex,
    },
    Class,
    AnnotationType,
    Array {
        num_values: u16,
        values: Vec<ElementValue>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElementValue {
    ConstValueIndex(ConstantPoolIndex),
    EnumConstValue {
        type_name_index: ConstantPoolIndex,
        const_name_index: ConstantPoolIndex,
    },
    ClassInfoIndex(ConstantPoolIndex),
    Annotation(Annotation),
    Array {
        num_values: u16,
        values: Vec<ElementValue>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterAnnotation {
    pub num_annotations: u16,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAnnotation {
    pub target_type: u8,
    pub target_info: TypeAnnotationTargetInfo,
    pub target_path: TypePath,
    pub type_index: ConstantPoolIndex,
    pub num_element_value_pairs: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAnnotationTargetInfo {
    pub target_info: TypeAnnotationTargetInfoType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeAnnotationTargetInfoType {
    TypeParameter {
        type_parameter_index: ConstantPoolIndex,
    },
    SuperType {
        super_type_index: ConstantPoolIndex,
    },
    TypeParameterBound {
        type_parameter_index: ConstantPoolIndex,
        bound_index: ConstantPoolIndex,
    },
    Empty,
    FormalParameter {
        formal_parameter_index: ConstantPoolIndex,
    },
    Throws {
        throws_type_index: ConstantPoolIndex,
    },
    LocalVar {
        table: Vec<LocalVarTargetTableEntry>,
    },
    Catch {
        exception_table_index: ConstantPoolIndex,
    },
    Offset {
        offset: u16,
    },
    TypeArgument {
        offset: u16,
        type_argument_index: ConstantPoolIndex,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalVarTargetTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub index: ConstantPoolIndex,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypePath {
    pub path_length: u8,
    pub path: Vec<TypePathEntry>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypePathEntry {
    pub type_path_kind: u8,
    pub type_argument_index: ConstantPoolIndex,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StackMapFrame {
    SameFrame {
        frame_type: u8,
    },
    SameLocals1StackItemFrame {
        frame_type: u8,
        stack: VerificationTypeInfo,
    },
    SameLocals1StackItemFrameExtended {
        frame_type: u8,
        offset_delta: u16,
        stack: VerificationTypeInfo,
    },
    ChopFrame {
        frame_type: u8,
        offset_delta: u16,
    },
    SameFrameExtended {
        frame_type: u8,
        offset_delta: u16,
    },
    AppendFrame {
        frame_type: u8,
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
    },
    FullFrame {
        frame_type: u8,
        offset_delta: u16,
        number_of_locals: u16,
        locals: Vec<VerificationTypeInfo>,
        number_of_stack_items: u16,
        stack: Vec<VerificationTypeInfo>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct InnerClass {
    pub inner_class_info_index: ConstantPoolIndex,
    pub outer_class_info_index: ConstantPoolIndex,
    pub inner_name_index: ConstantPoolIndex,
    pub inner_class_access_flags: InnerClassAccessFlags,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: ConstantPoolIndex,
    pub num_bootstrap_arguments: u16,
    pub bootstrap_arguments: Vec<ConstantPoolIndex>,
}

#[derive(Debug)]
pub struct RecordComponent {
    pub name_index: ConstantPoolIndex,
    pub descriptor_index: ConstantPoolIndex,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodParameter {
    pub name_index: ConstantPoolIndex,
    pub access_flags: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Requires {
    pub requires_index: ConstantPoolIndex,
    pub requires_flags: u16,
    pub requires_version_index: ConstantPoolIndex,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Exports {
    pub exports_index: ConstantPoolIndex,
    pub exports_flags: u16,
    pub exports_to_count: u16,
    pub exports_to_index: Vec<ConstantPoolIndex>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Opens {
    pub opens_index: ConstantPoolIndex,
    pub opens_flags: u16,
    pub opens_to_count: u16,
    pub opens_to_index: Vec<ConstantPoolIndex>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Provides {
    pub provides_index: ConstantPoolIndex,
    pub provides_with_count: u16,
    pub provides_with_index: Vec<ConstantPoolIndex>,
}

#[derive(Debug)]
pub struct ConstantValueInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub constantvalue_index: ConstantPoolIndex,
}

#[derive(Debug)]
pub struct CodeInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub max_stack: u16,
    pub max_locals: u16,
    pub code_length: u32,
    pub code: Vec<u8>,
    pub exception_table_length: u16,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct StackMapTableInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub number_of_entries: u16,
    pub entries: Vec<StackMapFrame>,
}

#[derive(Debug)]
pub struct ExceptionsInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub number_of_exceptions: u16,
    pub exception_index_table: Vec<ConstantPoolIndex>,
}

#[derive(Debug)]
pub struct InnerClassesInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub number_of_classes: u16,
    pub classes: Vec<InnerClass>,
}

#[derive(Debug)]
pub struct EnclosingMethodInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub class_index: ConstantPoolIndex,
    pub method_index: ConstantPoolIndex,
}

#[derive(Debug)]
pub struct SyntheticInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
}

#[derive(Debug)]
pub struct SignatureInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub signature_index: ConstantPoolIndex,
}

#[derive(Debug)]
pub struct SourceFileInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub sourcefile_index: ConstantPoolIndex,
}

#[derive(Debug)]
pub struct SourceDebugExtensionInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub debug_extension: Vec<u8>,
}

#[derive(Debug)]
pub struct LineNumberTableInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub line_number_table_length: u16,
    pub line_number_table: Vec<LineNumberTableEntry>,
}

#[derive(Debug)]
pub struct LocalVariableTableInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub local_variable_table_length: u16,
    pub local_variable_table: Vec<LocalVariableTableEntry>,
}

#[derive(Debug)]
pub struct LocalVariableTypeTableInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub local_variable_type_table_length: u16,
    pub local_variable_type_table: Vec<LocalVariableTypeTableEntry>,
}

#[derive(Debug)]
pub struct DeprecatedInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
}

#[derive(Debug)]
pub struct RuntimeVisibleAnnotationsInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub num_annotations: u16,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug)]
pub struct RuntimeInvisibleAnnotationsInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub num_annotations: u16,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug)]
pub struct RuntimeVisibleParameterAnnotationsInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub num_parameters: u8,
    pub parameter_annotations: Vec<ParameterAnnotation>,
}

#[derive(Debug)]
pub struct RuntimeInvisibleParameterAnnotationsInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub num_parameters: u8,
    pub parameter_annotations: Vec<ParameterAnnotation>,
}

#[derive(Debug)]
pub struct RuntimeVisibleTypeAnnotationsInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub num_annotations: u16,
    pub annotations: Vec<TypeAnnotation>,
}

#[derive(Debug)]
pub struct RuntimeInvisibleTypeAnnotationsInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub num_annotations: u16,
    pub annotations: Vec<TypeAnnotation>,
}

#[derive(Debug)]
pub struct AnnotationDefaultInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub default_value: ElementValue,
}

#[derive(Debug)]
pub struct BootstrapMethodsInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub num_bootstrap_methods: u16,
    pub bootstrap_methods: Vec<BootstrapMethod>,
}

#[derive(Debug)]
pub struct MethodParametersInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub parameters_count: u8,
    pub parameters: Vec<MethodParameter>,
}

#[derive(Debug)]
pub struct ModuleInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub module_name_index: ConstantPoolIndex,
    pub module_flags: u16,
    pub module_version_index: ConstantPoolIndex,
    pub requires_count: u16,
    pub requires: Vec<Requires>,
    pub exports_count: u16,
    pub exports: Vec<Exports>,
    pub opens_count: u16,
    pub opens: Vec<Opens>,
    pub uses_count: u16,
    pub uses_index: Vec<ConstantPoolIndex>,
    pub provides_count: u16,
    pub provides: Vec<Provides>,
}

#[derive(Debug)]
pub struct ModulePackagesInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub package_count: u16,
    pub package_index: Vec<ConstantPoolIndex>,
}

#[derive(Debug)]
pub struct ModuleMainClassInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub main_class_index: ConstantPoolIndex,
}

#[derive(Debug)]
pub struct NestHostInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub host_class_index: ConstantPoolIndex,
}

#[derive(Debug)]
pub struct NestMembersInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub number_of_classes: u16,
    pub classes: Vec<ConstantPoolIndex>,
}

#[derive(Debug)]
pub struct RecordInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub component_count: u16,
    pub components: Vec<Attribute>,
}

#[derive(Debug)]
pub struct PermittedSubtypesInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub number_of_classes: u16,
    pub classes: Vec<String>,
}
