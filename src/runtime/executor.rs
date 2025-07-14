use std::sync::Arc;
use std::collections::HashMap;
use crate::core::{Program, Node, OpCode, NodeFlag, Capability};
use crate::runtime::{ExecutionContext, Value, Function, RuntimeError, Result, MemoryReference};

pub struct Executor {
    context: ExecutionContext,
}

impl Executor {
    pub fn new(program: Program) -> Self {
        Executor {
            context: ExecutionContext::new(program),
        }
    }

    pub fn grant_capability(&mut self, cap: Capability) {
        self.context.grant_capability(cap);
    }

    pub fn set_argument(&mut self, index: usize, value: Value) {
        // Set argument at predefined slots (1000+)
        self.context.set_value(1000 + index as u32, value);
    }

    pub fn set_argc(&mut self, count: usize) {
        // Set argument count at slot 999
        self.context.set_value(999, Value::Int(count as i64));
    }

    pub fn execute(&mut self) -> Result<Value> {
        let entry_point = self.context.program.metadata.entry_point;
        self.execute_node(entry_point)
    }

    fn execute_node(&mut self, node_id: u32) -> Result<Value> {
        let node = self.context.get_node(node_id)
            .ok_or(RuntimeError::InvalidNodeRef(node_id))?
            .clone();

        // Check if we've already computed this value
        if let Some(value) = self.context.get_value(node.result_id) {
            return Ok(value.clone());
        }

        // Execute based on opcode
        let result = match OpCode::try_from(node.opcode) {
            Ok(opcode) => self.execute_opcode(opcode, &node)?,
            Err(_) => return Err(RuntimeError::UnknownOpcode(node.opcode)),
        };

        // Store the result
        self.context.set_value(node.result_id, result.clone());

        Ok(result)
    }

    fn execute_opcode(&mut self, opcode: OpCode, node: &Node) -> Result<Value> {
        match opcode {
            OpCode::Nop => Ok(Value::Nil),
            OpCode::Return => self.execute_return(node),
            OpCode::Call => self.execute_call(node),
            OpCode::Branch => self.execute_branch(node),
            
            // Arithmetic
            OpCode::Add => self.execute_binary_arithmetic(node, |a, b| a + b),
            OpCode::Sub => self.execute_binary_arithmetic(node, |a, b| a - b),
            OpCode::Mul => self.execute_binary_arithmetic(node, |a, b| a * b),
            OpCode::Div => self.execute_division(node),
            OpCode::Mod => self.execute_modulo(node),
            
            // Comparison
            OpCode::Eq => self.execute_comparison(node, |a, b| a == b),
            OpCode::Ne => self.execute_comparison(node, |a, b| a != b),
            OpCode::Lt => self.execute_numeric_comparison(node, |a, b| a < b),
            OpCode::Le => self.execute_numeric_comparison(node, |a, b| a <= b),
            OpCode::Gt => self.execute_numeric_comparison(node, |a, b| a > b),
            OpCode::Ge => self.execute_numeric_comparison(node, |a, b| a >= b),
            
            // Logical
            OpCode::And => self.execute_logical_and(node),
            OpCode::Or => self.execute_logical_or(node),
            OpCode::Not => self.execute_logical_not(node),
            OpCode::Xor => self.execute_logical_xor(node),
            
            // Constants
            OpCode::ConstInt => self.execute_const_int(node),
            OpCode::ConstFloat => self.execute_const_float(node),
            OpCode::ConstString => self.execute_const_string(node),
            OpCode::ConstBool => self.execute_const_bool(node),
            
            // Data structures
            OpCode::CreateArray => self.execute_create_array(node),
            OpCode::CreateMap => self.execute_create_map(node),
            OpCode::ArrayGet => self.execute_array_get(node),
            OpCode::ArraySet => self.execute_array_set(node),
            OpCode::MapGet => self.execute_map_get(node),
            OpCode::MapSet => self.execute_map_set(node),
            
            // Functions
            OpCode::DefineFunc => self.execute_define_func(node),
            OpCode::CreateClosure => self.execute_create_closure(node),
            
            // IO
            OpCode::Print => self.execute_print(node),
            
            // Memory operations
            OpCode::Alloc => self.execute_alloc(node),
            OpCode::Free => self.execute_free(node),
            OpCode::Load => self.execute_load(node),
            OpCode::Store => self.execute_store(node),
            OpCode::LoadArg => self.execute_load_arg(node),
            
            // Async operations
            OpCode::AsyncBegin => self.execute_async_begin(node),
            OpCode::AsyncAwait => self.execute_async_await(node),
            OpCode::AsyncComplete => self.execute_async_complete(node),
            
            _ => Err(RuntimeError::InvalidOperation(
                format!("Opcode {:?} not implemented", opcode)
            )),
        }
    }

