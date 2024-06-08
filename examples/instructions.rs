use rjvm::bytecode::attributes::{CodeInfo, Container};
use rjvm::bytecode::pool::ConstantPool;
use rjvm::bytecode::reader::attributes::{
    CodeAttributeFactory, ConstantValueAttributeFactory, InnerClassesAttributeFactory,
    LineNumberTableAttributeFactory, LocalVariableTableAttributeFactory,
    MethodParametersAttributeFactory, NestMembersAttributeFactory,
    RuntimeInvisibleAnnotationsAttributeFactory, SourceFileAttributeFactory,
};
use rjvm::bytecode::reader::containers::read_classfile;
use rjvm::bytecode::reader::BufferedReader;
use rjvm::decoder::instructions::parse_instruction;

fn main() {
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

    println!("cp size: {}", cf.constant_pool.size());

    println!("Class: {}", constant_pool.text_of(cf.this_class).unwrap());
    // print all methods and their instructions
    cf.methods.iter().for_each(|method| {
        let attr_code = method.attributes.get("Code");

        // create a new buffer that contains just the code and nothing else...
        if let Some(attr) = attr_code {
            let attr = attr.as_any_ref().downcast_ref::<CodeInfo>().unwrap();
            let mut code_reader = BufferedReader::new(&attr.code);
            while !code_reader.has_remaining_data() {
                let opcode = code_reader.take::<u8>().unwrap();
                let instr = parse_instruction(opcode, &mut code_reader)
                    .expect("instruction should be parsed");
                println!("  0x{opcode:02x}: {}", instr.opcode());
            }
        }
        println!();
    });
}
