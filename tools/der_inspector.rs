// DER Binary Inspector - Shows the actual binary structure of .der files

use std::fs::File;
use std::io::Read;
use der::core::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file.der>", args[0]);
        return;
    }

    println!("=== DER Binary Inspector ===\n");
    
    // Read raw bytes
    match std::fs::read(&args[1]) {
        Ok(bytes) => {
            println!("File: {}", args[1]);
            println!("Size: {} bytes\n", bytes.len());
            
            // Show header
            println!("Header (16 bytes):");
            println!("  Magic: {} {} {} {} ({})", 
                bytes[0], bytes[1], bytes[2], bytes[3],
                String::from_utf8_lossy(&bytes[0..4]));
            
            let version = u16::from_le_bytes([bytes[4], bytes[5]]);
            println!("  Version: {}.{}", version >> 8, version & 0xFF);
            
            let flags = u16::from_le_bytes([bytes[6], bytes[7]]);
            println!("  Flags: 0x{:04X}", flags);
            
            let chunks = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
            println!("  Chunks: {}", chunks);
            
            println!("\nRaw hex dump (first 64 bytes):");
            for i in 0..bytes.len().min(64) {
                if i % 16 == 0 {
                    print!("\n  {:04X}: ", i);
                }
                print!("{:02X} ", bytes[i]);
            }
            println!("\n");
            
            // Try to parse as DER
            let mut file = File::open(&args[1]).unwrap();
            let mut deserializer = DERDeserializer::new(file);
            
            match deserializer.read_program() {
                Ok(program) => {
                    println!("Successfully parsed DER program!");
                    println!("  Nodes: {}", program.nodes.len());
                    println!("  Entry point: Node {}", program.metadata.entry_point);
                    
                    // Show nodes
                    println!("\nNodes:");
                    for (i, node) in program.nodes.iter().enumerate() {
                        println!("  [{}] OpCode: 0x{:04X}, Result: {}, Args: {} [{:?}]",
                            i, node.opcode, node.result_id, node.arg_count,
                            &node.args[..node.arg_count as usize]);
                    }
                    
                    // Show constants
                    println!("\nConstants:");
                    for (i, s) in program.constants.strings.iter().enumerate() {
                        println!("  String[{}]: \"{}\"", i, s);
                    }
                    for (i, n) in program.constants.integers.iter().enumerate() {
                        println!("  Int[{}]: {}", i, n);
                    }
                }
                Err(e) => {
                    println!("Error parsing DER: {}", e);
                }
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}