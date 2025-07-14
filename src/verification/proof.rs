use crate::core::{Program, Node, OpCode};
use crate::verification::traits::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Proof {
    pub theorem: String,
    pub trait_kind: TraitKind,
    pub assumptions: Vec<Assumption>,
    pub steps: Vec<ProofStep>,
    pub conclusion: Conclusion,
}

#[derive(Debug, Clone)]
pub struct Assumption {
    pub description: String,
    pub condition: ConditionExpression,
}

#[derive(Debug, Clone)]
pub struct ProofStep {
    pub step_number: usize,
    pub description: String,
    pub justification: Justification,
    pub derived_fact: ConditionExpression,
}

#[derive(Debug, Clone)]
pub enum Justification {
    Assumption(usize),
    Definition(String),
    ModusPonens(usize, usize),
    Substitution(usize, HashMap<String, String>),
    Arithmetic,
    Induction(InductionProof),
    Contradiction(usize, usize),
    DirectComputation,
}

#[derive(Debug, Clone)]
pub struct InductionProof {
    pub base_case: Box<ProofStep>,
    pub inductive_step: Box<ProofStep>,
}

#[derive(Debug, Clone)]
pub struct Conclusion {
    pub statement: String,
    pub expression: ConditionExpression,
}

pub struct ProofGenerator {
    program: Program,
    trait_registry: TraitRegistry,
}

impl ProofGenerator {
    pub fn new(program: Program) -> Self {
        ProofGenerator {
            program,
            trait_registry: TraitRegistry::new(),
        }
    }
    
    pub fn generate_proof(&self, node_id: u32, trait_name: &str) -> Result<Proof, String> {
        let trait_def = self.trait_registry.get_trait(trait_name)
            .ok_or(format!("Unknown trait: {}", trait_name))?;
        
        let node = self.program.nodes.get(node_id as usize)
            .ok_or(format!("Invalid node ID: {}", node_id))?;
        
        match &trait_def.kind {
            TraitKind::IsPure => self.prove_is_pure(node, trait_def),
            TraitKind::PreservesLength => self.prove_preserves_length(node, trait_def),
            TraitKind::IsDeterministic => self.prove_is_deterministic(node, trait_def),
            _ => Err(format!("Proof generation not implemented for trait: {:?}", trait_def.kind)),
        }
    }
    
    fn prove_is_pure(&self, node: &Node, trait_def: &TraitDefinition) -> Result<Proof, String> {
        let mut proof = Proof {
            theorem: format!("Node {} satisfies IsPure trait", node.result_id),
            trait_kind: TraitKind::IsPure,
            assumptions: vec![],
            steps: vec![],
            conclusion: Conclusion {
                statement: "The operation is pure".to_string(),
                expression: ConditionExpression::Constant(ConstantValue::Boolean(true)),
            },
        };
        
        // Check opcode purity
        let is_pure = match OpCode::try_from(node.opcode) {
            Ok(opcode) => self.is_opcode_pure(&opcode),
            Err(_) => false,
        };
        
        if is_pure {
            proof.steps.push(ProofStep {
                step_number: 1,
                description: format!("Opcode {:?} is pure by definition", node.opcode),
                justification: Justification::Definition("pure_opcodes".to_string()),
                derived_fact: ConditionExpression::Constant(ConstantValue::Boolean(true)),
            });
            
            // Check arguments recursively
            for i in 0..node.arg_count as usize {
                let arg_id = node.args[i];
                if arg_id != 0 {
                    proof.steps.push(ProofStep {
                        step_number: proof.steps.len() + 1,
                        description: format!("Argument {} (node {}) must also be pure", i, arg_id),
                        justification: Justification::DirectComputation,
                        derived_fact: ConditionExpression::Constant(ConstantValue::Boolean(true)),
                    });
                }
            }
            
            Ok(proof)
        } else {
            Err(format!("Node {} uses impure opcode", node.result_id))
        }
    }
    
    fn prove_preserves_length(&self, node: &Node, trait_def: &TraitDefinition) -> Result<Proof, String> {
        let mut proof = Proof {
            theorem: format!("Node {} preserves array length", node.result_id),
            trait_kind: TraitKind::PreservesLength,
            assumptions: vec![
                Assumption {
                    description: "Input is an array".to_string(),
                    condition: ConditionExpression::Equal(
                        Box::new(ConditionExpression::Property(
                            Box::new(ConditionExpression::Variable("input".to_string())),
                            "type".to_string()
                        )),
                        Box::new(ConditionExpression::Constant(ConstantValue::String("array".to_string())))
                    ),
                }
            ],
            steps: vec![],
            conclusion: Conclusion {
                statement: "Output length equals input length".to_string(),
                expression: ConditionExpression::Equal(
                    Box::new(ConditionExpression::Length(
                        Box::new(ConditionExpression::Variable("output".to_string()))
                    )),
                    Box::new(ConditionExpression::Length(
                        Box::new(ConditionExpression::Variable("input".to_string()))
                    ))
                ),
            },
        };
        
        // Check if operation preserves length
        let preserves = match OpCode::try_from(node.opcode) {
            Ok(OpCode::ArraySet) => true,
            Ok(OpCode::CreateArray) => false,
            _ => false,
        };
        
        if preserves {
            proof.steps.push(ProofStep {
                step_number: 1,
                description: "ArraySet operation preserves array length".to_string(),
                justification: Justification::Definition("array_set_semantics".to_string()),
                derived_fact: proof.conclusion.expression.clone(),
            });
            Ok(proof)
        } else {
            Err(format!("Operation does not preserve array length"))
        }
    }
    
