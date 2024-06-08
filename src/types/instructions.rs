pub trait Instruction {
    /// Returns the name of the instruction.
    ///
    /// WARNING: This should only be used for debugging purposes because there is no guarantee
    /// that no other `Instruction` implementation will return the same name.
    fn name(&self) -> &'static str {
        "[unknown instruction]"
    }

    /// Returns the opcode of the instruction as defined in the JVM specification.
    fn opcode(&self) -> u8;

    /// Returns the size of the instruction in bytes.
    ///
    /// WARNING: We assume that the default size of an instruction is 1 byte. If the
    /// instruction should have a different size, the implementing struct should override
    /// this.
    fn size(&self) -> usize {
        1
    }

    fn to_bytecode_string(&self) -> String {
        let mut str = String::new();
        str.push_str(self.name());

        self.arguments().into_iter().for_each(|arg| {
            str.push_str(&format!(" #{}", arg));
        });

        str
    }

    fn arguments(&self) -> Vec<u16> {
        vec![]
    }

    fn arguments_size(&self) -> usize {
        self.arguments().len()
    }

    fn writes_local(&self) -> bool {
        false
    }

    fn index_of_next_instruction(&self, current: usize, is_wide: bool) -> usize {
        _ = is_wide;
        current + self.size()
    }
}

pub trait InstructionInfo {
    const OPCODE: u8;
    const MNEMONIC: &'static str;
}

impl Instruction for Box<dyn Instruction> {
    fn name(&self) -> &'static str {
        self.as_ref().name()
    }

    fn opcode(&self) -> u8 {
        self.as_ref().opcode()
    }

    fn size(&self) -> usize {
        self.as_ref().size()
    }

    fn to_bytecode_string(&self) -> String {
        self.as_ref().to_bytecode_string()
    }
}

/// A trait that allows to treat any `Instruction` as a trait object.
///
/// WARNING: This trait should never be implemented manually, because it is already implemented for
/// all types that implements `std::fmt::Debug + Instruction + 'static`.
pub trait AnyInstruction {
    /// Returns a reference to the `dyn std::any::Any` trait object.
    fn as_any_ref(&self) -> &dyn std::any::Any;
    /// Returns a mutable reference to the `dyn std::any::Any` trait object.
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    /// Returns the name of the instruction.
    fn any_name(&self) -> &'static str;
    /// Returns the opcode of the instruction.
    fn any_opcode(&self) -> u8;
    /// Returns the size of the instruction in bytes.
    fn any_size(&self) -> usize;
}

impl std::fmt::Debug for dyn AnyInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.any_name())
    }
}

impl<T: Instruction + InstructionInfo + 'static> AnyInstruction for T {
    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn any_name(&self) -> &'static str {
        self.name()
    }

    fn any_opcode(&self) -> u8 {
        self.opcode()
    }

    fn any_size(&self) -> usize {
        self.size()
    }
}

pub struct Aaload;

impl InstructionInfo for Aaload {
    const OPCODE: u8 = 0x32;
    const MNEMONIC: &'static str = "aaload";
}

impl Instruction for Aaload {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x32
    }
}

pub struct Aastore {
    pub args: Vec<u16>,
}

impl InstructionInfo for Aastore {
    const MNEMONIC: &'static str = "aastore";
    const OPCODE: u8 = 0x53;
}

impl Instruction for Aastore {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x53
    }

    fn arguments(&self) -> Vec<u16> {
        self.args.clone()
    }
}

pub struct AConstNull;

impl InstructionInfo for AConstNull {
    const MNEMONIC: &'static str = "aastore";
    const OPCODE: u8 = 0x01;
}

impl Instruction for AConstNull {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x01
    }
}

pub struct Aload {
    pub args: Vec<u16>,
}

impl InstructionInfo for Aload {
    const MNEMONIC: &'static str = "aload";
    const OPCODE: u8 = 0x19;
}

impl Instruction for Aload {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }

    fn arguments(&self) -> Vec<u16> {
        self.args.clone()
    }
}

pub struct Aload0;

impl InstructionInfo for Aload0 {
    const OPCODE: u8 = 0x2a;
    const MNEMONIC: &'static str = "aload_0";
}

impl Instruction for Aload0 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Aload1;

impl InstructionInfo for Aload1 {
    const OPCODE: u8 = 0x2b;
    const MNEMONIC: &'static str = "aload_1";
}

impl Instruction for Aload1 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Aload2;

impl InstructionInfo for Aload2 {
    const OPCODE: u8 = 0x2c;
    const MNEMONIC: &'static str = "aload_2";
}

impl Instruction for Aload2 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Aload3;

impl InstructionInfo for Aload3 {
    const OPCODE: u8 = 0x2d;
    const MNEMONIC: &'static str = "aload_3";
}

impl Instruction for Aload3 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Anewarray {
    pub args: Vec<u16>,
}

impl InstructionInfo for Anewarray {
    const MNEMONIC: &'static str = "anewarray";
    const OPCODE: u8 = 0xbd;
}

impl Instruction for Anewarray {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }

    fn arguments(&self) -> Vec<u16> {
        self.args.clone()
    }
}

pub struct Areturn;

impl InstructionInfo for Areturn {
    const MNEMONIC: &'static str = "areturn";
    const OPCODE: u8 = 0xb0;
}

impl Instruction for Areturn {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Arraylength;

impl InstructionInfo for Arraylength {
    const MNEMONIC: &'static str = "arraylength";
    const OPCODE: u8 = 0xbe;
}

impl Instruction for Arraylength {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Astore {
    pub args: Vec<u16>,
}

impl InstructionInfo for Astore {
    const MNEMONIC: &'static str = "astore";
    const OPCODE: u8 = 0x3a;
}

impl Instruction for Astore {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }

