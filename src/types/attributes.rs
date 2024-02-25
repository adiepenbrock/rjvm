use crate::types::flags::InnerClassAccessFlags;

#[derive(Debug, Clone, PartialEq)]
pub struct ExceptionTableEntry {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
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
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalVariableTypeTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    pub type_index: u16,
    pub num_element_value_pairs: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElementValuePair {
    pub element_name_index: u16,
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
        type_name_index: u16,
        const_name_index: u16,
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
    ConstValueIndex(u16),
    EnumConstValue {
        type_name_index: u16,
        const_name_index: u16,
    },
    ClassInfoIndex(u16),
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
    pub type_index: u16,
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
        type_parameter_index: u8,
    },
    SuperType {
        super_type_index: u16,
    },
    TypeParameterBound {
        type_parameter_index: u8,
        bound_index: u8,
    },
    Empty,
    FormalParameter {
        formal_parameter_index: u8,
    },
    Throws {
        throws_type_index: u16,
    },
    LocalVar {
        table: Vec<LocalVarTargetTableEntry>,
    },
    Catch {
        exception_table_index: u16,
    },
    Offset {
        offset: u16,
    },
    TypeArgument {
        offset: u16,
        type_argument_index: u8,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalVarTargetTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub index: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypePath {
    pub path_length: u8,
    pub path: Vec<TypePathEntry>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypePathEntry {
    pub type_path_kind: u8,
    pub type_argument_index: u8,
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
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: InnerClassAccessFlags,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: u16,
    pub num_bootstrap_arguments: u16,
    pub bootstrap_arguments: Vec<u16>,
}

#[derive(Debug)]
pub struct RecordComponent {
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodParameter {
    pub name_index: u16,
    pub access_flags: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Requires {
    pub requires_index: u16,
    pub requires_flags: u16,
    pub requires_version_index: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Exports {
    pub exports_index: u16,
    pub exports_flags: u16,
    pub exports_to_count: u16,
    pub exports_to_index: Vec<u16>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Opens {
    pub opens_index: u16,
    pub opens_flags: u16,
    pub opens_to_count: u16,
    pub opens_to_index: Vec<u16>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Provides {
    pub provides_index: u16,
    pub provides_with_count: u16,
    pub provides_with_index: Vec<u16>,
}

#[derive(Debug)]
pub struct ConstantValueInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub constantvalue_index: u16,
}

#[derive(Debug)]
pub struct CodeInfo {
    pub attribute_name_index: u16,
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
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub number_of_entries: u16,
    pub entries: Vec<StackMapFrame>,
}

#[derive(Debug)]
pub struct ExceptionsInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub number_of_exceptions: u16,
    pub exception_index_table: Vec<u16>,
}

#[derive(Debug)]
pub struct InnerClassesInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub number_of_classes: u16,
    pub classes: Vec<InnerClass>,
}

#[derive(Debug)]
pub struct EnclosingMethodInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub class_index: u16,
    pub method_index: u16,
}

#[derive(Debug)]
pub struct SyntheticInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
}

#[derive(Debug)]
pub struct SignatureInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub signature_index: u16,
}

#[derive(Debug)]
pub struct SourceFileInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub sourcefile_index: u16,
}

#[derive(Debug)]
pub struct SourceDebugExtensionInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub debug_extension: Vec<u8>,
}

#[derive(Debug)]
pub struct LineNumberTableInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub line_number_table_length: u16,
    pub line_number_table: Vec<LineNumberTableEntry>,
}

#[derive(Debug)]
pub struct LocalVariableTableInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub local_variable_table_length: u16,
    pub local_variable_table: Vec<LocalVariableTableEntry>,
}

#[derive(Debug)]
pub struct LocalVariableTypeTableInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub local_variable_type_table_length: u16,
    pub local_variable_type_table: Vec<LocalVariableTypeTableEntry>,
}

#[derive(Debug)]
pub struct DeprecatedInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
}

#[derive(Debug)]
pub struct RuntimeVisibleAnnotationsInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub num_annotations: u16,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug)]
pub struct RuntimeInvisibleAnnotationsInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub num_annotations: u16,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug)]
pub struct RuntimeVisibleParameterAnnotationsInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub num_parameters: u8,
    pub parameter_annotations: Vec<ParameterAnnotation>,
}

#[derive(Debug)]
pub struct RuntimeInvisibleParameterAnnotationsInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub num_parameters: u8,
    pub parameter_annotations: Vec<ParameterAnnotation>,
}

#[derive(Debug)]
pub struct RuntimeVisibleTypeAnnotationsInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub num_annotations: u16,
    pub annotations: Vec<TypeAnnotation>,
}

#[derive(Debug)]
pub struct RuntimeInvisibleTypeAnnotationsInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub num_annotations: u16,
    pub annotations: Vec<TypeAnnotation>,
}

#[derive(Debug)]
pub struct AnnotationDefaultInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub default_value: ElementValue,
}

#[derive(Debug)]
pub struct BootstrapMethodsInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub num_bootstrap_methods: u16,
    pub bootstrap_methods: Vec<BootstrapMethod>,
}

#[derive(Debug)]
pub struct MethodParametersInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub parameters_count: u8,
    pub parameters: Vec<MethodParameter>,
}

#[derive(Debug)]
pub struct ModuleInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub module_name_index: u16,
    pub module_flags: u16,
    pub module_version_index: u16,
    pub requires_count: u16,
    pub requires: Vec<Requires>,
    pub exports_count: u16,
    pub exports: Vec<Exports>,
    pub opens_count: u16,
    pub opens: Vec<Opens>,
    pub uses_count: u16,
    pub uses_index: Vec<u16>,
    pub provides_count: u16,
    pub provides: Vec<Provides>,
}

#[derive(Debug)]
pub struct ModulePackagesInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub package_count: u16,
    pub package_index: Vec<u16>,
}

#[derive(Debug)]
pub struct ModuleMainClassInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub main_class_index: u16,
}

#[derive(Debug)]
pub struct NestHostInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub host_class_index: u16,
}

#[derive(Debug)]
pub struct NestMembersInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub number_of_classes: u16,
    pub classes: Vec<u16>,
}

#[derive(Debug)]
pub struct RecordInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub component_count: u16,
    pub components: Vec<Attribute>,
}

#[derive(Debug)]
pub struct PermittedSubtypesInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub number_of_classes: u16,
    pub classes: Vec<u16>,
}

#[derive(Debug)]
pub struct Attribute {
    pub info: Box<dyn std::any::Any>,
}
