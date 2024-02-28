use crate::{decoder::error::DecodingError, types::constants::ConstantPool};

pub mod attributes;
pub mod class_file;
pub mod constants;
pub mod error;
pub mod fields;
pub mod methods;

mod buffer;

pub use crate::decoder::buffer::BufferedReader;

pub trait Decodable<T> {
    fn decode(
        buffer: &mut BufferedReader,
        constant_pool: &ConstantPool,
    ) -> Result<T, DecodingError>;
}