    fn get_arg_value(&mut self, node: &Node, arg_index: usize) -> Result<Value> {
        if arg_index >= node.arg_count as usize {
            return Err(RuntimeError::InvalidArgCount {
                expected: arg_index + 1,
                actual: node.arg_count as usize,
            });
        }

        let arg_id = node.args[arg_index];
        if arg_id == 0 {
            return Ok(Value::Nil);
        }

        // First check if we already have a computed value (prevents infinite recursion)
        if let Some(value) = self.context.get_value(arg_id) {
            return Ok(value.clone());
        }

        // Check if it's a node reference that needs execution
        if let Some(_arg_node) = self.context.get_node(arg_id) {
            self.execute_node(arg_id)
        } else {
            // It's neither a computed value nor a node - this is an error
            Err(RuntimeError::InvalidNodeRef(arg_id))
        }
    }

    fn execute_return(&mut self, node: &Node) -> Result<Value> {
        if node.arg_count > 0 {
            self.get_arg_value(node, 0)
        } else {
            Ok(Value::Nil)
        }
    }

    fn execute_call(&mut self, node: &Node) -> Result<Value> {
        let func_value = self.get_arg_value(node, 0)?;
        
        match func_value {
            Value::Function(func) => {
                self.context.push_frame(func.node_id, Some(node.result_id))?;
                
                // Set up arguments as local values
                for i in 1..node.arg_count as usize {
                    let arg_value = self.get_arg_value(node, i)?;
                    if let Some(frame) = self.context.current_frame_mut() {
                        frame.locals.insert(i as u32, arg_value);
                    }
                }
                
                let result = self.execute_node(func.node_id)?;
                self.context.pop_frame();
                Ok(result)
            }
            _ => Err(RuntimeError::TypeMismatch {
                expected: "function".to_string(),
                actual: func_value.type_name().to_string(),
            }),
        }
    }

    fn execute_branch(&mut self, node: &Node) -> Result<Value> {
        let condition = self.get_arg_value(node, 0)?;
        
        if condition.is_truthy() {
            self.get_arg_value(node, 1)
        } else if node.arg_count > 2 {
            self.get_arg_value(node, 2)
        } else {
            Ok(Value::Nil)
        }
    }