    fn arguments(&self) -> Vec<u16> {
        self.args.clone()
    }
}

pub struct Astore0;

impl InstructionInfo for Astore0 {
    const MNEMONIC: &'static str = "astore_0";
    const OPCODE: u8 = 0x4b;
}

impl Instruction for Astore0 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Astore1;

impl InstructionInfo for Astore1 {
    const MNEMONIC: &'static str = "astore_1";
    const OPCODE: u8 = 0x4c;
}

impl Instruction for Astore1 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Astore2;

impl InstructionInfo for Astore2 {
    const MNEMONIC: &'static str = "astore_2";
    const OPCODE: u8 = 0x4d;
}

impl Instruction for Astore2 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Astore3;

impl InstructionInfo for Astore3 {
    const MNEMONIC: &'static str = "astore_3";
    const OPCODE: u8 = 0x4e;
}

impl Instruction for Astore3 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Athrow;

impl InstructionInfo for Athrow {
    const MNEMONIC: &'static str = "athrow";
    const OPCODE: u8 = 0xbf;
}

impl Instruction for Athrow {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Baload;

impl InstructionInfo for Baload {
    const MNEMONIC: &'static str = "baload";
    const OPCODE: u8 = 0x33;
}

impl Instruction for Baload {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Bastore;

impl InstructionInfo for Bastore {
    const MNEMONIC: &'static str = "bastore";
    const OPCODE: u8 = 0x54;
}

impl Instruction for Bastore {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Bipush {
    pub args: Vec<u16>,
}

impl InstructionInfo for Bipush {
    const MNEMONIC: &'static str = "bipush";
    const OPCODE: u8 = 0x10;
}

impl Instruction for Bipush {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }

    fn arguments(&self) -> Vec<u16> {
        self.args.clone()
    }
}

pub struct Caload;

impl InstructionInfo for Caload {
    const MNEMONIC: &'static str = "caload";
    const OPCODE: u8 = 0x34;
}

impl Instruction for Caload {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Castore;

impl InstructionInfo for Castore {
    const MNEMONIC: &'static str = "castore";
    const OPCODE: u8 = 0x55;
}

impl Instruction for Castore {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Checkcast {
    pub args: Vec<u16>,
}

impl InstructionInfo for Checkcast {
    const MNEMONIC: &'static str = "checkcast";
    const OPCODE: u8 = 0xc0;
}

impl Instruction for Checkcast {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }

    fn arguments(&self) -> Vec<u16> {
        self.args.clone()
    }
}

pub struct D2f;

impl InstructionInfo for D2f {
    const MNEMONIC: &'static str = "d2f";
    const OPCODE: u8 = 0x90;
}

impl Instruction for D2f {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct D2i;

impl InstructionInfo for D2i {
    const MNEMONIC: &'static str = "d2i";
    const OPCODE: u8 = 0x8e;
}

impl Instruction for D2i {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct D2l;

impl InstructionInfo for D2l {
    const MNEMONIC: &'static str = "d2l";
    const OPCODE: u8 = 0x8f;
}

impl Instruction for D2l {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Dadd;

impl InstructionInfo for Dadd {
    const MNEMONIC: &'static str = "dadd";
    const OPCODE: u8 = 0x63;
}

impl Instruction for Dadd {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Daload;

impl InstructionInfo for Daload {
    const MNEMONIC: &'static str = "daload";
    const OPCODE: u8 = 0x31;
}

impl Instruction for Daload {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Dastore;

impl InstructionInfo for Dastore {
    const MNEMONIC: &'static str = "dastore";
    const OPCODE: u8 = 0x52;
}

impl Instruction for Dastore {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Dcmpg;

impl InstructionInfo for Dcmpg {
    const MNEMONIC: &'static str = "dcmpg";
    const OPCODE: u8 = 0x98;
}

impl Instruction for Dcmpg {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Dcmpl;

impl InstructionInfo for Dcmpl {
    const MNEMONIC: &'static str = "dcmpl";
    const OPCODE: u8 = 0x97;
}

impl Instruction for Dcmpl {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Dconst0;

impl InstructionInfo for Dconst0 {
    const MNEMONIC: &'static str = "dconst_0";
    const OPCODE: u8 = 0xe;
}

impl Instruction for Dconst0 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Dconst1;

impl InstructionInfo for Dconst1 {
    const MNEMONIC: &'static str = "dconst_1";
    const OPCODE: u8 = 0xf;
}

impl Instruction for Dconst1 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Ddiv;

impl InstructionInfo for Ddiv {
    const MNEMONIC: &'static str = "ddiv";
    const OPCODE: u8 = 0x6f;
}

impl Instruction for Ddiv {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Dload {
    pub args: Vec<u16>,
}

impl InstructionInfo for Dload {
    const MNEMONIC: &'static str = "dload";
    const OPCODE: u8 = 0x18;
}

impl Instruction for Dload {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }

    fn arguments(&self) -> Vec<u16> {
        self.args.clone()
    }
}

pub struct Dload0;

impl InstructionInfo for Dload0 {
    const MNEMONIC: &'static str = "dload_0";
    const OPCODE: u8 = 0x26;
}

impl Instruction for Dload0 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Dload1;

impl InstructionInfo for Dload1 {
    const MNEMONIC: &'static str = "dload_1";
    const OPCODE: u8 = 0x27;
}

impl Instruction for Dload1 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Dload2;

impl InstructionInfo for Dload2 {
    const MNEMONIC: &'static str = "dload_2";
    const OPCODE: u8 = 0x28;
}

impl Instruction for Dload2 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Dload3;

impl InstructionInfo for Dload3 {
    const MNEMONIC: &'static str = "dload_3";
    const OPCODE: u8 = 0x29;
}

impl Instruction for Dload3 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Dmul;

impl InstructionInfo for Dmul {
    const MNEMONIC: &'static str = "dmul";
    const OPCODE: u8 = 0x6b;
}

impl Instruction for Dmul {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Dneg;

impl InstructionInfo for Dneg {
    const MNEMONIC: &'static str = "dneg";
    const OPCODE: u8 = 0x77;
}

impl Instruction for Dneg {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Drem;

impl InstructionInfo for Drem {
    const MNEMONIC: &'static str = "drem";
    const OPCODE: u8 = 0x73;
}

impl Instruction for Drem {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Dreturn;

impl InstructionInfo for Dreturn {
    const MNEMONIC: &'static str = "dreturn";
    const OPCODE: u8 = 0xaf;
}

impl Instruction for Dreturn {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Dstore {
    pub args: Vec<u16>,
}

impl InstructionInfo for Dstore {
    const MNEMONIC: &'static str = "dstore";
    const OPCODE: u8 = 0x39;
}

impl Instruction for Dstore {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }

