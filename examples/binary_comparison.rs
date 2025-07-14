// This example shows the difference between Rust source and DER binary

use der::core::*;
use der::runtime::*;
use std::fs::{File, write};

fn main() {
    println!("=== Rust Source vs DER Binary ===\n");
    
    // 1. Show Rust source code
    let rust_source = r#"
fn main() {
    println!("Hello from Rust!");
}
"#;
    
    println!("1. Rust source code (hello.rs):");
    println!("{}", rust_source);
    write("hello.rs", rust_source).unwrap();
    
    println!("   Size: {} bytes", rust_source.len());
    println!("   Type: Human-readable text\n");
    
    // 2. Create equivalent DER binary
    let mut program = Program::new();
    let msg_idx = program.constants.add_string("Hello from DER!".to_string());
    let str_node = Node::new(OpCode::ConstString, 1).with_args(&[msg_idx]);
    let print_node = Node::new(OpCode::Print, 2).with_args(&[1]);
    
    program.add_node(str_node);
    let entry = program.add_node(print_node);
    program.set_entry_point(entry);
    program.header.chunk_count = 3;
    
    // Save DER binary
    let file = File::create("hello_der.der").unwrap();
    let mut serializer = DERSerializer::new(file);
    serializer.write_program(&program).unwrap();
    
    println!("2. DER binary (hello_der.der):");
    
    // Read and display binary
    let der_bytes = std::fs::read("hello_der.der").unwrap();
    println!("   Size: {} bytes", der_bytes.len());
    println!("   Type: Binary computational graph");
    println!("\n   First 32 bytes (hex):");
    print!("   ");
    for (i, byte) in der_bytes.iter().take(32).enumerate() {
        if i > 0 && i % 8 == 0 { print!(" "); }
        print!("{:02X}", byte);
    }
    println!("\n");
    
    // 3. Show execution difference
    println!("3. Execution comparison:");
    
    println!("\n   Rust execution:");
    println!("   $ rustc hello.rs -o hello");
    println!("   $ ./hello");
    println!("   Hello from Rust!");
    
    println!("\n   DER execution:");
    println!("   $ der run hello_der.der");
    print!("   ");
    let mut executor = Executor::new(program);
    executor.execute().unwrap();
    
    // 4. Key differences
    println!("\n4. Key Differences:");
    println!("   ┌─────────────────┬──────────────────┬─────────────────┐");
    println!("   │ Aspect          │ Rust             │ DER             │");
    println!("   ├─────────────────┼──────────────────┼─────────────────┤");
    println!("   │ Source format   │ Text (.rs)       │ Binary (.der)   │");
    println!("   │ Syntax          │ Human-readable   │ None (binary)   │");
    println!("   │ Created by      │ Humans           │ AI              │");
    println!("   │ Compilation     │ rustc → binary   │ Already binary  │");
    println!("   │ Execution       │ OS runs binary   │ DER runtime     │");
    println!("   └─────────────────┴──────────────────┴─────────────────┘");
    
    // 5. Show that DER has no source representation
    println!("\n5. Can we show DER 'source code'? NO!");
    println!("   DER programs exist only as binary graphs.");
    println!("   The closest we can show is a visualization:");
    
    let mut renderer = TextRenderer::new(program);
    println!("\n{}", renderer.render());
    
    // Clean up
    std::fs::remove_file("hello.rs").ok();
    std::fs::remove_file("hello_der.der").ok();
}