    fn execute_binary_arithmetic<F>(&mut self, node: &Node, op: F) -> Result<Value>
    where
        F: Fn(f64, f64) -> f64,
    {
        let left = self.get_arg_value(node, 0)?;
        let right = self.get_arg_value(node, 1)?;

        match (&left, &right) {
            (Value::Int(a), Value::Int(b)) => {
                let result = op(*a as f64, *b as f64);
                if result.fract() == 0.0 {
                    Ok(Value::Int(result as i64))
                } else {
                    Ok(Value::Float(result))
                }
            }
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(op(*a, *b))),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(op(*a as f64, *b))),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(op(*a, *b as f64))),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "numeric".to_string(),
                actual: format!("{} and {}", left.type_name(), right.type_name()),
            }),
        }
    }

    fn execute_division(&mut self, node: &Node) -> Result<Value> {
        let right = self.get_arg_value(node, 1)?;
        
        match &right {
            Value::Int(0) => {
                return Err(RuntimeError::DivisionByZero);
            }
            Value::Float(f) if *f == 0.0 => {
                return Err(RuntimeError::DivisionByZero);
            }
            _ => {}
        }

        self.execute_binary_arithmetic(node, |a, b| a / b)
    }

    fn execute_modulo(&mut self, node: &Node) -> Result<Value> {
        let left = self.get_arg_value(node, 0)?;
        let right = self.get_arg_value(node, 1)?;

        match (&left, &right) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::Int(a % b))
            }
            _ => Err(RuntimeError::TypeMismatch {
                expected: "integer".to_string(),
                actual: format!("{} and {}", left.type_name(), right.type_name()),
            }),
        }
    }

    fn execute_comparison<F>(&mut self, node: &Node, op: F) -> Result<Value>
    where
        F: Fn(&Value, &Value) -> bool,
    {
        let left = self.get_arg_value(node, 0)?;
        let right = self.get_arg_value(node, 1)?;
        Ok(Value::Bool(op(&left, &right)))
    }

    fn execute_numeric_comparison<F>(&mut self, node: &Node, op: F) -> Result<Value>
    where
        F: Fn(f64, f64) -> bool,
    {
        let left = self.get_arg_value(node, 0)?;
        let right = self.get_arg_value(node, 1)?;

        let result = match (&left, &right) {
            (Value::Int(a), Value::Int(b)) => op(*a as f64, *b as f64),
            (Value::Float(a), Value::Float(b)) => op(*a, *b),
            (Value::Int(a), Value::Float(b)) => op(*a as f64, *b),
            (Value::Float(a), Value::Int(b)) => op(*a, *b as f64),
            _ => return Err(RuntimeError::TypeMismatch {
                expected: "numeric".to_string(),
                actual: format!("{} and {}", left.type_name(), right.type_name()),
            }),
        };

        Ok(Value::Bool(result))
    }

    fn execute_logical_and(&mut self, node: &Node) -> Result<Value> {
        let left = self.get_arg_value(node, 0)?;
        if !left.is_truthy() {
            return Ok(Value::Bool(false));
        }
        let right = self.get_arg_value(node, 1)?;
        Ok(Value::Bool(right.is_truthy()))
    }

    fn execute_logical_or(&mut self, node: &Node) -> Result<Value> {
        let left = self.get_arg_value(node, 0)?;
        if left.is_truthy() {
            return Ok(Value::Bool(true));
        }
        let right = self.get_arg_value(node, 1)?;
        Ok(Value::Bool(right.is_truthy()))
    }

    fn execute_logical_not(&mut self, node: &Node) -> Result<Value> {
        let value = self.get_arg_value(node, 0)?;
        Ok(Value::Bool(!value.is_truthy()))
    }

    fn execute_logical_xor(&mut self, node: &Node) -> Result<Value> {
        let left = self.get_arg_value(node, 0)?;
        let right = self.get_arg_value(node, 1)?;
        Ok(Value::Bool(left.is_truthy() != right.is_truthy()))
    }

    fn execute_const_int(&mut self, node: &Node) -> Result<Value> {
        let index = node.args[0];
        self.context.program.constants.get_int(index)
            .map(Value::Int)
            .ok_or(RuntimeError::InvalidConstantIndex(index))
    }

    fn execute_const_float(&mut self, node: &Node) -> Result<Value> {
        let index = node.args[0];
        self.context.program.constants.get_float(index)
            .map(Value::Float)
            .ok_or(RuntimeError::InvalidConstantIndex(index))
    }

    fn execute_const_string(&mut self, node: &Node) -> Result<Value> {
        let index = node.args[0];
        self.context.program.constants.get_string(index)
            .map(|s| Value::String(s.clone()))
            .ok_or(RuntimeError::InvalidConstantIndex(index))
    }

    fn execute_const_bool(&mut self, node: &Node) -> Result<Value> {
        let index = node.args[0];
        self.context.program.constants.get_bool(index)
            .map(Value::Bool)
            .ok_or(RuntimeError::InvalidConstantIndex(index))
    }

    fn execute_create_array(&mut self, node: &Node) -> Result<Value> {
        let mut array = Vec::new();
        for i in 0..node.arg_count as usize {
            array.push(self.get_arg_value(node, i)?);
        }
        Ok(Value::Array(array))
    }

    fn execute_create_map(&mut self, node: &Node) -> Result<Value> {
        Ok(Value::Map(HashMap::new()))
    }

    fn execute_array_get(&mut self, node: &Node) -> Result<Value> {
        let array = self.get_arg_value(node, 0)?;
        let index = self.get_arg_value(node, 1)?;

        match (&array, &index) {
            (Value::Array(arr), Value::Int(idx)) => {
                let idx = *idx as usize;
                arr.get(idx)
                    .cloned()
                    .ok_or(RuntimeError::ArrayIndexOutOfBounds {
                        index: idx,
                        length: arr.len(),
                    })
            }
            _ => Err(RuntimeError::TypeMismatch {
                expected: "array and integer".to_string(),
                actual: format!("{} and {}", array.type_name(), index.type_name()),
            }),
        }
    }

    fn execute_array_set(&mut self, node: &Node) -> Result<Value> {
        let mut array = self.get_arg_value(node, 0)?;
        let index = self.get_arg_value(node, 1)?;
        let value = self.get_arg_value(node, 2)?;

        match (&mut array, &index) {
            (Value::Array(arr), Value::Int(idx)) => {
                let idx = *idx as usize;
                if idx >= arr.len() {
                    return Err(RuntimeError::ArrayIndexOutOfBounds {
                        index: idx,
                        length: arr.len(),
                    });
                }
                arr[idx] = value;
                Ok(array)
            }
            _ => Err(RuntimeError::TypeMismatch {
                expected: "array and integer".to_string(),
                actual: format!("{} and {}", array.type_name(), index.type_name()),
            }),
        }
    }

    fn execute_map_get(&mut self, node: &Node) -> Result<Value> {
        let map = self.get_arg_value(node, 0)?;
        let key = self.get_arg_value(node, 1)?;

        match (&map, &key) {
            (Value::Map(m), Value::String(k)) => {
                m.get(k)
                    .cloned()
                    .ok_or(RuntimeError::MapKeyNotFound(k.clone()))
            }
            _ => Err(RuntimeError::TypeMismatch {
                expected: "map and string".to_string(),
                actual: format!("{} and {}", map.type_name(), key.type_name()),
            }),
        }
    }

    fn execute_map_set(&mut self, node: &Node) -> Result<Value> {
        let mut map = self.get_arg_value(node, 0)?;
        let key = self.get_arg_value(node, 1)?;
        let value = self.get_arg_value(node, 2)?;

        match (&mut map, &key) {
            (Value::Map(m), Value::String(k)) => {
                m.insert(k.clone(), value);
                Ok(map)
            }
            _ => Err(RuntimeError::TypeMismatch {
                expected: "map and string".to_string(),
                actual: format!("{} and {}", map.type_name(), key.type_name()),
            }),
        }
    }

    fn execute_define_func(&mut self, node: &Node) -> Result<Value> {
        let func = Function {
            node_id: node.args[0],
            arity: node.args[1] as usize,
            captured_values: HashMap::new(),
        };
        Ok(Value::Function(Arc::new(func)))
    }

    fn execute_create_closure(&mut self, node: &Node) -> Result<Value> {
        let base_func = self.get_arg_value(node, 0)?;
        
        match base_func {
            Value::Function(func) => {
                let mut new_func = (*func).clone();
                
                // Capture current environment values
                for i in 1..node.arg_count as usize {
                    let capture_id = node.args[i];
                    if let Some(value) = self.context.get_value(capture_id) {
                        new_func.captured_values.insert(capture_id, value.clone());
                    }
                }
                
                Ok(Value::Function(Arc::new(new_func)))
            }
            _ => Err(RuntimeError::TypeMismatch {
                expected: "function".to_string(),
                actual: base_func.type_name().to_string(),
            }),
        }
    }

    fn execute_print(&mut self, node: &Node) -> Result<Value> {
        for i in 0..node.arg_count as usize {
            let value = self.get_arg_value(node, i)?;
            print!("{}", value.to_string());
            if i < node.arg_count as usize - 1 {
                print!(" ");
            }
        }
        println!();
        Ok(Value::Nil)
    }
    
    fn execute_alloc(&mut self, node: &Node) -> Result<Value> {
        // Get size to allocate
        let size_value = self.get_arg_value(node, 0)?;
        let size = match size_value {
            Value::Int(s) if s > 0 => s as usize,
            _ => return Err(RuntimeError::TypeMismatch {
                expected: "positive integer".to_string(),
                actual: size_value.type_name().to_string(),
            }),
        };
        
        // Get initial value (optional, defaults to Nil)
        let initial_value = if node.arg_count > 1 {
            self.get_arg_value(node, 1)?
        } else {
            Value::Nil
        };
        
        // Allocate memory
        let address = self.context.memory.allocate(size, initial_value)?;
        
        Ok(Value::MemoryRef(MemoryReference {
            address,
            offset: 0,
        }))
    }
    
    fn execute_free(&mut self, node: &Node) -> Result<Value> {
        let mem_ref = self.get_arg_value(node, 0)?;
        
        match mem_ref {
            Value::MemoryRef(ref_val) => {
                self.context.memory.free(ref_val.address)?;
                Ok(Value::Nil)
            }
            _ => Err(RuntimeError::TypeMismatch {
                expected: "memory reference".to_string(),
                actual: mem_ref.type_name().to_string(),
            }),
        }
    }
    
    fn execute_load(&mut self, node: &Node) -> Result<Value> {
        let mem_ref = self.get_arg_value(node, 0)?;
        
        match mem_ref {
            Value::MemoryRef(ref_val) => {
                self.context.memory.load(ref_val.address)
            }
            _ => Err(RuntimeError::TypeMismatch {
                expected: "memory reference".to_string(),
                actual: mem_ref.type_name().to_string(),
            }),
        }
    }
    
    fn execute_store(&mut self, node: &Node) -> Result<Value> {
        let mem_ref = self.get_arg_value(node, 0)?;
        let value = self.get_arg_value(node, 1)?;
        
        match mem_ref {
            Value::MemoryRef(ref_val) => {
                self.context.memory.store(ref_val.address, value.clone())?;
                Ok(value)
            }
            _ => Err(RuntimeError::TypeMismatch {
                expected: "memory reference".to_string(),
                actual: mem_ref.type_name().to_string(),
            }),
        }
    }
    
    fn execute_load_arg(&mut self, node: &Node) -> Result<Value> {
        let arg_index = self.get_arg_value(node, 0)?;
        
        match arg_index {
            Value::Int(index) => {
                // Load argument from predefined slot (1000 + index)
                let arg_slot = 1000 + index as u32;
                self.context.get_value(arg_slot)
                    .cloned()
                    .ok_or(RuntimeError::InvalidOperation(format!("Argument {} not found", index)))
            }
            _ => Err(RuntimeError::TypeMismatch {
                expected: "integer".to_string(),
                actual: arg_index.type_name().to_string(),
            }),
        }
    }
    
    fn execute_async_begin(&mut self, node: &Node) -> Result<Value> {
        let handle = self.context.async_runtime.begin_async();
        Ok(Value::AsyncHandle(handle))
    }
    
    fn execute_async_await(&mut self, node: &Node) -> Result<Value> {
        let handle_value = self.get_arg_value(node, 0)?;
        
        match handle_value {
            Value::AsyncHandle(handle) => {
                // Check if the async operation is complete
                match self.context.async_runtime.get_result(&handle)? {
                    Some(result) => Ok(result),
                    None => {
                        // Still pending - in a real implementation this would yield
                        // For now, we'll return a special pending value
                        Ok(Value::AsyncHandle(handle))
                    }
                }
            }
            _ => Err(RuntimeError::TypeMismatch {
                expected: "async handle".to_string(),
                actual: handle_value.type_name().to_string(),
            }),
        }
    }
    
    fn execute_async_complete(&mut self, node: &Node) -> Result<Value> {
        let handle_value = self.get_arg_value(node, 0)?;
        let result_value = self.get_arg_value(node, 1)?;
        
        match handle_value {
            Value::AsyncHandle(handle) => {
                self.context.async_runtime.complete_async(&handle, result_value)?;
                Ok(Value::Nil)
            }
            _ => Err(RuntimeError::TypeMismatch {
                expected: "async handle".to_string(),
                actual: handle_value.type_name().to_string(),
            }),
        }
    }
}

