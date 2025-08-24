use crate::bytecode::reader::BufferedReader;
use crate::bytecode::BytecodeError;
use crate::types::instructions::*;

pub trait InstructionFactory {
    /// Create an `Instruction` and return it as a boxed trait object. To support instructions
    /// that have additional data, the `buffer` is passed to the factory method.
    fn create_instruction(
        buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError>;
}

// -----------------------------------------------------------------------------
//  - implement `InstructionFactory` traits on builtin instructions -
// -----------------------------------------------------------------------------
impl InstructionFactory for Aaload {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Aaload))
    }
}

impl InstructionFactory for Aastore {
    fn create_instruction(
        buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        let index = buffer.take::<u16>()?;
        Ok(Box::new(Aastore { args: vec![index] }))
    }
}

impl InstructionFactory for AConstNull {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(AConstNull))
    }
}

impl InstructionFactory for Aload {
    fn create_instruction(
        buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        let index = buffer.take::<u16>()?;
        Ok(Box::new(Aload { args: vec![index] }))
    }
}

impl InstructionFactory for Aload0 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Aload0))
    }
}

impl InstructionFactory for Aload1 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Aload1))
    }
}

impl InstructionFactory for Aload2 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Aload2))
    }
}

impl InstructionFactory for Aload3 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Aload3))
    }
}

impl InstructionFactory for Anewarray {
    fn create_instruction(
        buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        let index = buffer.take::<u16>()?;
        Ok(Box::new(Anewarray { args: vec![index] }))
    }
}

impl InstructionFactory for Areturn {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Areturn))
    }
}

impl InstructionFactory for Arraylength {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Arraylength))
    }
}

impl InstructionFactory for Astore {
    fn create_instruction(
        buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        let index = buffer.take::<u16>()?;
        Ok(Box::new(Astore { args: vec![index] }))
    }
}

impl InstructionFactory for Astore0 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Astore0))
    }
}

impl InstructionFactory for Astore1 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Astore1))
    }
}

impl InstructionFactory for Astore2 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Astore2))
    }
}

impl InstructionFactory for Astore3 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Astore3))
    }
}

impl InstructionFactory for Athrow {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Athrow))
    }
}

impl InstructionFactory for Baload {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Baload))
    }
}

impl InstructionFactory for Bastore {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Bastore))
    }
}

impl InstructionFactory for Bipush {
    fn create_instruction(
        buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        let index = buffer.take::<u16>()?;
        Ok(Box::new(Bipush { args: vec![index] }))
    }
}

impl InstructionFactory for Caload {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Caload))
    }
}

impl InstructionFactory for Castore {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Castore))
    }
}

impl InstructionFactory for Checkcast {
    fn create_instruction(
        buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        let index = buffer.take::<u16>()?;
        Ok(Box::new(Checkcast { args: vec![index] }))
    }
}

impl InstructionFactory for D2f {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(D2f))
    }
}

impl InstructionFactory for D2i {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(D2i))
    }
}

impl InstructionFactory for D2l {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(D2l))
    }
}

impl InstructionFactory for Dadd {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Dadd))
    }
}

impl InstructionFactory for Daload {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Daload))
    }
}

impl InstructionFactory for Dastore {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Dastore))
    }
}

impl InstructionFactory for Dcmpg {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Dcmpg))
    }
}

impl InstructionFactory for Dcmpl {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Dcmpl))
    }
}

impl InstructionFactory for Dconst0 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Dconst0))
    }
}

impl InstructionFactory for Dconst1 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Dconst1))
    }
}

impl InstructionFactory for Ddiv {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Ddiv))
    }
}

impl InstructionFactory for Dload {
    fn create_instruction(
        buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        let index = buffer.take::<u16>()?;
        Ok(Box::new(Dload { args: vec![index] }))
    }
}

impl InstructionFactory for Dload0 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Dload0))
    }
}

impl InstructionFactory for Dload1 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Dload1))
    }
}

impl InstructionFactory for Dload2 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Dload2))
    }
}

impl InstructionFactory for Dload3 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Dload3))
    }
}

impl InstructionFactory for Dmul {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Dmul))
    }
}

impl InstructionFactory for Dneg {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Dneg))
    }
}

impl InstructionFactory for Drem {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Drem))
    }
}

impl InstructionFactory for Dreturn {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Dreturn))
    }
}

impl InstructionFactory for Dstore {
    fn create_instruction(
        buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        let index = buffer.take::<u16>()?;
        Ok(Box::new(Dstore { args: vec![index] }))
    }
}

impl InstructionFactory for Dstore0 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Dstore0))
    }
}

impl InstructionFactory for Dstore1 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Dstore1))
    }
}

impl InstructionFactory for Dstore2 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Dstore2))
    }
}

impl InstructionFactory for Dstore3 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Dstore3))
    }
}

impl InstructionFactory for Dsub {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Dsub))
    }
}

