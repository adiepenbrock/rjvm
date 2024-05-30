use std::collections::HashMap;

use bytecode::attributes::{
    element_value_string, MethodParametersInfo, RuntimeInvisibleAnnotationsInfo,
    RuntimeVisibleAnnotationsInfo,
};
use bytecode::DescriptorKind;
use error::Error;

pub mod bytecode;
pub mod decoder;
pub mod error;
pub mod types;

// -----------------------------------------------------------------------------
//  - common stuff -
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Annotation {
    /// The name of the annotation.
    pub name: String,
    // TODO: change the value type to support other values than just strings
    pub field: HashMap<String, String>,
}

impl Annotation {
    pub fn from_bytecode(
        bytecode: &bytecode::attributes::Annotation,
        pool: &bytecode::pool::ConstantPool,
    ) -> Result<Annotation, Error> {
        let name = pool.text_of(bytecode.type_index).unwrap();
        let field = bytecode
            .element_value_pairs
            .iter()
            .map(|pair| {
                let key = pool.text_of(pair.element_name_index).unwrap();
                let value = match element_value_string(&pair.value, pool) {
                    Ok(value) => value,
                    Err(_) => unreachable!(),
                };
                (key, value)
            })
            .collect();
        Ok(Annotation { name, field })
    }
}

#[derive(Debug, PartialEq)]
pub struct TypeRef {
    pub name: String,
}

#[derive(Debug)]
pub struct Package {
    /// The name of the package.
    pub name: String,
}

// -----------------------------------------------------------------------------
//  - Method-specific stuff -
// -----------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone)]
#[repr(u16)]
pub enum MethodModifier {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,

    Static = 0x0008,
    Final = 0x0010,

    Synchronized = 0x0020,

    Varargs = 0x0080,
    Bridge = 0x0040,
    Native = 0x0100,
    Abstract = 0x0400,
    Strict = 0x0800,

    Synthetic = 0x1000,
    Enum = 0x4000,
}

impl MethodModifier {
    /// Returns the bytecode representation of the method modifier.
    pub fn bytecode(&self) -> u16 {
        // as stated in the docs, this should return the discriminant value of
        // the enum, which is the bytecode representation of the method modifier
        // (e.g. `MethodModifier::Public` should return `0x0001`).
        // see: https://doc.rust-lang.org/reference/items/enumerations.html#pointer-casting
        // TODO: check if there is a better (safe) way to do this...
        unsafe { *(self as *const Self as *const u16) }
    }
}

#[derive(Debug)]
pub struct Parameter {
    /// The name of the parameter.
    pub name: Option<String>,
    /// The position of the parameter within a method parameter's list.
    pub position: usize,
    /// The type of the parameter.
    pub ty: TypeRef,
}

#[derive(Debug)]
pub struct MethodBody {}

#[derive(Debug)]
pub struct Method {
    /// The name of the method.
    pub name: String,
    /// A list of all parameters of the method.
    pub parameters: Vec<Parameter>,
    /// The return type of the method.
    pub ty: Option<TypeRef>,
    /// A list of all annotations of the method.
    pub annotations: Vec<Annotation>,
    /// A list of all modifiers of the method.
    pub modifiers: Vec<MethodModifier>,
    /// The body of the method.
    pub body: Option<MethodBody>,
}

impl Method {
    pub fn from_bytecode(
        bytecode: &bytecode::Method,
        pool: &bytecode::pool::ConstantPool,
        // TODO: use a proper error type
    ) -> Result<Self, Error> {
        // -----------------------------------------------------------------------------
        //  - Transform parameters from bytecode representation to IR representation -
        // -----------------------------------------------------------------------------
        let attr_params: Vec<String> = bytecode
            .attributes
            .get("MethodParameters")
            .and_then(|attr| attr.as_any_ref().downcast_ref::<MethodParametersInfo>())
            .map(|params| {
                params
                    .parameters
                    .iter()
                    .map(|param| pool.text_of(param.name_index).unwrap())
                    .collect()
            })
            .unwrap_or_default();

        let desc_params = bytecode
            .descriptor
            .iter()
            .filter(|desc| desc.kind == DescriptorKind::Parameter)
            .collect::<Vec<&bytecode::Descriptor>>();

        let parameters: Vec<Parameter> = desc_params
            .iter()
            .enumerate()
            .map(|(i, desc)| {
                let name: Option<String> = attr_params.get(i).cloned();
                let ty = desc.ty.to_string();
                Parameter {
                    name,
                    ty: TypeRef { name: ty },
                    position: i,
                }
            })
            .collect();

        // -----------------------------------------------------------------------------
        //  - Extract the method's return type from the bytecode representation -
        // -----------------------------------------------------------------------------
        let ret_ty: Option<TypeRef> = bytecode.descriptor.iter().find_map(|desc| {
            if desc.kind == DescriptorKind::Return {
                Some(TypeRef {
                    name: desc.ty.to_string(),
                })
            } else {
                None
            }
        });

        // -----------------------------------------------------------------------------
        //  - Transform annotations from bytecode representation to IR representation -
        // -----------------------------------------------------------------------------
        // TODO: we should add support to read only annotations that are available in the
        // desired JVM specification ...
        let mut annotations: Vec<Annotation> = vec![];
        if let Some(attr) =
            bytecode.get_attribute::<RuntimeVisibleAnnotationsInfo>("RuntimeVisibleAnnotations")
        {
            let items: Vec<Annotation> = attr
                .annotations
                .iter()
                .map(|item| Annotation::from_bytecode(item, pool).unwrap())
                .collect();
            annotations.extend(items);
        }

        if let Some(attr) =
            bytecode.get_attribute::<RuntimeInvisibleAnnotationsInfo>("RuntimeInvisibleAnnotations")
        {
            let items: Vec<Annotation> = attr
                .annotations
                .iter()
                .map(|item| Annotation::from_bytecode(item, pool).unwrap())
                .collect();
            annotations.extend(items);
        }

        let method = Method {
            name: bytecode.name.clone(),
            parameters,
            ty: ret_ty,
            annotations,
            modifiers: vec![],
            body: None,
        };

        Ok(method)
    }
}