impl TryFrom<u16> for OpCode {
    type Error = ();

    fn try_from(value: u16) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0000 => Ok(OpCode::Nop),
            0x0001 => Ok(OpCode::Return),
            0x0002 => Ok(OpCode::Call),
            0x0003 => Ok(OpCode::Branch),
            
            0x0100 => Ok(OpCode::Add),
            0x0101 => Ok(OpCode::Sub),
            0x0102 => Ok(OpCode::Mul),
            0x0103 => Ok(OpCode::Div),
            0x0104 => Ok(OpCode::Mod),
            
            0x0200 => Ok(OpCode::Eq),
            0x0201 => Ok(OpCode::Ne),
            0x0202 => Ok(OpCode::Lt),
            0x0203 => Ok(OpCode::Le),
            0x0204 => Ok(OpCode::Gt),
            0x0205 => Ok(OpCode::Ge),
            
            0x0300 => Ok(OpCode::And),
            0x0301 => Ok(OpCode::Or),
            0x0302 => Ok(OpCode::Not),
            0x0303 => Ok(OpCode::Xor),
            
            0x0400 => Ok(OpCode::Load),
            0x0401 => Ok(OpCode::Store),
            0x0402 => Ok(OpCode::Alloc),
            0x0403 => Ok(OpCode::Free),
            0x0404 => Ok(OpCode::LoadArg),
            
