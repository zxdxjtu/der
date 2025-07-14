use crate::runtime::Value;
use crate::verification::traits::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Constraint {
    pub name: String,
    pub expression: ConstraintExpression,
    pub severity: ConstraintSeverity,
}

#[derive(Debug, Clone)]
pub enum ConstraintSeverity {
    Error,    // Must be satisfied
    Warning,  // Should be satisfied
    Info,     // Nice to have
}

#[derive(Debug, Clone)]
pub enum ConstraintExpression {
    // Type constraints
    TypeIs(String, TypeConstraint),
    TypeCompatible(String, String),
    
    // Value constraints  
    InRange(String, RangeConstraint),
    NotNull(String),
    Unique(Vec<String>),
    
    // Relationship constraints
    LessThan(String, String),
    GreaterThan(String, String),
    Equal(String, String),
    NotEqual(String, String),
    
    // Array constraints
    ArrayLength(String, LengthConstraint),
    ArraySorted(String, SortOrder),
    ArrayContains(String, Value),
    
    // Logical combinations
    All(Vec<ConstraintExpression>),
    Any(Vec<ConstraintExpression>),
    Not(Box<ConstraintExpression>),
}

#[derive(Debug, Clone)]
pub enum RangeConstraint {
    Integer { min: Option<i64>, max: Option<i64> },
    Float { min: Option<f64>, max: Option<f64> },
}

#[derive(Debug, Clone)]
pub enum LengthConstraint {
    Exact(usize),
    Min(usize),
    Max(usize),
    Range(usize, usize),
}

#[derive(Debug, Clone)]
pub enum SortOrder {
    Ascending,
    Descending,
}

pub struct ConstraintChecker {
    constraints: Vec<Constraint>,
    values: HashMap<String, Value>,
}

