use rjvm::{
    decoder::{instructions::parse_instruction, BufferedReader},
    types::{attributes::CodeInfo, constants::ConstantPool, elements::ClassFile},
};

fn main() {
    let input = include_bytes!("./testdata/org/example/Simple.class");
    let mut buffer = BufferedReader::new(input);
    let mut constant_pool = ConstantPool::new();
    let cf = ClassFile::decode(&mut buffer, &mut constant_pool);

    let cf = cf.expect("class file should be decoded");

    println!(
        "Class: {}",
        constant_pool.text_of_value(cf.this_class as usize).unwrap()
    );
    // print all methods and their instructions
    cf.methods.iter().for_each(|method| {
        let attr = method
            .attributes
            .iter()
            .find(|attr| attr.get::<CodeInfo>().is_some());
        println!("Name: {}()", method.name);

        let code_attr = attr.unwrap().get::<CodeInfo>().unwrap();
        // create a new buffer that contains just the code and nothing else...
        let mut code_reader = BufferedReader::new(&code_attr.code);
        while !code_reader.has_remaining_data() {
            let opcode = code_reader.take::<u8>().unwrap();
            let instr =
                parse_instruction(opcode, &mut code_reader).expect("instruction should be parsed");
            println!("  0x{opcode:02x}: {}", instr);
        }
        println!();
    });
}