impl InstructionFactory for Dup {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Dup))
    }
}

impl InstructionFactory for DupX1 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(DupX1))
    }
}

impl InstructionFactory for DupX2 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(DupX2))
    }
}

impl InstructionFactory for Dup2 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Dup2))
    }
}

impl InstructionFactory for Dup2X1 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Dup2X1))
    }
}

impl InstructionFactory for Dup2X2 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Dup2X2))
    }
}

impl InstructionFactory for F2D {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(F2D))
    }
}

impl InstructionFactory for F2I {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(F2I))
    }
}

impl InstructionFactory for F2L {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(F2L))
    }
}

impl InstructionFactory for Fadd {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Fadd))
    }
}

impl InstructionFactory for Faload {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Faload))
    }
}

impl InstructionFactory for Fastore {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Fastore))
    }
}

impl InstructionFactory for Fcmpg {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Fcmpg))
    }
}

impl InstructionFactory for Fcmpl {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Fcmpl))
    }
}

impl InstructionFactory for Fconst0 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Fconst0))
    }
}

impl InstructionFactory for Fconst1 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Fconst1))
    }
}

impl InstructionFactory for Fconst2 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Fconst2))
    }
}

impl InstructionFactory for Fdiv {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Fdiv))
    }
}

impl InstructionFactory for Fload {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Fload))
    }
}

impl InstructionFactory for Fload0 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Fload0))
    }
}

impl InstructionFactory for Fload1 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Fload1))
    }
}

impl InstructionFactory for Fload2 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Fload2))
    }
}

impl InstructionFactory for Fload3 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Fload3))
    }
}

impl InstructionFactory for Fmul {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Fmul))
    }
}

impl InstructionFactory for Fneg {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Fneg))
    }
}

impl InstructionFactory for Frem {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Frem))
    }
}

impl InstructionFactory for Freturn {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Freturn))
    }
}

impl InstructionFactory for Fstore {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Fstore))
    }
}

impl InstructionFactory for Fstore0 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Fstore0))
    }
}

impl InstructionFactory for Fstore1 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Fstore1))
    }
}

impl InstructionFactory for Fstore2 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Fstore2))
    }
}

impl InstructionFactory for Fstore3 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Fstore3))
    }
}

impl InstructionFactory for Fsub {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Fsub))
    }
}

impl InstructionFactory for Getfield {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Getfield))
    }
}

impl InstructionFactory for Getstatic {
    fn create_instruction(
        buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        let index = buffer.take::<u8>()?;
        let index2 = buffer.take::<u8>()?;

        let value = ((index as u16) << 8) | index2 as u16;
        Ok(Box::new(Getstatic { args: vec![value] }))
    }
}

impl InstructionFactory for Goto {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Goto))
    }
}

impl InstructionFactory for GotoW {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(GotoW))
    }
}

impl InstructionFactory for I2b {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(I2b))
    }
}

impl InstructionFactory for I2c {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(I2c))
    }
}

impl InstructionFactory for I2d {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(I2d))
    }
}

impl InstructionFactory for I2f {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(I2f))
    }
}

impl InstructionFactory for I2l {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(I2l))
    }
}

impl InstructionFactory for I2s {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(I2s))
    }
}

impl InstructionFactory for Iadd {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Iadd))
    }
}

impl InstructionFactory for Iaload {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Iaload))
    }
}

impl InstructionFactory for Iand {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Iand))
    }
}

impl InstructionFactory for Iastore {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Iastore))
    }
}

impl InstructionFactory for IconstM1 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(IconstM1))
    }
}

impl InstructionFactory for Iconst0 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Iconst0))
    }
}

impl InstructionFactory for Iconst1 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Iconst1))
    }
}

impl InstructionFactory for Iconst2 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Iconst2))
    }
}

impl InstructionFactory for Iconst3 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Iconst3))
    }
}

impl InstructionFactory for Iconst4 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Iconst4))
    }
}

impl InstructionFactory for Iconst5 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Iconst5))
    }
}

impl InstructionFactory for Idiv {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Idiv))
    }
}

impl InstructionFactory for IfAcmpeq {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(IfAcmpeq))
    }
}

impl InstructionFactory for IfAcmpne {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(IfAcmpne))
    }
}

impl InstructionFactory for IfIcmpeq {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(IfIcmpeq))
    }
}

impl InstructionFactory for IfIcmpge {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(IfIcmpge))
    }
}

impl InstructionFactory for IfIcmpgt {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(IfIcmpgt))
    }
}

impl InstructionFactory for IfIcmple {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(IfIcmple))
    }
}

impl InstructionFactory for IfIcmplt {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(IfIcmplt))
    }
}

impl InstructionFactory for IfIcmpne {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(IfIcmpne))
    }
}

impl InstructionFactory for Ifeq {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Ifeq))
    }
}

impl InstructionFactory for Ifge {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Ifge))
    }
}

