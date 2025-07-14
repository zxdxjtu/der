use crate::core::*;
use crate::runtime::*;

fn create_test_program() -> Program {
    Program::new()
}

#[test]
fn test_arithmetic_operations() {
    let mut program = create_test_program();
    
    // Create program: 10 + 20
    let const1_idx = program.constants.add_int(10);
    let const2_idx = program.constants.add_int(20);
    
    let node1 = Node::new(OpCode::ConstInt, 1).with_args(&[const1_idx]);
    let node2 = Node::new(OpCode::ConstInt, 2).with_args(&[const2_idx]);
    let node3 = Node::new(OpCode::Add, 3).with_args(&[1, 2]);
    
    program.add_node(node1);
    program.add_node(node2);
    let result = program.add_node(node3);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::Int(30) => {},
        _ => panic!("Expected Int(30), got {:?}", result),
    }
}

#[test]
fn test_subtraction() {
    let mut program = create_test_program();
    
    let const1_idx = program.constants.add_int(50);
    let const2_idx = program.constants.add_int(20);
    
    let node1 = Node::new(OpCode::ConstInt, 1).with_args(&[const1_idx]);
    let node2 = Node::new(OpCode::ConstInt, 2).with_args(&[const2_idx]);
    let node3 = Node::new(OpCode::Sub, 3).with_args(&[1, 2]);
    
    program.add_node(node1);
    program.add_node(node2);
    let result = program.add_node(node3);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::Int(30) => {},
        _ => panic!("Expected Int(30), got {:?}", result),
    }
}

#[test]
fn test_multiplication() {
    let mut program = create_test_program();
    
    let const1_idx = program.constants.add_int(6);
    let const2_idx = program.constants.add_int(7);
    
    let node1 = Node::new(OpCode::ConstInt, 1).with_args(&[const1_idx]);
    let node2 = Node::new(OpCode::ConstInt, 2).with_args(&[const2_idx]);
    let node3 = Node::new(OpCode::Mul, 3).with_args(&[1, 2]);
    
    program.add_node(node1);
    program.add_node(node2);
    let result = program.add_node(node3);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::Int(42) => {},
        _ => panic!("Expected Int(42), got {:?}", result),
    }
}

#[test]
fn test_division() {
    let mut program = create_test_program();
    
    let const1_idx = program.constants.add_float(10.0);
    let const2_idx = program.constants.add_float(4.0);
    
    let node1 = Node::new(OpCode::ConstFloat, 1).with_args(&[const1_idx]);
    let node2 = Node::new(OpCode::ConstFloat, 2).with_args(&[const2_idx]);
    let node3 = Node::new(OpCode::Div, 3).with_args(&[1, 2]);
    
    program.add_node(node1);
    program.add_node(node2);
    let result = program.add_node(node3);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::Float(f) if (f - 2.5).abs() < 0.001 => {},
        _ => panic!("Expected Float(2.5), got {:?}", result),
    }
}

#[test]
fn test_division_by_zero() {
    let mut program = create_test_program();
    
    let const1_idx = program.constants.add_int(10);
    let const2_idx = program.constants.add_int(0);
    
    let node1 = Node::new(OpCode::ConstInt, 1).with_args(&[const1_idx]);
    let node2 = Node::new(OpCode::ConstInt, 2).with_args(&[const2_idx]);
    let node3 = Node::new(OpCode::Div, 3).with_args(&[1, 2]);
    
    program.add_node(node1);
    program.add_node(node2);
    let result = program.add_node(node3);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute();
    
    assert!(matches!(result, Err(RuntimeError::DivisionByZero)));
}

