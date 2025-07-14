use der::core::*;
use der::runtime::*;
use der::visualization::*;
use der::compiler::*;
use der::verification::*;
use std::fs::File;

fn main() {
    println!("=== DER: Comprehensive Demo ===\n");

    // Demo 1: AI-generated program with formal verification
    demo_ai_with_verification();
    
    println!("\n" + "=".repeat(60) + "\n");
    
    // Demo 2: Memory management and safety
    demo_memory_management();
    
    println!("\n" + "=".repeat(60) + "\n");
    
    // Demo 3: Async operations
    demo_async_operations();
    
    println!("\n" + "=".repeat(60) + "\n");
    
    // Demo 4: Complex program with all features
    demo_comprehensive_program();
}

fn demo_ai_with_verification() {
    println!("1. AI-Generated Program with Formal Verification");
    println!("-".repeat(50));
    
    let mut generator = AICodeGenerator::new();
    
    let prompt = "multiply 5 and 8";
    println!("User Intent: \"{}\"", prompt);
    
    match generator.generate_from_prompt(prompt) {
        Ok(program) => {
            // Verify the program
            let verifier = Verifier::new(program.clone());
            let verification_result = verifier.verify_program();
            
            println!("\nVerification Result:");
            println!("  Valid: {}", verification_result.is_valid);
            println!("  Errors: {}", verification_result.errors.len());
            println!("  Warnings: {}", verification_result.warnings.len());
            
            // Check safety
            let safety = verifier.verify_safety();
            println!("\nSafety Analysis:");
            println!("  Memory Safe: {}", safety.memory_safe);
            println!("  Deterministic: {}", safety.deterministic);
            println!("  Has Unsafe Operations: {}", safety.has_unsafe_operations);
            
            // Execute
            let mut executor = Executor::new(program.clone());
            match executor.execute() {
                Ok(result) => println!("\nExecution Result: {}", result.to_string()),
                Err(e) => println!("\nExecution Error: {}", e),
            }
            
            // Show traits
            if !program.metadata.traits.is_empty() {
                println!("\nProgram Traits:");
                for trait_def in &program.metadata.traits {
                    println!("  - {}", trait_def.name);
                }
            }
        }
        Err(e) => println!("Generation error: {}", e),
    }
}

fn demo_memory_management() {
    println!("2. Memory Management and Reference Counting");
    println!("-".repeat(50));
    
    let mut program = Program::new();
    
    // Allocate memory for a counter
    let size_idx = program.constants.add_int(8);
    let init_idx = program.constants.add_int(0);
    
    let size = Node::new(OpCode::ConstInt, 1).with_args(&[size_idx]);
    let init = Node::new(OpCode::ConstInt, 2).with_args(&[init_idx]);
    let alloc = Node::new(OpCode::Alloc, 3).with_args(&[1, 2]);
    
    // Increment counter 3 times
    let one_idx = program.constants.add_int(1);
    let one = Node::new(OpCode::ConstInt, 4).with_args(&[one_idx]);
    
    // Load, add 1, store (3 times)
    let mut next_id = 5;
    let mut last_store = 3; // Start with alloc node
    
    for i in 0..3 {
        let load = Node::new(OpCode::Load, next_id).with_args(&[3]); // Always load from original alloc
        let add = Node::new(OpCode::Add, next_id + 1).with_args(&[next_id, 4]);
        let store = Node::new(OpCode::Store, next_id + 2).with_args(&[3, next_id + 1]);
        
        program.add_node(load);
        program.add_node(add);
        last_store = program.add_node(store) as u32;
        next_id += 3;
    }
    
    // Final load
    let final_load = Node::new(OpCode::Load, next_id).with_args(&[3]);
    
    // Add all nodes
    program.add_node(size);
    program.add_node(init);
    program.add_node(alloc);
    program.add_node(one);
    let result = program.add_node(final_load);
    program.set_entry_point(result);
    
    println!("Program: Allocate counter, increment 3 times");
    
    let mut executor = Executor::new(program);
    match executor.execute() {
        Ok(result) => {
            println!("Final counter value: {}", result.to_string());
            
            // Show memory stats
            let stats = executor.context.memory.get_stats();
            println!("\nMemory Statistics:");
            println!("  Total Allocated: {} bytes", stats.total_allocated);
            println!("  Active Objects: {}", stats.active_objects);
            println!("  Total References: {}", stats.total_refs);
        }
        Err(e) => println!("Execution error: {}", e),
    }
}

