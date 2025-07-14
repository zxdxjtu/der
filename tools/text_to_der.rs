// Demonstrates that DER has NO text syntax - this tool shows what would happen
// if someone tried to create a "DER source file"

use der::compiler::*;
use der::core::*;
use std::fs::File;

fn main() {
    println!("=== DER Has No Text Syntax Demo ===\n");
    
    println!("Traditional languages have source code like:");
    println!("  Python: print('Hello')")
    println!("  Rust:   println!(\"Hello\");");
    println!("  Java:   System.out.println(\"Hello\");");
    
    println!("\nBut DER has NO text syntax!");
    println!("Instead, you describe intent in natural language:\n");
    
    let intents = vec![
        "print Hello World",
        "add 5 and 3",
        "create an array with 1, 2, 3",
        "multiply 10 by 20 and print the result",
    ];
    
    for intent in intents {
        println!("Intent: \"{}\"", intent);
        
        let mut generator = AICodeGenerator::new();
        match generator.generate_from_prompt(intent) {
            Ok(program) => {
                // Show binary representation
                println!("  → Generates binary graph with {} nodes", program.nodes.len());
                
                // Save to .der file
                let filename = intent.replace(" ", "_") + ".der";
                if let Ok(file) = File::create(&filename) {
                    let mut serializer = DERSerializer::new(file);
                    if serializer.write_program(&program).is_ok() {
                        println!("  → Saved as: {}", filename);
                        
                        // Show file size
                        if let Ok(metadata) = std::fs::metadata(&filename) {
                            println!("  → Binary size: {} bytes", metadata.len());
                        }
                    }
                }
            }
            Err(e) => println!("  → Error: {}", e),
        }
        println!();
    }
    
    println!("Key points:");
    println!("- There is NO der_source.txt file");
    println!("- There is NO DER syntax to learn");
    println!("- AI directly generates binary .der files");
    println!("- Humans use natural language, not code");
}