#[test]
fn test_comparison_operations() {
    let mut program = create_test_program();
    
    let const1_idx = program.constants.add_int(10);
    let const2_idx = program.constants.add_int(20);
    
    let node1 = Node::new(OpCode::ConstInt, 1).with_args(&[const1_idx]);
    let node2 = Node::new(OpCode::ConstInt, 2).with_args(&[const2_idx]);
    let node3 = Node::new(OpCode::Lt, 3).with_args(&[1, 2]);
    
    program.add_node(node1);
    program.add_node(node2);
    let result = program.add_node(node3);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::Bool(true) => {},
        _ => panic!("Expected Bool(true), got {:?}", result),
    }
}

#[test]
fn test_logical_operations() {
    let mut program = create_test_program();
    
    let true_idx = program.constants.add_bool(true);
    let false_idx = program.constants.add_bool(false);
    
    let node1 = Node::new(OpCode::ConstBool, 1).with_args(&[true_idx]);
    let node2 = Node::new(OpCode::ConstBool, 2).with_args(&[false_idx]);
    let node3 = Node::new(OpCode::And, 3).with_args(&[1, 2]);
    
    program.add_node(node1);
    program.add_node(node2);
    let result = program.add_node(node3);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::Bool(false) => {},
        _ => panic!("Expected Bool(false), got {:?}", result),
    }
}

#[test]
fn test_branch_true() {
    let mut program = create_test_program();
    
    let true_idx = program.constants.add_bool(true);
    let val1_idx = program.constants.add_int(100);
    let val2_idx = program.constants.add_int(200);
    
    let cond_node = Node::new(OpCode::ConstBool, 1).with_args(&[true_idx]);
    let then_node = Node::new(OpCode::ConstInt, 2).with_args(&[val1_idx]);
    let else_node = Node::new(OpCode::ConstInt, 3).with_args(&[val2_idx]);
    let branch_node = Node::new(OpCode::Branch, 4).with_args(&[1, 2, 3]);
    
    program.add_node(cond_node);
    program.add_node(then_node);
    program.add_node(else_node);
    let result = program.add_node(branch_node);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::Int(100) => {},
        _ => panic!("Expected Int(100), got {:?}", result),
    }
}

#[test]
fn test_branch_false() {
    let mut program = create_test_program();
    
    let false_idx = program.constants.add_bool(false);
    let val1_idx = program.constants.add_int(100);
    let val2_idx = program.constants.add_int(200);
    
    let cond_node = Node::new(OpCode::ConstBool, 1).with_args(&[false_idx]);
    let then_node = Node::new(OpCode::ConstInt, 2).with_args(&[val1_idx]);
    let else_node = Node::new(OpCode::ConstInt, 3).with_args(&[val2_idx]);
    let branch_node = Node::new(OpCode::Branch, 4).with_args(&[1, 2, 3]);
    
    program.add_node(cond_node);
    program.add_node(then_node);
    program.add_node(else_node);
    let result = program.add_node(branch_node);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::Int(200) => {},
        _ => panic!("Expected Int(200), got {:?}", result),
    }
}

#[test]
fn test_array_operations() {
    let mut program = create_test_program();
    
    let val1_idx = program.constants.add_int(10);
    let val2_idx = program.constants.add_int(20);
    let val3_idx = program.constants.add_int(30);
    let index_idx = program.constants.add_int(1);
    
    let elem1 = Node::new(OpCode::ConstInt, 1).with_args(&[val1_idx]);
    let elem2 = Node::new(OpCode::ConstInt, 2).with_args(&[val2_idx]);
    let elem3 = Node::new(OpCode::ConstInt, 3).with_args(&[val3_idx]);
    let array = Node::new(OpCode::CreateArray, 4).with_args(&[1, 2, 3]);
    let index = Node::new(OpCode::ConstInt, 5).with_args(&[index_idx]);
    let get = Node::new(OpCode::ArrayGet, 6).with_args(&[4, 5]);
    
    program.add_node(elem1);
    program.add_node(elem2);
    program.add_node(elem3);
    program.add_node(array);
    program.add_node(index);
    let result = program.add_node(get);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::Int(20) => {},
        _ => panic!("Expected Int(20), got {:?}", result),
    }
}