impl ConstraintChecker {
    pub fn new() -> Self {
        ConstraintChecker {
            constraints: Vec::new(),
            values: HashMap::new(),
        }
    }
    
    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }
    
    pub fn set_value(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }
    
    pub fn check_all(&self) -> Vec<ConstraintViolation> {
        let mut violations = Vec::new();
        
        for constraint in &self.constraints {
            if let Err(violation) = self.check_constraint(&constraint.expression) {
                violations.push(ConstraintViolation {
                    constraint_name: constraint.name.clone(),
                    severity: constraint.severity.clone(),
                    message: violation,
                });
            }
        }
        
        violations
    }
    
    fn check_constraint(&self, expr: &ConstraintExpression) -> Result<(), String> {
        match expr {
            ConstraintExpression::TypeIs(var_name, expected_type) => {
                self.check_type_constraint(var_name, expected_type)
            }
            
            ConstraintExpression::InRange(var_name, range) => {
                self.check_range_constraint(var_name, range)
            }
            
            ConstraintExpression::NotNull(var_name) => {
                match self.values.get(var_name) {
                    Some(Value::Nil) => Err(format!("{} is null", var_name)),
                    None => Err(format!("{} is not defined", var_name)),
                    _ => Ok(()),
                }
            }
            
            ConstraintExpression::LessThan(left, right) => {
                self.check_comparison(left, right, |a, b| a < b)
            }
            
            ConstraintExpression::GreaterThan(left, right) => {
                self.check_comparison(left, right, |a, b| a > b)
            }
            
            ConstraintExpression::Equal(left, right) => {
                let left_val = self.values.get(left)
                    .ok_or(format!("{} not found", left))?;
                let right_val = self.values.get(right)
                    .ok_or(format!("{} not found", right))?;
                
                if left_val == right_val {
                    Ok(())
                } else {
                    Err(format!("{} != {}", left, right))
                }
            }
            
            ConstraintExpression::ArrayLength(var_name, length_constraint) => {
                self.check_array_length(var_name, length_constraint)
            }
            
            ConstraintExpression::ArraySorted(var_name, order) => {
                self.check_array_sorted(var_name, order)
            }
            
            ConstraintExpression::All(constraints) => {
                for constraint in constraints {
                    self.check_constraint(constraint)?;
                }
                Ok(())
            }
            
            ConstraintExpression::Any(constraints) => {
                for constraint in constraints {
                    if self.check_constraint(constraint).is_ok() {
                        return Ok(());
                    }
                }
                Err("None of the constraints were satisfied".to_string())
            }
            
            ConstraintExpression::Not(constraint) => {
                match self.check_constraint(constraint) {
                    Ok(()) => Err("Constraint should not be satisfied".to_string()),
                    Err(_) => Ok(()),
                }
            }
            
            _ => Err("Constraint not implemented".to_string()),
        }
    }
    
    fn check_type_constraint(&self, var_name: &str, expected_type: &TypeConstraint) -> Result<(), String> {
        let value = self.values.get(var_name)
            .ok_or(format!("{} not found", var_name))?;
        
        match (value, expected_type) {
            (Value::Int(_), TypeConstraint::Integer) => Ok(()),
            (Value::Float(_), TypeConstraint::Float) => Ok(()),
            (Value::Bool(_), TypeConstraint::Boolean) => Ok(()),
            (Value::String(_), TypeConstraint::String) => Ok(()),
            (Value::Array(_), TypeConstraint::Array(_)) => Ok(()), // TODO: check element types
            (Value::Map(_), TypeConstraint::Map(_, _)) => Ok(()), // TODO: check key/value types
            _ => Err(format!("{} has wrong type", var_name)),
        }
    }
    
    fn check_range_constraint(&self, var_name: &str, range: &RangeConstraint) -> Result<(), String> {
        let value = self.values.get(var_name)
            .ok_or(format!("{} not found", var_name))?;
        
        match (value, range) {
            (Value::Int(n), RangeConstraint::Integer { min, max }) => {
                if let Some(min_val) = min {
                    if n < min_val {
                        return Err(format!("{} is less than {}", n, min_val));
                    }
                }
                if let Some(max_val) = max {
                    if n > max_val {
                        return Err(format!("{} is greater than {}", n, max_val));
                    }
                }
                Ok(())
            }
            (Value::Float(f), RangeConstraint::Float { min, max }) => {
                if let Some(min_val) = min {
                    if f < min_val {
                        return Err(format!("{} is less than {}", f, min_val));
                    }
                }
                if let Some(max_val) = max {
                    if f > max_val {
                        return Err(format!("{} is greater than {}", f, max_val));
                    }
                }
                Ok(())
            }
            _ => Err("Type mismatch for range constraint".to_string()),
        }
    }
    
    fn check_comparison<F>(&self, left: &str, right: &str, op: F) -> Result<(), String>
    where
        F: Fn(f64, f64) -> bool,
    {
        let left_val = self.values.get(left)
            .ok_or(format!("{} not found", left))?;
        let right_val = self.values.get(right)
            .ok_or(format!("{} not found", right))?;
        
        let (left_num, right_num) = match (left_val, right_val) {
            (Value::Int(a), Value::Int(b)) => (*a as f64, *b as f64),
            (Value::Float(a), Value::Float(b)) => (*a, *b),
            (Value::Int(a), Value::Float(b)) => (*a as f64, *b),
            (Value::Float(a), Value::Int(b)) => (*a, *b as f64),
            _ => return Err("Cannot compare non-numeric values".to_string()),
        };
        
        if op(left_num, right_num) {
            Ok(())
        } else {
            Err(format!("Comparison failed: {} vs {}", left_num, right_num))
        }
    }
    
    fn check_array_length(&self, var_name: &str, constraint: &LengthConstraint) -> Result<(), String> {
        let value = self.values.get(var_name)
            .ok_or(format!("{} not found", var_name))?;
        
        match value {
            Value::Array(arr) => {
                let len = arr.len();
                match constraint {
                    LengthConstraint::Exact(expected) => {
                        if len == *expected {
                            Ok(())
                        } else {
                            Err(format!("Array length {} != {}", len, expected))
                        }
                    }
                    LengthConstraint::Min(min) => {
                        if len >= *min {
                            Ok(())
                        } else {
                            Err(format!("Array length {} < {}", len, min))
                        }
                    }
                    LengthConstraint::Max(max) => {
                        if len <= *max {
                            Ok(())
                        } else {
                            Err(format!("Array length {} > {}", len, max))
                        }
                    }
                    LengthConstraint::Range(min, max) => {
                        if len >= *min && len <= *max {
                            Ok(())
                        } else {
                            Err(format!("Array length {} not in range [{}, {}]", len, min, max))
                        }
                    }
                }
            }
            _ => Err(format!("{} is not an array", var_name)),
        }
    }
    
    fn check_array_sorted(&self, var_name: &str, order: &SortOrder) -> Result<(), String> {
        let value = self.values.get(var_name)
            .ok_or(format!("{} not found", var_name))?;
        
        match value {
            Value::Array(arr) => {
                if arr.is_empty() || arr.len() == 1 {
                    return Ok(());
                }
                
                for i in 1..arr.len() {
                    let prev = &arr[i - 1];
                    let curr = &arr[i];
                    
                    let cmp_result = match (prev, curr) {
                        (Value::Int(a), Value::Int(b)) => a.cmp(b),
                        (Value::Float(a), Value::Float(b)) => {
                            if a < b {
                                std::cmp::Ordering::Less
                            } else if a > b {
                                std::cmp::Ordering::Greater
                            } else {
                                std::cmp::Ordering::Equal
                            }
                        }
                        _ => return Err("Cannot compare array elements".to_string()),
                    };
                    
                    match order {
                        SortOrder::Ascending => {
                            if cmp_result == std::cmp::Ordering::Greater {
                                return Err("Array is not sorted in ascending order".to_string());
                            }
                        }
                        SortOrder::Descending => {
                            if cmp_result == std::cmp::Ordering::Less {
                                return Err("Array is not sorted in descending order".to_string());
                            }
                        }
                    }
                }
                Ok(())
            }
            _ => Err(format!("{} is not an array", var_name)),
        }
    }
}

#[derive(Debug)]
pub struct ConstraintViolation {
    pub constraint_name: String,
    pub severity: ConstraintSeverity,
    pub message: String,
}