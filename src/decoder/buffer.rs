pub trait FromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BufferedReaderError>;
}

impl FromBytes for u8 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BufferedReaderError> {
        if bytes.len() != std::mem::size_of::<u8>() {
            return Err(BufferedReaderError::InvalidData);
        }
        Ok(bytes[0])
    }
}

impl FromBytes for u16 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BufferedReaderError> {
        if bytes.len() != std::mem::size_of::<u16>() {
            return Err(BufferedReaderError::InvalidData);
        }
        Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
    }
}

impl FromBytes for u32 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BufferedReaderError> {
        if bytes.len() != std::mem::size_of::<u32>() {
            return Err(BufferedReaderError::InvalidData);
        }
        Ok(u32::from_be_bytes(bytes.try_into().unwrap()))
    }
}

impl FromBytes for i8 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BufferedReaderError> {
        if bytes.len() != std::mem::size_of::<i8>() {
            return Err(BufferedReaderError::InvalidData);
        }
        Ok(i8::from_be_bytes([bytes[0]]))
    }
}

impl FromBytes for i16 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BufferedReaderError> {
        if bytes.len() != std::mem::size_of::<i16>() {
            return Err(BufferedReaderError::InvalidData);
        }
        Ok(i16::from_be_bytes([bytes[0], bytes[1]]))
    }
}

impl FromBytes for i32 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BufferedReaderError> {
        if bytes.len() != std::mem::size_of::<i32>() {
            return Err(BufferedReaderError::InvalidData);
        }
        Ok(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
}

impl FromBytes for i64 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BufferedReaderError> {
        if bytes.len() != std::mem::size_of::<i64>() {
            return Err(BufferedReaderError::InvalidData);
        }
        Ok(i64::from_be_bytes(bytes.try_into().unwrap()))
    }
}

impl FromBytes for f32 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BufferedReaderError> {
        if bytes.len() != std::mem::size_of::<f32>() {
            return Err(BufferedReaderError::InvalidData);
        }
        Ok(f32::from_be_bytes(bytes.try_into().unwrap()))
    }
}

impl FromBytes for f64 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BufferedReaderError> {
        if bytes.len() != std::mem::size_of::<f64>() {
            return Err(BufferedReaderError::InvalidData);
        }
        Ok(f64::from_be_bytes(bytes.try_into().unwrap()))
    }
}

impl FromBytes for Vec<u8> {
    fn from_bytes(bytes: &[u8]) -> Result<Self, BufferedReaderError> {
        Ok(bytes.to_vec())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BufferedReaderError {
    UnexpectedEndOfData,
    InvalidData,
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

    fn advance(&mut self, n: usize) -> Result<&'a [u8], BufferedReaderError> {
        if self.position + n > self.size {
            Err(BufferedReaderError::UnexpectedEndOfData)
        } else {
            let slice = &self.data[self.position..self.position + n];
            self.position += n;
            Ok(slice)
        }
    }

    pub fn take<T>(&mut self) -> Result<T, BufferedReaderError>
    where
        T: FromBytes,
    {
        let length = std::mem::size_of::<T>();
        let slice = self.advance(length)?;
        T::from_bytes(slice)
    }

    pub fn peek_bytes<T>(&self) -> Result<T, BufferedReaderError>
    where
        T: FromBytes,
    {
        let length = std::mem::size_of::<T>();
        let slice = &self.data[self.position..self.position + length];
        T::from_bytes(slice)
    }

    pub fn take_bytes(&mut self, length: usize) -> Result<&'a [u8], BufferedReaderError> {
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

    /// Indicates whether the buffer has remaining data to be read.
    pub fn has_remaining_data(&self) -> bool {
        self.position == self.data.len()
    }
}
