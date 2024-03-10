use rjvm::bytecode::pool::ConstantPool;
use rjvm::bytecode::reader::containers::read_classfile;
use rjvm::bytecode::reader::BufferedReader;

fn main() {
    let input = include_bytes!("./testdata/org/example/Simple.class");
    let mut buffer = BufferedReader::new(input);
    let mut constant_pool = ConstantPool::new();
    let cf = read_classfile(&mut buffer, &mut constant_pool).unwrap();

    cf.methods.iter().for_each(|method| {
        dbg!(&method.descriptor);
    })
}
