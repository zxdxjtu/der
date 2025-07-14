use crate::core::*;
use std::io::Cursor;

#[test]
fn test_file_header_creation() {
    let header = FileHeader::new(3);
    assert_eq!(header.magic, DER_MAGIC);
    assert_eq!(header.version, VERSION);
    assert_eq!(header.chunk_count, 3);
}

#[test]
fn test_node_creation() {
    let node = Node::new(OpCode::Add, 100);
    assert_eq!(node.opcode, OpCode::Add as u16);
    assert_eq!(node.result_id, 100);
    assert_eq!(node.arg_count, 0);
    assert!(node.timestamp > 0);
}

#[test]
fn test_node_with_args() {
    let node = Node::new(OpCode::Add, 100)
        .with_args(&[10, 20]);
    assert_eq!(node.arg_count, 2);
    assert_eq!(node.args[0], 10);
    assert_eq!(node.args[1], 20);
    assert_eq!(node.args[2], 0);
}

#[test]
fn test_node_flags() {
    let mut node = Node::new(OpCode::Add, 100);
    
    node.set_flag(NodeFlag::IsAsync);
    assert!(node.has_flag(NodeFlag::IsAsync));
    assert!(!node.has_flag(NodeFlag::IsPure));
    
    node.set_flag(NodeFlag::IsPure);
    assert!(node.has_flag(NodeFlag::IsAsync));
    assert!(node.has_flag(NodeFlag::IsPure));
}

#[test]
fn test_constant_pool() {
    let mut pool = ConstantPool::new();
    
    let int_idx = pool.add_int(42);
    let float_idx = pool.add_float(3.14);
    let string_idx = pool.add_string("hello".to_string());
    let bool_idx = pool.add_bool(true);
    
    assert_eq!(pool.get_int(int_idx), Some(42));
    assert_eq!(pool.get_float(float_idx), Some(3.14));
    assert_eq!(pool.get_string(string_idx).map(|s| s.as_str()), Some("hello"));
    assert_eq!(pool.get_bool(bool_idx), Some(true));
}

#[test]
fn test_program_creation() {
    let mut program = Program::new();
    
    let node1 = Node::new(OpCode::ConstInt, 1);
    let node2 = Node::new(OpCode::ConstInt, 2);
    let node3 = Node::new(OpCode::Add, 3).with_args(&[1, 2]);
    
    let idx1 = program.add_node(node1);
    let idx2 = program.add_node(node2);
    let idx3 = program.add_node(node3);
    
    assert_eq!(idx1, 0);
    assert_eq!(idx2, 1);
    assert_eq!(idx3, 2);
    
    program.set_entry_point(idx3);
    assert_eq!(program.metadata.entry_point, idx3);
}

#[test]
fn test_serialization_deserialization() {
    use crate::core::{DERSerializer, DERDeserializer};
    
    // Create a simple program
    let mut program = Program::new();
    
    // Add some constants
    let const1_idx = program.constants.add_int(10);
    let const2_idx = program.constants.add_int(20);
    
    // Create nodes for: 10 + 20
    let node1 = Node::new(OpCode::ConstInt, 1).with_args(&[const1_idx]);
    let node2 = Node::new(OpCode::ConstInt, 2).with_args(&[const2_idx]);
    let node3 = Node::new(OpCode::Add, 3).with_args(&[1, 2]);
    
    program.add_node(node1);
    program.add_node(node2);
    let result_node = program.add_node(node3);
    program.set_entry_point(result_node);
    
    // Add metadata
    program.require_capability(Capability::FileSystem);
    program.metadata.traits.push(Trait {
        name: "Addition".to_string(),
        preconditions: vec!["inputs are integers".to_string()],
        postconditions: vec!["result is sum".to_string()],
    });
    
    // Update chunk count
    program.header.chunk_count = 3; // META, IMPL, CNST
    
    // Serialize
    let mut buffer = Vec::new();
    let mut serializer = DERSerializer::new(&mut buffer);
    serializer.write_program(&program).unwrap();
    
    // Deserialize
    let mut cursor = Cursor::new(buffer);
    let mut deserializer = DERDeserializer::new(&mut cursor);
    let loaded_program = deserializer.read_program().unwrap();
    
    // Verify
    assert_eq!(loaded_program.nodes.len(), 3);
    assert_eq!(loaded_program.metadata.entry_point, result_node);
    assert_eq!(loaded_program.metadata.required_capabilities.len(), 1);
    assert_eq!(loaded_program.metadata.traits.len(), 1);
    assert_eq!(loaded_program.constants.get_int(const1_idx), Some(10));
    assert_eq!(loaded_program.constants.get_int(const2_idx), Some(20));
}

#[test]
fn test_opcode_range() {
    // Test that all opcodes can be converted to u16 and back
    let opcodes = vec![
        OpCode::Nop, OpCode::Return, OpCode::Call, OpCode::Branch,
        OpCode::Add, OpCode::Sub, OpCode::Mul, OpCode::Div, OpCode::Mod,
        OpCode::Eq, OpCode::Ne, OpCode::Lt, OpCode::Le, OpCode::Gt, OpCode::Ge,
        OpCode::And, OpCode::Or, OpCode::Not, OpCode::Xor,
        OpCode::ConstInt, OpCode::ConstFloat, OpCode::ConstString, OpCode::ConstBool,
        OpCode::CreateArray, OpCode::ArrayGet, OpCode::ArraySet,
        OpCode::CreateMap, OpCode::MapGet, OpCode::MapSet,
        OpCode::DefineFunc, OpCode::CreateClosure,
        OpCode::Print,
    ];
    
    for opcode in opcodes {
        let value = opcode as u16;
        assert!(value <= 0xFFFF);
    }
}