#[test]
fn test_map_operations() {
    let mut program = create_test_program();
    
    let key_idx = program.constants.add_string("test_key".to_string());
    let val_idx = program.constants.add_int(42);
    
    let map_node = Node::new(OpCode::CreateMap, 1);
    let key_node = Node::new(OpCode::ConstString, 2).with_args(&[key_idx]);
    let val_node = Node::new(OpCode::ConstInt, 3).with_args(&[val_idx]);
    let set_node = Node::new(OpCode::MapSet, 4).with_args(&[1, 2, 3]);
    let get_node = Node::new(OpCode::MapGet, 5).with_args(&[4, 2]);
    
    program.add_node(map_node);
    program.add_node(key_node);
    program.add_node(val_node);
    program.add_node(set_node);
    let result = program.add_node(get_node);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::Int(42) => {},
        _ => panic!("Expected Int(42), got {:?}", result),
    }
}

#[test]
fn test_string_operations() {
    let mut program = create_test_program();
    
    let str1_idx = program.constants.add_string("Hello".to_string());
    let str2_idx = program.constants.add_string("World".to_string());
    
    let str1_node = Node::new(OpCode::ConstString, 1).with_args(&[str1_idx]);
    let str2_node = Node::new(OpCode::ConstString, 2).with_args(&[str2_idx]);
    let eq_node = Node::new(OpCode::Eq, 3).with_args(&[1, 2]);
    
    program.add_node(str1_node);
    program.add_node(str2_node);
    let result = program.add_node(eq_node);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::Bool(false) => {},
        _ => panic!("Expected Bool(false), got {:?}", result),
    }
}

#[test]
fn test_value_truthiness() {
    assert_eq!(Value::Nil.is_truthy(), false);
    assert_eq!(Value::Bool(true).is_truthy(), true);
    assert_eq!(Value::Bool(false).is_truthy(), false);
    assert_eq!(Value::Int(0).is_truthy(), false);
    assert_eq!(Value::Int(1).is_truthy(), true);
    assert_eq!(Value::Float(0.0).is_truthy(), false);
    assert_eq!(Value::Float(1.0).is_truthy(), true);
    assert_eq!(Value::String("".to_string()).is_truthy(), false);
    assert_eq!(Value::String("hello".to_string()).is_truthy(), true);
    assert_eq!(Value::Array(vec![]).is_truthy(), false);
    assert_eq!(Value::Array(vec![Value::Int(1)]).is_truthy(), true);
}

#[test]
fn test_complex_expression() {
    let mut program = create_test_program();
    
    // Create program: (10 + 20) * (5 - 3)
    let c10 = program.constants.add_int(10);
    let c20 = program.constants.add_int(20);
    let c5 = program.constants.add_int(5);
    let c3 = program.constants.add_int(3);
    
    let n10 = Node::new(OpCode::ConstInt, 1).with_args(&[c10]);
    let n20 = Node::new(OpCode::ConstInt, 2).with_args(&[c20]);
    let add = Node::new(OpCode::Add, 3).with_args(&[1, 2]); // 10 + 20 = 30
    
    let n5 = Node::new(OpCode::ConstInt, 4).with_args(&[c5]);
    let n3 = Node::new(OpCode::ConstInt, 5).with_args(&[c3]);
    let sub = Node::new(OpCode::Sub, 6).with_args(&[4, 5]); // 5 - 3 = 2
    
    let mul = Node::new(OpCode::Mul, 7).with_args(&[3, 6]); // 30 * 2 = 60
    
    program.add_node(n10);
    program.add_node(n20);
    program.add_node(add);
    program.add_node(n5);
    program.add_node(n3);
    program.add_node(sub);
    let result = program.add_node(mul);
    program.set_entry_point(result);
    
    let mut executor = Executor::new(program);
    let result = executor.execute().unwrap();
    
    match result {
        Value::Int(60) => {},
        _ => panic!("Expected Int(60), got {:?}", result),
    }
}