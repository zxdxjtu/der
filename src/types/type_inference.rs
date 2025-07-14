use crate::core::{Program, Node, OpCode};
use crate::types::type_system::*;
use std::collections::HashMap;

pub struct TypeInferencer {
    node_types: HashMap<u32, Type>,
    constraints: Vec<TypeConstraint>,
}

#[derive(Debug, Clone)]
pub struct TypeConstraint {
    pub node_id: u32,
    pub expected_type: Type,
    pub reason: String,
}

impl TypeInferencer {
    pub fn new() -> Self {
        TypeInferencer {
            node_types: HashMap::new(),
            constraints: Vec::new(),
        }
    }
    
    pub fn infer_types(&mut self, program: &Program) -> Result<HashMap<u32, Type>, String> {
        // First pass: collect initial types and constraints
        for node in &program.nodes {
            self.collect_constraints(node, program)?;
        }
        
        // Solve constraints
        self.solve_constraints()?;
        
        // Return inferred types
        Ok(self.node_types.clone())
    }
    
    fn collect_constraints(&mut self, node: &Node, program: &Program) -> Result<(), String> {
        match OpCode::try_from(node.opcode) {
            Ok(OpCode::ConstInt) => {
                self.node_types.insert(node.result_id, Type::Int);
            }
            Ok(OpCode::ConstFloat) => {
                self.node_types.insert(node.result_id, Type::Float);
            }
            Ok(OpCode::ConstString) => {
                self.node_types.insert(node.result_id, Type::String);
            }
            Ok(OpCode::ConstBool) => {
                self.node_types.insert(node.result_id, Type::Bool);
            }
            Ok(OpCode::Add) | Ok(OpCode::Sub) | Ok(OpCode::Mul) | Ok(OpCode::Div) => {
                // Arithmetic operations preserve numeric type
                self.constraints.push(TypeConstraint {
                    node_id: node.result_id,
                    expected_type: Type::Any, // Will be refined based on inputs
                    reason: "Arithmetic operation".to_string(),
                });
            }
            Ok(OpCode::Eq) | Ok(OpCode::Ne) | Ok(OpCode::Lt) | Ok(OpCode::Le) | Ok(OpCode::Gt) | Ok(OpCode::Ge) => {
                // Comparison operations return bool
                self.node_types.insert(node.result_id, Type::Bool);
            }
            Ok(OpCode::CreateArray) => {
                self.constraints.push(TypeConstraint {
                    node_id: node.result_id,
                    expected_type: Type::Array(Box::new(Type::Any)),
                    reason: "Array creation".to_string(),
                });
            }
            Ok(OpCode::ArrayGet) => {
                self.constraints.push(TypeConstraint {
                    node_id: node.result_id,
                    expected_type: Type::Any,
                    reason: "Array element access".to_string(),
                });
            }
            Ok(OpCode::CreateMap) => {
                self.node_types.insert(node.result_id, Type::Map(Box::new(Type::Any), Box::new(Type::Any)));
            }
            Ok(OpCode::MapGet) => {
                self.constraints.push(TypeConstraint {
                    node_id: node.result_id,
                    expected_type: Type::Any,
                    reason: "Map value access".to_string(),
                });
            }
            Ok(OpCode::Load) => {
                self.constraints.push(TypeConstraint {
                    node_id: node.result_id,
                    expected_type: Type::Any,
                    reason: "Memory load".to_string(),
                });
            }
            Ok(OpCode::AsyncBegin) => {
                self.constraints.push(TypeConstraint {
                    node_id: node.result_id,
                    expected_type: Type::NodeRef,
                    reason: "Async operation".to_string(),
                });
            }
            _ => {
                // Default to Any for unknown opcodes
                self.node_types.insert(node.result_id, Type::Any);
            }
        }
        
        Ok(())
    }
    
    fn solve_constraints(&mut self) -> Result<(), String> {
        // Simple constraint solver - can be enhanced
        let mut changed = true;
        let mut iterations = 0;
        
        while changed && iterations < 100 {
            changed = false;
            iterations += 1;
            
            for constraint in &self.constraints {
                if !self.node_types.contains_key(&constraint.node_id) {
                    self.node_types.insert(constraint.node_id, constraint.expected_type.clone());
                    changed = true;
                }
            }
        }
        
        if iterations >= 100 {
            return Err("Type inference did not converge".to_string());
        }
        
        Ok(())
    }
    
    pub fn unify(&self, t1: &Type, t2: &Type) -> Result<Type, String> {
        match (t1, t2) {
            (Type::Any, t) | (t, Type::Any) => Ok(t.clone()),
            (Type::Int, Type::Int) => Ok(Type::Int),
            (Type::Float, Type::Float) => Ok(Type::Float),
            (Type::String, Type::String) => Ok(Type::String),
            (Type::Bool, Type::Bool) => Ok(Type::Bool),
            (Type::Nil, Type::Nil) => Ok(Type::Nil),
            (Type::Array(e1), Type::Array(e2)) => {
                let elem_type = self.unify(e1, e2)?;
                Ok(Type::Array(Box::new(elem_type)))
            }
            (Type::Map(k1, v1), Type::Map(k2, v2)) => {
                let key_type = self.unify(k1, k2)?;
                let val_type = self.unify(v1, v2)?;
                Ok(Type::Map(Box::new(key_type), Box::new(val_type)))
            }
            _ => Err(format!("Cannot unify types {:?} and {:?}", t1, t2)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_type_inference() {
        let mut program = Program::new();
        
        // Add some nodes
        let idx = program.constants.add_int(42);
        let node = Node::new(OpCode::ConstInt, 1).with_args(&[idx]);
        program.add_node(node);
        
        let mut inferencer = TypeInferencer::new();
        let types = inferencer.infer_types(&program).unwrap();
        
        assert_eq!(types.get(&1), Some(&Type::Int));
    }
}