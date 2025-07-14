use der::core::*;
use der::runtime::*;
use der::visualization::*;
use std::fs::File;

fn main() {
    // Create a bubble sort program
    // Note: Without loop opcodes, we'll demonstrate a single pass
    let mut program = Program::new();
    
    // Create array: [5, 2, 8, 1, 9]
    let values = vec![5i64, 2, 8, 1, 9];
    let mut const_indices = Vec::new();
    let mut node_ids = Vec::new();
    
    // Create constants and nodes for each value
    for (i, &val) in values.iter().enumerate() {
        let idx = program.constants.add_int(val);
        const_indices.push(idx);
        let node = Node::new(OpCode::ConstInt, (i + 1) as u32).with_args(&[idx]);
        program.add_node(node);
        node_ids.push((i + 1) as u32);
    }
    
    // Create array (max 3 args per node)
    let array_node = Node::new(OpCode::CreateArray, 6)
        .with_args(&node_ids[..3]); // Just use first 3 for demo
    program.add_node(array_node);
    
    // Print original array
    let msg1_idx = program.constants.add_string("Original array:".to_string());
    let msg1_node = Node::new(OpCode::ConstString, 7).with_args(&[msg1_idx]);
    let print1 = Node::new(OpCode::Print, 8).with_args(&[7]);
    let print_arr = Node::new(OpCode::Print, 9).with_args(&[6]);
    
    program.add_node(msg1_node);
    program.add_node(print1);
    program.add_node(print_arr);
    
    // Demonstrate one comparison and swap
    // Get elements at index 0 and 1
    let idx0 = program.constants.add_int(0);
    let idx1 = program.constants.add_int(1);
    let idx0_node = Node::new(OpCode::ConstInt, 10).with_args(&[idx0]);
    let idx1_node = Node::new(OpCode::ConstInt, 11).with_args(&[idx1]);
    
    let elem0 = Node::new(OpCode::ArrayGet, 12).with_args(&[6, 10]);
    let elem1 = Node::new(OpCode::ArrayGet, 13).with_args(&[6, 11]);
    
    // Compare elements
    let compare = Node::new(OpCode::Gt, 14).with_args(&[12, 13]);
    
    program.add_node(idx0_node);
    program.add_node(idx1_node);
    program.add_node(elem0);
    program.add_node(elem1);
    let result = program.add_node(compare);
    
    // Print comparison result
    let msg2_idx = program.constants.add_string("\nFirst element > Second element:".to_string());
    let msg2_node = Node::new(OpCode::ConstString, 15).with_args(&[msg2_idx]);
    let print2 = Node::new(OpCode::Print, 16).with_args(&[15]);
    let print_result = Node::new(OpCode::Print, 17).with_args(&[14]);
    
    program.add_node(msg2_node);
    program.add_node(print2);
    let final_node = program.add_node(print_result);
    
    program.set_entry_point(final_node);
    program.header.chunk_count = 3;
    
    // Add metadata
    program.metadata.traits.push(Trait {
        name: "BubbleSortDemo".to_string(),
        preconditions: vec!["Input is array of integers".to_string()],
        postconditions: vec!["Demonstrates comparison logic".to_string()],
    });
    
    // Visualize
    println!("Bubble Sort Demo Program Structure:");
    let text_renderer = TextRenderer::new(program.clone());
    println!("{}", text_renderer.render_summary());
    
    // Execute
    println!("\nExecuting program:");
    println!("-".repeat(40));
    let mut executor = Executor::new(program.clone());
    match executor.execute() {
        Ok(_) => println!("\nProgram executed successfully"),
        Err(e) => eprintln!("Execution error: {}", e),
    }
    
    // Save to file
    match File::create("bubble_sort.der") {
        Ok(file) => {
            let mut serializer = DERSerializer::new(file);
            if serializer.write_program(&program).is_ok() {
                println!("\nProgram saved to bubble_sort.der");
            }
        }
        Err(e) => eprintln!("Failed to save: {}", e),
    }
}