fn demo_async_operations() {
    println!("3. Asynchronous Operations");
    println!("-".repeat(50));
    
    let mut program = Program::new();
    
    // Simulate two async operations running in parallel
    println!("Program: Two async operations computing in parallel");
    
    // Start two async operations
    let async1 = Node::new(OpCode::AsyncBegin, 1);
    let async2 = Node::new(OpCode::AsyncBegin, 2);
    
    // Simulate computation for first async (factorial of 5)
    let five_idx = program.constants.add_int(5);
    let fact_idx = program.constants.add_int(120); // 5! = 120
    let five = Node::new(OpCode::ConstInt, 3).with_args(&[five_idx]);
    let fact = Node::new(OpCode::ConstInt, 4).with_args(&[fact_idx]);
    
    // Simulate computation for second async (sum 1 to 10)
    let sum_idx = program.constants.add_int(55); // sum(1..10) = 55
    let sum = Node::new(OpCode::ConstInt, 5).with_args(&[sum_idx]);
    
    // Complete both async operations
    let complete1 = Node::new(OpCode::AsyncComplete, 6).with_args(&[1, 4]);
    let complete2 = Node::new(OpCode::AsyncComplete, 7).with_args(&[2, 5]);
    
    // Await both results
    let await1 = Node::new(OpCode::AsyncAwait, 8).with_args(&[1]);
    let await2 = Node::new(OpCode::AsyncAwait, 9).with_args(&[2]);
    
    // Add the results
    let add = Node::new(OpCode::Add, 10).with_args(&[8, 9]);
    
    program.add_node(async1);
    program.add_node(async2);
    program.add_node(five);
    program.add_node(fact);
    program.add_node(sum);
    program.add_node(complete1);
    program.add_node(complete2);
    program.add_node(await1);
    program.add_node(await2);
    let result = program.add_node(add);
    program.set_entry_point(result);
    
    println!("  Async 1: Computing factorial(5) = 120");
    println!("  Async 2: Computing sum(1..10) = 55");
    
    let mut executor = Executor::new(program);
    match executor.execute() {
        Ok(result) => println!("\nCombined result: {}", result.to_string()),
        Err(e) => println!("Execution error: {}", e),
    }
}

fn demo_comprehensive_program() {
    println!("4. Comprehensive Program: Map-Reduce with Memory");
    println!("-".repeat(50));
    
    let mut program = Program::new();
    
    // Create an array of numbers
    let nums = vec![10, 20, 30, 40, 50];
    let mut num_nodes = Vec::new();
    
    for (i, &num) in nums.iter().enumerate() {
        let idx = program.constants.add_int(num);
        let node = Node::new(OpCode::ConstInt, (i + 1) as u32).with_args(&[idx]);
        num_nodes.push((i + 1) as u32);
        program.add_node(node);
    }
    
    // Create array
    let mut array_args = num_nodes[..3].to_vec(); // DER nodes support max 3 args
    let array1 = Node::new(OpCode::CreateArray, 6).with_args(&array_args);
    
    // For demonstration, we'll just use the first 3 elements
    program.add_node(array1);
    
    // Allocate memory for accumulator
    let size_idx = program.constants.add_int(8);
    let zero_idx = program.constants.add_int(0);
    let size = Node::new(OpCode::ConstInt, 7).with_args(&[size_idx]);
    let zero = Node::new(OpCode::ConstInt, 8).with_args(&[zero_idx]);
    let accum = Node::new(OpCode::Alloc, 9).with_args(&[7, 8]);
    
    program.add_node(size);
    program.add_node(zero);
    program.add_node(accum);
    
    // Map operation: double each element and accumulate
    let two_idx = program.constants.add_int(2);
    let two = Node::new(OpCode::ConstInt, 10).with_args(&[two_idx]);
    program.add_node(two);
    
    let mut next_id = 11;
    for i in 0..3 {
        // Get array element
        let idx_const = program.constants.add_int(i);
        let idx_node = Node::new(OpCode::ConstInt, next_id).with_args(&[idx_const]);
        let get = Node::new(OpCode::ArrayGet, next_id + 1).with_args(&[6, next_id]);
        
        // Double it
        let double = Node::new(OpCode::Mul, next_id + 2).with_args(&[next_id + 1, 10]);
        
        // Load accumulator
        let load = Node::new(OpCode::Load, next_id + 3).with_args(&[9]);
        
        // Add to accumulator
        let add = Node::new(OpCode::Add, next_id + 4).with_args(&[next_id + 3, next_id + 2]);
        
        // Store back
        let store = Node::new(OpCode::Store, next_id + 5).with_args(&[9, next_id + 4]);
        
        program.add_node(idx_node);
        program.add_node(get);
        program.add_node(double);
        program.add_node(load);
        program.add_node(add);
        program.add_node(store);
        
        next_id += 6;
    }
    
    // Final load of accumulator
    let final_load = Node::new(OpCode::Load, next_id).with_args(&[9]);
    let result = program.add_node(final_load);
    program.set_entry_point(result);
    
    // Add metadata
    program.metadata.traits.push(Trait {
        name: "MapReduce".to_string(),
        preconditions: vec!["Input is array of integers".to_string()],
        postconditions: vec!["Result is sum of doubled elements".to_string()],
    });
    
    // Visualize the program
    println!("Program Structure (Mermaid):");
    let graph_renderer = GraphRenderer::new(program.clone());
    let mermaid = graph_renderer.render_to_mermaid();
    // Print first few lines of mermaid diagram
    for line in mermaid.lines().take(10) {
        println!("  {}", line);
    }
    println!("  ... (truncated)");
    
    // Execute
    println!("\nExecution:");
    println!("  Input: {:?}", nums[..3].to_vec());
    println!("  Operation: Double each and sum");
    
    let mut executor = Executor::new(program.clone());
    match executor.execute() {
        Ok(result) => {
            println!("  Result: {}", result.to_string());
            println!("  Expected: {} (10*2 + 20*2 + 30*2)", 10*2 + 20*2 + 30*2);
        }
        Err(e) => println!("  Execution error: {}", e),
    }
    
    // Save to file
    if let Ok(file) = File::create("demo_program.der") {
        let mut serializer = DERSerializer::new(file);
        if serializer.write_program(&program).is_ok() {
            println!("\nProgram saved to: demo_program.der");
        }
    }
}

// Example is in the main function above