    fn arguments(&self) -> Vec<u16> {
        self.args.clone()
    }
}

pub struct Dstore0;

impl InstructionInfo for Dstore0 {
    const MNEMONIC: &'static str = "dstore_0";
    const OPCODE: u8 = 0x47;
}

impl Instruction for Dstore0 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Dstore1;

impl InstructionInfo for Dstore1 {
    const MNEMONIC: &'static str = "dstore_1";
    const OPCODE: u8 = 0x48;
}

impl Instruction for Dstore1 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Dstore2;

impl InstructionInfo for Dstore2 {
    const MNEMONIC: &'static str = "dstore_2";
    const OPCODE: u8 = 0x49;
}

impl Instruction for Dstore2 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Dstore3;

impl InstructionInfo for Dstore3 {
    const MNEMONIC: &'static str = "dstore_3";
    const OPCODE: u8 = 0x4a;
}

impl Instruction for Dstore3 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Dsub;

impl InstructionInfo for Dsub {
    const MNEMONIC: &'static str = "dsub";
    const OPCODE: u8 = 0x67;
}

impl Instruction for Dsub {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Dup;

impl InstructionInfo for Dup {
    const MNEMONIC: &'static str = "dup";
    const OPCODE: u8 = 0x59;
}

impl Instruction for Dup {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct DupX1;

impl InstructionInfo for DupX1 {
    const MNEMONIC: &'static str = "dup_x1";
    const OPCODE: u8 = 0x5a;
}

impl Instruction for DupX1 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct DupX2;

impl InstructionInfo for DupX2 {
    const MNEMONIC: &'static str = "dup_x2";
    const OPCODE: u8 = 0x5b;
}

impl Instruction for DupX2 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Dup2;

impl InstructionInfo for Dup2 {
    const MNEMONIC: &'static str = "dup2";
    const OPCODE: u8 = 0x5c;
}

impl Instruction for Dup2 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Dup2X1;

impl InstructionInfo for Dup2X1 {
    const MNEMONIC: &'static str = "dup2_x1";
    const OPCODE: u8 = 0x5d;
}

impl Instruction for Dup2X1 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Dup2X2;

impl InstructionInfo for Dup2X2 {
    const MNEMONIC: &'static str = "dup2_x2";
    const OPCODE: u8 = 0x5e;
}

impl Instruction for Dup2X2 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct F2D;

impl InstructionInfo for F2D {
    const MNEMONIC: &'static str = "f2d";
    const OPCODE: u8 = 0x8d;
}

impl Instruction for F2D {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct F2I;

impl InstructionInfo for F2I {
    const MNEMONIC: &'static str = "f2i";
    const OPCODE: u8 = 0x8b;
}

impl Instruction for F2I {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct F2L;

impl InstructionInfo for F2L {
    const MNEMONIC: &'static str = "f2l";
    const OPCODE: u8 = 0x8c;
}

impl Instruction for F2L {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Fadd;

impl InstructionInfo for Fadd {
    const MNEMONIC: &'static str = "fadd";
    const OPCODE: u8 = 0x62;
}

impl Instruction for Fadd {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Faload;

impl InstructionInfo for Faload {
    const MNEMONIC: &'static str = "faload";
    const OPCODE: u8 = 0x30;
}

impl Instruction for Faload {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Fastore;

impl InstructionInfo for Fastore {
    const MNEMONIC: &'static str = "fastore";
    const OPCODE: u8 = 0x51;
}

impl Instruction for Fastore {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Fcmpg;

impl InstructionInfo for Fcmpg {
    const MNEMONIC: &'static str = "fcmpg";
    const OPCODE: u8 = 0x96;
}

impl Instruction for Fcmpg {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Fcmpl;

impl InstructionInfo for Fcmpl {
    const MNEMONIC: &'static str = "fcmpl";
    const OPCODE: u8 = 0x95;
}

impl Instruction for Fcmpl {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Fconst0;

impl InstructionInfo for Fconst0 {
    const MNEMONIC: &'static str = "fconst_0";
    const OPCODE: u8 = 0x0b;
}

impl Instruction for Fconst0 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Fconst1;

impl InstructionInfo for Fconst1 {
    const MNEMONIC: &'static str = "fconst_1";
    const OPCODE: u8 = 0x0c;
}

impl Instruction for Fconst1 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Fconst2;

impl InstructionInfo for Fconst2 {
    const MNEMONIC: &'static str = "fconst_2";
    const OPCODE: u8 = 0x0d;
}

impl Instruction for Fconst2 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Fdiv;

impl InstructionInfo for Fdiv {
    const MNEMONIC: &'static str = "fdiv";
    const OPCODE: u8 = 0x6e;
}

impl Instruction for Fdiv {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Fload;

impl InstructionInfo for Fload {
    const MNEMONIC: &'static str = "fload";
    const OPCODE: u8 = 0x17;
}

impl Instruction for Fload {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Fload0;

impl InstructionInfo for Fload0 {
    const MNEMONIC: &'static str = "fload_0";
    const OPCODE: u8 = 0x22;
}

impl Instruction for Fload0 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Fload1;

impl InstructionInfo for Fload1 {
    const MNEMONIC: &'static str = "fload_1";
    const OPCODE: u8 = 0x23;
}

impl Instruction for Fload1 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Fload2;

impl InstructionInfo for Fload2 {
    const MNEMONIC: &'static str = "fload_2";
    const OPCODE: u8 = 0x24;
}

impl Instruction for Fload2 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Fload3;

impl InstructionInfo for Fload3 {
    const MNEMONIC: &'static str = "fload_3";
    const OPCODE: u8 = 0x25;
}

impl Instruction for Fload3 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Fmul;

impl InstructionInfo for Fmul {
    const MNEMONIC: &'static str = "fmul";
    const OPCODE: u8 = 0x6a;
}

impl Instruction for Fmul {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Fneg;

impl InstructionInfo for Fneg {
    const MNEMONIC: &'static str = "fneg";
    const OPCODE: u8 = 0x76;
}

impl Instruction for Fneg {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Frem;

impl InstructionInfo for Frem {
    const MNEMONIC: &'static str = "frem";
    const OPCODE: u8 = 0x72;
}

impl Instruction for Frem {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Freturn;

impl InstructionInfo for Freturn {
    const MNEMONIC: &'static str = "freturn";
    const OPCODE: u8 = 0xae;
}

impl Instruction for Freturn {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Fstore;

impl InstructionInfo for Fstore {
    const MNEMONIC: &'static str = "fstore";
    const OPCODE: u8 = 0x38;
}

impl Instruction for Fstore {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Fstore0;

impl InstructionInfo for Fstore0 {
    const MNEMONIC: &'static str = "fstore_0";
    const OPCODE: u8 = 0x43;
}

impl Instruction for Fstore0 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Fstore1;

impl InstructionInfo for Fstore1 {
    const MNEMONIC: &'static str = "fstore_1";
    const OPCODE: u8 = 0x44;
}

impl Instruction for Fstore1 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Fstore2;

impl InstructionInfo for Fstore2 {
    const MNEMONIC: &'static str = "fstore_2";
    const OPCODE: u8 = 0x45;
}

impl Instruction for Fstore2 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Fstore3;

impl InstructionInfo for Fstore3 {
    const MNEMONIC: &'static str = "fstore_3";
    const OPCODE: u8 = 0x46;
}

impl Instruction for Fstore3 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Fsub;

impl InstructionInfo for Fsub {
    const MNEMONIC: &'static str = "fsub";
    const OPCODE: u8 = 0x66;
}

impl Instruction for Fsub {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        Self::OPCODE
    }
}

pub struct Getfield;

impl InstructionInfo for Getfield {
    const MNEMONIC: &'static str = "getfield";
    const OPCODE: u8 = 0xb4;
}

impl Instruction for Getfield {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xb4
    }
}

pub struct Getstatic {
    pub args: Vec<u16>,
}

impl InstructionInfo for Getstatic {
    const MNEMONIC: &'static str = "getstatic";
    const OPCODE: u8 = 0xb2;
}

impl Instruction for Getstatic {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xb2
    }