impl Method {
    /// Returns the name of the method.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Tries to find a parameter by the given `name`. Returns `None` if no parameter with the given
    /// `name` can be found.
    pub fn parameter_by_name(&self, name: &str) -> Option<&Parameter> {
        self.parameters
            .iter()
            .find(|p| p.name.as_deref() == Some(name))
    }

    /// Tries to find a parameter by the given `index`. Returns `None` if no parameter with the
    /// given `index` can be found.
    pub fn parameter_by_index(&self, index: usize) -> Option<&Parameter> {
        self.parameters.get(index)
    }

    /// Checks whether the method has a parameter with the given `name`.
    pub fn has_parameter_by_name(&self, name: &str) -> bool {
        self.parameters
            .iter()
            .any(|p| p.name.as_deref() == Some(name))
    }

    /// Checks whether the method has a parameter with the given `index`.
    pub fn has_parameter_by_index(&self, index: usize) -> bool {
        self.parameters.get(index).is_some()
    }

    /// Tries to find an annotation by the given `name`. Returns `None` if no annotation with the
    /// given `name` can be found.
    pub fn get_annotation_by_name(&self, name: &str) -> Option<&Annotation> {
        self.annotations.iter().find(|a| a.name == name)
    }

    /// Tries to find an annotation by the given `index`. Returns `None` if no annotation with the
    /// given `index` can be found.
    pub fn get_annotation_by_index(&self, index: usize) -> Option<&Annotation> {
        self.annotations.get(index)
    }

    /// Checks whether the method has an annotation with the given `name`.
    pub fn has_annotation_by_name(&self, name: &str) -> bool {
        self.annotations.iter().any(|a| a.name == name)
    }

    /// Checks whether the method has an annotation with the given `index`.
    pub fn has_annotation_by_index(&self, index: usize) -> bool {
        self.annotations.get(index).is_some()
    }

    /// Checks whether the method has the given `modifier`.
    pub fn has_modifier(&self, modifier: MethodModifier) -> bool {
        self.modifiers.contains(&modifier)
    }

    // -----------------------------------------------------------------------------
    //  - convenience methods -
    // -----------------------------------------------------------------------------

    /// Convenience method to check whether the method is protected.
    pub fn is_protected(&self) -> bool {
        self.modifiers.contains(&MethodModifier::Protected)
    }

    /// Convenience method to check whether the method is public.
    pub fn is_public(&self) -> bool {
        self.modifiers.contains(&MethodModifier::Public)
    }

    /// Convenience method to check whether the method is private.
    pub fn is_private(&self) -> bool {
        self.modifiers.contains(&MethodModifier::Private)
    }

    /// Convenience method to check whether the method is static.
    pub fn is_static(&self) -> bool {
        self.modifiers.contains(&MethodModifier::Static)
    }

    /// Convenience method to check whether the method is final.
    pub fn is_final(&self) -> bool {
        self.modifiers.contains(&MethodModifier::Final)
    }
}

// -----------------------------------------------------------------------------
//  - Field-specific stuff -
// -----------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone)]
#[repr(u16)]
pub enum FieldModifier {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,

    Static = 0x0008,
    Final = 0x0010,

    Volatile = 0x0040,
    Transient = 0x0080,
    Synthetic = 0x1000,

    Enum = 0x4000,
}

impl FieldModifier {
    /// Returns the bytecode representation of the field modifier.
    pub fn bytecode(&self) -> u16 {
        // as stated in the docs, this should return the discriminant value of
        // the enum, which is the bytecode representation of the field modifier
        // (e.g. `FieldModifier::Public` should return `0x0001`).
        // see: https://doc.rust-lang.org/reference/items/enumerations.html#pointer-casting
        // TODO: check if there is a better (safe) way to do this...
        unsafe { *(self as *const Self as *const u16) }
    }
}

#[derive(Debug)]
pub struct Field {
    /// The name of the field.
    pub name: String,
    /// The type of the field.
    pub ty: TypeRef,
    /// A list of all modifiers of the field.
    pub modifiers: Vec<FieldModifier>,
    /// A list of all annotations of the field.
    pub annotations: Vec<Annotation>,
}

