use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    // Primitive types
    Nil,
    Bool,
    Int,
    Float,
    String,
    
    // Composite types
    Array(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Function(Vec<Type>, Box<Type>), // (params, return)
    
    // Reference types
    NodeRef,
    MemoryRef(Box<Type>),
    AsyncHandle(Box<Type>),
    
    // Type variables (for inference)
    TypeVar(u32),
    
    // Union types
    Union(Vec<Type>),
    
    // Special types
    Any,
    Never,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Nil => write!(f, "nil"),
            Type::Bool => write!(f, "bool"),
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::String => write!(f, "string"),
            Type::Array(elem) => write!(f, "array<{}>", elem),
            Type::Map(key, val) => write!(f, "map<{}, {}>", key, val),
            Type::Function(params, ret) => {
                write!(f, "fn(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ") -> {}", ret)
            }
            Type::NodeRef => write!(f, "noderef"),
            Type::MemoryRef(t) => write!(f, "memref<{}>", t),
            Type::AsyncHandle(t) => write!(f, "async<{}>", t),
            Type::TypeVar(id) => write!(f, "T{}", id),
            Type::Union(types) => {
                write!(f, "(")?;
                for (i, t) in types.iter().enumerate() {
                    if i > 0 {
                        write!(f, " | ")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, ")")
            }
            Type::Any => write!(f, "any"),
            Type::Never => write!(f, "never"),
        }
    }
}

