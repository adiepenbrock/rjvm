# rjvm

`rjvm` is a Rust crate that enables parsing of [JVM class files](https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-2.html#jvms-2.1). This crate supports Java at least up to Java SE 21.
**The scope of this crate is not to create a JVM, but to parse and write (in the future) JVM class files**.

## Getting Started

To integrate `rjvm` into you project, simply add it as a dependency to your `Cargo.toml` file:
```toml
[dependencies]
rjvm = "0.1.0"
```

To parse a class file, follow these steps:

- Read the class file into a byte array
- Create a `BufferedReader` from the byte array
- Initialize a mutable `ConstantPool` to store the constant pool entries
- Parse the ClassFile using the `ClassFile::decode` method.

```rust
let file = include_bytes!("../path/to/your/class/file.class");
let mut buffer = rjvm::decoder::BufferedReader::new(file);
let mut constant_pool = rjvm::types::constants::ConstantPool::new();
let class_file = rjvm::types::elements::ClassFile::decode(&mut buffer, &mut constant_pool);
```

## Examples
Find some simple examples on how to use `rjvm` in the `examples` directory of this repository.

- [`decoding.rs`](https://github.com/adiepenbrock/rjvm/blob/main/examples/decoding.rs): shows an example of how to parse a class file.
- [`instructions.rs`](https://github.com/adiepenbrock/rjvm/blob/main/examples/instructions.rs): shows an example of how to parse a class file and print all methods with their instructions.

## Roadmap
- [x] Parse class files with all related elements
- [x] Manage constant pools
- [ ] Read JAR files
- [ ] Write class files