impl Field {
    /// Returns the name of the field.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Checks whether the field has the given `modifier`.
    pub fn has_modifier(&self, modifier: FieldModifier) -> bool {
        self.modifiers.contains(&modifier)
    }

    /// Tries to find an annotation by the given `name`. Returns `None` if no annotation with the
    /// given `name` can be found.
    pub fn get_annotation_by_name(&self, name: &str) -> Option<&Annotation> {
        self.annotations.iter().find(|a| a.name == name)
    }

    /// Tries to find an annotation by the given `index`. Returns `None` if no annotation with the
    /// given `index` can be found.
    pub fn get_annotation_by_index(&self, index: usize) -> Option<&Annotation> {
        self.annotations.get(index)
    }

    // -----------------------------------------------------------------------------
    //  - convenience methods -
    // -----------------------------------------------------------------------------

    /// Convenience method to check whether the field is public.
    pub fn is_public(&self) -> bool {
        self.modifiers.contains(&FieldModifier::Public)
    }

    /// Convenience method to check whether the field is private.
    pub fn is_private(&self) -> bool {
        self.modifiers.contains(&FieldModifier::Private)
    }

    /// Convenience method to check whether the field is protected.
    pub fn is_protected(&self) -> bool {
        self.modifiers.contains(&FieldModifier::Protected)
    }
}

// -----------------------------------------------------------------------------
//  - Class-specific stuff -
// -----------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone)]
#[repr(u16)]
pub enum ClassModifier {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,

    Static = 0x0008,
    Final = 0x0010,

    Super = 0x0020,
    Interface = 0x0200,
    Abstract = 0x0400,
    Synthetic = 0x1000,
    Annotation = 0x2000,
    Enum = 0x4000,

    Module = 0x8000,
}

impl ClassModifier {
    /// Returns the bytecode representation of the class modifier.
    pub fn bytecode(&self) -> u16 {
        // as stated in the docs, this should return the discriminant value of
        // the enum, which is the bytecode representation of the class modifier
        // (e.g. `ClassModifier::Public` should return `0x0001`).
        // see: https://doc.rust-lang.org/reference/items/enumerations.html#pointer-casting
        // TODO: check if there is a better (safe) way to do this...
        unsafe { *(self as *const Self as *const u16) }
    }
}

#[derive(Debug)]
pub struct Class {
    /// The name of the package the class belongs to.
    pub package: Option<String>,
    /// The name of the class.
    pub name: String,
    /// The name of the superclass of the class.
    pub superclass: Option<String>,
    /// A list of all modifiers of the class.
    pub modifiers: Vec<ClassModifier>,
    /// A list of all annotations of the class.
    pub annotations: Vec<Annotation>,
    /// A list of all methods of the class.
    pub methods: Vec<Method>,
    /// A list of all fields of the class.
    pub fields: Vec<Field>,
}

impl Class {
    /// Returns the simple name of the class.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the fully qualified name of the class.
    pub fn fullname(&self) -> String {
        if let Some(package) = &self.package {
            format!("{}.{}", package, self.name)
        } else {
            self.name.to_string()
        }
    }

    /// Checks whether the class has the given `modifier`.
    pub fn has_modifier(&self, modifier: ClassModifier) -> bool {
        self.modifiers.contains(&modifier)
    }

    /// Tries to find an annotation by the given `name`. Returns `None` if no annotation with the
    /// given `name` can be found.
    pub fn get_annotation_by_name(&self, name: &str) -> Option<&Annotation> {
        self.annotations.iter().find(|a| a.name == name)
    }

    /// Tries to find an annotation by the given `index`. Returns `None` if no annotation with the
    /// given `index` can be found.
    pub fn get_annotation_by_index(&self, index: usize) -> Option<&Annotation> {
        self.annotations.get(index)
    }

    /// Checks whether the class has an annotation with the given `name`.
    pub fn has_annotation_by_name(&self, name: &str) -> bool {
        self.annotations.iter().any(|a| a.name == name)
    }

    /// Checks whether the class has an annotation with the given `index`.
    pub fn has_annotation_by_index(&self, index: usize) -> bool {
        self.annotations.get(index).is_some()
    }

    /// Tries to find a field by the given `name`. Returns `None` if no field with the given
    /// `name` can be found.
    pub fn get_field_by_name(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.name == name)
    }

    /// Tries to find a field by the given `index`. Returns `None` if no field with the given
    /// `index` can be found.
    pub fn get_field_by_index(&self, index: usize) -> Option<&Field> {
        self.fields.get(index)
    }

    /// Tries to find a method by the given `name`. Returns `None` if no method with the given
    /// `name` can be found.
    pub fn get_method_by_name(&self, name: &str) -> Option<&Method> {
        self.methods.iter().find(|m| m.name == name)
    }

    /// Tries to find a method by the given `index`. Returns `None` if no method with the given
    /// `index` can be found.
    pub fn get_method_by_index(&self, index: usize) -> Option<&Method> {
        self.methods.get(index)
    }
}
