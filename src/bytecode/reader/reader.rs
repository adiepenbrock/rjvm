use crate::bytecode::BytecodeError;

pub trait FromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BytecodeError>;
}

impl FromBytes for u8 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BytecodeError> {
        if bytes.len() != std::mem::size_of::<u8>() {
            return Err(BytecodeError::InvalidData);
        }
        Ok(bytes[0])
    }
}

impl FromBytes for u16 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BytecodeError> {
        if bytes.len() != std::mem::size_of::<u16>() {
            return Err(BytecodeError::InvalidData);
        }
        Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
    }
}

impl FromBytes for u32 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BytecodeError> {
        if bytes.len() != std::mem::size_of::<u32>() {
            return Err(BytecodeError::InvalidData);
        }
        // TODO: we should find a better way to convert bytes to u32 without
        // converting it by hand...
        Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
}

impl FromBytes for i8 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BytecodeError> {
        if bytes.len() != std::mem::size_of::<i8>() {
            return Err(BytecodeError::InvalidData);
        }
        Ok(i8::from_be_bytes([bytes[0]]))
    }
}

impl FromBytes for i16 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BytecodeError> {
        if bytes.len() != std::mem::size_of::<i16>() {
            return Err(BytecodeError::InvalidData);
        }
        Ok(i16::from_be_bytes([bytes[0], bytes[1]]))
    }
}

impl FromBytes for i32 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BytecodeError> {
        if bytes.len() != std::mem::size_of::<i32>() {
            return Err(BytecodeError::InvalidData);
        }
        Ok(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
}

impl FromBytes for i64 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BytecodeError> {
        if bytes.len() != std::mem::size_of::<i64>() {
            return Err(BytecodeError::InvalidData);
        }
        Ok(i64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }
}

impl FromBytes for f32 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BytecodeError> {
        if bytes.len() != std::mem::size_of::<f32>() {
            return Err(BytecodeError::InvalidData);
        }
        Ok(f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
}

impl FromBytes for f64 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BytecodeError> {
        if bytes.len() != std::mem::size_of::<f64>() {
            return Err(BytecodeError::InvalidData);
        }
        Ok(f64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }
}

impl FromBytes for Vec<u8> {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BytecodeError> {
        Ok(bytes.to_vec())
    }
}

#[derive(Debug, Clone)]
pub struct BufferedReader<'a> {
    data: &'a [u8],
    position: usize,
    size: usize,
}

impl<'a> BufferedReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            position: 0,
            size: data.len(),
        }
    }

    fn advance(&mut self, n: usize) -> Result<&'a [u8], BytecodeError> {
        if self.position + n > self.size {
            Err(BytecodeError::UnexpectedEndOfData)
        } else {
            let slice = &self.data[self.position..self.position + n];
            self.position += n;
            Ok(slice)
        }
    }

    pub fn take<T>(&mut self) -> Result<T, BytecodeError>
    where
        T: FromBytes,
    {
        let length = std::mem::size_of::<T>();
        let slice = self.advance(length)?;
        T::from_bytes(slice)
    }

    pub fn peek_bytes<T>(&self) -> Result<T, BytecodeError>
    where
        T: FromBytes,
    {
        let length = std::mem::size_of::<T>();
        let slice = &self.data[self.position..self.position + length];
        T::from_bytes(slice)
    }

    pub fn take_bytes(&mut self, length: usize) -> Result<&'a [u8], BytecodeError> {
        self.advance(length)
    }

    /// Returns the size of [BufferedReader's](BufferedReader) data in bytes.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the current position of [BufferedReader](BufferedReader) in bytes.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Indicates whether the reader has remaining data to be read.
    pub fn has_remaining_data(&self) -> bool {
        self.position == self.data.len()
    }
}