impl InstructionFactory for Ifgt {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Ifgt))
    }
}

impl InstructionFactory for Ifle {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Ifle))
    }
}

impl InstructionFactory for Iflt {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Iflt))
    }
}

impl InstructionFactory for Ifne {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Ifne))
    }
}

impl InstructionFactory for Ifnonnull {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Ifnonnull))
    }
}

impl InstructionFactory for Ifnull {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Ifnull))
    }
}

impl InstructionFactory for Iinc {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Iinc))
    }
}

impl InstructionFactory for Iload {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Iload))
    }
}

impl InstructionFactory for Iload0 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Iload0))
    }
}

impl InstructionFactory for Iload1 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Iload1))
    }
}

impl InstructionFactory for Iload2 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Iload2))
    }
}

impl InstructionFactory for Iload3 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Iload3))
    }
}

impl InstructionFactory for Imul {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Imul))
    }
}

impl InstructionFactory for Ineg {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Ineg))
    }
}

impl InstructionFactory for Instanceof {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Instanceof))
    }
}

impl InstructionFactory for Invokedynamic {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Invokedynamic))
    }
}

impl InstructionFactory for Invokeinterface {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Invokeinterface))
    }
}

impl InstructionFactory for Invokespecial {
    fn create_instruction(
        buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        let index = buffer.take::<u16>()?;
        Ok(Box::new(Invokespecial { args: vec![index] }))
    }
}

impl InstructionFactory for Invokestatic {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Invokestatic))
    }
}

impl InstructionFactory for Invokevirtual {
    fn create_instruction(
        buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        let index = buffer.take::<u16>()?;
        Ok(Box::new(Invokevirtual { args: vec![index] }))
    }
}

impl InstructionFactory for Ior {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Ior))
    }
}

impl InstructionFactory for Irem {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Irem))
    }
}

impl InstructionFactory for Ireturn {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Ireturn))
    }
}

impl InstructionFactory for Ishl {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Ishl))
    }
}

impl InstructionFactory for Ishr {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Ishr))
    }
}

impl InstructionFactory for Istore {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Istore))
    }
}

impl InstructionFactory for Istore0 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Istore0))
    }
}

impl InstructionFactory for Istore1 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Istore1))
    }
}

impl InstructionFactory for Istore2 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Istore2))
    }
}

impl InstructionFactory for Istore3 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Istore3))
    }
}

impl InstructionFactory for Isub {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Isub))
    }
}

impl InstructionFactory for Iushr {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Iushr))
    }
}

impl InstructionFactory for Ixor {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Ixor))
    }
}

impl InstructionFactory for Jsr {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Jsr))
    }
}

impl InstructionFactory for JsrW {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(JsrW))
    }
}

impl InstructionFactory for L2D {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(L2D))
    }
}

impl InstructionFactory for L2F {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(L2F))
    }
}

impl InstructionFactory for L2I {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(L2I))
    }
}

impl InstructionFactory for Ladd {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Ladd))
    }
}

impl InstructionFactory for Laload {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Laload))
    }
}

impl InstructionFactory for Land {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Land))
    }
}

impl InstructionFactory for Lastore {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lastore))
    }
}

impl InstructionFactory for Lcmp {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lcmp))
    }
}

impl InstructionFactory for Lconst0 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lconst0))
    }
}

impl InstructionFactory for Lconst1 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lconst1))
    }
}

impl InstructionFactory for Ldc {
    fn create_instruction(
        buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        let index = buffer.take::<u8>()?;
        Ok(Box::new(Ldc {
            args: vec![index.into()],
        }))
    }
}

impl InstructionFactory for LdcW {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(LdcW))
    }
}

impl InstructionFactory for Ldc2W {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Ldc2W))
    }
}

impl InstructionFactory for Ldiv {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Ldiv))
    }
}

impl InstructionFactory for Lload {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lload))
    }
}

impl InstructionFactory for Lload0 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lload0))
    }
}

impl InstructionFactory for Lload1 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lload1))
    }
}

impl InstructionFactory for Lload2 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lload2))
    }
}

impl InstructionFactory for Lload3 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lload3))
    }
}

impl InstructionFactory for Lmul {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lmul))
    }
}

impl InstructionFactory for Lneg {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lneg))
    }
}

impl InstructionFactory for LookupSwitch {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(LookupSwitch))
    }
}

impl InstructionFactory for Lor {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lor))
    }
}

impl InstructionFactory for Lrem {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lrem))
    }
}

impl InstructionFactory for Lreturn {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lreturn))
    }
}

impl InstructionFactory for Lshl {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lshl))
    }
}

impl InstructionFactory for Lshr {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lshr))
    }
}

impl InstructionFactory for Lstore {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lstore))
    }
}

impl InstructionFactory for Lstore0 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lstore0))
    }
}

impl InstructionFactory for Lstore1 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lstore1))
    }
}

impl InstructionFactory for Lstore2 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lstore2))
    }
}

