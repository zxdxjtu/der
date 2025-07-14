// DER to WebAssembly compiler - demonstrates DER as a portable binary format

use der::core::*;
use std::fs::File;
use std::io::Write;

fn main() {
    println!("=== DER to WebAssembly Compiler ===\n");
    
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file.der>", args[0]);
        return;
    }
    
    // Load DER program
    let mut file = File::open(&args[1]).unwrap();
    let mut deserializer = DERDeserializer::new(file);
    let program = deserializer.read_program().unwrap();
    
    println!("Compiling {} to WebAssembly...", args[1]);
    println!("  Nodes: {}", program.nodes.len());
    
    // Generate WASM
    let wasm = compile_to_wasm(&program);
    
    // Save .wat (WebAssembly Text format for readability)
    let wat_filename = args[1].replace(".der", ".wat");
    std::fs::write(&wat_filename, &wasm).unwrap();
    
    println!("\n✓ Generated {}", wat_filename);
    println!("\nThis shows DER can compile to any target!");
}

fn compile_to_wasm(program: &Program) -> String {
    let mut wat = String::new();
    
    // WASM module header
    wat.push_str("(module\n");
    wat.push_str("  ;; Generated from DER program\n");
    wat.push_str("  (import \"env\" \"print_i32\" (func $print_i32 (param i32)))\n");
    wat.push_str("  (import \"env\" \"print_str\" (func $print_str (param i32 i32)))\n");
    
    // Memory for constants
    wat.push_str("  (memory 1)\n");
    wat.push_str("  (data (i32.const 0)");
    
    // Add string constants
    let mut offset = 0;
    let mut string_offsets = Vec::new();
    for s in &program.constants.strings {
        string_offsets.push(offset);
        wat.push_str(&format!(" \"{}\\00\"", s));
        offset += s.len() + 1;
    }
    wat.push_str(")\n");
    
    // Main function
    wat.push_str("\n  (func $main (export \"main\")\n");
    
    // Compile each node
    for node in &program.nodes {
        compile_node(&mut wat, node, &program.constants, &string_offsets);
    }
    
    wat.push_str("  )\n");
    wat.push_str(")\n");
    
    wat
}

fn compile_node(wat: &mut String, node: &Node, constants: &ConstantPool, string_offsets: &[usize]) {
    wat.push_str(&format!("    ;; Node {} - ", node.result_id));
    
    match OpCode::try_from(node.opcode) {
        Ok(OpCode::ConstInt) => {
            if let Some(val) = constants.get_int(node.args[0]) {
                wat.push_str(&format!("ConstInt {}\n", val));
                wat.push_str(&format!("    i32.const {}\n", val));
            }
        }
        Ok(OpCode::ConstString) => {
            let idx = node.args[0] as usize;
            if idx < string_offsets.len() {
                wat.push_str(&format!("ConstString\n"));
                wat.push_str(&format!("    i32.const {} ;; string offset\n", string_offsets[idx]));
                wat.push_str(&format!("    i32.const {} ;; string length\n", 
                    constants.get_string(node.args[0]).map(|s| s.len()).unwrap_or(0)));
            }
        }
        Ok(OpCode::Add) => {
            wat.push_str("Add\n");
            wat.push_str("    i32.add\n");
        }
        Ok(OpCode::Print) => {
            wat.push_str("Print\n");
            wat.push_str("    call $print_i32\n");
        }
        _ => {
            wat.push_str(&format!("OpCode {:04X} (not implemented)\n", node.opcode));
        }
    }
}