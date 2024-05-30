use std::collections::HashMap;

use super::pool::ConstantPool;
use super::reader::BufferedReader;
use super::BytecodeError;
use crate::bytecode::flags::InnerClassAccessFlags;
use crate::bytecode::pool::ConstantPoolIndex;

pub trait Attribute {
    /// Returns the name of the attribute.
    ///
    /// WARNING: This should only be used for debugging purposes because there is no guarantee that
    /// no other `AttributeInfo` implementation will return the same name.
    fn name(&self) -> &'static str {
        "[unknown attribute]"
    }
}

pub trait AttributeFactory: std::fmt::Debug {
    fn make(
        &self,
        reader: &mut BufferedReader,
        pool: &mut ConstantPool,
        container: &Container,
    ) -> Result<Box<dyn AnyAttribute>, BytecodeError>;
}

#[derive(Debug)]
pub struct Container {
    inner: HashMap<&'static str, Box<dyn AttributeFactory>>,
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl Container {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &'static str, factory: impl AttributeFactory + 'static) {
        self.inner.insert(name, Box::new(factory));
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Box<dyn AttributeFactory>> {
        self.inner.get(name)
    }
}

impl Attribute for Box<dyn Attribute> {
    fn name(&self) -> &'static str {
        self.as_ref().name()
    }
}

pub trait AnyAttribute: std::fmt::Debug {
    fn as_any_ref(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn name_any(&self) -> &'static str;
}

impl<T: std::fmt::Debug + Attribute + 'static> AnyAttribute for T {
    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn name_any(&self) -> &'static str {
        self.name()
    }
}

impl Attribute for ConstantValueInfo {
    fn name(&self) -> &'static str {
        "ConstantValue"
    }
}

impl Attribute for CodeInfo {
    fn name(&self) -> &'static str {
        "Code"
    }
}

impl Attribute for StackMapTableInfo {
    fn name(&self) -> &'static str {
        "StackMapTable"
    }
}

impl Attribute for ExceptionsInfo {
    fn name(&self) -> &'static str {
        "Exceptions"
    }
}

impl Attribute for InnerClassesInfo {
    fn name(&self) -> &'static str {
        "InnerClasses"
    }
}

impl Attribute for EnclosingMethodInfo {
    fn name(&self) -> &'static str {
        "EnclosingMethod"
    }
}

impl Attribute for SyntheticInfo {
    fn name(&self) -> &'static str {
        "Synthetic"
    }
}

impl Attribute for SignatureInfo {
    fn name(&self) -> &'static str {
        "Signature"
    }
}

impl Attribute for SourceFileInfo {
    fn name(&self) -> &'static str {
        "SourceFile"
    }
}

impl Attribute for SourceDebugExtensionInfo {
    fn name(&self) -> &'static str {
        "SourceDebugExtension"
    }
}

impl Attribute for LineNumberTableInfo {
    fn name(&self) -> &'static str {
        "LineNumberTable"
    }
}

impl Attribute for LocalVariableTableInfo {
    fn name(&self) -> &'static str {
        "LocalVariableTable"
    }
}

impl Attribute for LocalVariableTypeTableInfo {
    fn name(&self) -> &'static str {
        "LocalVariableTypeTable"
    }
}

impl Attribute for DeprecatedInfo {
    fn name(&self) -> &'static str {
        "Deprecated"
    }
}

impl Attribute for RuntimeVisibleAnnotationsInfo {
    fn name(&self) -> &'static str {
        "RuntimeVisibleAnnotations"
    }
}

impl Attribute for RuntimeInvisibleAnnotationsInfo {
    fn name(&self) -> &'static str {
        "RuntimeInvisibleAnnotations"
    }
}

impl Attribute for RuntimeVisibleParameterAnnotationsInfo {
    fn name(&self) -> &'static str {
        "RuntimeVisibleParameterAnnotations"
    }
}

impl Attribute for RuntimeInvisibleParameterAnnotationsInfo {
    fn name(&self) -> &'static str {
        "RuntimeInvisibleParameterAnnotations"
    }
}

impl Attribute for RuntimeVisibleTypeAnnotationsInfo {
    fn name(&self) -> &'static str {
        "RuntimeVisibleTypeAnnotations"
    }
}

impl Attribute for RuntimeInvisibleTypeAnnotationsInfo {
    fn name(&self) -> &'static str {
        "RuntimeInvisibleTypeAnnotations"
    }
}

impl Attribute for AnnotationDefaultInfo {
    fn name(&self) -> &'static str {
        "AnnotationDefault"
    }
}

impl Attribute for BootstrapMethodsInfo {
    fn name(&self) -> &'static str {
        "BootstrapMethods"
    }
}

impl Attribute for MethodParametersInfo {
    fn name(&self) -> &'static str {
        "MethodParameters"
    }
}

impl Attribute for ModuleInfo {
    fn name(&self) -> &'static str {
        "Module"
    }
}

impl Attribute for ModulePackagesInfo {
    fn name(&self) -> &'static str {
        "ModulePackages"
    }
}

impl Attribute for ModuleMainClassInfo {
    fn name(&self) -> &'static str {
        "ModuleMainClass"
    }
}

impl Attribute for NestHostInfo {
    fn name(&self) -> &'static str {
        "NestHost"
    }
}

impl Attribute for NestMembersInfo {
    fn name(&self) -> &'static str {
        "NestMembers"
    }
}

impl Attribute for RecordInfo {
    fn name(&self) -> &'static str {
        "Record"
    }
}

impl Attribute for PermittedSubtypesInfo {
    fn name(&self) -> &'static str {
        "PermittedSubtypes"
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
        class: ConstantPoolIndex,
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

pub fn element_value_string(
    value: &ElementValue,
    pool: &ConstantPool,
) -> Result<String, BytecodeError> {
    match value {
        ElementValue::ConstValueIndex(idx) => match pool.text_of(idx.clone()) {
            Some(str) => Ok(str.to_string()),
            None => Err(BytecodeError::ConstantPoolEntryNotFound),
        },
        ElementValue::EnumConstValue {
            type_name_index: _,
            const_name_index: _,
        } => {
            todo!()
        }
        ElementValue::ClassInfoIndex(idx) => match pool.text_of(idx.clone()) {
            Some(str) => Ok(str.to_string()),
            None => Err(BytecodeError::ConstantPoolEntryNotFound),
        },
        ElementValue::Annotation(_annotation) => {
            todo!()
        }
        ElementValue::Array { values: _, .. } => {
            todo!()
        }
    }
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
    pub attributes: Vec<Box<dyn AnyAttribute>>,
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
    pub attributes: Vec<Box<dyn AnyAttribute>>,
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
    pub components: Vec<Box<dyn AnyAttribute>>,
}

#[derive(Debug)]
pub struct PermittedSubtypesInfo {
    pub attribute_name_index: ConstantPoolIndex,
    pub attribute_length: u32,
    pub number_of_classes: u16,
    pub classes: Vec<String>,
}
