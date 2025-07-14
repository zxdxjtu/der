use crate::core::{Program, Node, OpCode};
use crate::types::type_system::*;
use crate::runtime::Value;
use std::collections::HashMap;

pub struct TypeChecker {
    env: TypeEnvironment,
    node_types: HashMap<u32, Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut env = TypeEnvironment::new();
        env.add_builtin_functions();
        
        TypeChecker {
            env,
            node_types: HashMap::new(),
        }
    }
    
    pub fn check_program(&mut self, program: &Program) -> Result<(), String> {
        // Type check each node
        for node in &program.nodes {
            self.check_node(node, program)?;
        }
        
        // Verify entry point exists
        let entry_type = self.node_types.get(&program.metadata.entry_point)
            .ok_or("Entry point node not found")?;
        
        Ok(())
    }
    
    fn check_node(&mut self, node: &Node, program: &Program) -> Result<Type, String> {
        // Check if already typed
        if let Some(ty) = self.node_types.get(&node.result_id) {
            return Ok(ty.clone());
        }
        
        let node_type = match OpCode::try_from(node.opcode) {
            Ok(OpCode::ConstInt) => {
                Type::Int
            }
            Ok(OpCode::ConstFloat) => {
                Type::Float
            }
            Ok(OpCode::ConstString) => {
                Type::String
            }
            Ok(OpCode::ConstBool) => {
                Type::Bool
            }
            Ok(OpCode::Add) | Ok(OpCode::Sub) | Ok(OpCode::Mul) | Ok(OpCode::Div) => {
                // Arithmetic operations preserve numeric type
                let left_type = self.get_arg_type(node, 0, program)?;
                let right_type = self.get_arg_type(node, 1, program)?;
                
                match (&left_type, &right_type) {
                    (Type::Int, Type::Int) => Type::Int,
                    (Type::Float, _) | (_, Type::Float) => Type::Float,
                    _ => return Err(format!("Type error: cannot apply arithmetic to {:?} and {:?}", left_type, right_type)),
                }
            }
            Ok(OpCode::Eq) | Ok(OpCode::Ne) | Ok(OpCode::Lt) | Ok(OpCode::Le) | Ok(OpCode::Gt) | Ok(OpCode::Ge) => {
                Type::Bool
            }
            Ok(OpCode::Print) => {
                Type::Nil
            }
            Ok(OpCode::CreateArray) => {
                // Infer array element type from first element
                if node.arg_count > 0 {
                    let elem_type = self.get_arg_type(node, 0, program)?;
                    Type::Array(Box::new(elem_type))
                } else {
                    Type::Array(Box::new(Type::Any))
                }
            }
            Ok(OpCode::ArrayGet) => {
                let array_type = self.get_arg_type(node, 0, program)?;
                match array_type {
                    Type::Array(elem_type) => *elem_type,
                    _ => return Err("Type error: ArrayGet requires array type".to_string()),
                }
            }
            _ => Type::Any,
        };
        
        self.node_types.insert(node.result_id, node_type.clone());
        Ok(node_type)
    }
    
    fn get_arg_type(&mut self, node: &Node, arg_idx: usize, program: &Program) -> Result<Type, String> {
        if arg_idx >= node.arg_count as usize {
            return Err("Invalid argument index".to_string());
        }
        
        let arg_id = node.args[arg_idx];
        if arg_id == 0 {
            return Ok(Type::Nil);
        }
        
        // Find the node that produces this result
        let arg_node = program.nodes.iter()
            .find(|n| n.result_id == arg_id)
            .ok_or(format!("Node {} not found", arg_id))?;
        
        self.check_node(arg_node, program)
    }
}