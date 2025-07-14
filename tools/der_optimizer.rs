// DER Optimizer - Demonstrates graph optimization at binary level

use der::core::*;
use std::fs::File;
use std::collections::{HashMap, HashSet};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file.der>", args[0]);
        return;
    }
    
    println!("=== DER Binary Optimizer ===\n");
    
    // Load program
    let mut file = File::open(&args[1]).unwrap();
    let mut deserializer = DERDeserializer::new(file);
    let mut program = deserializer.read_program().unwrap();
    
    println!("Original program:");
    println!("  Nodes: {}", program.nodes.len());
    println!("  Size: {} bytes", std::fs::metadata(&args[1]).unwrap().len());
    
    // Optimize
    let optimized = optimize_program(&mut program);
    
    println!("\nOptimizations applied:");
    for opt in &optimized {
        println!("  - {}", opt);
    }
    
    // Save optimized version
    let output = args[1].replace(".der", "_optimized.der");
    let file = File::create(&output).unwrap();
    let mut serializer = DERSerializer::new(file);
    serializer.write_program(&program).unwrap();
    
    println!("\nOptimized program:");
    println!("  Nodes: {}", program.nodes.len());
    println!("  Size: {} bytes", std::fs::metadata(&output).unwrap().len());
    println!("  Saved to: {}", output);
}

fn optimize_program(program: &mut Program) -> Vec<String> {
    let mut optimizations = Vec::new();
    
    // 1. Constant folding
    let folded = constant_folding(program);
    if folded > 0 {
        optimizations.push(format!("Constant folding: {} nodes", folded));
    }
    
    // 2. Dead code elimination
    let eliminated = dead_code_elimination(program);
    if eliminated > 0 {
        optimizations.push(format!("Dead code elimination: {} nodes", eliminated));
    }
    
    // 3. Common subexpression elimination
    let cse = common_subexpression_elimination(program);
    if cse > 0 {
        optimizations.push(format!("Common subexpression elimination: {} duplicates", cse));
    }
    
    optimizations
}

fn constant_folding(program: &mut Program) -> usize {
    let mut folded = 0;
    let mut const_values: HashMap<u32, i64> = HashMap::new();
    
    // First pass: identify constant values
    for node in &program.nodes {
        match OpCode::try_from(node.opcode) {
            Ok(OpCode::ConstInt) => {
                if let Some(val) = program.constants.get_int(node.args[0]) {
                    const_values.insert(node.result_id, val);
                }
            }
            _ => {}
        }
    }
    
    // Second pass: fold constant operations
    for i in 0..program.nodes.len() {
        let node = &program.nodes[i];
        match OpCode::try_from(node.opcode) {
            Ok(OpCode::Add) => {
                if let (Some(&a), Some(&b)) = (
                    const_values.get(&node.args[0]),
                    const_values.get(&node.args[1])
                ) {
                    // Replace with constant
                    let result = a + b;
                    let idx = program.constants.add_int(result);
                    program.nodes[i] = Node::new(OpCode::ConstInt, node.result_id)
                        .with_args(&[idx]);
                    const_values.insert(node.result_id, result);
                    folded += 1;
                }
            }
            Ok(OpCode::Mul) => {
                if let (Some(&a), Some(&b)) = (
                    const_values.get(&node.args[0]),
                    const_values.get(&node.args[1])
                ) {
                    let result = a * b;
                    let idx = program.constants.add_int(result);
                    program.nodes[i] = Node::new(OpCode::ConstInt, node.result_id)
                        .with_args(&[idx]);
                    const_values.insert(node.result_id, result);
                    folded += 1;
                }
            }
            _ => {}
        }
    }
    
    folded
}

fn dead_code_elimination(program: &mut Program) -> usize {
    let mut used_nodes = HashSet::new();
    let mut to_visit = vec![program.metadata.entry_point as u32];
    
    // Mark all reachable nodes
    while let Some(node_idx) = to_visit.pop() {
        if used_nodes.insert(node_idx) {
            if let Some(node) = program.nodes.iter()
                .find(|n| n.result_id == node_idx) {
                for i in 0..node.arg_count as usize {
                    if node.args[i] != 0 {
                        to_visit.push(node.args[i]);
                    }
                }
            }
        }
    }
    
    // Remove unused nodes
    let original_len = program.nodes.len();
    program.nodes.retain(|node| used_nodes.contains(&node.result_id));
    
    original_len - program.nodes.len()
}

fn common_subexpression_elimination(program: &mut Program) -> usize {
    let mut eliminated = 0;
    let mut expr_map: HashMap<(u16, [u32; 3]), u32> = HashMap::new();
    
    for i in 0..program.nodes.len() {
        let node = &program.nodes[i];
        let key = (node.opcode, node.args);
        
        if let Some(&existing_id) = expr_map.get(&key) {
            // Found duplicate - update references
            let old_id = node.result_id;
            for j in 0..program.nodes.len() {
                for k in 0..3 {
                    if program.nodes[j].args[k] == old_id {
                        program.nodes[j].args[k] = existing_id;
                    }
                }
            }
            eliminated += 1;
        } else {
            expr_map.insert(key, node.result_id);
        }
    }
    
    eliminated
}