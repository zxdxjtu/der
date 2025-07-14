use crate::core::{Program, Node, OpCode};
use crate::runtime::{Executor, Value};
use crate::verification::{ProofChecker, ConstraintChecker, Constraint, ConstraintExpression, ConstraintSeverity};
use std::collections::HashMap;

pub struct Verifier {
    program: Program,
    proof_checker: ProofChecker,
}

impl Verifier {
    pub fn new(program: Program) -> Self {
        Verifier {
            program,
            proof_checker: ProofChecker::new(),
        }
    }
    
    pub fn verify_program(&self) -> VerificationResult {
        let mut result = VerificationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            info: Vec::new(),
        };
        
        // Verify each node
        for (idx, node) in self.program.nodes.iter().enumerate() {
            if let Err(e) = self.verify_node(node) {
                result.errors.push(VerificationError {
                    node_id: node.result_id,
                    message: e,
                });
                result.is_valid = false;
            }
        }
        
        // Verify program traits
        for trait_def in &self.program.metadata.traits {
            if let Err(e) = self.verify_trait(&trait_def.name) {
                result.errors.push(VerificationError {
                    node_id: self.program.metadata.entry_point,
                    message: e,
                });
                result.is_valid = false;
            }
        }
        
        // Run constraint checks
        let constraint_violations = self.check_program_constraints();
        for violation in constraint_violations {
            match violation.severity {
                ConstraintSeverity::Error => {
                    result.errors.push(VerificationError {
                        node_id: 0,
                        message: violation.message,
                    });
                    result.is_valid = false;
                }
                ConstraintSeverity::Warning => {
                    result.warnings.push(violation.message);
                }
                ConstraintSeverity::Info => {
                    result.info.push(violation.message);
                }
            }
        }
        
        result
    }
    
    fn verify_node(&self, node: &Node) -> Result<(), String> {
        // Verify opcode is valid
        let opcode = OpCode::try_from(node.opcode)
            .map_err(|_| format!("Invalid opcode: {}", node.opcode))?;
        
        // Verify argument count
        let expected_args = self.get_expected_arg_count(&opcode);
        if let Some(expected) = expected_args {
            if node.arg_count != expected {
                return Err(format!(
                    "Opcode {:?} expects {} arguments, got {}",
                    opcode, expected, node.arg_count
                ));
            }
        }
        
        // Verify argument references are valid
        for i in 0..node.arg_count as usize {
            let arg_id = node.args[i];
            if arg_id != 0 {
                // Check if the referenced node exists
                let found = self.program.nodes.iter()
                    .any(|n| n.result_id == arg_id);
                if !found {
                    return Err(format!("Invalid argument reference: {}", arg_id));
                }
            }
        }
        
        Ok(())
    }
    
    fn verify_trait(&self, trait_name: &str) -> Result<(), String> {
        // Check if we can generate and verify a proof for this trait
        self.proof_checker.check_trait_satisfaction(
            &self.program,
            self.program.metadata.entry_point,
            trait_name
        )?;
        Ok(())
    }
    
    fn check_program_constraints(&self) -> Vec<crate::verification::constraints::ConstraintViolation> {
        let mut checker = ConstraintChecker::new();
        
        // Add standard constraints
        checker.add_constraint(Constraint {
            name: "no_cycles".to_string(),
            expression: ConstraintExpression::All(vec![]), // TODO: implement cycle detection
            severity: ConstraintSeverity::Error,
        });
        
        // Run a test execution to get values
        let mut executor = Executor::new(self.program.clone());
        if let Ok(result) = executor.execute() {
            checker.set_value("result".to_string(), result);
        }
        
        checker.check_all()
    }
    
    fn get_expected_arg_count(&self, opcode: &OpCode) -> Option<u8> {
        match opcode {
            OpCode::Nop => Some(0),
            OpCode::Return => Some(1),
            OpCode::Call => None, // Variable args
            OpCode::Branch => Some(3),
            
            OpCode::Add | OpCode::Sub | OpCode::Mul | OpCode::Div | OpCode::Mod => Some(2),
            OpCode::Eq | OpCode::Ne | OpCode::Lt | OpCode::Le | OpCode::Gt | OpCode::Ge => Some(2),
            OpCode::And | OpCode::Or | OpCode::Xor => Some(2),
            OpCode::Not => Some(1),
            
            OpCode::ConstInt | OpCode::ConstFloat | OpCode::ConstString | OpCode::ConstBool => Some(1),
            
            OpCode::CreateArray => None, // Variable args
            OpCode::CreateMap => Some(0),
            OpCode::ArrayGet | OpCode::MapGet => Some(2),
            OpCode::ArraySet | OpCode::MapSet => Some(3),
            
            OpCode::DefineFunc => Some(2),
            OpCode::CreateClosure => None, // Variable args
            
            OpCode::Print => None, // Variable args
            
            _ => None,
        }
    }
    
    pub fn verify_safety(&self) -> SafetyAnalysis {
        let mut analysis = SafetyAnalysis {
            has_unsafe_operations: false,
            memory_safe: true,
            deterministic: true,
            side_effects: Vec::new(),
        };
        
        for node in &self.program.nodes {
            if let Ok(opcode) = OpCode::try_from(node.opcode) {
                match opcode {
                    OpCode::ExternalCall => {
                        analysis.has_unsafe_operations = true;
                        analysis.side_effects.push(format!("External call at node {}", node.result_id));
                    }
                    OpCode::Free => {
                        analysis.memory_safe = false;
                        analysis.side_effects.push(format!("Manual memory management at node {}", node.result_id));
                    }
                    OpCode::Print | OpCode::Read => {
                        analysis.side_effects.push(format!("I/O operation at node {}", node.result_id));
                    }
                    _ => {}
                }
            }
        }
        
        analysis
    }
}

#[derive(Debug)]
pub struct VerificationResult {
    pub is_valid: bool,
    pub errors: Vec<VerificationError>,
    pub warnings: Vec<String>,
    pub info: Vec<String>,
}

#[derive(Debug)]
pub struct VerificationError {
    pub node_id: u32,
    pub message: String,
}

#[derive(Debug)]
pub struct SafetyAnalysis {
    pub has_unsafe_operations: bool,
    pub memory_safe: bool,
    pub deterministic: bool,
    pub side_effects: Vec<String>,
}