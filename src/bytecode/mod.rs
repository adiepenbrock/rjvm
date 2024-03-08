pub mod pool;

#[derive(Debug, Clone, PartialEq)]
pub enum BytecodeError {
    ConstantPoolEntryAlreadyExists,
    ConstantPoolEntryNotFound,
}

pub type Result<T, E = BytecodeError> = core::result::Result<T, E>;