impl Type {
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Int | Type::Float)
    }
    
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            Type::Nil | Type::Bool | Type::Int | Type::Float | Type::String
        )
    }
    
    pub fn is_reference(&self) -> bool {
        matches!(
            self,
            Type::NodeRef | Type::MemoryRef(_) | Type::AsyncHandle(_)
        )
    }
    
    pub fn is_compatible_with(&self, other: &Type) -> bool {
        match (self, other) {
            // Any type is compatible with anything
            (Type::Any, _) | (_, Type::Any) => true,
            
            // Never is compatible with nothing
            (Type::Never, _) | (_, Type::Never) => false,
            
            // Same types are compatible
            (a, b) if a == b => true,
            
            // Numeric types are compatible
            (Type::Int, Type::Float) | (Type::Float, Type::Int) => true,
            
            // Array compatibility
            (Type::Array(a), Type::Array(b)) => a.is_compatible_with(b),
            
            // Map compatibility
            (Type::Map(k1, v1), Type::Map(k2, v2)) => {
                k1.is_compatible_with(k2) && v1.is_compatible_with(v2)
            }
            
            // Function compatibility (contravariant params, covariant return)
            (Type::Function(params1, ret1), Type::Function(params2, ret2)) => {
                params1.len() == params2.len() &&
                params1.iter().zip(params2.iter()).all(|(p1, p2)| p2.is_compatible_with(p1)) &&
                ret1.is_compatible_with(ret2)
            }
            
            // Union compatibility
            (Type::Union(types), other) => types.iter().any(|t| t.is_compatible_with(other)),
            (other, Type::Union(types)) => types.iter().any(|t| other.is_compatible_with(t)),
            
            _ => false,
        }
    }
    
    pub fn common_type(&self, other: &Type) -> Option<Type> {
        match (self, other) {
            // Same type
            (a, b) if a == b => Some(a.clone()),
            
            // Numeric promotion
            (Type::Int, Type::Float) | (Type::Float, Type::Int) => Some(Type::Float),
            
            // Any type
            (Type::Any, other) | (other, Type::Any) => Some(other.clone()),
            
            // Arrays with common element type
            (Type::Array(a), Type::Array(b)) => {
                a.common_type(b).map(|t| Type::Array(Box::new(t)))
            }
            
            // Otherwise create union
            _ => Some(Type::Union(vec![self.clone(), other.clone()])),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeSignature {
    pub params: Vec<Type>,
    pub return_type: Type,
    pub type_params: Vec<u32>, // Generic type parameters
    pub constraints: Vec<TypeConstraint>,
}

#[derive(Debug, Clone)]
pub enum TypeConstraint {
    Numeric(u32),           // Type var must be numeric
    Comparable(u32),        // Type var must be comparable
    Equatable(u32),         // Type var must support equality
    HasLength(u32),         // Type var must have length (array, string)
    Callable(u32),          // Type var must be callable
    SameAs(u32, u32),       // Two type vars must be the same
}

pub struct TypeEnvironment {
    pub variables: HashMap<String, Type>,
    pub functions: HashMap<String, TypeSignature>,
    pub type_vars: HashMap<u32, Type>,
    next_type_var: u32,
}

impl TypeEnvironment {
    pub fn new() -> Self {
        TypeEnvironment {
            variables: HashMap::new(),
            functions: HashMap::new(),
            type_vars: HashMap::new(),
            next_type_var: 0,
        }
    }
    
    pub fn new_type_var(&mut self) -> Type {
        let var = Type::TypeVar(self.next_type_var);
        self.next_type_var += 1;
        var
    }
    
    pub fn bind_type_var(&mut self, var_id: u32, ty: Type) {
        self.type_vars.insert(var_id, ty);
    }
    
    pub fn resolve_type(&self, ty: &Type) -> Type {
        match ty {
            Type::TypeVar(id) => {
                if let Some(resolved) = self.type_vars.get(id) {
                    self.resolve_type(resolved)
                } else {
                    ty.clone()
                }
            }
            Type::Array(elem) => Type::Array(Box::new(self.resolve_type(elem))),
            Type::Map(key, val) => Type::Map(
                Box::new(self.resolve_type(key)),
                Box::new(self.resolve_type(val))
            ),
            Type::Function(params, ret) => Type::Function(
                params.iter().map(|p| self.resolve_type(p)).collect(),
                Box::new(self.resolve_type(ret))
            ),
            Type::MemoryRef(inner) => Type::MemoryRef(Box::new(self.resolve_type(inner))),
            Type::AsyncHandle(inner) => Type::AsyncHandle(Box::new(self.resolve_type(inner))),
            Type::Union(types) => Type::Union(
                types.iter().map(|t| self.resolve_type(t)).collect()
            ),
            _ => ty.clone(),
        }
    }
    
    pub fn add_builtin_functions(&mut self) {
        // Arithmetic operations
        self.functions.insert("add".to_string(), TypeSignature {
            params: vec![Type::TypeVar(0), Type::TypeVar(0)],
            return_type: Type::TypeVar(0),
            type_params: vec![0],
            constraints: vec![TypeConstraint::Numeric(0)],
        });
        
        // Comparison operations
        self.functions.insert("eq".to_string(), TypeSignature {
            params: vec![Type::TypeVar(0), Type::TypeVar(0)],
            return_type: Type::Bool,
            type_params: vec![0],
            constraints: vec![TypeConstraint::Equatable(0)],
        });
        
        // Array operations
        self.functions.insert("array_get".to_string(), TypeSignature {
            params: vec![Type::Array(Box::new(Type::TypeVar(0))), Type::Int],
            return_type: Type::TypeVar(0),
            type_params: vec![0],
            constraints: vec![],
        });
        
        // Memory operations
        self.functions.insert("alloc".to_string(), TypeSignature {
            params: vec![Type::Int, Type::TypeVar(0)],
            return_type: Type::MemoryRef(Box::new(Type::TypeVar(0))),
            type_params: vec![0],
            constraints: vec![],
        });
        
        // Async operations
        self.functions.insert("async_begin".to_string(), TypeSignature {
            params: vec![],
            return_type: Type::AsyncHandle(Box::new(Type::TypeVar(0))),
            type_params: vec![0],
            constraints: vec![],
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_type_compatibility() {
        assert!(Type::Int.is_compatible_with(&Type::Int));
        assert!(Type::Int.is_compatible_with(&Type::Float));
        assert!(Type::Float.is_compatible_with(&Type::Int));
        assert!(!Type::Int.is_compatible_with(&Type::String));
        
        assert!(Type::Any.is_compatible_with(&Type::Int));
        assert!(Type::Int.is_compatible_with(&Type::Any));
        
        let array_int = Type::Array(Box::new(Type::Int));
        let array_float = Type::Array(Box::new(Type::Float));
        assert!(array_int.is_compatible_with(&array_float));
    }
    
    #[test]
    fn test_common_type() {
        assert_eq!(
            Type::Int.common_type(&Type::Float),
            Some(Type::Float)
        );
        
        assert_eq!(
            Type::Int.common_type(&Type::Int),
            Some(Type::Int)
        );
        
        let union = Type::Int.common_type(&Type::String).unwrap();
        match union {
            Type::Union(types) => {
                assert_eq!(types.len(), 2);
                assert!(types.contains(&Type::Int));
                assert!(types.contains(&Type::String));
            }
            _ => panic!("Expected union type"),
        }
    }
    
    #[test]
    fn test_type_display() {
        assert_eq!(Type::Int.to_string(), "int");
        assert_eq!(Type::Array(Box::new(Type::Int)).to_string(), "array<int>");
        assert_eq!(
            Type::Map(Box::new(Type::String), Box::new(Type::Int)).to_string(),
            "map<string, int>"
        );
        assert_eq!(
            Type::Function(vec![Type::Int, Type::Int], Box::new(Type::Int)).to_string(),
            "fn(int, int) -> int"
        );
    }
}