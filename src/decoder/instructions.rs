use crate::{decoder::BufferedReader, types::instructions::Instruction};

pub fn parse_instruction(op: u8, buffer: &mut BufferedReader) -> Option<Instruction> {
    match op {
        0x32 => Some(Instruction::Aaload),
        0x53 => Some(Instruction::Aastore),
        0x01 => Some(Instruction::AConstNull),
        0x19 => {
            let index = buffer.take::<u8>().unwrap();
            Some(Instruction::Aload(index))
        }
        0x2a => Some(Instruction::Aload0),
        0x2b => Some(Instruction::Aload1),
        0x2c => Some(Instruction::Aload2),
        0x2d => Some(Instruction::Aload3),
        0xbd => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::Anewarray(index))
        }
        0xb0 => Some(Instruction::Areturn),
        0xbe => Some(Instruction::Arraylength),
        0x3a => {
            let index = buffer.take::<u8>().unwrap();
            Some(Instruction::Astore(index))
        }
        0x4b => Some(Instruction::Astore0),
        0x4c => Some(Instruction::Astore1),
        0x4d => Some(Instruction::Astore2),
        0x4e => Some(Instruction::Astore3),
        0xbf => Some(Instruction::Athrow),
        0x33 => Some(Instruction::Baload),
        0x54 => Some(Instruction::Bastore),
        0x10 => {
            let index = buffer.take::<u8>().unwrap();
            Some(Instruction::Bipush(index))
        }
        0x34 => Some(Instruction::Caload),
        0x55 => Some(Instruction::Castore),
        0xc0 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::Checkcast(index))
        }
        0x90 => Some(Instruction::D2f),
        0x8e => Some(Instruction::D2i),
        0x8f => Some(Instruction::D2l),
        0x63 => Some(Instruction::Dadd),
        0x31 => Some(Instruction::Daload),
        0x52 => Some(Instruction::Dastore),
        0x98 => Some(Instruction::Dcmpg),
        0x97 => Some(Instruction::Dcmpl),
        0x0e => Some(Instruction::Dconst0),
        0x0f => Some(Instruction::Dconst1),
        0x6f => Some(Instruction::Ddiv),
        0x18 => {
            let index = buffer.take::<u8>().unwrap();
            Some(Instruction::Dload(index))
        }
        0x26 => Some(Instruction::Dload0),
        0x27 => Some(Instruction::Dload1),
        0x28 => Some(Instruction::Dload2),
        0x29 => Some(Instruction::Dload3),
        0x6b => Some(Instruction::Dmul),
        0x77 => Some(Instruction::Dneg),
        0x73 => Some(Instruction::Drem),
        0xaf => Some(Instruction::Dreturn),
        0x39 => {
            let index = buffer.take::<u8>().unwrap();
            Some(Instruction::Dstore(index))
        }
        0x47 => Some(Instruction::Dstore0),
        0x48 => Some(Instruction::Dstore1),
        0x49 => Some(Instruction::Dstore2),
        0x4a => Some(Instruction::Dstore3),
        0x67 => Some(Instruction::Dsub),
        0x59 => Some(Instruction::Dup),
        0x5a => Some(Instruction::DupX1),
        0x5b => Some(Instruction::DupX2),
        0x5c => Some(Instruction::Dup2),
        0x5d => Some(Instruction::Dup2X1),
        0x5e => Some(Instruction::Dup2X2),
        0x8d => Some(Instruction::F2d),
        0x8b => Some(Instruction::F2i),
        0x8c => Some(Instruction::F2l),
        0x62 => Some(Instruction::Fadd),
        0x30 => Some(Instruction::Faload),
        0x51 => Some(Instruction::Fastore),
        0x96 => Some(Instruction::Fcmpg),
        0x95 => Some(Instruction::Fcmpl),
        0x0b => Some(Instruction::Fconst0),
        0x0c => Some(Instruction::Fconst1),
        0x0d => Some(Instruction::Fconst2),
        0x6e => Some(Instruction::Fdiv),
        0x17 => {
            let index = buffer.take::<u8>().unwrap();
            Some(Instruction::Fload(index))
        }
        0x22 => Some(Instruction::Fload0),
        0x23 => Some(Instruction::Fload1),
        0x24 => Some(Instruction::Fload2),
        0x25 => Some(Instruction::Fload3),
        0x6a => Some(Instruction::Fmul),
        0x76 => Some(Instruction::Fneg),
        0x72 => Some(Instruction::Frem),
        0xae => Some(Instruction::Freturn),
        0x38 => {
            let index = buffer.take::<u8>().unwrap();
            Some(Instruction::Fstore(index))
        }
        0x43 => Some(Instruction::Fstore0),
        0x44 => Some(Instruction::Fstore1),
        0x45 => Some(Instruction::Fstore2),
        0x46 => Some(Instruction::Fstore3),
        0x66 => Some(Instruction::Fsub),
        0xb4 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::Getfield(index))
        }
        0xb2 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::Getstatic(index))
        }
        0xa7 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::Goto(index))
        }
        0xc8 => {
            // let index = buffer.take::<u16>().unwrap();
            // Some(Instruction::GotoW(index))
            todo!("GotoW")
        }
        0x91 => Some(Instruction::I2b),
        0x92 => Some(Instruction::I2c),
        0x87 => Some(Instruction::I2d),
        0x86 => Some(Instruction::I2f),
        0x85 => Some(Instruction::I2l),
        0x93 => Some(Instruction::I2s),
        0x60 => Some(Instruction::Iadd),
        0x2e => Some(Instruction::Iaload),
        0x7e => Some(Instruction::Iand),
        0x4f => Some(Instruction::Iastore),
        0x02 => Some(Instruction::IconstM1),
        0x03 => Some(Instruction::Iconst0),
        0x04 => Some(Instruction::Iconst1),
        0x05 => Some(Instruction::Iconst2),
        0x06 => Some(Instruction::Iconst3),
        0x07 => Some(Instruction::Iconst4),
        0x08 => Some(Instruction::Iconst5),
        0x6c => Some(Instruction::Idiv),
        0xa5 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::IfAcmpeq(index))
        }
        0xa6 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::IfAcmpne(index))
        }
        0x9f => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::IfIcmpeq(index))
        }
        0xa0 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::IfIcmpne(index))
        }
        0xa1 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::IfIcmplt(index))
        }
        0xa2 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::IfIcmpge(index))
        }
        0xa3 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::IfIcmpgt(index))
        }
        0xa4 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::IfIcmple(index))
        }
        0x99 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::Ifeq(index))
        }
        0x9a => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::Ifne(index))
        }
        0x9b => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::Iflt(index))
        }
        0x9c => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::Ifge(index))
        }
        0x9d => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::Ifgt(index))
        }
        0x9e => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::Ifle(index))
        }
        0xc7 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::Ifnonnull(index))
        }
        0xc6 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::Ifnull(index))
        }
        0x84 => {
            let index = buffer.take::<u8>().unwrap();
            let offset = buffer.take::<i8>().unwrap();
            Some(Instruction::Iinc(index, offset))
        }
        0x15 => {
            let index = buffer.take::<u8>().unwrap();
            Some(Instruction::Iload(index))
        }
        0x1a => Some(Instruction::Iload0),
        0x1b => Some(Instruction::Iload1),
        0x1c => Some(Instruction::Iload2),
        0x1d => Some(Instruction::Iload3),
        0x68 => Some(Instruction::Imul),
        0x74 => Some(Instruction::Ineg),
        0xc1 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::Instanceof(index))
        }
        0xba => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::Invokedynamic(index))
        }
        0xb9 => {
            let index = buffer.take::<u16>().unwrap();
            let count = buffer.take::<u8>().unwrap();
            Some(Instruction::Invokeinterface(index, count))
        }
        0xb7 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::Invokespecial(index))
        }
        0xb8 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::Invokestatic(index))
        }
        0xb6 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::Invokevirtual(index))
        }
        0x80 => Some(Instruction::Ior),
        0x70 => Some(Instruction::Irem),
        0xac => Some(Instruction::Ireturn),
        0x78 => Some(Instruction::Ishl),
        0x7a => Some(Instruction::Ishr),
        0x36 => {
            let index = buffer.take::<u8>().unwrap();
            Some(Instruction::Istore(index))
        }
        0x3b => Some(Instruction::Istore0),
        0x3c => Some(Instruction::Istore1),
        0x3d => Some(Instruction::Istore2),
        0x3e => Some(Instruction::Istore3),
        0x64 => Some(Instruction::Isub),
        0x7c => Some(Instruction::Iushr),
        0x82 => Some(Instruction::Ixor),
        0xa8 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::Jsr(index))
        }
        0xc9 => Some(Instruction::JsrW),
        0x8a => Some(Instruction::L2d),
        0x89 => Some(Instruction::L2f),
        0x88 => Some(Instruction::L2i),
        0x61 => Some(Instruction::Ladd),
        0x2f => Some(Instruction::Laload),
        0x7f => Some(Instruction::Land),
        0x50 => Some(Instruction::Lastore),
        0x94 => Some(Instruction::Lcmp),
        0x09 => Some(Instruction::Lconst0),
        0x0a => Some(Instruction::Lconst1),
        0x12 => {
            let index = buffer.take::<u8>().unwrap();
            Some(Instruction::Ldc(index))
        }
        0x13 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::LdcW(index))
        }
        0x14 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::Ldc2W(index))
        }
        0x6d => Some(Instruction::Ldiv),
        0x16 => {
            let index = buffer.take::<u8>().unwrap();
            Some(Instruction::Lload(index))
        }
        0x1e => Some(Instruction::Lload0),
        0x1f => Some(Instruction::Lload1),
        0x20 => Some(Instruction::Lload2),
        0x21 => Some(Instruction::Lload3),
        0x69 => Some(Instruction::Lmul),
        0x75 => Some(Instruction::Lneg),
        0xab => todo!("lookupswitch"),
        0x81 => Some(Instruction::Lor),
        0x71 => Some(Instruction::Lrem),
        0xad => Some(Instruction::Lreturn),
        0x79 => Some(Instruction::Lshl),
        0x7b => Some(Instruction::Lshr),
        0x37 => {
            let index = buffer.take::<u8>().unwrap();
            Some(Instruction::Lstore(index))
        }
        0x3f => Some(Instruction::Lstore0),
        0x40 => Some(Instruction::Lstore1),
        0x41 => Some(Instruction::Lstore2),
        0x42 => Some(Instruction::Lstore3),
        0x65 => Some(Instruction::Lsub),
        0x7d => Some(Instruction::Lushr),
        0x83 => Some(Instruction::Lxor),
        0xc2 => Some(Instruction::Monitorenter),
        0xc3 => Some(Instruction::Monitorexit),
        0xc5 => {
            let index = buffer.take::<u16>().unwrap();
            let count = buffer.take::<u8>().unwrap();
            Some(Instruction::Multianewarray(index, count))
        }
        0xbb => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::New(index))
        }
        0xbc => todo!("newarray"),
        0x00 => Some(Instruction::Nop),
        0x57 => Some(Instruction::Pop),
        0x58 => Some(Instruction::Pop2),
        0xb5 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::Putfield(index))
        }
        0xb3 => {
            let index = buffer.take::<u16>().unwrap();
            Some(Instruction::Putstatic(index))
        }
        0xa9 => {
            let index = buffer.take::<u8>().unwrap();
            Some(Instruction::Ret(index))
        }
        0xb1 => Some(Instruction::Return),
        0x35 => Some(Instruction::Saload),
        0x56 => Some(Instruction::Sastore),
        0x11 => {
            let index = buffer.take::<i16>().unwrap();
            Some(Instruction::Sipush(index))
        }
        0x5f => Some(Instruction::Swap),
        0xaa => todo!("tableswitch"),
        0xc4 => todo!("wide"),
        _ => None,
    }
}