    fn arguments(&self) -> Vec<u16> {
        self.args.clone()
    }
}

pub struct Goto;

impl InstructionInfo for Goto {
    const MNEMONIC: &'static str = "goto";
    const OPCODE: u8 = 0xa7;
}

impl Instruction for Goto {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xa7
    }
}

pub struct GotoW;

impl InstructionInfo for GotoW {
    const MNEMONIC: &'static str = "goto_w";
    const OPCODE: u8 = 0xc8;
}

impl Instruction for GotoW {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xc8
    }
}

pub struct I2b;

impl InstructionInfo for I2b {
    const MNEMONIC: &'static str = "i2b";
    const OPCODE: u8 = 0x91;
}

impl Instruction for I2b {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x91
    }
}

pub struct I2c;

impl InstructionInfo for I2c {
    const MNEMONIC: &'static str = "i2c";
    const OPCODE: u8 = 0x92;
}

impl Instruction for I2c {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x92
    }
}

pub struct I2d;

impl InstructionInfo for I2d {
    const MNEMONIC: &'static str = "i2d";
    const OPCODE: u8 = 0x87;
}

impl Instruction for I2d {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x87
    }
}

pub struct I2f;

impl InstructionInfo for I2f {
    const MNEMONIC: &'static str = "i2f";
    const OPCODE: u8 = 0x86;
}

impl Instruction for I2f {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x86
    }
}

pub struct I2l;

impl InstructionInfo for I2l {
    const MNEMONIC: &'static str = "i2l";
    const OPCODE: u8 = 0x85;
}

impl Instruction for I2l {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x85
    }
}

pub struct I2s;

impl InstructionInfo for I2s {
    const MNEMONIC: &'static str = "i2s";
    const OPCODE: u8 = 0x93;
}

impl Instruction for I2s {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x93
    }
}

pub struct Iadd;

impl InstructionInfo for Iadd {
    const MNEMONIC: &'static str = "iadd";
    const OPCODE: u8 = 0x60;
}

impl Instruction for Iadd {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x60
    }
}

pub struct Iaload;

impl InstructionInfo for Iaload {
    const MNEMONIC: &'static str = "iaload";
    const OPCODE: u8 = 0x2e;
}

impl Instruction for Iaload {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x2e
    }
}

pub struct Iand;

impl InstructionInfo for Iand {
    const MNEMONIC: &'static str = "iand";
    const OPCODE: u8 = 0x7e;
}

impl Instruction for Iand {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x7e
    }
}

pub struct Iastore;

impl InstructionInfo for Iastore {
    const MNEMONIC: &'static str = "iastore";
    const OPCODE: u8 = 0x4f;
}

impl Instruction for Iastore {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x4f
    }
}

pub struct IconstM1;

impl InstructionInfo for IconstM1 {
    const MNEMONIC: &'static str = "iconst_m1";
    const OPCODE: u8 = 0x2;
}

impl Instruction for IconstM1 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x2
    }
}

pub struct Iconst0;

impl InstructionInfo for Iconst0 {
    const MNEMONIC: &'static str = "iconst_0";
    const OPCODE: u8 = 0x3;
}

impl Instruction for Iconst0 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x3
    }
}

pub struct Iconst1;

impl InstructionInfo for Iconst1 {
    const MNEMONIC: &'static str = "iconst_1";
    const OPCODE: u8 = 0x4;
}

impl Instruction for Iconst1 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x4
    }
}

pub struct Iconst2;

impl InstructionInfo for Iconst2 {
    const MNEMONIC: &'static str = "iconst_2";
    const OPCODE: u8 = 0x5;
}

impl Instruction for Iconst2 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x5
    }
}

pub struct Iconst3;

impl InstructionInfo for Iconst3 {
    const MNEMONIC: &'static str = "iconst_3";
    const OPCODE: u8 = 0x6;
}

impl Instruction for Iconst3 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x6
    }
}

pub struct Iconst4;

impl InstructionInfo for Iconst4 {
    const MNEMONIC: &'static str = "iconst_4";
    const OPCODE: u8 = 0x7;
}

impl Instruction for Iconst4 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x7
    }
}

pub struct Iconst5;

impl InstructionInfo for Iconst5 {
    const MNEMONIC: &'static str = "iconst_5";
    const OPCODE: u8 = 0x8;
}

impl Instruction for Iconst5 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x8
    }
}

pub struct Idiv;

impl InstructionInfo for Idiv {
    const MNEMONIC: &'static str = "idiv";
    const OPCODE: u8 = 0x6c;
}

impl Instruction for Idiv {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x6c
    }
}

pub struct IfAcmpeq;

impl InstructionInfo for IfAcmpeq {
    const MNEMONIC: &'static str = "if_acmpeq";
    const OPCODE: u8 = 0xa5;
}

impl Instruction for IfAcmpeq {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xa5
    }
}

pub struct IfAcmpne;

impl InstructionInfo for IfAcmpne {
    const MNEMONIC: &'static str = "if_acmpne";
    const OPCODE: u8 = 0xa6;
}

impl Instruction for IfAcmpne {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xa6
    }
}

