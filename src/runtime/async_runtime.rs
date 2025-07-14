use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use crate::runtime::{Value, RuntimeError, Result};

#[derive(Debug, Clone)]
pub struct AsyncHandle {
    pub id: u64,
    pub state: Arc<Mutex<AsyncState>>,
}

#[derive(Debug)]
pub struct AsyncState {
    pub status: AsyncStatus,
    pub result: Option<Value>,
    pub waker: Option<Waker>,
    pub error: Option<RuntimeError>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AsyncStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

pub struct AsyncRuntime {
    next_id: u64,
    tasks: HashMap<u64, AsyncHandle>,
    pending_futures: Vec<Pin<Box<dyn Future<Output = Result<Value>> + Send>>>,
}

impl AsyncRuntime {
    pub fn new() -> Self {
        AsyncRuntime {
            next_id: 1,
            tasks: HashMap::new(),
            pending_futures: Vec::new(),
        }
    }
    
    pub fn begin_async(&mut self) -> AsyncHandle {
        let id = self.next_id;
        self.next_id += 1;
        
        let state = Arc::new(Mutex::new(AsyncState {
            status: AsyncStatus::Pending,
            result: None,
            waker: None,
            error: None,
        }));
        
        let handle = AsyncHandle { id, state };
        self.tasks.insert(id, handle.clone());
        
        handle
    }
    
    pub fn await_async(&self, handle: &AsyncHandle) -> AsyncAwaiter {
        AsyncAwaiter {
            handle: handle.clone(),
        }
    }
    
    pub fn complete_async(&mut self, handle: &AsyncHandle, result: Value) -> Result<()> {
        let mut state = handle.state.lock().unwrap();
        
        if state.status != AsyncStatus::Pending && state.status != AsyncStatus::Running {
            return Err(RuntimeError::InvalidOperation(
                "Cannot complete async operation that is already completed".to_string()
            ));
        }
        
        state.status = AsyncStatus::Completed;
        state.result = Some(result);
        
        // Wake any waiting tasks
        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
        
        Ok(())
    }
    
    pub fn fail_async(&mut self, handle: &AsyncHandle, error: RuntimeError) -> Result<()> {
        let mut state = handle.state.lock().unwrap();
        
        if state.status != AsyncStatus::Pending && state.status != AsyncStatus::Running {
            return Err(RuntimeError::InvalidOperation(
                "Cannot fail async operation that is already completed".to_string()
            ));
        }
        
        state.status = AsyncStatus::Failed;
        state.error = Some(error);
        
        // Wake any waiting tasks
        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
        
        Ok(())
    }
    
    pub fn get_status(&self, handle: &AsyncHandle) -> AsyncStatus {
        let state = handle.state.lock().unwrap();
        state.status.clone()
    }
    
    pub fn get_result(&self, handle: &AsyncHandle) -> Result<Option<Value>> {
        let state = handle.state.lock().unwrap();
        
        match state.status {
            AsyncStatus::Completed => Ok(state.result.clone()),
            AsyncStatus::Failed => {
                if let Some(ref error) = state.error {
                    Err(error.clone())
                } else {
                    Err(RuntimeError::InvalidOperation("Async operation failed".to_string()))
                }
            }
            _ => Ok(None),
        }
    }
    
    pub fn cleanup_completed(&mut self) {
        let completed_ids: Vec<u64> = self.tasks.iter()
            .filter(|(_, handle)| {
                let state = handle.state.lock().unwrap();
                state.status == AsyncStatus::Completed || state.status == AsyncStatus::Failed
            })
            .map(|(id, _)| *id)
            .collect();
        
        for id in completed_ids {
            self.tasks.remove(&id);
        }
    }
}

pub struct AsyncAwaiter {
    handle: AsyncHandle,
}

impl Future for AsyncAwaiter {
    type Output = Result<Value>;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.handle.state.lock().unwrap();
        
