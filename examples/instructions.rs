use rjvm::bytecode::attributes::Attribute;
use rjvm::bytecode::pool::ConstantPool;
use rjvm::bytecode::reader::containers::read_classfile;
use rjvm::bytecode::reader::BufferedReader;
use rjvm::decoder::instructions::parse_instruction;

fn main() {
    let input = include_bytes!("./testdata/org/example/Simple.class");
    let mut buffer = BufferedReader::new(input);
    let mut constant_pool = ConstantPool::new();
    let cf = read_classfile(&mut buffer, &mut constant_pool).unwrap();

    println!("Class: {}", constant_pool.text_of(cf.this_class).unwrap());
    // print all methods and their instructions
    cf.methods.iter().for_each(|method| {
        let attr = method
            .attributes
            .iter()
            .find(|attr| matches!(attr, Attribute::Code(_)));
        println!("Name: {}()", method.name);

        let code = if let Attribute::Code(code) = attr.unwrap() {
            code
        } else {
            panic!("Code attribute not found");
        };

        // create a new buffer that contains just the code and nothing else...
        let mut code_reader = BufferedReader::new(&code.code);
        while !code_reader.has_remaining_data() {
            let opcode = code_reader.take::<u8>().unwrap();
            let instr =
                parse_instruction(opcode, &mut code_reader).expect("instruction should be parsed");
            println!("  0x{opcode:02x}: {}", instr);
        }
        println!();
    });
}