pub struct IfIcmpeq;

impl InstructionInfo for IfIcmpeq {
    const MNEMONIC: &'static str = "if_icmpeq";
    const OPCODE: u8 = 0x9f;
}

impl Instruction for IfIcmpeq {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x9f
    }
}

pub struct IfIcmpge;

impl InstructionInfo for IfIcmpge {
    const MNEMONIC: &'static str = "if_icmpge";
    const OPCODE: u8 = 0xa2;
}

impl Instruction for IfIcmpge {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xa2
    }
}

pub struct IfIcmpgt;

impl InstructionInfo for IfIcmpgt {
    const MNEMONIC: &'static str = "if_icmpgt";
    const OPCODE: u8 = 0xa3;
}

impl Instruction for IfIcmpgt {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xa3
    }
}

pub struct IfIcmple;

impl InstructionInfo for IfIcmple {
    const MNEMONIC: &'static str = "if_icmple";
    const OPCODE: u8 = 0xa4;
}

impl Instruction for IfIcmple {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xa4
    }
}

pub struct IfIcmplt;

impl InstructionInfo for IfIcmplt {
    const MNEMONIC: &'static str = "if_icmplt";
    const OPCODE: u8 = 0xa1;
}

impl Instruction for IfIcmplt {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xa1
    }
}

pub struct IfIcmpne;

impl InstructionInfo for IfIcmpne {
    const MNEMONIC: &'static str = "if_icmpne";
    const OPCODE: u8 = 0xa0;
}

impl Instruction for IfIcmpne {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xa0
    }
}

pub struct Ifeq;

impl InstructionInfo for Ifeq {
    const MNEMONIC: &'static str = "ifeq";
    const OPCODE: u8 = 0x99;
}

impl Instruction for Ifeq {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x99
    }
}

pub struct Ifge;

impl InstructionInfo for Ifge {
    const MNEMONIC: &'static str = "ifge";
    const OPCODE: u8 = 0x9c;
}

impl Instruction for Ifge {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x9c
    }
}

pub struct Ifgt;

impl InstructionInfo for Ifgt {
    const MNEMONIC: &'static str = "ifgt";
    const OPCODE: u8 = 0x9d;
}

impl Instruction for Ifgt {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x9d
    }
}

pub struct Ifle;

impl InstructionInfo for Ifle {
    const MNEMONIC: &'static str = "ifle";
    const OPCODE: u8 = 0x9e;
}

impl Instruction for Ifle {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x9e
    }
}

pub struct Iflt;

impl InstructionInfo for Iflt {
    const MNEMONIC: &'static str = "iflt";
    const OPCODE: u8 = 0x9b;
}

impl Instruction for Iflt {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x9b
    }
}

pub struct Ifne;

impl InstructionInfo for Ifne {
    const MNEMONIC: &'static str = "ifne";
    const OPCODE: u8 = 0x9a;
}

impl Instruction for Ifne {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x9a
    }
}

pub struct Ifnonnull;

impl InstructionInfo for Ifnonnull {
    const MNEMONIC: &'static str = "ifnonnull";
    const OPCODE: u8 = 0xc7;
}

impl Instruction for Ifnonnull {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xc7
    }
}

pub struct Ifnull;

impl InstructionInfo for Ifnull {
    const MNEMONIC: &'static str = "ifnull";
    const OPCODE: u8 = 0xc6;
}

impl Instruction for Ifnull {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xc6
    }
}

pub struct Iinc;

impl InstructionInfo for Iinc {
    const MNEMONIC: &'static str = "iinc";
    const OPCODE: u8 = 0x84;
}

impl Instruction for Iinc {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x84
    }
}

pub struct Iload;

impl InstructionInfo for Iload {
    const MNEMONIC: &'static str = "iload";
    const OPCODE: u8 = 0x15;
}

impl Instruction for Iload {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x15
    }
}

pub struct Iload0;

impl InstructionInfo for Iload0 {
    const MNEMONIC: &'static str = "iload_0";
    const OPCODE: u8 = 0x1a;
}

impl Instruction for Iload0 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x1a
    }
}

pub struct Iload1;

impl InstructionInfo for Iload1 {
    const MNEMONIC: &'static str = "iload_1";
    const OPCODE: u8 = 0x1b;
}

impl Instruction for Iload1 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x1b
    }
}

pub struct Iload2;

impl InstructionInfo for Iload2 {
    const MNEMONIC: &'static str = "iload_2";
    const OPCODE: u8 = 0x1c;
}

impl Instruction for Iload2 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x1c
    }
}

pub struct Iload3;

