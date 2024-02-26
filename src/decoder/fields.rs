use crate::{
    decoder::{buffer::Buffer, error::DecodingError, Decodable},
    types::{
        attributes::Attribute, constants::ConstantPool, elements::Field, flags::FieldAccessFlags,
    },
};

impl Decodable<Field> for Field {
    fn decode(buffer: &mut Buffer, constant_pool: &ConstantPool) -> Result<Field, DecodingError> {
        let access_flags = buffer.take::<u16>().unwrap();
        let access_flags = FieldAccessFlags::from_bits(access_flags).unwrap();

        let name_index = buffer.take::<u16>().unwrap();
        let name = constant_pool.text_of_value(name_index as usize).unwrap();

        let descriptor_index = buffer.take::<u16>().unwrap();
        let descriptor = constant_pool
            .text_of_value(descriptor_index as usize)
            .unwrap();

        let attributes_count = buffer.take::<u16>().unwrap();
        let attributes = (0..attributes_count)
            .map(|_| Attribute::decode(buffer, constant_pool).unwrap())
            .collect();

        Ok(Field {
            name,
            descriptor,
            access_flags,
            attributes,
        })
    }
}
