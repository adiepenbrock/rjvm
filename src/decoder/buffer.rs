pub trait FromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BufferError>;
}

impl FromBytes for u8 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BufferError> {
        if bytes.len() != std::mem::size_of::<u8>() {
            return Err(BufferError::InvalidData);
        }
        Ok(bytes[0])
    }
}

impl FromBytes for u16 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BufferError> {
        if bytes.len() != std::mem::size_of::<u16>() {
            return Err(BufferError::InvalidData);
        }
        Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
    }
}

impl FromBytes for u32 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BufferError> {
        if bytes.len() != std::mem::size_of::<u32>() {
            return Err(BufferError::InvalidData);
        }
        Ok(u32::from_be_bytes(bytes.try_into().unwrap()))
    }
}

impl FromBytes for i32 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BufferError> {
        if bytes.len() != std::mem::size_of::<i32>() {
            return Err(BufferError::InvalidData);
        }
        Ok(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
}

impl FromBytes for i64 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BufferError> {
        if bytes.len() != std::mem::size_of::<i64>() {
            return Err(BufferError::InvalidData);
        }
        Ok(i64::from_be_bytes(bytes.try_into().unwrap()))
    }
}

impl FromBytes for f32 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BufferError> {
        if bytes.len() != std::mem::size_of::<f32>() {
            return Err(BufferError::InvalidData);
        }
        Ok(f32::from_be_bytes(bytes.try_into().unwrap()))
    }
}

impl FromBytes for f64 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BufferError> {
        if bytes.len() != std::mem::size_of::<f64>() {
            return Err(BufferError::InvalidData);
        }
        Ok(f64::from_be_bytes(bytes.try_into().unwrap()))
    }
}

impl FromBytes for Vec<u8> {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BufferError> {
        Ok(bytes.to_vec())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BufferError {
    UnexpectedEndOfData,
    InvalidData,
}

#[derive(Debug, Clone)]
pub struct Buffer<'a> {
    pub data: &'a [u8],
    pub position: usize,
}

impl<'a> Buffer<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, position: 0 }
    }

    fn advance(&mut self, n: usize) -> Result<&'a [u8], BufferError> {
        if self.position + n > self.data.len() {
            Err(BufferError::UnexpectedEndOfData)
        } else {
            let slice = &self.data[self.position..self.position + n];
            self.position += n;
            Ok(slice)
        }
    }

    pub fn take<T>(&mut self) -> Result<T, BufferError>
    where
        T: FromBytes,
    {
        let length = std::mem::size_of::<T>();
        let slice = self.advance(length)?;
        T::from_bytes(slice)
    }

    pub fn peek_bytes<T>(&self) -> Result<T, BufferError>
    where
        T: FromBytes,
    {
        let length = std::mem::size_of::<T>();
        let slice = &self.data[self.position..self.position + length];
        T::from_bytes(slice)
    }

    pub fn take_length(&mut self, length: usize) -> Result<&'a [u8], BufferError> {
        self.advance(length)
    }

    pub fn peek_length(&self, length: usize) -> Result<&'a [u8], BufferError> {
        if self.position + length > self.data.len() {
            Err(BufferError::UnexpectedEndOfData)
        } else {
            Ok(&self.data[self.position..self.position + length])
        }
    }
}
