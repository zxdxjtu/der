use std::collections::HashMap;
use crate::runtime::{Value, RuntimeError, Result};

#[derive(Debug)]
pub struct MemoryManager {
    heap: HashMap<u64, HeapObject>,
    next_address: u64,
    total_allocated: usize,
    allocation_limit: usize,
}

#[derive(Debug, Clone)]
pub struct HeapObject {
    pub address: u64,
    pub size: usize,
    pub value: Value,
    pub ref_count: usize,
    pub is_freed: bool,
}

#[derive(Debug, Clone)]
pub struct MemoryReference {
    pub address: u64,
    pub offset: usize,
}

impl MemoryManager {
    pub fn new() -> Self {
        MemoryManager {
            heap: HashMap::new(),
            next_address: 0x1000, // Start at a non-zero address
            total_allocated: 0,
            allocation_limit: 1024 * 1024 * 1024, // 1GB limit
        }
    }
    
    pub fn allocate(&mut self, size: usize, initial_value: Value) -> Result<u64> {
        if self.total_allocated + size > self.allocation_limit {
            return Err(RuntimeError::InvalidOperation(
                "Memory allocation limit exceeded".to_string()
            ));
        }
        
        let address = self.next_address;
        self.next_address += size as u64;
        self.total_allocated += size;
        
        let heap_obj = HeapObject {
            address,
            size,
            value: initial_value,
            ref_count: 1,
            is_freed: false,
        };
        
        self.heap.insert(address, heap_obj);
        Ok(address)
    }
    
    pub fn load(&self, address: u64) -> Result<Value> {
        let obj = self.heap.get(&address)
            .ok_or_else(|| RuntimeError::InvalidOperation(
                format!("Invalid memory address: 0x{:x}", address)
            ))?;
        
        if obj.is_freed {
            return Err(RuntimeError::InvalidOperation(
                format!("Accessing freed memory at 0x{:x}", address)
            ));
        }
        
        Ok(obj.value.clone())
    }
    
    pub fn store(&mut self, address: u64, value: Value) -> Result<()> {
        let obj = self.heap.get_mut(&address)
            .ok_or_else(|| RuntimeError::InvalidOperation(
                format!("Invalid memory address: 0x{:x}", address)
            ))?;
        
        if obj.is_freed {
            return Err(RuntimeError::InvalidOperation(
                format!("Writing to freed memory at 0x{:x}", address)
            ));
        }
        
        obj.value = value;
        Ok(())
    }
    
    pub fn free(&mut self, address: u64) -> Result<()> {
        let obj = self.heap.get_mut(&address)
            .ok_or_else(|| RuntimeError::InvalidOperation(
                format!("Invalid memory address: 0x{:x}", address)
            ))?;
        
        if obj.is_freed {
            return Err(RuntimeError::InvalidOperation(
                format!("Double free at 0x{:x}", address)
            ));
        }
        
        obj.is_freed = true;
        self.total_allocated -= obj.size;
        Ok(())
    }
    
    pub fn add_ref(&mut self, address: u64) -> Result<()> {
        let obj = self.heap.get_mut(&address)
            .ok_or_else(|| RuntimeError::InvalidOperation(
                format!("Invalid memory address: 0x{:x}", address)
            ))?;
        
        if obj.is_freed {
            return Err(RuntimeError::InvalidOperation(
                format!("Adding reference to freed memory at 0x{:x}", address)
            ));
        }
        
        obj.ref_count += 1;
        Ok(())
    }
    
    pub fn release_ref(&mut self, address: u64) -> Result<()> {
        let obj = self.heap.get_mut(&address)
            .ok_or_else(|| RuntimeError::InvalidOperation(
                format!("Invalid memory address: 0x{:x}", address)
            ))?;
        
        if obj.ref_count == 0 {
            return Err(RuntimeError::InvalidOperation(
                format!("Reference count underflow at 0x{:x}", address)
            ));
        }
        
        obj.ref_count -= 1;
        
        // Auto-free when ref count reaches 0
        if obj.ref_count == 0 && !obj.is_freed {
            self.free(address)?;
        }
        
        Ok(())
    }
    
