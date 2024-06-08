use rjvm::{
    bytecode::{
        attributes::{CodeInfo, Container},
        pool::ConstantPool,
        reader::{
            attributes::{
                CodeAttributeFactory, ConstantValueAttributeFactory, InnerClassesAttributeFactory,
                LineNumberTableAttributeFactory, LocalVariableTableAttributeFactory,
                MethodParametersAttributeFactory, NestMembersAttributeFactory,
                RuntimeInvisibleAnnotationsAttributeFactory, SourceFileAttributeFactory,
            },
            containers::read_classfile,
            BufferedReader,
        },
        Method,
    },
    decoder::instructions::parse_instruction,
    types::instructions::Instruction,
};

pub fn main() {
    let input = include_bytes!("./testdata/org/example/Simple.class");

    // -----------------------------------------------------------------------------
    //  - Initialize a container with all available attributes -
    // -----------------------------------------------------------------------------
    let mut container = Container::new();
    container.register("ConstantValue", ConstantValueAttributeFactory);
    container.register("Code", CodeAttributeFactory);
    container.register("LineNumberTable", LineNumberTableAttributeFactory);
    container.register(
        "RuntimeInvisibleAnnotations",
        RuntimeInvisibleAnnotationsAttributeFactory,
    );
    container.register("SourceFile", SourceFileAttributeFactory);
    container.register("NestMembers", NestMembersAttributeFactory);
    container.register("InnerClasses", InnerClassesAttributeFactory);
    container.register("LocalVariableTable", LocalVariableTableAttributeFactory);
    container.register("MethodParameters", MethodParametersAttributeFactory);

    let mut buffer = BufferedReader::new(input);
    let mut constant_pool = ConstantPool::new();
    let cf = read_classfile(&mut buffer, &mut constant_pool, &container).unwrap();

    cf.methods
        .iter()
        .for_each(|method| process_method(method, &constant_pool));
}

pub fn process_method(method: &Method, pool: &ConstantPool) {
    println!("processing method: {}", method.name);

    let Some(code) = method.get_attribute::<CodeInfo>("Code") else {
        eprintln!("skippign method; no code found for method");
        return;
    };

    let mut reader = BufferedReader::new(&code.code);
    let mut instructions: Vec<Box<dyn Instruction>> = Vec::new();
    while !reader.has_remaining_data() {
        let opcode = reader.take::<u8>().unwrap();
        let Ok(instr) = parse_instruction(opcode, &mut reader) else {
            eprintln!("error while parsing instruction with opcode {}", opcode);
            continue;
        };
        instructions.push(instr);
    }

    for instr in instructions {
        println!("{}", instr.to_bytecode_string());
    }
}
