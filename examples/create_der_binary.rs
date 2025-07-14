// This Rust program CREATES a .der binary file
// The .der file is the actual DER program

use der::core::*;
use std::fs::File;

fn main() {
    println!("Creating a native DER binary program...\n");
    
    // Build a DER program (not Rust code!)
    let mut program = Program::new();
    
    // Add data to DER program
    let msg_idx = program.constants.add_string("I am a DER program!".to_string());
    let num_idx = program.constants.add_int(42);
    
    // Create DER nodes (computation graph)
    let str_node = Node::new(OpCode::ConstString, 1).with_args(&[msg_idx]);
    let num_node = Node::new(OpCode::ConstInt, 2).with_args(&[num_idx]);
    let print_str = Node::new(OpCode::Print, 3).with_args(&[1]);
    let print_num = Node::new(OpCode::Print, 4).with_args(&[2]);
    
    // Build the DER program
    program.add_node(str_node);
    program.add_node(num_node);
    program.add_node(print_str);
    let final_node = program.add_node(print_num);
    program.set_entry_point(final_node);
    
    // Set DER metadata
    program.header.chunk_count = 3;
    program.header.magic = [0x44, 0x45, 0x52, 0x21]; // "DER!"
    
    // Save as BINARY .der file (NOT Rust source!)
    let file = File::create("example.der").unwrap();
    let mut serializer = DERSerializer::new(file);
    serializer.write_program(&program).unwrap();
    
    println!("✓ Created example.der (binary DER program)");
    println!("\nThis is NOT a Rust file!");
    println!("It's a binary computational graph.");
    
    // Show the binary content
    let bytes = std::fs::read("example.der").unwrap();
    println!("\nFirst 16 bytes of DER binary:");
    for (i, byte) in bytes.iter().take(16).enumerate() {
        print!("{:02X} ", byte);
        if i == 3 { print!("(magic) "); }
    }
    println!("\n\nTotal size: {} bytes", bytes.len());
}