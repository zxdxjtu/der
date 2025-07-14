use crate::core::*;
use crate::runtime::*;
use crate::verification::*;

#[test]
fn test_trait_registry() {
    let registry = TraitRegistry::new();
    
    // Check builtin traits are registered
    assert!(registry.get_trait("IsSorted").is_some());
    assert!(registry.get_trait("PreservesLength").is_some());
    assert!(registry.get_trait("IsPure").is_some());
    
    // Check trait list
    let traits = registry.list_traits();
    assert!(traits.contains(&"IsSorted"));
    assert!(traits.contains(&"PreservesLength"));
    assert!(traits.contains(&"IsPure"));
}

#[test]
fn test_proof_generation_pure_operation() {
    let mut program = Program::new();
    
    // Create a pure computation: 10 + 20
    let c10 = program.constants.add_int(10);
    let c20 = program.constants.add_int(20);
    
    let n1 = Node::new(OpCode::ConstInt, 1).with_args(&[c10]);
    let n2 = Node::new(OpCode::ConstInt, 2).with_args(&[c20]);
    let n3 = Node::new(OpCode::Add, 3).with_args(&[1, 2]);
    
    program.add_node(n1);
    program.add_node(n2);
    let result_idx = program.add_node(n3);
    
    let generator = ProofGenerator::new(program);
    let proof = generator.generate_proof(result_idx as u32, "IsPure");
    
    assert!(proof.is_ok());
    let proof = proof.unwrap();
    assert_eq!(proof.trait_kind, TraitKind::IsPure);
    assert!(!proof.steps.is_empty());
}

#[test]
fn test_proof_generation_impure_operation() {
    let mut program = Program::new();
    
    // Create an impure operation: Print
    let msg = program.constants.add_string("Hello".to_string());
    let n1 = Node::new(OpCode::ConstString, 1).with_args(&[msg]);
    let n2 = Node::new(OpCode::Print, 2).with_args(&[1]);
    
    program.add_node(n1);
    let result_idx = program.add_node(n2);
    
    let generator = ProofGenerator::new(program);
    let proof = generator.generate_proof(result_idx as u32, "IsPure");
    
    assert!(proof.is_err());
}

#[test]
fn test_constraint_checker_type_constraints() {
    let mut checker = ConstraintChecker::new();
    
    // Add type constraint
    checker.add_constraint(Constraint {
        name: "x_is_integer".to_string(),
        expression: ConstraintExpression::TypeIs("x".to_string(), TypeConstraint::Integer),
        severity: ConstraintSeverity::Error,
    });
    
    // Set correct type
    checker.set_value("x".to_string(), Value::Int(42));
    let violations = checker.check_all();
    assert!(violations.is_empty());
    
    // Set wrong type
    checker.set_value("x".to_string(), Value::String("not an int".to_string()));
    let violations = checker.check_all();
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].constraint_name, "x_is_integer");
}

#[test]
fn test_constraint_checker_range_constraints() {
    let mut checker = ConstraintChecker::new();
    
    // Add range constraint
    checker.add_constraint(Constraint {
        name: "x_in_range".to_string(),
        expression: ConstraintExpression::InRange(
            "x".to_string(),
            RangeConstraint::Integer { min: Some(0), max: Some(100) }
        ),
        severity: ConstraintSeverity::Error,
    });
    
    // Value in range
    checker.set_value("x".to_string(), Value::Int(50));
    let violations = checker.check_all();
    assert!(violations.is_empty());
    
    // Value below range
    checker.set_value("x".to_string(), Value::Int(-10));
    let violations = checker.check_all();
    assert_eq!(violations.len(), 1);
    
    // Value above range
    checker.set_value("x".to_string(), Value::Int(150));
    let violations = checker.check_all();
    assert_eq!(violations.len(), 1);
}

