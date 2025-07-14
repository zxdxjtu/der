use crate::core::*;
use crate::runtime::*;
use std::io::Cursor;
use tempfile::NamedTempFile;
use std::fs::File;

#[test]
fn test_full_pipeline() {
    // Create a program that calculates factorial of 5
    let mut program = create_factorial_program();
    
    // Serialize to file
    let temp_file = NamedTempFile::new().unwrap();
    let file = File::create(temp_file.path()).unwrap();
    let mut serializer = DERSerializer::new(file);
    serializer.write_program(&program).unwrap();
    
    // Deserialize from file
    let file = File::open(temp_file.path()).unwrap();
    let mut deserializer = DERDeserializer::new(file);
    let loaded_program = deserializer.read_program().unwrap();
    
    // Execute the loaded program
    let mut executor = Executor::new(loaded_program);
    let result = executor.execute().unwrap();
    
    // Factorial of 5 is 120
    match result {
        Value::Int(120) => {},
        _ => panic!("Expected Int(120), got {:?}", result),
    }
}

fn create_factorial_program() -> Program {
    let mut program = Program::new();
    
    // Constants
    let c0 = program.constants.add_int(0);
    let c1 = program.constants.add_int(1);
    let c5 = program.constants.add_int(5);
    
    // Node IDs for clarity
    let n_input = 1;      // Input value (5)
    let n_zero = 2;       // Constant 0
    let n_one = 3;        // Constant 1
    let n_eq_check = 4;   // Check if input == 0
    let n_branch = 5;     // Branch on condition
    let n_sub = 6;        // input - 1
    let n_recurse = 7;    // factorial(input - 1)
    let n_multiply = 8;   // input * factorial(input - 1)
    let n_func = 9;       // Function definition
    let n_call = 10;      // Call factorial(5)
    
    // Build the factorial function
    // if (n == 0) return 1; else return n * factorial(n - 1)
    
    let input = Node::new(OpCode::ConstInt, n_input).with_args(&[c5]);
    let zero = Node::new(OpCode::ConstInt, n_zero).with_args(&[c0]);
    let one = Node::new(OpCode::ConstInt, n_one).with_args(&[c1]);
    let eq_check = Node::new(OpCode::Eq, n_eq_check).with_args(&[n_input, n_zero]);
    
    // For the recursive case
    let sub_one = Node::new(OpCode::Sub, n_sub).with_args(&[n_input, n_one]);
    let recurse = Node::new(OpCode::Call, n_recurse).with_args(&[n_func, n_sub]);
    let multiply = Node::new(OpCode::Mul, n_multiply).with_args(&[n_input, n_recurse]);
    
    // Branch: if eq_check then one else multiply
    let branch = Node::new(OpCode::Branch, n_branch).with_args(&[n_eq_check, n_one, n_multiply]);
    
    // Define the factorial function
    let func = Node::new(OpCode::DefineFunc, n_func).with_args(&[n_branch, 1]);
    
    // Call factorial(5)
    let call = Node::new(OpCode::Call, n_call).with_args(&[n_func, n_input]);
    
    // Add all nodes
    program.add_node(input);
    program.add_node(zero);
    program.add_node(one);
    program.add_node(eq_check);
    program.add_node(branch);
    program.add_node(sub_one);
    program.add_node(recurse);
    program.add_node(multiply);
    program.add_node(func);
    program.add_node(call);
    
    program.set_entry_point(n_call - 1); // Adjust for 0-based indexing
    program.header.chunk_count = 3; // META, IMPL, CNST
    
    program
}

#[test]
fn test_capabilities() {
    let mut program = Program::new();
    
    // Try to use Print without capability
    let str_idx = program.constants.add_string("Hello".to_string());
    let str_node = Node::new(OpCode::ConstString, 1).with_args(&[str_idx]);
    let print_node = Node::new(OpCode::Print, 2).with_args(&[1]);
    
    program.add_node(str_node);
    let result = program.add_node(print_node);
    program.set_entry_point(result);
    
    // This should work since Print doesn't require special capabilities
    let mut executor = Executor::new(program);
    let result = executor.execute();
    assert!(result.is_ok());
}

#[test]
fn test_node_caching() {
    let mut program = Program::new();
    
    // Create a computation that would be expensive if repeated
    let c10 = program.constants.add_int(10);
    let c20 = program.constants.add_int(20);
    
    let n10 = Node::new(OpCode::ConstInt, 1).with_args(&[c10]);
    let n20 = Node::new(OpCode::ConstInt, 2).with_args(&[c20]);
    let add = Node::new(OpCode::Add, 3).with_args(&[1, 2]);
    
    // Use the result multiple times
    let mul1 = Node::new(OpCode::Mul, 4).with_args(&[3, 3]); // result * result
    let mul2 = Node::new(OpCode::Mul, 5).with_args(&[3, 4]); // result * (result * result)
    
    program.add_node(n10);
    program.add_node(n20);
    program.add_node(add);
    program.add_node(mul1);
    let result = program.add_node(mul2);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    // (10 + 20) = 30, 30 * 30 = 900, 30 * 900 = 27000
    match result {
        Value::Int(27000) => {},
        _ => panic!("Expected Int(27000), got {:?}", result),
    }
}

#[test]
fn test_error_propagation() {
    let mut program = Program::new();
    
    // Create a program with an invalid array access
    let arr = Node::new(OpCode::CreateArray, 1); // Empty array
    let idx = program.constants.add_int(0);
    let idx_node = Node::new(OpCode::ConstInt, 2).with_args(&[idx]);
    let get = Node::new(OpCode::ArrayGet, 3).with_args(&[1, 2]);
    
    program.add_node(arr);
    program.add_node(idx_node);
    let result = program.add_node(get);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute();
    
    assert!(matches!(result, Err(RuntimeError::ArrayIndexOutOfBounds { .. })));
}

#[test]
fn test_mixed_types() {
    let mut program = Program::new();
    
    // Test mixed int/float arithmetic
    let int_idx = program.constants.add_int(10);
    let float_idx = program.constants.add_float(2.5);
    
    let int_node = Node::new(OpCode::ConstInt, 1).with_args(&[int_idx]);
    let float_node = Node::new(OpCode::ConstFloat, 2).with_args(&[float_idx]);
    let mul = Node::new(OpCode::Mul, 3).with_args(&[1, 2]);
    
    program.add_node(int_node);
    program.add_node(float_node);
    let result = program.add_node(mul);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::Float(f) if (f - 25.0).abs() < 0.001 => {},
        _ => panic!("Expected Float(25.0), got {:?}", result),
    }
}