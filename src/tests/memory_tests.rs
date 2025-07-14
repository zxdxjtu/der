use crate::core::*;
use crate::runtime::*;

#[test]
fn test_memory_allocation() {
    let mut program = Program::new();
    
    // Allocate 100 bytes
    let size_idx = program.constants.add_int(100);
    let size_node = Node::new(OpCode::ConstInt, 1).with_args(&[size_idx]);
    let alloc_node = Node::new(OpCode::Alloc, 2).with_args(&[1]);
    
    program.add_node(size_node);
    let result = program.add_node(alloc_node);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::MemoryRef(ref_val) => {
            assert!(ref_val.address > 0);
            assert_eq!(ref_val.offset, 0);
        }
        _ => panic!("Expected MemoryRef, got {:?}", result),
    }
}

#[test]
fn test_memory_store_and_load() {
    let mut program = Program::new();
    
    // Allocate memory
    let size_idx = program.constants.add_int(8);
    let size_node = Node::new(OpCode::ConstInt, 1).with_args(&[size_idx]);
    let alloc_node = Node::new(OpCode::Alloc, 2).with_args(&[1]);
    
    // Value to store
    let value_idx = program.constants.add_int(42);
    let value_node = Node::new(OpCode::ConstInt, 3).with_args(&[value_idx]);
    
    // Store value
    let store_node = Node::new(OpCode::Store, 4).with_args(&[2, 3]);
    
    // Load value
    let load_node = Node::new(OpCode::Load, 5).with_args(&[2]);
    
    program.add_node(size_node);
    program.add_node(alloc_node);
    program.add_node(value_node);
    program.add_node(store_node);
    let result = program.add_node(load_node);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::Int(42) => {},
        _ => panic!("Expected Int(42), got {:?}", result),
    }
}

#[test]
fn test_memory_free() {
    let mut program = Program::new();
    
    // Allocate memory
    let size_idx = program.constants.add_int(8);
    let size_node = Node::new(OpCode::ConstInt, 1).with_args(&[size_idx]);
    let alloc_node = Node::new(OpCode::Alloc, 2).with_args(&[1]);
    
    // Free memory
    let free_node = Node::new(OpCode::Free, 3).with_args(&[2]);
    
    // Try to load from freed memory (should fail)
    let load_node = Node::new(OpCode::Load, 4).with_args(&[2]);
    
    program.add_node(size_node);
    program.add_node(alloc_node);
    program.add_node(free_node);
    let result = program.add_node(load_node);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute();
    
    // Should error when accessing freed memory
    assert!(result.is_err());
    match result {
        Err(RuntimeError::InvalidOperation(msg)) => {
            assert!(msg.contains("freed memory"));
        }
        _ => panic!("Expected InvalidOperation error"),
    }
}

#[test]
fn test_memory_with_initial_value() {
    let mut program = Program::new();
    
    // Size and initial value
    let size_idx = program.constants.add_int(8);
    let init_idx = program.constants.add_string("Hello".to_string());
    
    let size_node = Node::new(OpCode::ConstInt, 1).with_args(&[size_idx]);
    let init_node = Node::new(OpCode::ConstString, 2).with_args(&[init_idx]);
    let alloc_node = Node::new(OpCode::Alloc, 3).with_args(&[1, 2]);
    
    // Load value
    let load_node = Node::new(OpCode::Load, 4).with_args(&[3]);
    
    program.add_node(size_node);
    program.add_node(init_node);
    program.add_node(alloc_node);
    let result = program.add_node(load_node);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::String(s) if s == "Hello" => {},
        _ => panic!("Expected String(Hello), got {:?}", result),
    }
}