impl InstructionFactory for Lstore3 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lstore3))
    }
}

impl InstructionFactory for Lsub {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lsub))
    }
}

impl InstructionFactory for Lushr {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lushr))
    }
}

impl InstructionFactory for Lxor {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Lxor))
    }
}

impl InstructionFactory for Monitorenter {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Monitorenter))
    }
}

impl InstructionFactory for Monitorexit {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Monitorexit))
    }
}

impl InstructionFactory for Multianewarray {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Multianewarray))
    }
}

impl InstructionFactory for New {
    fn create_instruction(
        buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        let index = buffer.take::<u16>()?;
        Ok(Box::new(New { args: vec![index] }))
    }
}

impl InstructionFactory for Newarray {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Newarray))
    }
}

impl InstructionFactory for Nop {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Nop))
    }
}

impl InstructionFactory for Pop {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Pop))
    }
}

impl InstructionFactory for Pop2 {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Pop2))
    }
}

impl InstructionFactory for Putfield {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Putfield))
    }
}

impl InstructionFactory for Putstatic {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Putstatic))
    }
}

impl InstructionFactory for Ret {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Ret))
    }
}

impl InstructionFactory for Return {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Return))
    }
}

impl InstructionFactory for Saload {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Saload))
    }
}

impl InstructionFactory for Sastore {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Sastore))
    }
}

impl InstructionFactory for Sipush {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Sipush))
    }
}

impl InstructionFactory for Swap {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Swap))
    }
}

impl InstructionFactory for Tableswitch {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Tableswitch))
    }
}

