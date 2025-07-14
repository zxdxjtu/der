// This demonstrates how DER differs from every other programming language

use der::core::*;
use der::runtime::*;
use std::fs::{File, write};

fn main() {
    println!("=== DER vs Other Programming Languages ===\n");
    
    // Show how different languages represent the same program
    let languages = vec![
        ("Python", "hello.py", "print('Hello, World!')"),
        ("JavaScript", "hello.js", "console.log('Hello, World!');"),
        ("Java", "Hello.java", "System.out.println(\"Hello, World!\");"),
        ("C", "hello.c", "printf(\"Hello, World!\\n\");"),
        ("Rust", "hello.rs", "println!(\"Hello, World!\");"),
    ];
    
    println!("Traditional languages use TEXT files:\n");
    
    for (lang, file, code) in &languages {
        println!("{} ({})", lang, file);
        println!("├─ Content: {}", code);
        println!("├─ Type: UTF-8 text");
        println!("├─ Created by: Human typing");
        println!("└─ Size: {} bytes\n", code.len());
    }
    
    println!("DER uses BINARY files:\n");
    
    // Create DER program
    let mut program = Program::new();
    let msg_idx = program.constants.add_string("Hello, World!".to_string());
    let str_node = Node::new(OpCode::ConstString, 1).with_args(&[msg_idx]);
    let print_node = Node::new(OpCode::Print, 2).with_args(&[1]);
    
    program.add_node(str_node);
    let entry = program.add_node(print_node);
    program.set_entry_point(entry);
    program.header.chunk_count = 3;
    
    // Save as binary
    let file = File::create("hello.der").unwrap();
    let mut serializer = DERSerializer::new(file);
    serializer.write_program(&program).unwrap();
    
    let der_bytes = std::fs::read("hello.der").unwrap();
    
    println!("DER (hello.der)");
    println!("├─ Content: [BINARY DATA - Cannot show as text]");
    println!("├─ Type: DER binary format");
    println!("├─ Created by: AI generating computational graph");
    println!("└─ Size: {} bytes", der_bytes.len());
    
    println!("\n┌────────────────────────────────────────┐");
    println!("│  Why DER Has No Source Code Format    │");
    println!("├────────────────────────────────────────┤");
    println!("│ 1. Designed for AI, not humans        │");
    println!("│ 2. Direct binary → faster generation  │");
    println!("│ 3. No parsing errors possible         │");
    println!("│ 4. No syntax debates or style wars    │");
    println!("│ 5. Truly language-agnostic            │");
    println!("└────────────────────────────────────────┘");
    
    println!("\nExecution comparison:");
    println!("\nOther languages: Source → Parse → AST → Compile → Execute");
    println!("DER:            Binary Graph → Execute (no parsing!)");
    
    println!("\n╔═══════════════════════════════════════╗");
    println!("║  DER is NOT a 'compiled language'     ║");
    println!("║  DER is NOT an 'interpreted language' ║");
    println!("║  DER is a 'binary-native language'    ║");
    println!("╚═══════════════════════════════════════╝");
    
    // Show what happens when you try to read DER as text
    println!("\nWhat happens if you try to read hello.der as text?");
    print!("Raw bytes: ");
    for (i, byte) in der_bytes.iter().take(32).enumerate() {
        if i == 16 { print!("\n           "); }
        print!("{:02X} ", byte);
    }
    println!("...\n");
    
    println!("As text: {}", String::from_utf8_lossy(&der_bytes[..32]));
    println!("         ^^^ This is meaningless - DER is not text!");
    
    // Clean up
    std::fs::remove_file("hello.der").ok();
}