            0x0500 => Ok(OpCode::ConstInt),
            0x0501 => Ok(OpCode::ConstFloat),
            0x0502 => Ok(OpCode::ConstString),
            0x0503 => Ok(OpCode::ConstBool),
            
            0x0600 => Ok(OpCode::CreateArray),
            0x0601 => Ok(OpCode::CreateMap),
            0x0602 => Ok(OpCode::ArrayGet),
            0x0603 => Ok(OpCode::ArraySet),
            0x0604 => Ok(OpCode::MapGet),
            0x0605 => Ok(OpCode::MapSet),
            
            0x0700 => Ok(OpCode::DefineFunc),
            0x0701 => Ok(OpCode::CreateClosure),
            
            0x0800 => Ok(OpCode::Cast),
            0x0801 => Ok(OpCode::TypeOf),
            
            0x0900 => Ok(OpCode::Print),
            0x0901 => Ok(OpCode::Read),
            
            0x0A00 => Ok(OpCode::UICreateElement),
            0x0A01 => Ok(OpCode::UISetAttribute),
            0x0A02 => Ok(OpCode::UIAppendChild),
            
            0x0B00 => Ok(OpCode::AsyncBegin),
            0x0B01 => Ok(OpCode::AsyncAwait),
            0x0B02 => Ok(OpCode::AsyncComplete),
            
            0x0F00 => Ok(OpCode::ExternalCall),
            
            _ => Err(()),
        }
    }
}