impl InstructionFactory for Wide {
    fn create_instruction(
        _buffer: &mut BufferedReader,
    ) -> Result<Box<dyn Instruction>, BytecodeError> {
        Ok(Box::new(Wide))
    }
}
/// Parses an `Instruction` identified by its `opcode`and returns it as a boxed trait object. To
/// support instructions that have additional data, the `buffer` is passed to the factory method.
pub fn parse_instruction(
    op: u8,
    buffer: &mut BufferedReader,
) -> Result<Box<dyn Instruction>, BytecodeError> {
    match op {
        Aaload::OPCODE => Ok(Aaload::create_instruction(buffer)?),
        Aastore::OPCODE => Ok(Aastore::create_instruction(buffer)?),
        AConstNull::OPCODE => Ok(AConstNull::create_instruction(buffer)?),
        Aload::OPCODE => Ok(Aload::create_instruction(buffer)?),
        Aload0::OPCODE => Ok(Aload0::create_instruction(buffer)?),
        Aload1::OPCODE => Ok(Aload1::create_instruction(buffer)?),
        Aload2::OPCODE => Ok(Aload2::create_instruction(buffer)?),
        Aload3::OPCODE => Ok(Aload3::create_instruction(buffer)?),
        Anewarray::OPCODE => Ok(Anewarray::create_instruction(buffer)?),
        Areturn::OPCODE => Ok(Areturn::create_instruction(buffer)?),
        Arraylength::OPCODE => Ok(Arraylength::create_instruction(buffer)?),
        Astore::OPCODE => Ok(Astore::create_instruction(buffer)?),
        Astore0::OPCODE => Ok(Astore0::create_instruction(buffer)?),
        Astore1::OPCODE => Ok(Astore1::create_instruction(buffer)?),
        Astore2::OPCODE => Ok(Astore2::create_instruction(buffer)?),
        Astore3::OPCODE => Ok(Astore3::create_instruction(buffer)?),
        Athrow::OPCODE => Ok(Athrow::create_instruction(buffer)?),
        Baload::OPCODE => Ok(Baload::create_instruction(buffer)?),
        Bastore::OPCODE => Ok(Bastore::create_instruction(buffer)?),
        Bipush::OPCODE => Ok(Bipush::create_instruction(buffer)?),
        Caload::OPCODE => Ok(Caload::create_instruction(buffer)?),
        Castore::OPCODE => Ok(Castore::create_instruction(buffer)?),
        Checkcast::OPCODE => Ok(Checkcast::create_instruction(buffer)?),
        D2f::OPCODE => Ok(D2f::create_instruction(buffer)?),
        D2i::OPCODE => Ok(D2i::create_instruction(buffer)?),
        D2l::OPCODE => Ok(D2l::create_instruction(buffer)?),
        Dadd::OPCODE => Ok(Dadd::create_instruction(buffer)?),
        Daload::OPCODE => Ok(Daload::create_instruction(buffer)?),
        Dastore::OPCODE => Ok(Dastore::create_instruction(buffer)?),
        Dcmpg::OPCODE => Ok(Dcmpg::create_instruction(buffer)?),
        Dcmpl::OPCODE => Ok(Dcmpl::create_instruction(buffer)?),
        Dconst0::OPCODE => Ok(Dconst0::create_instruction(buffer)?),
        Dconst1::OPCODE => Ok(Dconst1::create_instruction(buffer)?),
        Ddiv::OPCODE => Ok(Ddiv::create_instruction(buffer)?),
        Dload::OPCODE => Ok(Dload::create_instruction(buffer)?),
        Dload0::OPCODE => Ok(Dload0::create_instruction(buffer)?),
        Dload1::OPCODE => Ok(Dload1::create_instruction(buffer)?),
        Dload2::OPCODE => Ok(Dload2::create_instruction(buffer)?),
        Dload3::OPCODE => Ok(Dload3::create_instruction(buffer)?),
        Dmul::OPCODE => Ok(Dmul::create_instruction(buffer)?),
        Dneg::OPCODE => Ok(Dneg::create_instruction(buffer)?),
        Drem::OPCODE => Ok(Drem::create_instruction(buffer)?),
        Dreturn::OPCODE => Ok(Dreturn::create_instruction(buffer)?),
        Dstore::OPCODE => Ok(Dstore::create_instruction(buffer)?),
        Dstore0::OPCODE => Ok(Dstore0::create_instruction(buffer)?),
        Dstore1::OPCODE => Ok(Dstore1::create_instruction(buffer)?),
        Dstore2::OPCODE => Ok(Dstore2::create_instruction(buffer)?),
        Dstore3::OPCODE => Ok(Dstore3::create_instruction(buffer)?),
        Dsub::OPCODE => Ok(Dsub::create_instruction(buffer)?),
        Dup::OPCODE => Ok(Dup::create_instruction(buffer)?),
        DupX1::OPCODE => Ok(DupX1::create_instruction(buffer)?),
        DupX2::OPCODE => Ok(DupX2::create_instruction(buffer)?),
        Dup2::OPCODE => Ok(Dup2::create_instruction(buffer)?),
        Dup2X1::OPCODE => Ok(Dup2X1::create_instruction(buffer)?),
        Dup2X2::OPCODE => Ok(Dup2X2::create_instruction(buffer)?),
        F2D::OPCODE => Ok(F2D::create_instruction(buffer)?),
        F2I::OPCODE => Ok(F2I::create_instruction(buffer)?),
        F2L::OPCODE => Ok(F2L::create_instruction(buffer)?),
        Fadd::OPCODE => Ok(Fadd::create_instruction(buffer)?),
        Faload::OPCODE => Ok(Faload::create_instruction(buffer)?),
        Fastore::OPCODE => Ok(Fastore::create_instruction(buffer)?),
        Fcmpg::OPCODE => Ok(Fcmpg::create_instruction(buffer)?),
        Fcmpl::OPCODE => Ok(Fcmpl::create_instruction(buffer)?),
        Fconst0::OPCODE => Ok(Fconst0::create_instruction(buffer)?),
        Fconst1::OPCODE => Ok(Fconst1::create_instruction(buffer)?),
        Fconst2::OPCODE => Ok(Fconst2::create_instruction(buffer)?),
        Fdiv::OPCODE => Ok(Fdiv::create_instruction(buffer)?),
        Fload::OPCODE => Ok(Fload::create_instruction(buffer)?),
        Fload0::OPCODE => Ok(Fload0::create_instruction(buffer)?),
        Fload1::OPCODE => Ok(Fload1::create_instruction(buffer)?),
        Fload2::OPCODE => Ok(Fload2::create_instruction(buffer)?),
        Fload3::OPCODE => Ok(Fload3::create_instruction(buffer)?),
        Fmul::OPCODE => Ok(Fmul::create_instruction(buffer)?),
        Fneg::OPCODE => Ok(Fneg::create_instruction(buffer)?),
        Frem::OPCODE => Ok(Frem::create_instruction(buffer)?),
        Freturn::OPCODE => Ok(Freturn::create_instruction(buffer)?),
        Fstore::OPCODE => Ok(Fstore::create_instruction(buffer)?),
        Fstore0::OPCODE => Ok(Fstore0::create_instruction(buffer)?),
        Fstore1::OPCODE => Ok(Fstore1::create_instruction(buffer)?),
        Fstore2::OPCODE => Ok(Fstore2::create_instruction(buffer)?),
        Fstore3::OPCODE => Ok(Fstore3::create_instruction(buffer)?),
        Fsub::OPCODE => Ok(Fsub::create_instruction(buffer)?),
        Getfield::OPCODE => Ok(Getfield::create_instruction(buffer)?),
        Getstatic::OPCODE => Ok(Getstatic::create_instruction(buffer)?),
        Goto::OPCODE => Ok(Goto::create_instruction(buffer)?),
        GotoW::OPCODE => Ok(GotoW::create_instruction(buffer)?),
        I2b::OPCODE => Ok(I2b::create_instruction(buffer)?),
        I2c::OPCODE => Ok(I2c::create_instruction(buffer)?),
        I2d::OPCODE => Ok(I2d::create_instruction(buffer)?),
        I2f::OPCODE => Ok(I2f::create_instruction(buffer)?),
        I2l::OPCODE => Ok(I2l::create_instruction(buffer)?),
        I2s::OPCODE => Ok(I2s::create_instruction(buffer)?),
        Iadd::OPCODE => Ok(Iadd::create_instruction(buffer)?),
        Iaload::OPCODE => Ok(Iaload::create_instruction(buffer)?),
        Iand::OPCODE => Ok(Iand::create_instruction(buffer)?),
        Iastore::OPCODE => Ok(Iastore::create_instruction(buffer)?),
        IconstM1::OPCODE => Ok(IconstM1::create_instruction(buffer)?),
        Iconst0::OPCODE => Ok(Iconst0::create_instruction(buffer)?),
        Iconst1::OPCODE => Ok(Iconst1::create_instruction(buffer)?),
        Iconst2::OPCODE => Ok(Iconst2::create_instruction(buffer)?),
        Iconst3::OPCODE => Ok(Iconst3::create_instruction(buffer)?),
        Iconst4::OPCODE => Ok(Iconst4::create_instruction(buffer)?),
        Iconst5::OPCODE => Ok(Iconst5::create_instruction(buffer)?),
        Idiv::OPCODE => Ok(Idiv::create_instruction(buffer)?),
        IfAcmpeq::OPCODE => Ok(IfAcmpeq::create_instruction(buffer)?),
        IfAcmpne::OPCODE => Ok(IfAcmpne::create_instruction(buffer)?),
        IfIcmpeq::OPCODE => Ok(IfIcmpeq::create_instruction(buffer)?),
        IfIcmpge::OPCODE => Ok(IfIcmpge::create_instruction(buffer)?),
        IfIcmpgt::OPCODE => Ok(IfIcmpgt::create_instruction(buffer)?),
        IfIcmple::OPCODE => Ok(IfIcmple::create_instruction(buffer)?),
        IfIcmplt::OPCODE => Ok(IfIcmplt::create_instruction(buffer)?),
        IfIcmpne::OPCODE => Ok(IfIcmpne::create_instruction(buffer)?),
        Ifeq::OPCODE => Ok(Ifeq::create_instruction(buffer)?),
        Ifge::OPCODE => Ok(Ifge::create_instruction(buffer)?),
        Ifgt::OPCODE => Ok(Ifgt::create_instruction(buffer)?),
        Ifle::OPCODE => Ok(Ifle::create_instruction(buffer)?),
        Iflt::OPCODE => Ok(Iflt::create_instruction(buffer)?),
        Ifne::OPCODE => Ok(Ifne::create_instruction(buffer)?),
        Ifnonnull::OPCODE => Ok(Ifnonnull::create_instruction(buffer)?),
        Ifnull::OPCODE => Ok(Ifnull::create_instruction(buffer)?),
        Iinc::OPCODE => Ok(Iinc::create_instruction(buffer)?),
        Iload::OPCODE => Ok(Iload::create_instruction(buffer)?),
        Iload0::OPCODE => Ok(Iload0::create_instruction(buffer)?),
        Iload1::OPCODE => Ok(Iload1::create_instruction(buffer)?),
        Iload2::OPCODE => Ok(Iload2::create_instruction(buffer)?),
        Iload3::OPCODE => Ok(Iload3::create_instruction(buffer)?),
        Imul::OPCODE => Ok(Imul::create_instruction(buffer)?),
        Ineg::OPCODE => Ok(Ineg::create_instruction(buffer)?),
        Instanceof::OPCODE => Ok(Instanceof::create_instruction(buffer)?),
        Invokedynamic::OPCODE => Ok(Invokedynamic::create_instruction(buffer)?),
        Invokeinterface::OPCODE => Ok(Invokeinterface::create_instruction(buffer)?),
        Invokespecial::OPCODE => Ok(Invokespecial::create_instruction(buffer)?),
        Invokestatic::OPCODE => Ok(Invokestatic::create_instruction(buffer)?),
        Invokevirtual::OPCODE => Ok(Invokevirtual::create_instruction(buffer)?),
        Ior::OPCODE => Ok(Ior::create_instruction(buffer)?),
        Irem::OPCODE => Ok(Irem::create_instruction(buffer)?),
        Ireturn::OPCODE => Ok(Ireturn::create_instruction(buffer)?),
        Ishl::OPCODE => Ok(Ishl::create_instruction(buffer)?),
        Ishr::OPCODE => Ok(Ishr::create_instruction(buffer)?),
        Istore::OPCODE => Ok(Istore::create_instruction(buffer)?),
        Istore0::OPCODE => Ok(Istore0::create_instruction(buffer)?),
        Istore1::OPCODE => Ok(Istore1::create_instruction(buffer)?),
        Istore2::OPCODE => Ok(Istore2::create_instruction(buffer)?),
        Istore3::OPCODE => Ok(Istore3::create_instruction(buffer)?),
        Isub::OPCODE => Ok(Isub::create_instruction(buffer)?),
        Iushr::OPCODE => Ok(Iushr::create_instruction(buffer)?),
        Ixor::OPCODE => Ok(Ixor::create_instruction(buffer)?),
        Jsr::OPCODE => Ok(Jsr::create_instruction(buffer)?),
        JsrW::OPCODE => Ok(JsrW::create_instruction(buffer)?),
        L2D::OPCODE => Ok(L2D::create_instruction(buffer)?),
        L2F::OPCODE => Ok(L2F::create_instruction(buffer)?),
        L2I::OPCODE => Ok(L2I::create_instruction(buffer)?),
        Ladd::OPCODE => Ok(Ladd::create_instruction(buffer)?),
        Laload::OPCODE => Ok(Laload::create_instruction(buffer)?),
        Land::OPCODE => Ok(Land::create_instruction(buffer)?),
        Lastore::OPCODE => Ok(Lastore::create_instruction(buffer)?),
        Lcmp::OPCODE => Ok(Lcmp::create_instruction(buffer)?),
        Lconst0::OPCODE => Ok(Lconst0::create_instruction(buffer)?),
        Lconst1::OPCODE => Ok(Lconst1::create_instruction(buffer)?),
        Ldc::OPCODE => Ok(Ldc::create_instruction(buffer)?),
        LdcW::OPCODE => Ok(LdcW::create_instruction(buffer)?),
        Ldc2W::OPCODE => Ok(Ldc2W::create_instruction(buffer)?),
        Ldiv::OPCODE => Ok(Ldiv::create_instruction(buffer)?),
        Lload::OPCODE => Ok(Lload::create_instruction(buffer)?),
        Lload0::OPCODE => Ok(Lload0::create_instruction(buffer)?),
        Lload1::OPCODE => Ok(Lload1::create_instruction(buffer)?),
        Lload2::OPCODE => Ok(Lload2::create_instruction(buffer)?),
        Lload3::OPCODE => Ok(Lload3::create_instruction(buffer)?),
        Lmul::OPCODE => Ok(Lmul::create_instruction(buffer)?),
        Lneg::OPCODE => Ok(Lneg::create_instruction(buffer)?),
        LookupSwitch::OPCODE => Ok(LookupSwitch::create_instruction(buffer)?),
        Lor::OPCODE => Ok(Lor::create_instruction(buffer)?),
        Lrem::OPCODE => Ok(Lrem::create_instruction(buffer)?),
        Lreturn::OPCODE => Ok(Lreturn::create_instruction(buffer)?),
        Lshl::OPCODE => Ok(Lshl::create_instruction(buffer)?),
        Lshr::OPCODE => Ok(Lshr::create_instruction(buffer)?),
        Lstore::OPCODE => Ok(Lstore::create_instruction(buffer)?),
        Lstore0::OPCODE => Ok(Lstore0::create_instruction(buffer)?),
        Lstore1::OPCODE => Ok(Lstore1::create_instruction(buffer)?),
        Lstore2::OPCODE => Ok(Lstore2::create_instruction(buffer)?),
        Lstore3::OPCODE => Ok(Lstore3::create_instruction(buffer)?),
        Lsub::OPCODE => Ok(Lsub::create_instruction(buffer)?),
        Lushr::OPCODE => Ok(Lushr::create_instruction(buffer)?),
        Lxor::OPCODE => Ok(Lxor::create_instruction(buffer)?),
        Monitorenter::OPCODE => Ok(Monitorenter::create_instruction(buffer)?),
        Monitorexit::OPCODE => Ok(Monitorexit::create_instruction(buffer)?),
        Multianewarray::OPCODE => Ok(Multianewarray::create_instruction(buffer)?),
        New::OPCODE => Ok(New::create_instruction(buffer)?),
        Newarray::OPCODE => Ok(Newarray::create_instruction(buffer)?),
        Nop::OPCODE => Ok(Nop::create_instruction(buffer)?),
        Pop::OPCODE => Ok(Pop::create_instruction(buffer)?),
        Pop2::OPCODE => Ok(Pop2::create_instruction(buffer)?),
        Putfield::OPCODE => Ok(Putfield::create_instruction(buffer)?),
        Putstatic::OPCODE => Ok(Putstatic::create_instruction(buffer)?),
        Ret::OPCODE => Ok(Ret::create_instruction(buffer)?),
        Return::OPCODE => Ok(Return::create_instruction(buffer)?),
        Saload::OPCODE => Ok(Saload::create_instruction(buffer)?),
        Sastore::OPCODE => Ok(Sastore::create_instruction(buffer)?),
        Sipush::OPCODE => Ok(Sipush::create_instruction(buffer)?),
        Swap::OPCODE => Ok(Swap::create_instruction(buffer)?),
        Tableswitch::OPCODE => Ok(Tableswitch::create_instruction(buffer)?),
        Wide::OPCODE => Ok(Wide::create_instruction(buffer)?),
        _ => Err(BytecodeError::UnsupportedInstruction),
    }
}