impl InstructionInfo for Iload3 {
    const MNEMONIC: &'static str = "iload_3";
    const OPCODE: u8 = 0x1d;
}

impl Instruction for Iload3 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x1d
    }
}

pub struct Imul;

impl InstructionInfo for Imul {
    const MNEMONIC: &'static str = "imul";
    const OPCODE: u8 = 0x68;
}

impl Instruction for Imul {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x68
    }
}

pub struct Ineg;

impl InstructionInfo for Ineg {
    const MNEMONIC: &'static str = "ineg";
    const OPCODE: u8 = 0x74;
}

impl Instruction for Ineg {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x74
    }
}

pub struct Instanceof;

impl InstructionInfo for Instanceof {
    const MNEMONIC: &'static str = "instanceof";
    const OPCODE: u8 = 0xc1;
}

impl Instruction for Instanceof {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xc1
    }
}

pub struct Invokedynamic;

impl InstructionInfo for Invokedynamic {
    const MNEMONIC: &'static str = "invokedynamic";
    const OPCODE: u8 = 0xba;
}

impl Instruction for Invokedynamic {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xba
    }
}

pub struct Invokeinterface;

impl InstructionInfo for Invokeinterface {
    const MNEMONIC: &'static str = "invokeinterface";
    const OPCODE: u8 = 0xb9;
}

impl Instruction for Invokeinterface {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xb9
    }
}

pub struct Invokespecial {
    pub args: Vec<u16>,
}

impl InstructionInfo for Invokespecial {
    const MNEMONIC: &'static str = "invokespecial";
    const OPCODE: u8 = 0xb7;
}

impl Instruction for Invokespecial {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xb7
    }

    fn arguments(&self) -> Vec<u16> {
        self.args.clone()
    }
}

pub struct Invokestatic;

impl InstructionInfo for Invokestatic {
    const MNEMONIC: &'static str = "invokestatic";
    const OPCODE: u8 = 0xb8;
}

impl Instruction for Invokestatic {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xb8
    }
}

pub struct Invokevirtual {
    pub args: Vec<u16>,
}

impl InstructionInfo for Invokevirtual {
    const MNEMONIC: &'static str = "invokevirtual";
    const OPCODE: u8 = 0xb6;
}

impl Instruction for Invokevirtual {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xb6
    }

    fn arguments(&self) -> Vec<u16> {
        self.args.clone()
    }
}

pub struct Ior;

impl InstructionInfo for Ior {
    const MNEMONIC: &'static str = "ior";
    const OPCODE: u8 = 0x80;
}

impl Instruction for Ior {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x80
    }
}

pub struct Irem;

impl InstructionInfo for Irem {
    const MNEMONIC: &'static str = "irem";
    const OPCODE: u8 = 0x70;
}

impl Instruction for Irem {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x70
    }
}

pub struct Ireturn;

impl InstructionInfo for Ireturn {
    const MNEMONIC: &'static str = "ireturn";
    const OPCODE: u8 = 0xac;
}

impl Instruction for Ireturn {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xac
    }
}

pub struct Ishl;

impl InstructionInfo for Ishl {
    const MNEMONIC: &'static str = "ishl";
    const OPCODE: u8 = 0x78;
}

impl Instruction for Ishl {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x78
    }
}

pub struct Ishr;

impl InstructionInfo for Ishr {
    const MNEMONIC: &'static str = "ishr";
    const OPCODE: u8 = 0x7a;
}

impl Instruction for Ishr {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x7a
    }
}

pub struct Istore;

impl InstructionInfo for Istore {
    const MNEMONIC: &'static str = "istore";
    const OPCODE: u8 = 0x36;
}

impl Instruction for Istore {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x36
    }
}

pub struct Istore0;

impl InstructionInfo for Istore0 {
    const MNEMONIC: &'static str = "istore_0";
    const OPCODE: u8 = 0x3b;
}

impl Instruction for Istore0 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x3b
    }
}

pub struct Istore1;

impl InstructionInfo for Istore1 {
    const MNEMONIC: &'static str = "istore_1";
    const OPCODE: u8 = 0x3c;
}

impl Instruction for Istore1 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x3c
    }
}

pub struct Istore2;

impl InstructionInfo for Istore2 {
    const MNEMONIC: &'static str = "istore_2";
    const OPCODE: u8 = 0x3d;
}

impl Instruction for Istore2 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x3d
    }
}

pub struct Istore3;

impl InstructionInfo for Istore3 {
    const MNEMONIC: &'static str = "istore_3";
    const OPCODE: u8 = 0x3e;
}

impl Instruction for Istore3 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x3e
    }
}

pub struct Isub;

impl InstructionInfo for Isub {
    const MNEMONIC: &'static str = "isub";
    const OPCODE: u8 = 0x64;
}

impl Instruction for Isub {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x64
    }
}

pub struct Iushr;

impl InstructionInfo for Iushr {
    const MNEMONIC: &'static str = "iushr";
    const OPCODE: u8 = 0x7c;
}

impl Instruction for Iushr {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x7c
    }
}

pub struct Ixor;

impl InstructionInfo for Ixor {
    const MNEMONIC: &'static str = "ixor";
    const OPCODE: u8 = 0x82;
}

impl Instruction for Ixor {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x82
    }
}

pub struct Jsr;

impl InstructionInfo for Jsr {
    const MNEMONIC: &'static str = "jsr";
    const OPCODE: u8 = 0xa8;
}

impl Instruction for Jsr {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xa8
    }
}

pub struct JsrW;

impl InstructionInfo for JsrW {
    const MNEMONIC: &'static str = "jsr_w";
    const OPCODE: u8 = 0xc9;
}

impl Instruction for JsrW {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xc9
    }
}

pub struct L2D;

impl InstructionInfo for L2D {
    const MNEMONIC: &'static str = "l2d";
    const OPCODE: u8 = 0x8a;
}

impl Instruction for L2D {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x8a
    }
}

pub struct L2F;

impl InstructionInfo for L2F {
    const MNEMONIC: &'static str = "l2f";
    const OPCODE: u8 = 0x89;
}

