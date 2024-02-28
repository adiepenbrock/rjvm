use crate::{
    decoder::{buffer::BufferedReader, error::DecodingError, Decodable},
    types::{
        attributes::Attribute,
        constants::{ConstantPool, ConstantPoolEntry},
        elements::{ClassFile, ClassFileVersion, Field, Interface, Method},
        flags::ClassAccessFlags,
    },
};

impl ClassFile {
    pub fn decode(
        buffer: &mut BufferedReader,
        constant_pool: &mut ConstantPool,
    ) -> Result<ClassFile, DecodingError> {
        let magic_number = buffer.take::<u32>().unwrap();
        if magic_number != 0xCAFEBABE {
            return Err(DecodingError::InvalidClassFile);
        }

        let minor_version = buffer.take::<u16>().unwrap();
        let major_version = buffer.take::<u16>().unwrap();
        let version = ClassFileVersion {
            minor: minor_version,
            major: major_version,
        };

        let constant_pool_count = buffer.take::<u16>().unwrap();
        (0..constant_pool_count - 1).for_each(|_| {
            let entry = ConstantPoolEntry::decode(buffer, constant_pool).unwrap();
            constant_pool.add(entry);
        });

        let access_flags = buffer.take::<u16>().unwrap();
        let access_flags = match ClassAccessFlags::from_bits(access_flags) {
            Some(flags) => flags,
            None => return Err(DecodingError::InvalidClassFile),
        };

        let this_class = buffer.take::<u16>().unwrap();
        let super_class = buffer.take::<u16>().unwrap();

        let interfaces_count = buffer.take::<u16>().unwrap();
        let interfaces = (0..interfaces_count)
            .map(|_| Interface::decode(buffer, constant_pool).unwrap())
            .collect();

        let fields_count = buffer.take::<u16>().unwrap();
        let fields = (0..fields_count)
            .map(|_| Field::decode(buffer, constant_pool).unwrap())
            .collect();

        let methods_count = buffer.take::<u16>().unwrap();
        let methods = (0..methods_count)
            .map(|_| Method::decode(buffer, constant_pool).unwrap())
            .collect();

        let attributes_count = buffer.take::<u16>().unwrap();
        let attributes = (0..attributes_count)
            .map(|_| Attribute::decode(buffer, constant_pool).unwrap())
            .collect();

        Ok(ClassFile {
            magic_number,
            version,
            constant_pool_count,
            constant_pool: constant_pool.clone(),
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
}

impl Decodable<Interface> for Interface {
    fn decode(
        buffer: &mut BufferedReader,
        _constant_pool: &ConstantPool,
    ) -> Result<Interface, DecodingError> {
        let name_index = buffer.take::<u16>().unwrap();

        Ok(Interface { name_index })
    }
}
