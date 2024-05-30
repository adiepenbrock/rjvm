use rjvm::bytecode::attributes::{
    element_value_string, Container, MethodParametersInfo, RuntimeInvisibleAnnotationsInfo,
};
use rjvm::bytecode::pool::ConstantPool;
use rjvm::bytecode::reader::attributes::{
    CodeAttributeFactory, ConstantValueAttributeFactory, InnerClassesAttributeFactory,
    LineNumberTableAttributeFactory, LocalVariableTableAttributeFactory,
    MethodParametersAttributeFactory, NestMembersAttributeFactory,
    RuntimeInvisibleAnnotationsAttributeFactory, SourceFileAttributeFactory,
};
use rjvm::bytecode::reader::containers::read_classfile;
use rjvm::bytecode::reader::BufferedReader;
use rjvm::bytecode::{Descriptor, DescriptorKind};
use rjvm::{Annotation, Method, Parameter, TypeRef};

fn main() {
    let input = include_bytes!("./testdata/org/example/Simple.class");

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
    let mut cp = ConstantPool::new();
    let cf = read_classfile(&mut buffer, &mut cp, &container).unwrap();

    println!("Class name: {}", cp.text_of(cf.this_class).unwrap());

    cf.fields.iter().for_each(|field| {
        println!("Field {} - {}", field.name, field.descriptor);
    });

    let methods = cf
        .methods
        .iter()
        .map(|method| {
            let attr_params: Vec<String> = method
                .attributes
                .get("MethodParameters")
                .map(|attr| attr.as_any_ref().downcast_ref::<MethodParametersInfo>())
                .flatten()
                .map(|params| {
                    params
                        .parameters
                        .iter()
                        .map(|param| cp.text_of(param.name_index).unwrap())
                        .collect()
                })
                .unwrap_or(vec![]);

            let desc_params = method
                .descriptor
                .iter()
                .filter(|desc| desc.kind == DescriptorKind::Parameter)
                .collect::<Vec<&Descriptor>>();

            let parameters = desc_params
                .iter()
                .enumerate()
                .map(|(i, desc)| {
                    let name: Option<String> = attr_params.get(i).cloned();
                    let ty = desc.ty.to_string();
                    Parameter {
                        name,
                        ty: TypeRef { name: ty },
                        position: i,
                    }
                })
                .collect();

            let ty: Option<TypeRef> = method
                .descriptor
                .iter()
                .find(|desc| desc.kind == DescriptorKind::Return)
                .map(|desc| TypeRef {
                    name: desc.ty.to_string(),
                });

            let mut annotations: Vec<Annotation> = vec![];
            if let Some(attr) = method
                .get_attribute::<RuntimeInvisibleAnnotationsInfo>("RuntimeInvisibleAnnotations")
            {
                let items: Vec<Annotation> = attr
                    .annotations
                    .iter()
                    .map(|item| {
                        let name = cp.text_of(item.type_index).unwrap();
                        let fields = item.element_value_pairs.iter().map(|pair| {
                            let key = cp.text_of(pair.element_name_index).unwrap();
                            let value = match element_value_string(&pair.value, &cp) {
                                Ok(value) => value,
                                Err(_) => {
                                    // TODO: handle error case
                                    unreachable!()
                                }
                            };
                            (key, value)
                        });
                        Annotation {
                            name,
                            field: fields.collect(),
                        }
                    })
                    .collect();
                annotations.extend(items);
            }

            rjvm::Method {
                name: method.name.clone(),
                annotations,
                modifiers: vec![],
                parameters,
                body: None,
                ty,
            }
        })
        .collect::<Vec<Method>>();

    dbg!(methods);
}
