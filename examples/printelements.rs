use rjvm::types::descriptors::{Descriptor, DescriptorKind};
use rjvm::{
    decoder::BufferedReader,
    types::{
        constants::ConstantPool,
        descriptors::{BaseType, FieldType},
        elements::ClassFile,
    },
};

fn main() {
    let input = include_bytes!("./testdata/org/example/Simple.class");
    let mut buffer = BufferedReader::new(input);
    let mut cp = ConstantPool::new();
    let cf = ClassFile::decode(&mut buffer, &mut cp).unwrap();

    println!(
        "Class name: {}",
        cp.text_of_value(cf.this_class as usize).unwrap()
    );

    cf.fields.iter().for_each(|field| {
        println!("Field {} - {}", field.name, field.descriptor);
    });

    cf.methods.iter().for_each(|method| {
        let parameters: Vec<Descriptor> = method
            .descriptor
            .clone()
            .into_iter()
            .filter(|desc| desc.kind == DescriptorKind::Parameter)
            .collect();
        let return_ty: Descriptor = method
            .descriptor
            .clone()
            .into_iter()
            .find(|desc| desc.kind == DescriptorKind::Return)
            .unwrap_or(Descriptor {
                kind: DescriptorKind::Return,
                ty: FieldType::Base(BaseType::Void),
            });

        println!(
            "{} {}({})",
            return_ty,
            method.name,
            parameters
                .iter()
                .map(|p| p.ty.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
    });
}