        match state.status {
            AsyncStatus::Completed => {
                if let Some(result) = state.result.clone() {
                    Poll::Ready(Ok(result))
                } else {
                    Poll::Ready(Err(RuntimeError::InvalidOperation(
                        "Async operation completed without result".to_string()
                    )))
                }
            }
            AsyncStatus::Failed => {
                if let Some(error) = state.error.clone() {
                    Poll::Ready(Err(error))
                } else {
                    Poll::Ready(Err(RuntimeError::InvalidOperation(
                        "Async operation failed".to_string()
                    )))
                }
            }
            _ => {
                // Store waker for later notification
                state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

#[derive(Clone)]
pub struct AsyncPromise {
    pub handle: AsyncHandle,
    pub resolver: Arc<Mutex<Option<Box<dyn FnOnce(Value) + Send>>>>,
}

impl std::fmt::Debug for AsyncPromise {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncPromise")
            .field("handle", &self.handle)
            .field("resolver", &"<resolver>")
            .finish()
    }
}

impl AsyncPromise {
    pub fn new(handle: AsyncHandle) -> Self {
        AsyncPromise {
            handle,
            resolver: Arc::new(Mutex::new(None)),
        }
    }
    
    pub fn resolve(&self, value: Value) -> Result<()> {
        let mut state = self.handle.state.lock().unwrap();
        
        if state.status != AsyncStatus::Pending {
            return Err(RuntimeError::InvalidOperation(
                "Promise already resolved".to_string()
            ));
        }
        
        state.status = AsyncStatus::Completed;
        state.result = Some(value);
        
        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
        
        Ok(())
    }
    
    pub fn reject(&self, error: RuntimeError) -> Result<()> {
        let mut state = self.handle.state.lock().unwrap();
        
        if state.status != AsyncStatus::Pending {
            return Err(RuntimeError::InvalidOperation(
                "Promise already resolved".to_string()
            ));
        }
        
        state.status = AsyncStatus::Failed;
        state.error = Some(error);
        
        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_async_begin() {
        let mut runtime = AsyncRuntime::new();
        let handle = runtime.begin_async();
        
        assert_eq!(handle.id, 1);
        assert_eq!(runtime.get_status(&handle), AsyncStatus::Pending);
    }
    
    #[test]
    fn test_async_complete() {
        let mut runtime = AsyncRuntime::new();
        let handle = runtime.begin_async();
        
        runtime.complete_async(&handle, Value::Int(42)).unwrap();
        
        assert_eq!(runtime.get_status(&handle), AsyncStatus::Completed);
        assert_eq!(runtime.get_result(&handle).unwrap(), Some(Value::Int(42)));
    }
    
    #[test]
    fn test_async_fail() {
        let mut runtime = AsyncRuntime::new();
        let handle = runtime.begin_async();
        
        let error = RuntimeError::InvalidOperation("Test error".to_string());
        runtime.fail_async(&handle, error).unwrap();
        
        assert_eq!(runtime.get_status(&handle), AsyncStatus::Failed);
        assert!(runtime.get_result(&handle).is_err());
    }
    
    #[test]
    fn test_async_promise() {
        let mut runtime = AsyncRuntime::new();
        let handle = runtime.begin_async();
        let promise = AsyncPromise::new(handle.clone());
        
        promise.resolve(Value::String("Success".to_string())).unwrap();
        
        assert_eq!(runtime.get_status(&handle), AsyncStatus::Completed);
        assert_eq!(
            runtime.get_result(&handle).unwrap(),
            Some(Value::String("Success".to_string()))
        );
    }
    
    #[test]
    fn test_async_cleanup() {
        let mut runtime = AsyncRuntime::new();
        
        let handle1 = runtime.begin_async();
        let handle2 = runtime.begin_async();
        let handle3 = runtime.begin_async();
        
        runtime.complete_async(&handle1, Value::Nil).unwrap();
        runtime.fail_async(&handle2, RuntimeError::InvalidOperation("Test".to_string())).unwrap();
        
        assert_eq!(runtime.tasks.len(), 3);
        
        runtime.cleanup_completed();
        
        assert_eq!(runtime.tasks.len(), 1);
        assert!(runtime.tasks.contains_key(&handle3.id));
    }
}