impl Instruction for L2F {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x89
    }
}

pub struct L2I;

impl InstructionInfo for L2I {
    const MNEMONIC: &'static str = "l2i";
    const OPCODE: u8 = 0x88;
}

impl Instruction for L2I {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x88
    }
}

pub struct Ladd;

impl InstructionInfo for Ladd {
    const MNEMONIC: &'static str = "ladd";
    const OPCODE: u8 = 0x61;
}

impl Instruction for Ladd {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x61
    }
}

pub struct Laload;

impl InstructionInfo for Laload {
    const MNEMONIC: &'static str = "laload";
    const OPCODE: u8 = 0x2f;
}

impl Instruction for Laload {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x2f
    }
}

pub struct Land;

impl InstructionInfo for Land {
    const MNEMONIC: &'static str = "land";
    const OPCODE: u8 = 0x7f;
}

impl Instruction for Land {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x7f
    }
}

pub struct Lastore;

impl InstructionInfo for Lastore {
    const MNEMONIC: &'static str = "lastore";
    const OPCODE: u8 = 0x50;
}

impl Instruction for Lastore {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x50
    }
}

pub struct Lcmp;

impl InstructionInfo for Lcmp {
    const MNEMONIC: &'static str = "lcmp";
    const OPCODE: u8 = 0x94;
}

impl Instruction for Lcmp {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x94
    }
}

pub struct Lconst0;

impl InstructionInfo for Lconst0 {
    const MNEMONIC: &'static str = "lconst_0";
    const OPCODE: u8 = 0x09;
}

impl Instruction for Lconst0 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x09
    }
}

pub struct Lconst1;

impl InstructionInfo for Lconst1 {
    const MNEMONIC: &'static str = "lconst_1";
    const OPCODE: u8 = 0x0a;
}

impl Instruction for Lconst1 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x0a
    }
}

pub struct Ldc {
    pub args: Vec<u16>,
}

impl InstructionInfo for Ldc {
    const MNEMONIC: &'static str = "ldc";
    const OPCODE: u8 = 0x12;
}

impl Instruction for Ldc {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x12
    }

    fn arguments(&self) -> Vec<u16> {
        self.args.clone()
    }
}

pub struct LdcW;

impl InstructionInfo for LdcW {
    const MNEMONIC: &'static str = "ldc_w";
    const OPCODE: u8 = 0x13;
}

impl Instruction for LdcW {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x13
    }
}

pub struct Ldc2W;

impl InstructionInfo for Ldc2W {
    const MNEMONIC: &'static str = "ldc2_w";
    const OPCODE: u8 = 0x14;
}

impl Instruction for Ldc2W {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x14
    }
}

pub struct Ldiv;

impl InstructionInfo for Ldiv {
    const MNEMONIC: &'static str = "ldiv";
    const OPCODE: u8 = 0x6d;
}

impl Instruction for Ldiv {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x6d
    }
}

pub struct Lload;

impl InstructionInfo for Lload {
    const MNEMONIC: &'static str = "lload";
    const OPCODE: u8 = 0x16;
}

impl Instruction for Lload {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x16
    }
}

pub struct Lload0;

impl InstructionInfo for Lload0 {
    const MNEMONIC: &'static str = "lload_0";
    const OPCODE: u8 = 0x1e;
}

impl Instruction for Lload0 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x1e
    }
}

pub struct Lload1;

impl InstructionInfo for Lload1 {
    const MNEMONIC: &'static str = "lload_1";
    const OPCODE: u8 = 0x1f;
}

impl Instruction for Lload1 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x1f
    }
}

pub struct Lload2;

impl InstructionInfo for Lload2 {
    const MNEMONIC: &'static str = "lload_2";
    const OPCODE: u8 = 0x20;
}

impl Instruction for Lload2 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x20
    }
}

pub struct Lload3;

impl InstructionInfo for Lload3 {
    const MNEMONIC: &'static str = "lload_3";
    const OPCODE: u8 = 0x21;
}

impl Instruction for Lload3 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x21
    }
}

pub struct Lmul;

impl InstructionInfo for Lmul {
    const MNEMONIC: &'static str = "lmul";
    const OPCODE: u8 = 0x69;
}

impl Instruction for Lmul {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x69
    }
}

pub struct Lneg;

impl InstructionInfo for Lneg {
    const MNEMONIC: &'static str = "lneg";
    const OPCODE: u8 = 0x75;
}

impl Instruction for Lneg {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x75
    }
}

pub struct LookupSwitch;

impl InstructionInfo for LookupSwitch {
    const MNEMONIC: &'static str = "lookupswitch";
    const OPCODE: u8 = 0xab;
}

impl Instruction for LookupSwitch {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xab
    }
}

pub struct Lor;

impl InstructionInfo for Lor {
    const MNEMONIC: &'static str = "lor";
    const OPCODE: u8 = 0x81;
}

impl Instruction for Lor {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x81
    }
}

pub struct Lrem;

impl InstructionInfo for Lrem {
    const MNEMONIC: &'static str = "lrem";
    const OPCODE: u8 = 0x71;
}

impl Instruction for Lrem {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x71
    }
}

pub struct Lreturn;

impl InstructionInfo for Lreturn {
    const MNEMONIC: &'static str = "lreturn";
    const OPCODE: u8 = 0xad;
}

impl Instruction for Lreturn {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xad
    }
}

pub struct Lshl;

impl InstructionInfo for Lshl {
    const MNEMONIC: &'static str = "lshl";
    const OPCODE: u8 = 0x79;
}

impl Instruction for Lshl {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x79
    }
}

pub struct Lshr;

impl InstructionInfo for Lshr {
    const MNEMONIC: &'static str = "lshr";
    const OPCODE: u8 = 0x7b;
}

impl Instruction for Lshr {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x7b
    }
}

pub struct Lstore;

impl InstructionInfo for Lstore {
    const MNEMONIC: &'static str = "lstore";
    const OPCODE: u8 = 0x37;
}

impl Instruction for Lstore {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x37
    }
}

pub struct Lstore0;

