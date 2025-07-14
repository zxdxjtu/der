use der::core::*;
use der::runtime::*;
use der::visualization::*;
use der::compiler::*;

fn main() {
    println!("=== DER: Dynamic Execution Representation Demo ===\n");

    // Example 1: AI generates a simple computation
    println!("Example 1: AI-generated addition");
    demo_ai_generation();
    
    println!("\n" + "=".repeat(50) + "\n");

    // Example 2: Manual program construction
    println!("Example 2: Manual DER program (Fibonacci)");
    demo_manual_construction();
    
    println!("\n" + "=".repeat(50) + "\n");

    // Example 3: Visualization
    println!("Example 3: Program visualization");
    demo_visualization();
}

fn demo_ai_generation() {
    let mut generator = AICodeGenerator::new();
    
    // Simulate AI generating code from natural language
    let prompt = "add 10 and 20";
    println!("AI Prompt: \"{}\"", prompt);
    
    match generator.generate_from_prompt(prompt) {
        Ok(program) => {
            println!("\nGenerated DER program:");
            
            // Show the program structure
            let mut text_renderer = TextRenderer::new(program.clone());
            println!("{}", text_renderer.render());
            
            // Execute the program
            let mut executor = Executor::new(program);
            match executor.execute() {
                Ok(result) => println!("\nExecution result: {}", result.to_string()),
                Err(e) => println!("\nExecution error: {}", e),
            }
        }
        Err(e) => println!("Generation error: {}", e),
    }
}

fn demo_manual_construction() {
    // Create a program that calculates fibonacci(5)
    let mut program = Program::new();
    
    // Constants
    let c0 = program.constants.add_int(0);
    let c1 = program.constants.add_int(1);
    let c2 = program.constants.add_int(2);
    let c5 = program.constants.add_int(5);
    
    // Build fibonacci sequence: fib(5) = fib(4) + fib(3)
    // For simplicity, we'll compute it iteratively
    
    // Initialize: a = 0, b = 1
    let a = Node::new(OpCode::ConstInt, 1).with_args(&[c0]);
    let b = Node::new(OpCode::ConstInt, 2).with_args(&[c1]);
    
    // Step 1: c = a + b (1)
    let step1 = Node::new(OpCode::Add, 3).with_args(&[1, 2]);
    
    // Step 2: c = b + c (2)
    let step2 = Node::new(OpCode::Add, 4).with_args(&[2, 3]);
    
    // Step 3: c = c + step2 (3)
    let step3 = Node::new(OpCode::Add, 5).with_args(&[3, 4]);
    
    // Step 4: c = step2 + step3 (5)
    let step4 = Node::new(OpCode::Add, 6).with_args(&[4, 5]);
    
    // Add nodes to program
    program.add_node(a);
    program.add_node(b);
    program.add_node(step1);
    program.add_node(step2);
    program.add_node(step3);
    let result = program.add_node(step4);
    
    program.set_entry_point(result);
    
    // Add metadata
    program.metadata.traits.push(Trait {
        name: "Fibonacci".to_string(),
        preconditions: vec!["n >= 0".to_string()],
        postconditions: vec!["result is fibonacci(5)".to_string()],
    });
    
    // Show summary
    let text_renderer = TextRenderer::new(program.clone());
    println!("{}", text_renderer.render_summary());
    
    // Execute
    let mut executor = Executor::new(program);
    match executor.execute() {
        Ok(result) => println!("\nFibonacci(5) = {}", result.to_string()),
        Err(e) => println!("\nExecution error: {}", e),
    }
}

fn demo_visualization() {
    // Create a simple computation graph
    let mut program = Program::new();
    
    // Create: (10 + 20) * (30 - 25)
    let c10 = program.constants.add_int(10);
    let c20 = program.constants.add_int(20);
    let c30 = program.constants.add_int(30);
    let c25 = program.constants.add_int(25);
    
    let n10 = Node::new(OpCode::ConstInt, 1).with_args(&[c10]);
    let n20 = Node::new(OpCode::ConstInt, 2).with_args(&[c20]);
    let n30 = Node::new(OpCode::ConstInt, 3).with_args(&[c30]);
    let n25 = Node::new(OpCode::ConstInt, 4).with_args(&[c25]);
    
    let add = Node::new(OpCode::Add, 5).with_args(&[1, 2]);
    let sub = Node::new(OpCode::Sub, 6).with_args(&[3, 4]);
    let mul = Node::new(OpCode::Mul, 7).with_args(&[5, 6]);
    
    program.add_node(n10);
    program.add_node(n20);
    program.add_node(n30);
    program.add_node(n25);
    program.add_node(add);
    program.add_node(sub);
    let result = program.add_node(mul);
    
    program.set_entry_point(result);
    
    // Generate visualizations
    let graph_renderer = GraphRenderer::new(program.clone());
    
    println!("DOT format (for Graphviz):");
    println!("{}", graph_renderer.render_to_dot());
    
    println!("\nMermaid format (for documentation):");
    println!("{}", graph_renderer.render_to_mermaid());
    
    // Text visualization
    let mut text_renderer = TextRenderer::new(program.clone());
    println!("\nText representation:");
    println!("{}", text_renderer.render());
    
    // Execute to verify
    let mut executor = Executor::new(program);
    match executor.execute() {
        Ok(result) => println!("\nResult: {}", result.to_string()),
        Err(e) => println!("\nExecution error: {}", e),
    }
}