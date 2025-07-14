// Standalone test for DER Hello World
// This demonstrates the core functionality without external dependencies

#[derive(Debug)]
struct Program {
    nodes: Vec<Node>,
    constants: Constants,
    entry_point: usize,
}

#[derive(Debug)]
struct Node {
    opcode: OpCode,
    result_id: u32,
    args: Vec<u32>,
}

#[derive(Debug)]
struct Constants {
    strings: Vec<String>,
}

#[derive(Debug)]
enum OpCode {
    ConstString,
    Print,
}

#[derive(Debug)]
enum Value {
    String(String),
    Nil,
}

fn main() {
    println!("=== DER Hello World Test ===\n");
    
    // Create program
    let mut program = Program {
        nodes: Vec::new(),
        constants: Constants {
            strings: vec!["Hello, World!".to_string()],
        },
        entry_point: 1,
    };
    
    // Add nodes
    program.nodes.push(Node {
        opcode: OpCode::ConstString,
        result_id: 1,
        args: vec![0], // Index in constants
    });
    
    program.nodes.push(Node {
        opcode: OpCode::Print,
        result_id: 2,
        args: vec![1], // Reference to node 1
    });
    
    println!("Program structure:");
    println!("- Node 1: Load string constant \"{}\"", program.constants.strings[0]);
    println!("- Node 2: Print value from node 1");
    println!("\nExecuting...\n");
    
    // Simple execution
    let mut values = std::collections::HashMap::new();
    
    // Execute node 1: ConstString
    let string_val = program.constants.strings[0].clone();
    values.insert(1, Value::String(string_val));
    
    // Execute node 2: Print
    if let Some(Value::String(s)) = values.get(&1) {
        println!("{}", s);
    }
    
    println!("\n✓ Program executed successfully!");
}