#[test]
fn test_constraint_checker_array_constraints() {
    let mut checker = ConstraintChecker::new();
    
    // Add array length constraint
    checker.add_constraint(Constraint {
        name: "array_length".to_string(),
        expression: ConstraintExpression::ArrayLength(
            "arr".to_string(),
            LengthConstraint::Range(2, 5)
        ),
        severity: ConstraintSeverity::Error,
    });
    
    // Array with valid length
    checker.set_value("arr".to_string(), Value::Array(vec![
        Value::Int(1),
        Value::Int(2),
        Value::Int(3),
    ]));
    let violations = checker.check_all();
    assert!(violations.is_empty());
    
    // Array too short
    checker.set_value("arr".to_string(), Value::Array(vec![Value::Int(1)]));
    let violations = checker.check_all();
    assert_eq!(violations.len(), 1);
    
    // Array too long
    checker.set_value("arr".to_string(), Value::Array(vec![
        Value::Int(1),
        Value::Int(2),
        Value::Int(3),
        Value::Int(4),
        Value::Int(5),
        Value::Int(6),
    ]));
    let violations = checker.check_all();
    assert_eq!(violations.len(), 1);
}

#[test]
fn test_constraint_checker_sorted_array() {
    let mut checker = ConstraintChecker::new();
    
    // Add sorted constraint
    checker.add_constraint(Constraint {
        name: "array_sorted".to_string(),
        expression: ConstraintExpression::ArraySorted(
            "arr".to_string(),
            SortOrder::Ascending
        ),
        severity: ConstraintSeverity::Error,
    });
    
    // Sorted array
    checker.set_value("arr".to_string(), Value::Array(vec![
        Value::Int(1),
        Value::Int(2),
        Value::Int(3),
        Value::Int(4),
    ]));
    let violations = checker.check_all();
    assert!(violations.is_empty());
    
    // Unsorted array
    checker.set_value("arr".to_string(), Value::Array(vec![
        Value::Int(1),
        Value::Int(3),
        Value::Int(2),
        Value::Int(4),
    ]));
    let violations = checker.check_all();
    assert_eq!(violations.len(), 1);
}

#[test]
fn test_constraint_checker_logical_combinations() {
    let mut checker = ConstraintChecker::new();
    
    // Add complex constraint: x > 0 AND x < 100
    checker.add_constraint(Constraint {
        name: "x_valid_range".to_string(),
        expression: ConstraintExpression::All(vec![
            ConstraintExpression::GreaterThan("x".to_string(), "zero".to_string()),
            ConstraintExpression::LessThan("x".to_string(), "hundred".to_string()),
        ]),
        severity: ConstraintSeverity::Error,
    });
    
    checker.set_value("zero".to_string(), Value::Int(0));
    checker.set_value("hundred".to_string(), Value::Int(100));
    
    // Valid value
    checker.set_value("x".to_string(), Value::Int(50));
    let violations = checker.check_all();
    assert!(violations.is_empty());
    
    // Invalid value
    checker.set_value("x".to_string(), Value::Int(150));
    let violations = checker.check_all();
    assert_eq!(violations.len(), 1);
}

#[test]
fn test_verifier_valid_program() {
    let mut program = Program::new();
    
    // Create a valid program
    let c10 = program.constants.add_int(10);
    let c20 = program.constants.add_int(20);
    
    let n1 = Node::new(OpCode::ConstInt, 1).with_args(&[c10]);
    let n2 = Node::new(OpCode::ConstInt, 2).with_args(&[c20]);
    let n3 = Node::new(OpCode::Add, 3).with_args(&[1, 2]);
    
    program.add_node(n1);
    program.add_node(n2);
    let result = program.add_node(n3);
    program.set_entry_point(result);
    
    let verifier = Verifier::new(program);
    let result = verifier.verify_program();
    
    assert!(result.is_valid);
    assert!(result.errors.is_empty());
}

#[test]
fn test_verifier_invalid_opcode() {
    let mut program = Program::new();
    
    // Create node with invalid opcode
    let mut node = Node::new(OpCode::Add, 1);
    node.opcode = 0xFFFF; // Invalid opcode
    program.add_node(node);
    
    let verifier = Verifier::new(program);
    let result = verifier.verify_program();
    
    assert!(!result.is_valid);
    assert!(!result.errors.is_empty());
    assert!(result.errors[0].message.contains("Invalid opcode"));
}

