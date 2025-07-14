use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TraitKind {
    // Data properties
    IsSorted,
    IsUnique,
    PreservesLength,
    PreservesSum,
    
    // Type properties
    ReturnsType(TypeConstraint),
    AcceptsType(TypeConstraint),
    
    // Purity properties
    IsPure,
    IsDeterministic,
    HasNoSideEffects,
    
    // Memory properties
    IsMemorySafe,
    NoMemoryLeaks,
    BoundedMemoryUsage(usize),
    
    // Complexity properties
    TimeComplexity(ComplexityBound),
    SpaceComplexity(ComplexityBound),
    
    // Custom properties
    Custom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeConstraint {
    Integer,
    Float,
    Boolean,
    String,
    Array(Box<TypeConstraint>),
    Map(Box<TypeConstraint>, Box<TypeConstraint>),
    Function(Vec<TypeConstraint>, Box<TypeConstraint>),
    Union(Vec<TypeConstraint>),
    Any,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComplexityBound {
    Constant,
    Logarithmic,
    Linear,
    Quadratic,
    Polynomial(u32),
    Exponential,
}

#[derive(Debug, Clone)]
pub struct TraitDefinition {
    pub name: String,
    pub kind: TraitKind,
    pub preconditions: Vec<Condition>,
    pub postconditions: Vec<Condition>,
    pub invariants: Vec<Invariant>,
}

#[derive(Debug, Clone)]
pub struct Condition {
    pub description: String,
    pub expression: ConditionExpression,
}

#[derive(Debug, Clone)]
pub enum ConditionExpression {
    // Comparison
    Equal(Box<ConditionExpression>, Box<ConditionExpression>),
    NotEqual(Box<ConditionExpression>, Box<ConditionExpression>),
    LessThan(Box<ConditionExpression>, Box<ConditionExpression>),
    LessThanOrEqual(Box<ConditionExpression>, Box<ConditionExpression>),
    GreaterThan(Box<ConditionExpression>, Box<ConditionExpression>),
    GreaterThanOrEqual(Box<ConditionExpression>, Box<ConditionExpression>),
    
    // Logical
    And(Box<ConditionExpression>, Box<ConditionExpression>),
    Or(Box<ConditionExpression>, Box<ConditionExpression>),
    Not(Box<ConditionExpression>),
    Implies(Box<ConditionExpression>, Box<ConditionExpression>),
    
    // Quantifiers
    ForAll(String, Box<ConditionExpression>),
    Exists(String, Box<ConditionExpression>),
    
    // Values
    Variable(String),
    Constant(ConstantValue),
    Property(Box<ConditionExpression>, String),
    
    // Array operations
    Length(Box<ConditionExpression>),
    Element(Box<ConditionExpression>, Box<ConditionExpression>),
    Sum(Box<ConditionExpression>),
    
    // Function application
    Apply(Box<ConditionExpression>, Vec<ConditionExpression>),
}

#[derive(Debug, Clone)]
pub enum ConstantValue {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
}

#[derive(Debug, Clone)]
pub struct Invariant {
    pub description: String,
    pub expression: ConditionExpression,
    pub check_points: Vec<CheckPoint>,
}

#[derive(Debug, Clone)]
pub enum CheckPoint {
    BeforeExecution,
    AfterExecution,
    BeforeLoop,
    AfterLoop,
    OnStateChange,
}

pub struct TraitRegistry {
    traits: HashMap<String, TraitDefinition>,
}

impl TraitRegistry {
    pub fn new() -> Self {
        let mut registry = TraitRegistry {
            traits: HashMap::new(),
        };
        registry.register_builtin_traits();
        registry
    }
    
    fn register_builtin_traits(&mut self) {
        // IsSorted trait
        self.register_trait(TraitDefinition {
            name: "IsSorted".to_string(),
            kind: TraitKind::IsSorted,
            preconditions: vec![
                Condition {
                    description: "Input is an array".to_string(),
                    expression: ConditionExpression::Equal(
                        Box::new(ConditionExpression::Property(
                            Box::new(ConditionExpression::Variable("input".to_string())),
                            "type".to_string()
                        )),
                        Box::new(ConditionExpression::Constant(ConstantValue::String("array".to_string())))
                    ),
                }
            ],
            postconditions: vec![
                Condition {
                    description: "All adjacent elements are in order".to_string(),
                    expression: ConditionExpression::ForAll(
                        "i".to_string(),
                        Box::new(ConditionExpression::Implies(
                            Box::new(ConditionExpression::And(
                                Box::new(ConditionExpression::GreaterThanOrEqual(
                                    Box::new(ConditionExpression::Variable("i".to_string())),
                                    Box::new(ConditionExpression::Constant(ConstantValue::Integer(0)))
                                )),
                                Box::new(ConditionExpression::LessThan(
                                    Box::new(ConditionExpression::Variable("i".to_string())),
                                    Box::new(ConditionExpression::Length(
                                        Box::new(ConditionExpression::Variable("result".to_string()))
                                    ))
                                ))
                            )),
                            Box::new(ConditionExpression::LessThanOrEqual(
                                Box::new(ConditionExpression::Element(
                                    Box::new(ConditionExpression::Variable("result".to_string())),
                                    Box::new(ConditionExpression::Variable("i".to_string()))
                                )),
                                Box::new(ConditionExpression::Element(
                                    Box::new(ConditionExpression::Variable("result".to_string())),
                                    Box::new(ConditionExpression::Constant(ConstantValue::Integer(1)))
                                ))
                            ))
                        ))
                    ),
                }
            ],
            invariants: vec![],
        });
        
        // PreservesLength trait
        self.register_trait(TraitDefinition {
            name: "PreservesLength".to_string(),
            kind: TraitKind::PreservesLength,
            preconditions: vec![],
            postconditions: vec![
                Condition {
                    description: "Output length equals input length".to_string(),
                    expression: ConditionExpression::Equal(
                        Box::new(ConditionExpression::Length(
                            Box::new(ConditionExpression::Variable("result".to_string()))
                        )),
                        Box::new(ConditionExpression::Length(
                            Box::new(ConditionExpression::Variable("input".to_string()))
                        ))
                    ),
                }
            ],
            invariants: vec![],
        });
        
        // IsPure trait
        self.register_trait(TraitDefinition {
            name: "IsPure".to_string(),
            kind: TraitKind::IsPure,
            preconditions: vec![],
            postconditions: vec![
                Condition {
                    description: "No side effects".to_string(),
                    expression: ConditionExpression::Constant(ConstantValue::Boolean(true)),
                }
            ],
            invariants: vec![
                Invariant {
                    description: "No global state modifications".to_string(),
                    expression: ConditionExpression::Equal(
                        Box::new(ConditionExpression::Variable("global_state_before".to_string())),
                        Box::new(ConditionExpression::Variable("global_state_after".to_string()))
                    ),
                    check_points: vec![CheckPoint::AfterExecution],
                }
            ],
        });
    }
    
    pub fn register_trait(&mut self, trait_def: TraitDefinition) {
        self.traits.insert(trait_def.name.clone(), trait_def);
    }
    
    pub fn get_trait(&self, name: &str) -> Option<&TraitDefinition> {
        self.traits.get(name)
    }
    
    pub fn list_traits(&self) -> Vec<&str> {
        self.traits.keys().map(|s| s.as_str()).collect()
    }
}