impl InstructionInfo for Lstore0 {
    const MNEMONIC: &'static str = "lstore_0";
    const OPCODE: u8 = 0x3f;
}

impl Instruction for Lstore0 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x3f
    }
}

pub struct Lstore1;

impl InstructionInfo for Lstore1 {
    const MNEMONIC: &'static str = "lstore_1";
    const OPCODE: u8 = 0x40;
}

impl Instruction for Lstore1 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x40
    }
}

pub struct Lstore2;

impl InstructionInfo for Lstore2 {
    const MNEMONIC: &'static str = "lstore_2";
    const OPCODE: u8 = 0x41;
}

impl Instruction for Lstore2 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x41
    }
}

pub struct Lstore3;

impl InstructionInfo for Lstore3 {
    const MNEMONIC: &'static str = "lstore_3";
    const OPCODE: u8 = 0x42;
}

impl Instruction for Lstore3 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x42
    }
}

pub struct Lsub;

impl InstructionInfo for Lsub {
    const MNEMONIC: &'static str = "lsub";
    const OPCODE: u8 = 0x65;
}

impl Instruction for Lsub {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x65
    }
}

pub struct Lushr;

impl InstructionInfo for Lushr {
    const MNEMONIC: &'static str = "lushr";
    const OPCODE: u8 = 0x7d;
}

impl Instruction for Lushr {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x7d
    }
}

pub struct Lxor;

impl InstructionInfo for Lxor {
    const MNEMONIC: &'static str = "lxor";
    const OPCODE: u8 = 0x83;
}

impl Instruction for Lxor {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x83
    }
}

pub struct Monitorenter;

impl InstructionInfo for Monitorenter {
    const MNEMONIC: &'static str = "monitorenter";
    const OPCODE: u8 = 0xc2;
}

impl Instruction for Monitorenter {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xc2
    }
}

pub struct Monitorexit;

impl InstructionInfo for Monitorexit {
    const MNEMONIC: &'static str = "monitorexit";
    const OPCODE: u8 = 0xc3;
}

impl Instruction for Monitorexit {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xc3
    }
}

pub struct Multianewarray;

impl InstructionInfo for Multianewarray {
    const MNEMONIC: &'static str = "multianewarray";
    const OPCODE: u8 = 0xc5;
}

impl Instruction for Multianewarray {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xc5
    }
}

pub struct New {
    pub args: Vec<u16>,
}

impl InstructionInfo for New {
    const MNEMONIC: &'static str = "new";
    const OPCODE: u8 = 0xbb;
}

impl Instruction for New {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xbb
    }

    fn arguments(&self) -> Vec<u16> {
        self.args.clone()
    }
}

pub struct Newarray;

impl InstructionInfo for Newarray {
    const MNEMONIC: &'static str = "newarray";
    const OPCODE: u8 = 0xbc;
}

impl Instruction for Newarray {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xbc
    }
}

pub struct Nop;

impl InstructionInfo for Nop {
    const MNEMONIC: &'static str = "nop";
    const OPCODE: u8 = 0x0;
}

impl Instruction for Nop {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x0
    }
}

pub struct Pop;

impl InstructionInfo for Pop {
    const MNEMONIC: &'static str = "pop";
    const OPCODE: u8 = 0x57;
}

impl Instruction for Pop {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x57
    }
}

pub struct Pop2;

impl InstructionInfo for Pop2 {
    const MNEMONIC: &'static str = "pop2";
    const OPCODE: u8 = 0x58;
}

impl Instruction for Pop2 {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x58
    }
}

pub struct Putfield;

impl InstructionInfo for Putfield {
    const MNEMONIC: &'static str = "putfield";
    const OPCODE: u8 = 0xb5;
}

impl Instruction for Putfield {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xb5
    }
}

pub struct Putstatic;

impl InstructionInfo for Putstatic {
    const MNEMONIC: &'static str = "putstatic";
    const OPCODE: u8 = 0xb3;
}

impl Instruction for Putstatic {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xb3
    }
}

pub struct Ret;

impl InstructionInfo for Ret {
    const MNEMONIC: &'static str = "ret";
    const OPCODE: u8 = 0xa9;
}

impl Instruction for Ret {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xa9
    }
}

pub struct Return;

impl InstructionInfo for Return {
    const MNEMONIC: &'static str = "return";
    const OPCODE: u8 = 0xb1;
}

impl Instruction for Return {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xb1
    }
}

pub struct Saload;

impl InstructionInfo for Saload {
    const MNEMONIC: &'static str = "saload";
    const OPCODE: u8 = 0x35;
}

impl Instruction for Saload {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x35
    }
}

pub struct Sastore;

impl InstructionInfo for Sastore {
    const MNEMONIC: &'static str = "sastore";
    const OPCODE: u8 = 0x56;
}

impl Instruction for Sastore {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x56
    }
}

pub struct Sipush;

impl InstructionInfo for Sipush {
    const MNEMONIC: &'static str = "sipush";
    const OPCODE: u8 = 0x11;
}

impl Instruction for Sipush {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x11
    }
}

pub struct Swap;

impl InstructionInfo for Swap {
    const MNEMONIC: &'static str = "swap";
    const OPCODE: u8 = 0x5f;
}

impl Instruction for Swap {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0x5f
    }
}

pub struct Tableswitch;

impl InstructionInfo for Tableswitch {
    const MNEMONIC: &'static str = "tableswitch";
    const OPCODE: u8 = 0xaa;
}

impl Instruction for Tableswitch {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xaa
    }
}

pub struct Wide;

impl InstructionInfo for Wide {
    const MNEMONIC: &'static str = "wide";
    const OPCODE: u8 = 0xc4;
}

impl Instruction for Wide {
    fn name(&self) -> &'static str {
        Self::MNEMONIC
    }

    fn opcode(&self) -> u8 {
        0xc4
    }
}

#[cfg(test)]
mod tests {
    use crate::types::instructions::Instruction;

    use super::Aaload;

    #[test]
    fn test_instruction_returns_name() {
        let instr = Aaload;
        assert_eq!(format!("{}", instr.name()), "aaload");
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