#[test]
fn test_verifier_invalid_arg_count() {
    let mut program = Program::new();
    
    // Create Add node with wrong number of arguments
    let node = Node::new(OpCode::Add, 1).with_args(&[1]); // Add needs 2 args
    program.add_node(node);
    
    let verifier = Verifier::new(program);
    let result = verifier.verify_program();
    
    assert!(!result.is_valid);
    assert!(!result.errors.is_empty());
    assert!(result.errors[0].message.contains("expects 2 arguments"));
}

#[test]
fn test_verifier_invalid_arg_reference() {
    let mut program = Program::new();
    
    // Create node referencing non-existent node
    let node = Node::new(OpCode::Add, 1).with_args(&[99, 100]); // Invalid refs
    program.add_node(node);
    
    let verifier = Verifier::new(program);
    let result = verifier.verify_program();
    
    assert!(!result.is_valid);
    assert!(!result.errors.is_empty());
    assert!(result.errors[0].message.contains("Invalid argument reference"));
}

#[test]
fn test_safety_analysis() {
    let mut program = Program::new();
    
    // Create program with unsafe operations
    let msg = program.constants.add_string("Hello".to_string());
    let n1 = Node::new(OpCode::ConstString, 1).with_args(&[msg]);
    let n2 = Node::new(OpCode::Print, 2).with_args(&[1]);
    let n3 = Node::new(OpCode::ExternalCall, 3).with_args(&[1]);
    
    program.add_node(n1);
    program.add_node(n2);
    program.add_node(n3);
    
    let verifier = Verifier::new(program);
    let safety = verifier.verify_safety();
    
    assert!(safety.has_unsafe_operations);
    assert!(!safety.side_effects.is_empty());
    assert!(safety.side_effects.iter().any(|s| s.contains("I/O operation")));
    assert!(safety.side_effects.iter().any(|s| s.contains("External call")));
}

#[test]
fn test_proof_checker() {
    let proof = Proof {
        theorem: "Test theorem".to_string(),
        trait_kind: TraitKind::IsPure,
        assumptions: vec![
            Assumption {
                description: "Input is valid".to_string(),
                condition: ConditionExpression::Constant(ConstantValue::Boolean(true)),
            }
        ],
        steps: vec![
            ProofStep {
                step_number: 1,
                description: "Step 1".to_string(),
                justification: Justification::Assumption(0),
                derived_fact: ConditionExpression::Constant(ConstantValue::Boolean(true)),
            },
            ProofStep {
                step_number: 2,
                description: "Step 2".to_string(),
                justification: Justification::ModusPonens(0, 0),
                derived_fact: ConditionExpression::Constant(ConstantValue::Boolean(true)),
            },
        ],
        conclusion: Conclusion {
            statement: "Theorem is true".to_string(),
            expression: ConditionExpression::Constant(ConstantValue::Boolean(true)),
        },
    };
    
    let checker = ProofChecker::new();
    let result = checker.verify_proof(&proof);
    assert!(result.is_ok());
}

#[test]
fn test_proof_checker_invalid_reference() {
    let proof = Proof {
        theorem: "Test theorem".to_string(),
        trait_kind: TraitKind::IsPure,
        assumptions: vec![],
        steps: vec![
            ProofStep {
                step_number: 1,
                description: "Invalid step".to_string(),
                justification: Justification::Assumption(5), // Invalid assumption ref
                derived_fact: ConditionExpression::Constant(ConstantValue::Boolean(true)),
            },
        ],
        conclusion: Conclusion {
            statement: "Theorem is true".to_string(),
            expression: ConditionExpression::Constant(ConstantValue::Boolean(true)),
        },
    };
    
    let checker = ProofChecker::new();
    let result = checker.verify_proof(&proof);
    assert!(result.is_err());
}