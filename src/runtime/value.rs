use std::collections::HashMap;
use std::sync::Arc;
use crate::runtime::{MemoryReference, AsyncHandle};

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
    Function(Arc<Function>),
    NodeRef(u32),
    MemoryRef(MemoryReference),
    AsyncHandle(AsyncHandle),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub node_id: u32,
    pub arity: usize,
    pub captured_values: HashMap<u32, Value>,
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Nil => "nil",
            Value::Bool(_) => "bool",
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Map(_) => "map",
            Value::Function(_) => "function",
            Value::NodeRef(_) => "noderef",
            Value::MemoryRef(_) => "memoryref",
            Value::AsyncHandle(_) => "asynchandle",
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Map(m) => !m.is_empty(),
            _ => true,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Nil => "nil".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s.clone(),
            Value::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", elements.join(", "))
            }
            Value::Map(map) => {
                let pairs: Vec<String> = map.iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            }
            Value::Function(f) => format!("<function:{}>", f.node_id),
            Value::NodeRef(id) => format!("<node:{}>", id),
            Value::MemoryRef(r) => format!("<memory:0x{:x}+{}>", r.address, r.offset),
            Value::AsyncHandle(h) => format!("<async:{}>", h.id),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => (a - b).abs() < f64::EPSILON,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Map(a), Value::Map(b)) => a == b,
            (Value::NodeRef(a), Value::NodeRef(b)) => a == b,
            _ => false,
        }
    }
}