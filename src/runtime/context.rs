use std::collections::HashMap;
use crate::core::{Program, Capability};
use crate::runtime::{Value, RuntimeError, Result, MemoryManager, AsyncRuntime};

pub struct ExecutionContext {
    pub program: Program,
    pub values: HashMap<u32, Value>,
    pub call_stack: Vec<CallFrame>,
    pub granted_capabilities: Vec<Capability>,
    pub max_call_depth: usize,
    pub memory: MemoryManager,
    pub async_runtime: AsyncRuntime,
}

pub struct CallFrame {
    pub node_id: u32,
    pub locals: HashMap<u32, Value>,
    pub return_to: Option<u32>,
}

impl ExecutionContext {
    pub fn new(program: Program) -> Self {
        ExecutionContext {
            program,
            values: HashMap::new(),
            call_stack: Vec::new(),
            granted_capabilities: Vec::new(),
            max_call_depth: 1000,
            memory: MemoryManager::new(),
            async_runtime: AsyncRuntime::new(),
        }
    }

    pub fn grant_capability(&mut self, cap: Capability) {
        if !self.granted_capabilities.contains(&cap) {
            self.granted_capabilities.push(cap);
        }
    }

    pub fn check_capability(&self, cap: &Capability) -> Result<()> {
        if self.granted_capabilities.contains(cap) {
            Ok(())
        } else {
            Err(RuntimeError::MissingCapability(cap.clone()))
        }
    }

    pub fn push_frame(&mut self, node_id: u32, return_to: Option<u32>) -> Result<()> {
        if self.call_stack.len() >= self.max_call_depth {
            return Err(RuntimeError::StackOverflow);
        }

        self.call_stack.push(CallFrame {
            node_id,
            locals: HashMap::new(),
            return_to,
        });

        Ok(())
    }

    pub fn pop_frame(&mut self) -> Option<CallFrame> {
        self.call_stack.pop()
    }

    pub fn current_frame(&self) -> Option<&CallFrame> {
        self.call_stack.last()
    }

    pub fn current_frame_mut(&mut self) -> Option<&mut CallFrame> {
        self.call_stack.last_mut()
    }

    pub fn set_value(&mut self, result_id: u32, value: Value) {
        if let Some(frame) = self.current_frame_mut() {
            frame.locals.insert(result_id, value.clone());
        }
        self.values.insert(result_id, value);
    }

    pub fn get_value(&self, result_id: u32) -> Option<&Value> {
        // First check current frame locals
        if let Some(frame) = self.current_frame() {
            if let Some(value) = frame.locals.get(&result_id) {
                return Some(value);
            }
        }
        // Then check global values
        self.values.get(&result_id)
    }

    pub fn get_node(&self, result_id: u32) -> Option<&crate::core::Node> {
        self.program.nodes.iter().find(|n| n.result_id == result_id)
    }
}