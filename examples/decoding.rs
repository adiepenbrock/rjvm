use rjvm::{
    decoder::buffer::BufferedReader,
    types::{constants::ConstantPool, elements::ClassFile},
};

fn main() {
    let input = include_bytes!("./testdata/org/example/Simple.class");
    let mut buffer = BufferedReader::new(input);
    let mut constant_pool = ConstantPool::new();
    let cf = ClassFile::decode(&mut buffer, &mut constant_pool);

    dbg!(cf);
}
