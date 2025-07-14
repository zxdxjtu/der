use der::core::*;
use der::runtime::*;
use std::fs::File;

fn main() {
    // Create a simple Hello World program
    let mut program = Program::new();
    
    // Add string constant
    let hello_idx = program.constants.add_string("Hello, World!".to_string());
    
    // Create constant string node
    let str_node = Node::new(OpCode::ConstString, 1).with_args(&[hello_idx]);
    
    // Create print node
    let print_node = Node::new(OpCode::Print, 2).with_args(&[1]);
    
    // Add nodes to program
    program.add_node(str_node);
    let result = program.add_node(print_node);
    program.set_entry_point(result);
    
    // Set metadata
    program.header.chunk_count = 3;
    
    // Execute directly
    println!("Executing Hello World program:");
    let mut executor = Executor::new(program.clone());
    match executor.execute() {
        Ok(_) => println!("Program executed successfully"),
        Err(e) => eprintln!("Execution error: {}", e),
    }
    
    // Save to file
    match File::create("hello.der") {
        Ok(file) => {
            let mut serializer = DERSerializer::new(file);
            if serializer.write_program(&program).is_ok() {
                println!("\nProgram saved to hello.der");
            }
        }
        Err(e) => eprintln!("Failed to save: {}", e),
    }
}