/*
list of all instructions with their opcodes

aaload          0x32
aastore         0x53
aconst_null     0x10
aload           0x19
aload_0         0x2a
aload_1         0x2b
aload_2         0x2c
aload_3         0x2d
anewarray       0xbd
areturn         0xb0
arraylength     0xbe
astore          0x3a
astore_0        0x4b
astore_1        0x4c
astore_2        0x4d
astore_3        0x4e
athrow          0xbf
baload          0x33
bastore         0x54
bipush          0x10
caload          0x34
castore         0x55
checkcast       0xc0
d2f             0x90
d2i             0x8e
d2l             0x8f
dadd            0x63
daload          0x31
dastore         0x52
dcmpg           0x98
dcmpl           0x97
dconst_0        0xe0
dconst_1        0xf0
ddiv            0x6f
dload           0x18
dload_0         0x26
dload_1         0x27
dload_2         0x28
dload_3         0x29
dmul            0x6b
dneg            0x77
drem            0x73
dreturn         0xaf
dstore          0x39
dstore_0        0x47
dstore_1        0x48
dstore_2        0x49
dstore_3        0x4a
dsub            0x67
dup             0x59
dup_x1          0x5a
dup_x2          0x5b
dup2            0x5c
dup2_x1         0x5d
dup2_x2         0x5e
f2d             0x8d
f2i             0x8b
f2l             0x8c
fadd            0x63
faload          0x30
fastore         0x51
fcmpg           0x96
fcmpl           0x95
fconst_0        0xb0
fconst_1        0xc0
faconst_2       0xd0
fdiv            0x6e
fload           0x17
fload_0         0x22
fload_1         0x23
fload_2         0x24
fload_3         0x25
fmul            0x6a
fneg            0x76
frem            0x72
freturn         0xae
fstore          0x38
fstore_0        0x43
fstore_1        0x44
fstore_2        0x45
fstore_3        0x46
fsub            0x66
getfield        0xb4
getstatic       0xb2
goto            0xa7
goto_w          0xc8
i2b             0x91
i2c             0x92
i2d             0x87
i2f             0x86
i2l             0x45
i2s             0x93
iadd            0x60
iaload          0x2e
iand            0x7e
iastore         0x4f
iconst_m1       0x20
iconst_0        0x30
iconst_1        0x40
iconst_2        0x50
iconst_3        0x60
iconst_4        0x70
iconst_5        0x80
idiv            0x6c
if_acmpeq       0xa5
if_acmpne       0xa6
if_icmpeq       0x9f
if_icmpne       0xa0
if_icmplt       0xa1
if_icmpge       0xa2
if_icmpgt       0xa3
if_icmple       0xa4
ifeq            0x99
ifne            0x9a
iflt            0x9b
ifge            0x9c
ifgt            0x9d
ifle            0x9e
ifnonnull       0xc7
ifnull          0xc6
iinc            0x84
iload           0x15
ilaod_0         0x1a
iload_1         0x1b
iload_2         0x1c
iload_3         0x1d
imul            0x68
ineg            0x74
instanceof      0xc1
invokedynamic   0xba
invokeinterface 0xb9
invokespecial   0xb7
invokestatic    0xb8
invokevirtual   0xb6
ior             0x80
irem            0x70
ireturn         0xac
ishl            0x78
ishr            0x7a
istore          0x36
istore_0        0x3b
istore_1        0x3c
istore_2        0x3d
istore_3        0x3d
isub            0x64
iushr           0x7c
ixor            0x82
jsr             0xa8
jsr_w           0xc9
l2d             0x8a
l2f             0x89
l2i             0x88
ladd            0x61
laload          0x2f
land            0x7f
lastore         0x50
lcmp            0x94
lconst_0        0x90
lconst_1        0xa0
ldc             0x12
ldc_w           0x13
ldc2_w          0x14
ldiv            0x6d
lload           0x16
lload_0         0x1e
lload_1         0x1f
lload_2         0x20
lload_3         0x21
lmul            0x69
lneg            0x75
lookupswitch    0xab
lor             0x81
lrem            0x71
lreturn         0xad
lshl            0x79
lshr            0x7b
lstore          0x37
lstore_0        0x3f
lstore_1        0x40
lstore_2        0x41
lstore_3        0x42
lsub            0x65
lushr           0x7d
lxor            0x83
monitorenter    0xc2
monitorexit     0xc3
multianewarray  0xc5
new             0xbb
newarray        0xbc
nop             0x00
pop             0x57
pop2            0x58
putfield        0xb5
putstatic       0xb3
ret             0xa9
return          0xb1
saload          0x35
sastore         0x56
sipush          0x11
swap            0x5f
tableswitch     0xaa
wide            0xc4
*/
