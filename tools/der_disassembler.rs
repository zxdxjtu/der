// DER Disassembler - Shows DER binary in a readable format (NOT source code!)

use der::core::*;
use std::fs::File;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file.der>", args[0]);
        return;
    }
    
    println!("=== DER Disassembler ===");
    println!("Note: This is NOT source code - DER has no text syntax!");
    println!("This is a human-readable view of the binary structure.\n");
    
    // Load DER program
    let mut file = File::open(&args[1]).unwrap();
    let mut deserializer = DERDeserializer::new(file);
    
    match deserializer.read_program() {
        Ok(program) => {
            disassemble_program(&program);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn disassemble_program(program: &Program) {
    println!("; DER Binary Format v{}.{}", 
        program.header.version >> 8, 
        program.header.version & 0xFF);
    println!("; Chunks: {}", program.header.chunk_count);
    println!("; Entry Point: Node {}\n", program.metadata.entry_point);
    
    // Metadata section
    if !program.metadata.required_capabilities.is_empty() {
        println!("; Required Capabilities:");
        for cap in &program.metadata.required_capabilities {
            println!(";   - {:?}", cap);
        }
        println!();
    }
    
    if !program.metadata.traits.is_empty() {
        println!("; Program Traits:");
        for t in &program.metadata.traits {
            println!(";   {}", t.name);
            for pre in &t.preconditions {
                println!(";     PRE: {}", pre);
            }
            for post in &t.postconditions {
                println!(";     POST: {}", post);
            }
        }
        println!();
    }
    
    // Constants section
    println!("; === CONSTANTS ===");
    
    for (i, val) in program.constants.integers.iter().enumerate() {
        println!("; .int[{}] = {}", i, val);
    }
    
    for (i, val) in program.constants.floats.iter().enumerate() {
        println!("; .float[{}] = {}", i, val);
    }
    
    for (i, val) in program.constants.strings.iter().enumerate() {
        println!("; .str[{}] = \"{}\"", i, val.escape_default());
    }
    
    for (i, val) in program.constants.booleans.iter().enumerate() {
        println!("; .bool[{}] = {}", i, val);
    }
    
    // Implementation section
    println!("\n; === NODES ===");
    println!("; Format: [Index] ResultID: OPCODE <flags> (args) @timestamp\n");
    
    for (idx, node) in program.nodes.iter().enumerate() {
        print!("[{:03}] R{}: ", idx, node.result_id);
        
        // Opcode
        if let Ok(op) = OpCode::try_from(node.opcode) {
            print!("{:?}", op);
        } else {
            print!("OP_{:04X}", node.opcode);
        }
        
        // Flags
        if node.flags != 0 {
            print!(" <");
            if node.has_flag(NodeFlag::IsAsync) { print!("ASYNC "); }
            if node.has_flag(NodeFlag::IsPure) { print!("PURE "); }
            if node.has_flag(NodeFlag::IsUnsafe) { print!("UNSAFE "); }
            if node.has_flag(NodeFlag::HasSideEffects) { print!("SIDE_EFFECTS "); }
            if node.has_flag(NodeFlag::IsTerminal) { print!("TERMINAL "); }
            if node.has_flag(NodeFlag::IsEntryPoint) { print!("ENTRY "); }
            print!(">");
        }
        
        // Arguments
        if node.arg_count > 0 {
            print!(" (");
            for i in 0..node.arg_count as usize {
                if i > 0 { print!(", "); }
                if node.args[i] == 0 {
                    print!("nil");
                } else {
                    print!("R{}", node.args[i]);
                }
            }
            print!(")");
        }
        
        // Timestamp
        print!(" @{}", node.timestamp);
        
        // Add description for some opcodes
        match OpCode::try_from(node.opcode) {
            Ok(OpCode::ConstInt) => {
                if let Some(val) = program.constants.get_int(node.args[0]) {
                    print!(" ; = {}", val);
                }
            }
            Ok(OpCode::ConstString) => {
                if let Some(val) = program.constants.get_string(node.args[0]) {
                    print!(" ; = \"{}\"", val.escape_default());
                }
            }
            Ok(OpCode::Add) => print!(" ; R{} + R{}", node.args[0], node.args[1]),
            Ok(OpCode::Sub) => print!(" ; R{} - R{}", node.args[0], node.args[1]),
            Ok(OpCode::Mul) => print!(" ; R{} * R{}", node.args[0], node.args[1]),
            Ok(OpCode::Print) => print!(" ; output R{}", node.args[0]),
            _ => {}
        }
        
        println!();
    }
    
    println!("\n; === END ===");
}