use crate::bytecode::{
    flags::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags},
    pool::{ConstantPool, ConstantPoolIndex},
    reader::{attributes::read_attribute, constants::read_constant_pool_entry, BufferedReader},
    BaseType, BytecodeError, ClassFile, ClassFileVersion, Descriptor, DescriptorKind, Field,
    FieldType, Interface, Method,
};

pub fn read_classfile(
    reader: &mut BufferedReader,
    cp: &mut ConstantPool,
) -> Result<ClassFile, BytecodeError> {
    let magic_number = reader.take::<u32>()?;
    if magic_number != 0xCAFEBABE {
        return Err(BytecodeError::InvalidClassFile);
    }

    let minor_version = reader.take::<u16>()?;
    let major_version = reader.take::<u16>()?;
    let version = ClassFileVersion {
        minor: minor_version,
        major: major_version,
    };

    let constant_pool_count = reader.take::<u16>()?;
    for idx in 1..=constant_pool_count - 1 {
        let entry = read_constant_pool_entry(reader, cp)?;
        cp.insert(idx.into(), entry)?;
    }

    let access_flags = reader.take::<u16>()?;
    let access_flags = match ClassAccessFlags::from_bits(access_flags) {
        Some(flags) => flags,
        None => return Err(BytecodeError::InvalidClassFile),
    };

    let this_class = reader.take::<u16>()?;
    let this_class = ConstantPoolIndex::new(this_class);
    let super_class = reader.take::<u16>()?;
    let super_class = ConstantPoolIndex::new(super_class);

    let interfaces_count = reader.take::<u16>()?;
    let mut interfaces = Vec::with_capacity(interfaces_count as usize);
    for _ in 0..interfaces_count {
        let interface = read_interface(reader, cp)?;
        interfaces.push(interface);
    }

    let fields_count = reader.take::<u16>()?;
    let mut fields = Vec::with_capacity(fields_count as usize);
    for _ in 0..fields_count {
        let field = read_field(reader, cp)?;
        fields.push(field);
    }

    let methods_count = reader.take::<u16>()?;
    let mut methods = Vec::with_capacity(methods_count as usize);
    for _ in 0..methods_count {
        let method = read_method(reader, cp)?;
        methods.push(method);
    }

    let attributes_count = reader.take::<u16>()?;
    let mut attributes = Vec::with_capacity(attributes_count as usize);
    for _ in 0..attributes_count {
        let attribute = read_attribute(reader, cp)?;
        attributes.push(attribute);
    }

    Ok(ClassFile {
        magic_number,
        version,
        constant_pool_count,
        constant_pool: cp.clone(),
        access_flags,
        this_class,
        super_class,
        interfaces_count,
        interfaces,
        fields_count,
        fields,
        methods_count,
        methods,
        attributes_count,
        attributes,
    })
}

pub fn read_interface(
    reader: &mut BufferedReader,
    _cp: &mut ConstantPool,
) -> Result<Interface, BytecodeError> {
    let name_index = reader.take::<u16>()?;

    Ok(Interface {
        name_index: ConstantPoolIndex::new(name_index),
    })
}

pub fn read_field(
    reader: &mut BufferedReader,
    cp: &mut ConstantPool,
) -> Result<Field, BytecodeError> {
    let access_flags = reader.take::<u16>()?;
    let Some(access_flags) = FieldAccessFlags::from_bits(access_flags) else {
        return Err(BytecodeError::InvalidClassFile);
    };

    let name_index = reader.take::<u16>()?;
    let Some(name) = cp.text_of(name_index.into()) else {
        return Err(BytecodeError::InvalidClassFile);
    };

    let descriptor_index = reader.take::<u16>()?;
    let Some(descriptor) = cp.text_of(descriptor_index.into()) else {
        return Err(BytecodeError::InvalidClassFile);
    };
    let descriptor = Descriptor::parse_from_field(descriptor).unwrap_or(Descriptor {
        kind: DescriptorKind::Type,
        ty: FieldType::Base(BaseType::Void),
    });

    let attributes_count = reader.take::<u16>()?;
    let mut attributes = Vec::with_capacity(attributes_count as usize);
    for _ in 0..attributes_count {
        let attribute = read_attribute(reader, cp)?;
        attributes.push(attribute);
    }

    Ok(Field {
        name,
        descriptor,
        access_flags,
        attributes,
    })
}

pub fn read_method(
    reader: &mut BufferedReader,
    cp: &mut ConstantPool,
) -> Result<Method, BytecodeError> {
    let access_flags = reader.take::<u16>()?;
    let Some(access_flags) = MethodAccessFlags::from_bits(access_flags) else {
        return Err(BytecodeError::InvalidClassFile);
    };

    let name_index = reader.take::<u16>()?;
    let Some(name) = cp.text_of(name_index.into()) else {
        return Err(BytecodeError::InvalidClassFile);
    };

    let descriptor_index = reader.take::<u16>()?;
    let Some(descriptor) = cp.text_of(descriptor_index.into()) else {
        return Err(BytecodeError::InvalidClassFile);
    };
    let descriptor = Descriptor::parse_from_method(descriptor);

    let attributes_count = reader.take::<u16>()?;
    let mut attributes = Vec::with_capacity(attributes_count as usize);
    for _ in 0..attributes_count {
        let attribute = read_attribute(reader, cp)?;
        attributes.push(attribute);
    }

    Ok(Method {
        access_flags,
        name,
        descriptor,
        attributes,
    })
}