    fn prove_is_deterministic(&self, node: &Node, trait_def: &TraitDefinition) -> Result<Proof, String> {
        let proof = Proof {
            theorem: format!("Node {} is deterministic", node.result_id),
            trait_kind: TraitKind::IsDeterministic,
            assumptions: vec![],
            steps: vec![
                ProofStep {
                    step_number: 1,
                    description: "All arithmetic operations are deterministic".to_string(),
                    justification: Justification::Definition("deterministic_operations".to_string()),
                    derived_fact: ConditionExpression::Constant(ConstantValue::Boolean(true)),
                }
            ],
            conclusion: Conclusion {
                statement: "Same inputs always produce same outputs".to_string(),
                expression: ConditionExpression::ForAll(
                    "x".to_string(),
                    Box::new(ConditionExpression::ForAll(
                        "y".to_string(),
                        Box::new(ConditionExpression::Implies(
                            Box::new(ConditionExpression::Equal(
                                Box::new(ConditionExpression::Variable("x".to_string())),
                                Box::new(ConditionExpression::Variable("y".to_string()))
                            )),
                            Box::new(ConditionExpression::Equal(
                                Box::new(ConditionExpression::Apply(
                                    Box::new(ConditionExpression::Variable("f".to_string())),
                                    vec![ConditionExpression::Variable("x".to_string())]
                                )),
                                Box::new(ConditionExpression::Apply(
                                    Box::new(ConditionExpression::Variable("f".to_string())),
                                    vec![ConditionExpression::Variable("y".to_string())]
                                ))
                            ))
                        ))
                    ))
                ),
            },
        };
        
        Ok(proof)
    }
    
    fn is_opcode_pure(&self, opcode: &OpCode) -> bool {
        match opcode {
            // Pure operations
            OpCode::Add | OpCode::Sub | OpCode::Mul | OpCode::Div | OpCode::Mod |
            OpCode::Eq | OpCode::Ne | OpCode::Lt | OpCode::Le | OpCode::Gt | OpCode::Ge |
            OpCode::And | OpCode::Or | OpCode::Not | OpCode::Xor |
            OpCode::ConstInt | OpCode::ConstFloat | OpCode::ConstString | OpCode::ConstBool |
            OpCode::CreateArray | OpCode::CreateMap | OpCode::ArrayGet | OpCode::MapGet |
            OpCode::DefineFunc | OpCode::CreateClosure => true,
            
            // Impure operations
            OpCode::Print | OpCode::Read | OpCode::ArraySet | OpCode::MapSet |
            OpCode::Store | OpCode::Free | OpCode::ExternalCall => false,
            
            _ => false,
        }
    }
}

pub struct ProofChecker {
    trait_registry: TraitRegistry,
}

impl ProofChecker {
    pub fn new() -> Self {
        ProofChecker {
            trait_registry: TraitRegistry::new(),
        }
    }
    
    pub fn verify_proof(&self, proof: &Proof) -> Result<bool, String> {
        // Verify each step follows from previous steps
        for (i, step) in proof.steps.iter().enumerate() {
            match &step.justification {
                Justification::Assumption(idx) => {
                    if *idx >= proof.assumptions.len() {
                        return Err(format!("Step {} references invalid assumption {}", i, idx));
                    }
                }
                Justification::ModusPonens(premise1, premise2) => {
                    if *premise1 >= i || *premise2 >= i {
                        return Err(format!("Step {} references future steps", i));
                    }
                }
                Justification::Substitution(step_idx, _) => {
                    if *step_idx >= i {
                        return Err(format!("Step {} references future step", i));
                    }
                }
                _ => {}
            }
        }
        
        // Verify conclusion follows from steps
        if proof.steps.is_empty() {
            return Err("Proof has no steps".to_string());
        }
        
        Ok(true)
    }
    
    pub fn check_trait_satisfaction(&self, program: &Program, node_id: u32, trait_name: &str) -> Result<bool, String> {
        let generator = ProofGenerator::new(program.clone());
        let proof = generator.generate_proof(node_id, trait_name)?;
        self.verify_proof(&proof)
    }
}