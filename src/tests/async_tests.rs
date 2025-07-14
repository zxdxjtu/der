use crate::core::*;
use crate::runtime::*;

#[test]
fn test_async_begin() {
    let mut program = Program::new();
    
    let async_node = Node::new(OpCode::AsyncBegin, 1);
    
    let result = program.add_node(async_node);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::AsyncHandle(handle) => {
            assert!(handle.id > 0);
        }
        _ => panic!("Expected AsyncHandle, got {:?}", result),
    }
}

#[test]
fn test_async_complete() {
    let mut program = Program::new();
    
    // Begin async operation
    let begin_node = Node::new(OpCode::AsyncBegin, 1);
    
    // Value to complete with
    let value_idx = program.constants.add_int(42);
    let value_node = Node::new(OpCode::ConstInt, 2).with_args(&[value_idx]);
    
    // Complete the async operation
    let complete_node = Node::new(OpCode::AsyncComplete, 3).with_args(&[1, 2]);
    
    // Await the result
    let await_node = Node::new(OpCode::AsyncAwait, 4).with_args(&[1]);
    
    program.add_node(begin_node);
    program.add_node(value_node);
    program.add_node(complete_node);
    let result = program.add_node(await_node);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::Int(42) => {},
        _ => panic!("Expected Int(42), got {:?}", result),
    }
}

#[test]
fn test_async_await_pending() {
    let mut program = Program::new();
    
    // Begin async operation
    let begin_node = Node::new(OpCode::AsyncBegin, 1);
    
    // Await without completing (should still be pending)
    let await_node = Node::new(OpCode::AsyncAwait, 2).with_args(&[1]);
    
    program.add_node(begin_node);
    let result = program.add_node(await_node);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    // Should return the handle since it's still pending
    match result {
        Value::AsyncHandle(_) => {},
        _ => panic!("Expected AsyncHandle (pending), got {:?}", result),
    }
}

#[test]
fn test_multiple_async_operations() {
    let mut program = Program::new();
    
    // Begin two async operations
    let begin1 = Node::new(OpCode::AsyncBegin, 1);
    let begin2 = Node::new(OpCode::AsyncBegin, 2);
    
    // Values to complete with
    let val1_idx = program.constants.add_int(100);
    let val2_idx = program.constants.add_int(200);
    let val1 = Node::new(OpCode::ConstInt, 3).with_args(&[val1_idx]);
    let val2 = Node::new(OpCode::ConstInt, 4).with_args(&[val2_idx]);
    
    // Complete both
    let complete1 = Node::new(OpCode::AsyncComplete, 5).with_args(&[1, 3]);
    let complete2 = Node::new(OpCode::AsyncComplete, 6).with_args(&[2, 4]);
    
    // Await both and add results
    let await1 = Node::new(OpCode::AsyncAwait, 7).with_args(&[1]);
    let await2 = Node::new(OpCode::AsyncAwait, 8).with_args(&[2]);
    let add = Node::new(OpCode::Add, 9).with_args(&[7, 8]);
    
    program.add_node(begin1);
    program.add_node(begin2);
    program.add_node(val1);
    program.add_node(val2);
    program.add_node(complete1);
    program.add_node(complete2);
    program.add_node(await1);
    program.add_node(await2);
    let result = program.add_node(add);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::Int(300) => {},
        _ => panic!("Expected Int(300), got {:?}", result),
    }
}

#[test]
fn test_async_with_complex_value() {
    let mut program = Program::new();
    
    // Begin async operation
    let begin_node = Node::new(OpCode::AsyncBegin, 1);
    
    // Create array value
    let val1_idx = program.constants.add_int(10);
    let val2_idx = program.constants.add_int(20);
    let val3_idx = program.constants.add_int(30);
    
    let val1 = Node::new(OpCode::ConstInt, 2).with_args(&[val1_idx]);
    let val2 = Node::new(OpCode::ConstInt, 3).with_args(&[val2_idx]);
    let val3 = Node::new(OpCode::ConstInt, 4).with_args(&[val3_idx]);
    let array = Node::new(OpCode::CreateArray, 5).with_args(&[2, 3, 4]);
    
    // Complete with array
    let complete = Node::new(OpCode::AsyncComplete, 6).with_args(&[1, 5]);
    
    // Await and get first element
    let await_node = Node::new(OpCode::AsyncAwait, 7).with_args(&[1]);
    let idx = program.constants.add_int(0);
    let idx_node = Node::new(OpCode::ConstInt, 8).with_args(&[idx]);
    let get = Node::new(OpCode::ArrayGet, 9).with_args(&[7, 8]);
    
    program.add_node(begin_node);
    program.add_node(val1);
    program.add_node(val2);
    program.add_node(val3);
    program.add_node(array);
    program.add_node(complete);
    program.add_node(await_node);
    program.add_node(idx_node);
    let result = program.add_node(get);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::Int(10) => {},
        _ => panic!("Expected Int(10), got {:?}", result),
    }
}

#[test]
fn test_async_type_errors() {
    let mut program = Program::new();
    
    // Try to await a non-async value
    let val_idx = program.constants.add_int(42);
    let val_node = Node::new(OpCode::ConstInt, 1).with_args(&[val_idx]);
    let await_node = Node::new(OpCode::AsyncAwait, 2).with_args(&[1]);
    
    program.add_node(val_node);
    let result = program.add_node(await_node);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute();
    
    assert!(result.is_err());
    match result {
        Err(RuntimeError::TypeMismatch { expected, actual }) => {
            assert_eq!(expected, "async handle");
            assert_eq!(actual, "int");
        }
        _ => panic!("Expected TypeMismatch error"),
    }
}

#[test]
fn test_async_chain() {
    let mut program = Program::new();
    
    // Create a chain of async operations
    let begin1 = Node::new(OpCode::AsyncBegin, 1);
    
    // First async completes with 10
    let val1_idx = program.constants.add_int(10);
    let val1 = Node::new(OpCode::ConstInt, 2).with_args(&[val1_idx]);
    let complete1 = Node::new(OpCode::AsyncComplete, 3).with_args(&[1, 2]);
    
    // Second async operation
    let begin2 = Node::new(OpCode::AsyncBegin, 4);
    
    // Await first and multiply by 2
    let await1 = Node::new(OpCode::AsyncAwait, 5).with_args(&[1]);
    let two_idx = program.constants.add_int(2);
    let two = Node::new(OpCode::ConstInt, 6).with_args(&[two_idx]);
    let mul = Node::new(OpCode::Mul, 7).with_args(&[5, 6]);
    
    // Complete second with multiplied value
    let complete2 = Node::new(OpCode::AsyncComplete, 8).with_args(&[4, 7]);
    
    // Await final result
    let await2 = Node::new(OpCode::AsyncAwait, 9).with_args(&[4]);
    
    program.add_node(begin1);
    program.add_node(val1);
    program.add_node(complete1);
    program.add_node(begin2);
    program.add_node(await1);
    program.add_node(two);
    program.add_node(mul);
    program.add_node(complete2);
    let result = program.add_node(await2);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::Int(20) => {},
        _ => panic!("Expected Int(20), got {:?}", result),
    }
}