// This creates actual .der binary files to demonstrate DER programs

use der::core::*;
use der::runtime::*;
use std::fs::File;

fn main() {
    println!("Creating sample DER binary programs...\n");
    
    // 1. Hello World
    create_hello_world();
    
    // 2. Calculator
    create_calculator();
    
    // 3. Array Operations
    create_array_demo();
    
    println!("\n✓ Created 3 DER binary programs!");
    println!("\nTo inspect them:");
    println!("  hexdump -C hello_world.der");
    println!("  cargo run --bin der run hello_world.der");
    println!("\nThese are NOT text files - they are binary computation graphs!");
}

fn create_hello_world() {
    let mut program = Program::new();
    
    let msg = program.constants.add_string("Hello, DER World!".to_string());
    let str_node = Node::new(OpCode::ConstString, 1).with_args(&[msg]);
    let print = Node::new(OpCode::Print, 2).with_args(&[1]);
    
    program.add_node(str_node);
    let entry = program.add_node(print);
    program.set_entry_point(entry);
    program.header.chunk_count = 3;
    
    save_program("hello_world.der", &program);
}

fn create_calculator() {
    let mut program = Program::new();
    
    // Calculate: (15 + 25) * 2
    let n15 = program.constants.add_int(15);
    let n25 = program.constants.add_int(25);
    let n2 = program.constants.add_int(2);
    
    let node15 = Node::new(OpCode::ConstInt, 1).with_args(&[n15]);
    let node25 = Node::new(OpCode::ConstInt, 2).with_args(&[n25]);
    let add = Node::new(OpCode::Add, 3).with_args(&[1, 2]);
    let node2 = Node::new(OpCode::ConstInt, 4).with_args(&[n2]);
    let mul = Node::new(OpCode::Mul, 5).with_args(&[3, 4]);
    
    // Print result
    let msg = program.constants.add_string("Result: ".to_string());
    let str_node = Node::new(OpCode::ConstString, 6).with_args(&[msg]);
    let print_msg = Node::new(OpCode::Print, 7).with_args(&[6]);
    let print_result = Node::new(OpCode::Print, 8).with_args(&[5]);
    
    program.add_node(node15);
    program.add_node(node25);
    program.add_node(add);
    program.add_node(node2);
    program.add_node(mul);
    program.add_node(str_node);
    program.add_node(print_msg);
    let entry = program.add_node(print_result);
    program.set_entry_point(entry);
    program.header.chunk_count = 3;
    
    save_program("calculator.der", &program);
}

fn create_array_demo() {
    let mut program = Program::new();
    
    // Create array [10, 20, 30]
    let v1 = program.constants.add_int(10);
    let v2 = program.constants.add_int(20);
    let v3 = program.constants.add_int(30);
    
    let n1 = Node::new(OpCode::ConstInt, 1).with_args(&[v1]);
    let n2 = Node::new(OpCode::ConstInt, 2).with_args(&[v2]);
    let n3 = Node::new(OpCode::ConstInt, 3).with_args(&[v3]);
    let arr = Node::new(OpCode::CreateArray, 4).with_args(&[1, 2, 3]);
    
    // Get element at index 1
    let idx = program.constants.add_int(1);
    let idx_node = Node::new(OpCode::ConstInt, 5).with_args(&[idx]);
    let get = Node::new(OpCode::ArrayGet, 6).with_args(&[4, 5]);
    
    // Print
    let msg = program.constants.add_string("Array[1] = ".to_string());
    let str_node = Node::new(OpCode::ConstString, 7).with_args(&[msg]);
    let print_msg = Node::new(OpCode::Print, 8).with_args(&[7]);
    let print_val = Node::new(OpCode::Print, 9).with_args(&[6]);
    
    program.add_node(n1);
    program.add_node(n2);
    program.add_node(n3);
    program.add_node(arr);
    program.add_node(idx_node);
    program.add_node(get);
    program.add_node(str_node);
    program.add_node(print_msg);
    let entry = program.add_node(print_val);
    program.set_entry_point(entry);
    program.header.chunk_count = 3;
    
    save_program("array_demo.der", &program);
}

fn save_program(filename: &str, program: &Program) {
    let file = File::create(filename).unwrap();
    let mut serializer = DERSerializer::new(file);
    serializer.write_program(program).unwrap();
    
    // Show file info
    let metadata = std::fs::metadata(filename).unwrap();
    println!("Created {}: {} bytes", filename, metadata.len());
    
    // Show first few bytes
    let bytes = std::fs::read(filename).unwrap();
    print!("  Header: ");
    for byte in &bytes[..16.min(bytes.len())] {
        print!("{:02X} ", byte);
    }
    println!();
}