#[test]
fn test_multiple_allocations() {
    let mut program = Program::new();
    
    // Allocate three memory blocks
    let size_idx = program.constants.add_int(8);
    let size_node = Node::new(OpCode::ConstInt, 1).with_args(&[size_idx]);
    
    let alloc1 = Node::new(OpCode::Alloc, 2).with_args(&[1]);
    let alloc2 = Node::new(OpCode::Alloc, 3).with_args(&[1]);
    let alloc3 = Node::new(OpCode::Alloc, 4).with_args(&[1]);
    
    // Store different values
    let val1_idx = program.constants.add_int(10);
    let val2_idx = program.constants.add_int(20);
    let val3_idx = program.constants.add_int(30);
    
    let val1_node = Node::new(OpCode::ConstInt, 5).with_args(&[val1_idx]);
    let val2_node = Node::new(OpCode::ConstInt, 6).with_args(&[val2_idx]);
    let val3_node = Node::new(OpCode::ConstInt, 7).with_args(&[val3_idx]);
    
    let store1 = Node::new(OpCode::Store, 8).with_args(&[2, 5]);
    let store2 = Node::new(OpCode::Store, 9).with_args(&[3, 6]);
    let store3 = Node::new(OpCode::Store, 10).with_args(&[4, 7]);
    
    // Load middle value
    let load = Node::new(OpCode::Load, 11).with_args(&[3]);
    
    program.add_node(size_node);
    program.add_node(alloc1);
    program.add_node(alloc2);
    program.add_node(alloc3);
    program.add_node(val1_node);
    program.add_node(val2_node);
    program.add_node(val3_node);
    program.add_node(store1);
    program.add_node(store2);
    program.add_node(store3);
    let result = program.add_node(load);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::Int(20) => {},
        _ => panic!("Expected Int(20), got {:?}", result),
    }
}

#[test]
fn test_invalid_allocation_size() {
    let mut program = Program::new();
    
    // Try to allocate with negative size
    let size_idx = program.constants.add_int(-10);
    let size_node = Node::new(OpCode::ConstInt, 1).with_args(&[size_idx]);
    let alloc_node = Node::new(OpCode::Alloc, 2).with_args(&[1]);
    
    program.add_node(size_node);
    let result = program.add_node(alloc_node);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute();
    
    assert!(result.is_err());
    match result {
        Err(RuntimeError::TypeMismatch { expected, .. }) => {
            assert!(expected.contains("positive integer"));
        }
        _ => panic!("Expected TypeMismatch error"),
    }
}

#[test]
fn test_memory_update() {
    let mut program = Program::new();
    
    // Allocate and store initial value
    let size_idx = program.constants.add_int(8);
    let size_node = Node::new(OpCode::ConstInt, 1).with_args(&[size_idx]);
    let alloc_node = Node::new(OpCode::Alloc, 2).with_args(&[1]);
    
    let val1_idx = program.constants.add_int(100);
    let val1_node = Node::new(OpCode::ConstInt, 3).with_args(&[val1_idx]);
    let store1 = Node::new(OpCode::Store, 4).with_args(&[2, 3]);
    
    // Update with new value
    let val2_idx = program.constants.add_int(200);
    let val2_node = Node::new(OpCode::ConstInt, 5).with_args(&[val2_idx]);
    let store2 = Node::new(OpCode::Store, 6).with_args(&[2, 5]);
    
    // Load final value
    let load = Node::new(OpCode::Load, 7).with_args(&[2]);
    
    program.add_node(size_node);
    program.add_node(alloc_node);
    program.add_node(val1_node);
    program.add_node(store1);
    program.add_node(val2_node);
    program.add_node(store2);
    let result = program.add_node(load);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::Int(200) => {},
        _ => panic!("Expected Int(200), got {:?}", result),
    }
}

#[test]
fn test_memory_type_operations() {
    let mut program = Program::new();
    
    // Test with invalid memory operations
    let not_a_ref_idx = program.constants.add_int(42);
    let not_a_ref = Node::new(OpCode::ConstInt, 1).with_args(&[not_a_ref_idx]);
    
    // Try to free a non-memory-ref value
    let free_node = Node::new(OpCode::Free, 2).with_args(&[1]);
    
    program.add_node(not_a_ref);
    let result = program.add_node(free_node);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute();
    
    assert!(result.is_err());
    match result {
        Err(RuntimeError::TypeMismatch { expected, actual }) => {
            assert_eq!(expected, "memory reference");
            assert_eq!(actual, "int");
        }
        _ => panic!("Expected TypeMismatch error"),
    }
}