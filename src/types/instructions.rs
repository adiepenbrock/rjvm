#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Aaload,
    Aastore,
    AConstNull,
    Aload(u8),
    Aload0,
    Aload1,
    Aload2,
    Aload3,
    Anewarray(u16),
    Areturn,
    Arraylength,
    Astore(u8),
    Astore0,
    Astore1,
    Astore2,
    Astore3,
    Athrow,
    Baload,
    Bastore,
    Bipush(u8),
    Caload,
    Castore,
    Checkcast(u16),
    D2f,
    D2i,
    D2l,
    Dadd,
    Daload,
    Dastore,
    Dcmpg,
    Dcmpl,
    Dconst0,
    Dconst1,
    Ddiv,
    Dload(u8),
    Dload0,
    Dload1,
    Dload2,
    Dload3,
    Dmul,
    Dneg,
    Drem,
    Dreturn,
    Dstore(u8),
    Dstore0,
    Dstore1,
    Dstore2,
    Dstore3,
    Dsub,
    Dup,
    DupX1,
    DupX2,
    Dup2,
    Dup2X1,
    Dup2X2,
    F2d,
    F2i,
    F2l,
    Fadd,
    Faload,
    Fastore,
    Fcmpg,
    Fcmpl,
    Fconst0,
    Fconst1,
    Fconst2,
    Fdiv,
    Fload(u8),
    Fload0,
    Fload1,
    Fload2,
    Fload3,
    Fmul,
    Fneg,
    Frem,
    Freturn,
    Fstore(u8),
    Fstore0,
    Fstore1,
    Fstore2,
    Fstore3,
    Fsub,
    Getfield(u16),
    Getstatic(u16),
    Goto(u16),
    GotoW,
    I2b,
    I2c,
    I2d,
    I2f,
    I2l,
    I2s,
    Iadd,
    Iaload,
    Iand,
    Iastore,
    IconstM1,
    Iconst0,
    Iconst1,
    Iconst2,
    Iconst3,
    Iconst4,
    Iconst5,
    Idiv,
    IfAcmpeq(u16),
    IfAcmpne(u16),
    IfIcmpeq(u16),
    IfIcmpne(u16),
    IfIcmplt(u16),
    IfIcmpge(u16),
    IfIcmpgt(u16),
    IfIcmple(u16),
    Ifeq(u16),
    Ifne(u16),
    Iflt(u16),
    Ifge(u16),
    Ifgt(u16),
    Ifle(u16),
    Ifnonnull(u16),
    Ifnull(u16),
    Iinc(u8, i8),
    Iload(u8),
    Iload0,
    Iload1,
    Iload2,
    Iload3,
    Imul,
    Ineg,
    Instanceof(u16),
    Invokedynamic(u16),
    Invokeinterface(u16, u8),
    Invokespecial(u16),
    Invokestatic(u16),
    Invokevirtual(u16),
    Ior,
    Irem,
    Ireturn,
    Ishl,
    Ishr,
    Istore(u8),
    Istore0,
    Istore1,
    Istore2,
    Istore3,
    Isub,
    Iushr,
    Ixor,
    Jsr(u16),
    JsrW,
    L2d,
    L2f,
    L2i,
    Ladd,
    Laload,
    Land,
    Lastore,
    Lcmp,
    Lconst0,
    Lconst1,
    Ldc(u8),
    LdcW(u16),
    Ldc2W(u16),
    Ldiv,
    Lload(u8),
    Lload0,
    Lload1,
    Lload2,
    Lload3,
    Lmul,
    Lneg,
    Lookupswitch,
    Lor,
    Lrem,
    Lreturn,
    Lshl,
    Lshr,
    Lstore(u8),
    Lstore0,
    Lstore1,
    Lstore2,
    Lstore3,
    Lsub,
    Lushr,
    Lxor,
    Monitorenter,
    Monitorexit,
    Multianewarray(u16, u8),
    New(u16),
    Newarray(u8), // TODO???
    Nop,
    Pop,
    Pop2,
    Putfield(u16),
    Putstatic(u16),
    Ret(u8),
    Return,
    Saload,
    Sastore,
    Sipush(i16),
    Swap,
    Tableswitch,
    Wide,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Aaload => write!(f, "aaload"),
            Instruction::Aastore => write!(f, "aastore"),
            Instruction::AConstNull => write!(f, "aconst_null"),
            Instruction::Aload(index) => write!(f, "aload {}", index),
            Instruction::Aload0 => write!(f, "aload_0"),
            Instruction::Aload1 => write!(f, "aload_1"),
            Instruction::Aload2 => write!(f, "aload_2"),
            Instruction::Aload3 => write!(f, "aload_3"),
            Instruction::Anewarray(index) => write!(f, "anewarray {}", index),
            Instruction::Areturn => write!(f, "areturn"),
            Instruction::Arraylength => write!(f, "arraylength"),
            Instruction::Astore(index) => write!(f, "astore {}", index),
            Instruction::Astore0 => write!(f, "astore_0"),
            Instruction::Astore1 => write!(f, "astore_1"),
            Instruction::Astore2 => write!(f, "astore_2"),
            Instruction::Astore3 => write!(f, "astore_3"),
            Instruction::Athrow => write!(f, "athrow"),
            Instruction::Baload => write!(f, "baload"),
            Instruction::Bastore => write!(f, "bastore"),
            Instruction::Bipush(value) => write!(f, "bipush {}", value),
            Instruction::Caload => write!(f, "caload"),
            Instruction::Castore => write!(f, "castore"),
            Instruction::Checkcast(index) => write!(f, "checkcast {}", index),
            Instruction::D2f => write!(f, "d2f"),
            Instruction::D2i => write!(f, "d2i"),
            Instruction::D2l => write!(f, "d2l"),
            Instruction::Dadd => write!(f, "dadd"),
            Instruction::Daload => write!(f, "daload"),
            Instruction::Dastore => write!(f, "dastore"),
            Instruction::Dcmpg => write!(f, "dcmpg"),
            Instruction::Dcmpl => write!(f, "dcmpl"),
            Instruction::Dconst0 => write!(f, "dconst_0"),
            Instruction::Dconst1 => write!(f, "dconst_1"),
            Instruction::Ddiv => write!(f, "ddiv"),
            Instruction::Dload(index) => write!(f, "dload {}", index),
            Instruction::Dload0 => write!(f, "dload_0"),
            Instruction::Dload1 => write!(f, "dload_1"),
            Instruction::Dload2 => write!(f, "dload_2"),
            Instruction::Dload3 => write!(f, "dload_3"),
            Instruction::Dmul => write!(f, "dmul"),
            Instruction::Dneg => write!(f, "dneg"),
            Instruction::Drem => write!(f, "drem"),
            Instruction::Dreturn => write!(f, "dreturn"),
            Instruction::Dstore(index) => write!(f, "dstore {}", index),
            Instruction::Dstore0 => write!(f, "dstore_0"),
            Instruction::Dstore1 => write!(f, "dstore_1"),
            Instruction::Dstore2 => write!(f, "dstore_2"),
            Instruction::Dstore3 => write!(f, "dstore_3"),
            Instruction::Dsub => write!(f, "dsub"),
            Instruction::Dup => write!(f, "dup"),
            Instruction::DupX1 => write!(f, "dup_x1"),
            Instruction::DupX2 => write!(f, "dup_x2"),
            Instruction::Dup2 => write!(f, "dup2"),
            Instruction::Dup2X1 => write!(f, "dup2_x1"),
            Instruction::Dup2X2 => write!(f, "dup2_x2"),
            Instruction::F2d => write!(f, "f2d"),
            Instruction::F2i => write!(f, "f2i"),
            Instruction::F2l => write!(f, "f2l"),
            Instruction::Fadd => write!(f, "fadd"),
            Instruction::Faload => write!(f, "faload"),
            Instruction::Fastore => write!(f, "fastore"),
            Instruction::Fcmpg => write!(f, "fcmpg"),
            Instruction::Fcmpl => write!(f, "fcmpl"),
            Instruction::Fconst0 => write!(f, "fconst_0"),
            Instruction::Fconst1 => write!(f, "fconst_1"),
            Instruction::Fconst2 => write!(f, "fconst_2"),
            Instruction::Fdiv => write!(f, "fdiv"),
            Instruction::Fload(index) => write!(f, "fload {}", index),
            Instruction::Fload0 => write!(f, "fload_0"),
            Instruction::Fload1 => write!(f, "fload_1"),
            Instruction::Fload2 => write!(f, "fload_2"),
            Instruction::Fload3 => write!(f, "fload_3"),
            Instruction::Fmul => write!(f, "fmul"),
            Instruction::Fneg => write!(f, "fneg"),
            Instruction::Frem => write!(f, "frem"),
            Instruction::Freturn => write!(f, "freturn"),
            Instruction::Fstore(index) => write!(f, "fstore {}", index),
            Instruction::Fstore0 => write!(f, "fstore_0"),
            Instruction::Fstore1 => write!(f, "fstore_1"),
            Instruction::Fstore2 => write!(f, "fstore_2"),
            Instruction::Fstore3 => write!(f, "fstore_3"),
            Instruction::Fsub => write!(f, "fsub"),
            Instruction::Getfield(index) => write!(f, "getfield {}", index),
            Instruction::Getstatic(index) => write!(f, "getstatic {}", index),
            Instruction::Goto(offset) => write!(f, "goto {}", offset),
            Instruction::GotoW => write!(f, "goto_w"),
            Instruction::I2b => write!(f, "i2b"),
            Instruction::I2c => write!(f, "i2c"),
            Instruction::I2d => write!(f, "i2d"),
            Instruction::I2f => write!(f, "i2f"),
            Instruction::I2l => write!(f, "i2l"),
            Instruction::I2s => write!(f, "i2s"),
            Instruction::Iadd => write!(f, "iadd"),
            Instruction::Iaload => write!(f, "iaload"),
            Instruction::Iand => write!(f, "iand"),
            Instruction::Iastore => write!(f, "iastore"),
            Instruction::IconstM1 => write!(f, "iconst_m1"),
            Instruction::Iconst0 => write!(f, "iconst_0"),
            Instruction::Iconst1 => write!(f, "iconst_1"),
            Instruction::Iconst2 => write!(f, "iconst_2"),
            Instruction::Iconst3 => write!(f, "iconst_3"),
            Instruction::Iconst4 => write!(f, "iconst_4"),
            Instruction::Iconst5 => write!(f, "iconst_5"),
            Instruction::Idiv => write!(f, "idiv"),
            Instruction::IfAcmpeq(offset) => write!(f, "if_acmpeq {}", offset),
            Instruction::IfAcmpne(offset) => write!(f, "if_acmpne {}", offset),
            Instruction::IfIcmpeq(offset) => write!(f, "if_icmpeq {}", offset),
            Instruction::IfIcmpne(offset) => write!(f, "if_icmpne {}", offset),
            Instruction::IfIcmplt(offset) => write!(f, "if_icmplt {}", offset),
            Instruction::IfIcmpge(offset) => write!(f, "if_icmpge {}", offset),
            Instruction::IfIcmpgt(offset) => write!(f, "if_icmpgt {}", offset),
            Instruction::IfIcmple(offset) => write!(f, "if_icmple {}", offset),
            Instruction::Ifeq(offset) => write!(f, "ifeq {}", offset),
            Instruction::Ifne(offset) => write!(f, "ifne {}", offset),
            Instruction::Iflt(offset) => write!(f, "iflt {}", offset),
            Instruction::Ifge(offset) => write!(f, "ifge {}", offset),
            Instruction::Ifgt(offset) => write!(f, "ifgt {}", offset),
            Instruction::Ifle(offset) => write!(f, "ifle {}", offset),
            Instruction::Ifnonnull(offset) => write!(f, "ifnonnull {}", offset),
            Instruction::Ifnull(offset) => write!(f, "ifnull {}", offset),
            Instruction::Iinc(index, value) => write!(f, "iinc {} {}", index, value),
            Instruction::Iload(index) => write!(f, "iload {}", index),
            Instruction::Iload0 => write!(f, "iload_0"),
            Instruction::Iload1 => write!(f, "iload_1"),
            Instruction::Iload2 => write!(f, "iload_2"),
            Instruction::Iload3 => write!(f, "iload_3"),
            Instruction::Imul => write!(f, "imul"),
            Instruction::Ineg => write!(f, "ineg"),
            Instruction::Instanceof(index) => write!(f, "instanceof {}", index),
            Instruction::Invokedynamic(index) => write!(f, "invokedynamic {}", index),
            Instruction::Invokeinterface(index, offset) => {
                write!(f, "invokeinterface {} {}", index, offset)
            }
            Instruction::Invokespecial(index) => write!(f, "invokespecial {}", index),
            Instruction::Invokestatic(index) => write!(f, "invokestatic {}", index),
            Instruction::Invokevirtual(index) => write!(f, "invokevirtual {}", index),
            Instruction::Ior => write!(f, "ior"),
            Instruction::Irem => write!(f, "irem"),
            Instruction::Ireturn => write!(f, "ireturn"),
            Instruction::Ishl => write!(f, "ishl"),
            Instruction::Ishr => write!(f, "ishr"),
            Instruction::Istore(index) => write!(f, "istore {}", index),
            Instruction::Istore0 => write!(f, "istore_0"),
            Instruction::Istore1 => write!(f, "istore_1"),
            Instruction::Istore2 => write!(f, "istore_2"),
            Instruction::Istore3 => write!(f, "istore_3"),
            Instruction::Isub => write!(f, "isub"),
            Instruction::Iushr => write!(f, "iushr"),
            Instruction::Ixor => write!(f, "ixor"),
            Instruction::Jsr(offset) => write!(f, "jsr {}", offset),
            Instruction::JsrW => write!(f, "jsr_w"),
            Instruction::L2d => write!(f, "l2d"),
            Instruction::L2f => write!(f, "l2f"),
            Instruction::L2i => write!(f, "l2i"),
            Instruction::Ladd => write!(f, "ladd"),
            Instruction::Laload => write!(f, "laload"),
            Instruction::Land => write!(f, "land"),
            Instruction::Lastore => write!(f, "lastore"),
            Instruction::Lcmp => write!(f, "lcmp"),
            Instruction::Lconst0 => write!(f, "lconst_0"),
            Instruction::Lconst1 => write!(f, "lconst_1"),
            Instruction::Ldc(index) => write!(f, "ldc {}", index),
            Instruction::LdcW(index) => write!(f, "ldc_w {}", index),
            Instruction::Ldc2W(index) => write!(f, "ldc2_w {}", index),
            Instruction::Ldiv => write!(f, "ldiv"),
            Instruction::Lload(index) => write!(f, "lload {}", index),
            Instruction::Lload0 => write!(f, "lload_0"),
            Instruction::Lload1 => write!(f, "lload_1"),
            Instruction::Lload2 => write!(f, "lload_2"),
            Instruction::Lload3 => write!(f, "lload_3"),
            Instruction::Lmul => write!(f, "lmul"),
            Instruction::Lneg => write!(f, "lneg"),
            Instruction::Lookupswitch => write!(f, "lookupswitch"),
            Instruction::Lor => write!(f, "lor"),
            Instruction::Lrem => write!(f, "lrem"),
            Instruction::Lreturn => write!(f, "lreturn"),
            Instruction::Lshl => write!(f, "lshl"),
            Instruction::Lshr => write!(f, "lshr"),
            Instruction::Lstore(index) => write!(f, "lstore {}", index),
            Instruction::Lstore0 => write!(f, "lstore_0"),
            Instruction::Lstore1 => write!(f, "lstore_1"),
            Instruction::Lstore2 => write!(f, "lstore_2"),
            Instruction::Lstore3 => write!(f, "lstore_3"),
            Instruction::Lsub => write!(f, "lsub"),
            Instruction::Lushr => write!(f, "lushr"),
            Instruction::Lxor => write!(f, "lxor"),
            Instruction::Monitorenter => write!(f, "monitorenter"),
            Instruction::Monitorexit => write!(f, "monitorexit"),
            Instruction::Multianewarray(index, dimensions) => {
                write!(f, "multianewarray {} {}", index, dimensions)
            }
            Instruction::New(index) => write!(f, "new {}", index),
            Instruction::Newarray(index) => write!(f, "newarray {}", index),
            Instruction::Nop => write!(f, "nop"),
            Instruction::Pop => write!(f, "pop"),
            Instruction::Pop2 => write!(f, "pop2"),
            Instruction::Putfield(index) => write!(f, "putfield {}", index),
            Instruction::Putstatic(index) => write!(f, "putstatic {}", index),
            Instruction::Ret(index) => write!(f, "ret {}", index),
            Instruction::Return => write!(f, "return"),
            Instruction::Saload => write!(f, "saload"),
            Instruction::Sastore => write!(f, "sastore"),
            Instruction::Sipush(value) => write!(f, "sipush {}", value),
            Instruction::Swap => write!(f, "swap"),
            Instruction::Tableswitch => write!(f, "tableswitch"),
            Instruction::Wide => write!(f, "wide"),
        }
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
