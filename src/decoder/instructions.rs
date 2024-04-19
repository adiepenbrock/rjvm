use crate::bytecode::reader::BufferedReader;
use crate::bytecode::BytecodeError;
use crate::types::instructions::Instruction;

pub fn parse_instruction(
    op: u8,
    buffer: &mut BufferedReader,
) -> Result<Instruction, BytecodeError> {
    match op {
        0x32 => Ok(Instruction::Aaload),
        0x53 => Ok(Instruction::Aastore),
        0x01 => Ok(Instruction::AConstNull),
        0x19 => {
            let index = buffer.take::<u8>()?;
            Ok(Instruction::Aload(index))
        }
        0x2a => Ok(Instruction::Aload0),
        0x2b => Ok(Instruction::Aload1),
        0x2c => Ok(Instruction::Aload2),
        0x2d => Ok(Instruction::Aload3),
        0xbd => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::Anewarray(index))
        }
        0xb0 => Ok(Instruction::Areturn),
        0xbe => Ok(Instruction::Arraylength),
        0x3a => {
            let index = buffer.take::<u8>()?;
            Ok(Instruction::Astore(index))
        }
        0x4b => Ok(Instruction::Astore0),
        0x4c => Ok(Instruction::Astore1),
        0x4d => Ok(Instruction::Astore2),
        0x4e => Ok(Instruction::Astore3),
        0xbf => Ok(Instruction::Athrow),
        0x33 => Ok(Instruction::Baload),
        0x54 => Ok(Instruction::Bastore),
        0x10 => {
            let index = buffer.take::<u8>()?;
            Ok(Instruction::Bipush(index))
        }
        0x34 => Ok(Instruction::Caload),
        0x55 => Ok(Instruction::Castore),
        0xc0 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::Checkcast(index))
        }
        0x90 => Ok(Instruction::D2f),
        0x8e => Ok(Instruction::D2i),
        0x8f => Ok(Instruction::D2l),
        0x63 => Ok(Instruction::Dadd),
        0x31 => Ok(Instruction::Daload),
        0x52 => Ok(Instruction::Dastore),
        0x98 => Ok(Instruction::Dcmpg),
        0x97 => Ok(Instruction::Dcmpl),
        0x0e => Ok(Instruction::Dconst0),
        0x0f => Ok(Instruction::Dconst1),
        0x6f => Ok(Instruction::Ddiv),
        0x18 => {
            let index = buffer.take::<u8>()?;
            Ok(Instruction::Dload(index))
        }
        0x26 => Ok(Instruction::Dload0),
        0x27 => Ok(Instruction::Dload1),
        0x28 => Ok(Instruction::Dload2),
        0x29 => Ok(Instruction::Dload3),
        0x6b => Ok(Instruction::Dmul),
        0x77 => Ok(Instruction::Dneg),
        0x73 => Ok(Instruction::Drem),
        0xaf => Ok(Instruction::Dreturn),
        0x39 => {
            let index = buffer.take::<u8>()?;
            Ok(Instruction::Dstore(index))
        }
        0x47 => Ok(Instruction::Dstore0),
        0x48 => Ok(Instruction::Dstore1),
        0x49 => Ok(Instruction::Dstore2),
        0x4a => Ok(Instruction::Dstore3),
        0x67 => Ok(Instruction::Dsub),
        0x59 => Ok(Instruction::Dup),
        0x5a => Ok(Instruction::DupX1),
        0x5b => Ok(Instruction::DupX2),
        0x5c => Ok(Instruction::Dup2),
        0x5d => Ok(Instruction::Dup2X1),
        0x5e => Ok(Instruction::Dup2X2),
        0x8d => Ok(Instruction::F2d),
        0x8b => Ok(Instruction::F2i),
        0x8c => Ok(Instruction::F2l),
        0x62 => Ok(Instruction::Fadd),
        0x30 => Ok(Instruction::Faload),
        0x51 => Ok(Instruction::Fastore),
        0x96 => Ok(Instruction::Fcmpg),
        0x95 => Ok(Instruction::Fcmpl),
        0x0b => Ok(Instruction::Fconst0),
        0x0c => Ok(Instruction::Fconst1),
        0x0d => Ok(Instruction::Fconst2),
        0x6e => Ok(Instruction::Fdiv),
        0x17 => {
            let index = buffer.take::<u8>()?;
            Ok(Instruction::Fload(index))
        }
        0x22 => Ok(Instruction::Fload0),
        0x23 => Ok(Instruction::Fload1),
        0x24 => Ok(Instruction::Fload2),
        0x25 => Ok(Instruction::Fload3),
        0x6a => Ok(Instruction::Fmul),
        0x76 => Ok(Instruction::Fneg),
        0x72 => Ok(Instruction::Frem),
        0xae => Ok(Instruction::Freturn),
        0x38 => {
            let index = buffer.take::<u8>()?;
            Ok(Instruction::Fstore(index))
        }
        0x43 => Ok(Instruction::Fstore0),
        0x44 => Ok(Instruction::Fstore1),
        0x45 => Ok(Instruction::Fstore2),
        0x46 => Ok(Instruction::Fstore3),
        0x66 => Ok(Instruction::Fsub),
        0xb4 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::Getfield(index))
        }
        0xb2 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::Getstatic(index))
        }
        0xa7 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::Goto(index))
        }
        0xc8 => {
            // let index = buffer.take::<u16>()?;
            // Ok(Instruction::GotoW(index))
            todo!("GotoW")
        }
        0x91 => Ok(Instruction::I2b),
        0x92 => Ok(Instruction::I2c),
        0x87 => Ok(Instruction::I2d),
        0x86 => Ok(Instruction::I2f),
        0x85 => Ok(Instruction::I2l),
        0x93 => Ok(Instruction::I2s),
        0x60 => Ok(Instruction::Iadd),
        0x2e => Ok(Instruction::Iaload),
        0x7e => Ok(Instruction::Iand),
        0x4f => Ok(Instruction::Iastore),
        0x02 => Ok(Instruction::IconstM1),
        0x03 => Ok(Instruction::Iconst0),
        0x04 => Ok(Instruction::Iconst1),
        0x05 => Ok(Instruction::Iconst2),
        0x06 => Ok(Instruction::Iconst3),
        0x07 => Ok(Instruction::Iconst4),
        0x08 => Ok(Instruction::Iconst5),
        0x6c => Ok(Instruction::Idiv),
        0xa5 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::IfAcmpeq(index))
        }
        0xa6 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::IfAcmpne(index))
        }
        0x9f => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::IfIcmpeq(index))
        }
        0xa0 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::IfIcmpne(index))
        }
        0xa1 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::IfIcmplt(index))
        }
        0xa2 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::IfIcmpge(index))
        }
        0xa3 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::IfIcmpgt(index))
        }
        0xa4 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::IfIcmple(index))
        }
        0x99 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::Ifeq(index))
        }
        0x9a => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::Ifne(index))
        }
        0x9b => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::Iflt(index))
        }
        0x9c => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::Ifge(index))
        }
        0x9d => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::Ifgt(index))
        }
        0x9e => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::Ifle(index))
        }
        0xc7 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::Ifnonnull(index))
        }
        0xc6 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::Ifnull(index))
        }
        0x84 => {
            let index = buffer.take::<u8>()?;
            let offset = buffer.take::<i8>()?;
            Ok(Instruction::Iinc(index, offset))
        }
        0x15 => {
            let index = buffer.take::<u8>()?;
            Ok(Instruction::Iload(index))
        }
        0x1a => Ok(Instruction::Iload0),
        0x1b => Ok(Instruction::Iload1),
        0x1c => Ok(Instruction::Iload2),
        0x1d => Ok(Instruction::Iload3),
        0x68 => Ok(Instruction::Imul),
        0x74 => Ok(Instruction::Ineg),
        0xc1 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::Instanceof(index))
        }
        0xba => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::Invokedynamic(index))
        }
        0xb9 => {
            let index = buffer.take::<u16>()?;
            let count = buffer.take::<u8>()?;
            Ok(Instruction::Invokeinterface(index, count))
        }
        0xb7 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::Invokespecial(index))
        }
        0xb8 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::Invokestatic(index))
        }
        0xb6 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::Invokevirtual(index))
        }
        0x80 => Ok(Instruction::Ior),
        0x70 => Ok(Instruction::Irem),
        0xac => Ok(Instruction::Ireturn),
        0x78 => Ok(Instruction::Ishl),
        0x7a => Ok(Instruction::Ishr),
        0x36 => {
            let index = buffer.take::<u8>()?;
            Ok(Instruction::Istore(index))
        }
        0x3b => Ok(Instruction::Istore0),
        0x3c => Ok(Instruction::Istore1),
        0x3d => Ok(Instruction::Istore2),
        0x3e => Ok(Instruction::Istore3),
        0x64 => Ok(Instruction::Isub),
        0x7c => Ok(Instruction::Iushr),
        0x82 => Ok(Instruction::Ixor),
        0xa8 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::Jsr(index))
        }
        0xc9 => Ok(Instruction::JsrW),
        0x8a => Ok(Instruction::L2d),
        0x89 => Ok(Instruction::L2f),
        0x88 => Ok(Instruction::L2i),
        0x61 => Ok(Instruction::Ladd),
        0x2f => Ok(Instruction::Laload),
        0x7f => Ok(Instruction::Land),
        0x50 => Ok(Instruction::Lastore),
        0x94 => Ok(Instruction::Lcmp),
        0x09 => Ok(Instruction::Lconst0),
        0x0a => Ok(Instruction::Lconst1),
        0x12 => {
            let index = buffer.take::<u8>()?;
            Ok(Instruction::Ldc(index))
        }
        0x13 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::LdcW(index))
        }
        0x14 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::Ldc2W(index))
        }
        0x6d => Ok(Instruction::Ldiv),
        0x16 => {
            let index = buffer.take::<u8>()?;
            Ok(Instruction::Lload(index))
        }
        0x1e => Ok(Instruction::Lload0),
        0x1f => Ok(Instruction::Lload1),
        0x20 => Ok(Instruction::Lload2),
        0x21 => Ok(Instruction::Lload3),
        0x69 => Ok(Instruction::Lmul),
        0x75 => Ok(Instruction::Lneg),
        0xab => todo!("lookupswitch"),
        0x81 => Ok(Instruction::Lor),
        0x71 => Ok(Instruction::Lrem),
        0xad => Ok(Instruction::Lreturn),
        0x79 => Ok(Instruction::Lshl),
        0x7b => Ok(Instruction::Lshr),
        0x37 => {
            let index = buffer.take::<u8>()?;
            Ok(Instruction::Lstore(index))
        }
        0x3f => Ok(Instruction::Lstore0),
        0x40 => Ok(Instruction::Lstore1),
        0x41 => Ok(Instruction::Lstore2),
        0x42 => Ok(Instruction::Lstore3),
        0x65 => Ok(Instruction::Lsub),
        0x7d => Ok(Instruction::Lushr),
        0x83 => Ok(Instruction::Lxor),
        0xc2 => Ok(Instruction::Monitorenter),
        0xc3 => Ok(Instruction::Monitorexit),
        0xc5 => {
            let index = buffer.take::<u16>()?;
            let count = buffer.take::<u8>()?;
            Ok(Instruction::Multianewarray(index, count))
        }
        0xbb => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::New(index))
        }
        0xbc => todo!("newarray"),
        0x00 => Ok(Instruction::Nop),
        0x57 => Ok(Instruction::Pop),
        0x58 => Ok(Instruction::Pop2),
        0xb5 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::Putfield(index))
        }
        0xb3 => {
            let index = buffer.take::<u16>()?;
            Ok(Instruction::Putstatic(index))
        }
        0xa9 => {
            let index = buffer.take::<u8>()?;
            Ok(Instruction::Ret(index))
        }
        0xb1 => Ok(Instruction::Return),
        0x35 => Ok(Instruction::Saload),
        0x56 => Ok(Instruction::Sastore),
        0x11 => {
            let index = buffer.take::<i16>()?;
            Ok(Instruction::Sipush(index))
        }
        0x5f => Ok(Instruction::Swap),
        0xaa => todo!("tableswitch"),
        0xc4 => todo!("wide"),
        _ => Err(BytecodeError::UnsupportedInstruction),
    }
}