    pub fn get_stats(&self) -> MemoryStats {
        let mut active_objects = 0;
        let mut freed_objects = 0;
        let mut total_refs = 0;
        
        for obj in self.heap.values() {
            if obj.is_freed {
                freed_objects += 1;
            } else {
                active_objects += 1;
                total_refs += obj.ref_count;
            }
        }
        
        MemoryStats {
            total_allocated: self.total_allocated,
            active_objects,
            freed_objects,
            total_refs,
            heap_size: self.heap.len(),
        }
    }
    
    pub fn collect_garbage(&mut self) -> usize {
        let addresses_to_remove: Vec<u64> = self.heap.iter()
            .filter(|(_, obj)| obj.is_freed)
            .map(|(addr, _)| *addr)
            .collect();
        
        let count = addresses_to_remove.len();
        for addr in addresses_to_remove {
            self.heap.remove(&addr);
        }
        
        count
    }
}

#[derive(Debug)]
pub struct MemoryStats {
    pub total_allocated: usize,
    pub active_objects: usize,
    pub freed_objects: usize,
    pub total_refs: usize,
    pub heap_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_allocate_and_load() {
        let mut mem = MemoryManager::new();
        
        let addr = mem.allocate(8, Value::Int(42)).unwrap();
        let value = mem.load(addr).unwrap();
        
        assert_eq!(value, Value::Int(42));
    }
    
    #[test]
    fn test_store() {
        let mut mem = MemoryManager::new();
        
        let addr = mem.allocate(8, Value::Int(42)).unwrap();
        mem.store(addr, Value::Int(100)).unwrap();
        let value = mem.load(addr).unwrap();
        
        assert_eq!(value, Value::Int(100));
    }
    
    #[test]
    fn test_free() {
        let mut mem = MemoryManager::new();
        
        let addr = mem.allocate(8, Value::Int(42)).unwrap();
        mem.free(addr).unwrap();
        
        // Should error when accessing freed memory
        assert!(mem.load(addr).is_err());
        assert!(mem.store(addr, Value::Int(100)).is_err());
    }
    
    #[test]
    fn test_double_free() {
        let mut mem = MemoryManager::new();
        
        let addr = mem.allocate(8, Value::Int(42)).unwrap();
        mem.free(addr).unwrap();
        
        // Second free should error
        assert!(mem.free(addr).is_err());
    }
    
    #[test]
    fn test_reference_counting() {
        let mut mem = MemoryManager::new();
        
        let addr = mem.allocate(8, Value::Int(42)).unwrap();
        mem.add_ref(addr).unwrap();
        mem.add_ref(addr).unwrap();
        
        // Should have ref count of 3
        assert_eq!(mem.heap.get(&addr).unwrap().ref_count, 3);
        
        // Release refs
        mem.release_ref(addr).unwrap();
        mem.release_ref(addr).unwrap();
        
        // Still accessible with 1 ref
        assert!(mem.load(addr).is_ok());
        
        // Final release should auto-free
        mem.release_ref(addr).unwrap();
        assert!(mem.load(addr).is_err());
    }
    
    #[test]
    fn test_garbage_collection() {
        let mut mem = MemoryManager::new();
        
        let addr1 = mem.allocate(8, Value::Int(1)).unwrap();
        let addr2 = mem.allocate(8, Value::Int(2)).unwrap();
        let addr3 = mem.allocate(8, Value::Int(3)).unwrap();
        
        mem.free(addr1).unwrap();
        mem.free(addr3).unwrap();
        
        assert_eq!(mem.heap.len(), 3);
        let collected = mem.collect_garbage();
        assert_eq!(collected, 2);
        assert_eq!(mem.heap.len(), 1);
        
        // addr2 should still be accessible
        assert!(mem.load(addr2).is_ok());
    }
    
    #[test]
    fn test_memory_limit() {
        let mut mem = MemoryManager::new();
        mem.allocation_limit = 100;
        
        // Should succeed
        mem.allocate(50, Value::Nil).unwrap();
        mem.allocate(40, Value::Nil).unwrap();
        
        // Should fail - would exceed limit
        assert!(mem.allocate(20, Value::Nil).is_err());
    }
}