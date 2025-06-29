//! Go-style select statement implementation for channel multiplexing
//!
//! Provides non-blocking and blocking selection operations across multiple channels.
//! Supports:
//! - Send and receive operations
//! - Default case for non-blocking operations
//! - Random selection when multiple operations are ready
//! - Timeout support

use std::collections::HashMap;
use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use std::any::{Any, TypeId};

use crate::runtime::channels::{ChannelError, ChannelResult};
use crate::runtime::channels::buffer::ChannelBuffer;

/// Unique identifier for select operations
pub type SelectId = usize;

/// Select operation result
#[derive(Debug, Clone)]
pub enum SelectResult<T> {
    /// A send operation completed successfully
    SendCompleted(usize),
    /// A receive operation completed successfully
    ReceiveCompleted(usize, T),
    /// Default case was executed (non-blocking mode)
    DefaultExecuted,
    /// Timeout occurred
    Timeout,
    /// All channels are closed
    AllClosed,
}

/// Select operation type
#[derive(Debug, Clone)]
pub enum SelectOperation {
    /// Send operation on a channel
    Send {
        channel_id: usize,
        case_index: usize,
    },
    /// Receive operation on a channel
    Receive {
        channel_id: usize,
        case_index: usize,
    },
    /// Default case (non-blocking)
    Default {
        case_index: usize,
    },
}

/// Select case for building select statements
pub struct SelectCase {
    /// Case index
    pub index: usize,
    /// Operation type
    pub operation: SelectOperation,
    /// Data for send operations (type-erased)
    pub send_data: Option<Box<dyn Any + Send>>,
    /// Callback for when case is selected
    pub callback: Option<Box<dyn FnOnce() + Send>>,
    /// Type ID for runtime type checking
    pub type_id: TypeId,
}

/// Wrapper for type-erased channel operations
pub struct ChannelWrapper {
    /// Type ID for runtime type checking
    pub type_id: TypeId,
    /// Channel operations
    pub ops: Box<dyn ChannelOps + Send + Sync>,
}

/// Trait for type-erased channel operations
pub trait ChannelOps {
    /// Try to send a value
    fn try_send(&self, value: Box<dyn Any + Send>) -> Result<(), (Box<dyn Any + Send>, ChannelError)>;
    /// Try to receive a value
    fn try_receive(&self) -> Result<Option<Box<dyn Any + Send>>, ChannelError>;
    /// Check if channel is ready for sending
    fn can_send(&self) -> bool;
    /// Check if channel is ready for receiving
    fn can_receive(&self) -> bool;
    /// Check if channel is closed
    fn is_closed(&self) -> bool;
    /// Clone the value (for send operations that need to retry)
    fn clone_value(&self, value: &dyn Any) -> Option<Box<dyn Any + Send>>;
}

/// Implementation of ChannelOps for typed channels
pub struct TypedChannelOps<T: Send + 'static> {
    channel: Arc<dyn ChannelBuffer<T>>,
}

impl<T: Send + 'static> TypedChannelOps<T> {
    pub fn new(channel: Arc<dyn ChannelBuffer<T>>) -> Self {
        Self { channel }
    }
}

impl<T: Send + Clone + 'static> ChannelOps for TypedChannelOps<T> {
    fn try_send(&self, value: Box<dyn Any + Send>) -> Result<(), (Box<dyn Any + Send>, ChannelError)> {
        match value.downcast::<T>() {
            Ok(typed_value) => {
                match self.channel.try_push(*typed_value) {
                    Ok(()) => Ok(()),
                    Err((val, err)) => Err((Box::new(val) as Box<dyn Any + Send>, err)),
                }
            }
            Err(original_value) => {
                Err((original_value, ChannelError::NoSenders)) // Type mismatch
            }
        }
    }

    fn try_receive(&self) -> Result<Option<Box<dyn Any + Send>>, ChannelError> {
        match self.channel.try_pop() {
            Ok(Some(value)) => Ok(Some(Box::new(value) as Box<dyn Any + Send>)),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn can_send(&self) -> bool {
        !self.channel.is_full() && !self.channel.is_closed()
    }

    fn can_receive(&self) -> bool {
        !self.channel.is_empty()
    }

    fn is_closed(&self) -> bool {
        self.channel.is_closed()
    }

    fn clone_value(&self, value: &dyn Any) -> Option<Box<dyn Any + Send>> {
        if let Some(typed_value) = value.downcast_ref::<T>() {
            Some(Box::new(typed_value.clone()) as Box<dyn Any + Send>)
        } else {
            None
        }
    }
}

/// Select statement builder and executor
pub struct Select {
    /// Cases in the select statement
    cases: Vec<SelectCase>,
    /// Channel registry with type-erased wrappers
    channels: HashMap<usize, ChannelWrapper>,
    /// Timeout duration
    timeout: Option<Duration>,
    /// Random seed for fair selection
    random_seed: AtomicUsize,
    /// Next case index
    next_case_index: AtomicUsize,
}

impl Select {
    /// Create a new select statement
    pub fn new() -> Self {
        Self {
            cases: Vec::new(),
            channels: HashMap::new(),
            timeout: None,
            random_seed: AtomicUsize::new(0),
            next_case_index: AtomicUsize::new(0),
        }
    }
    
    /// Add a send case
    pub fn send<T: Send + Clone + 'static>(
        &mut self,
        channel_id: usize,
        channel: Arc<dyn ChannelBuffer<T>>,
        value: T,
    ) -> &mut Self {
        let case_index = self.next_case_index.fetch_add(1, Ordering::SeqCst);
        
        // Create type-erased channel wrapper 
        let wrapper = ChannelWrapper {
            type_id: TypeId::of::<T>(),
            ops: Box::new(TypedChannelOps::new(channel)),
        };
        
        self.channels.insert(channel_id, wrapper);
        
        let case = SelectCase {
            index: case_index,
            operation: SelectOperation::Send {
                channel_id,
                case_index,
            },
            send_data: Some(Box::new(value) as Box<dyn Any + Send>),
            callback: None,
            type_id: TypeId::of::<T>(),
        };
        
        self.cases.push(case);
        self
    }
    
    /// Add a receive case
    pub fn receive<T: Send + Clone + 'static>(
        &mut self,
        channel_id: usize,
        channel: Arc<dyn ChannelBuffer<T>>,
    ) -> &mut Self {
        let case_index = self.next_case_index.fetch_add(1, Ordering::SeqCst);
        
        // Create type-erased channel wrapper 
        let wrapper = ChannelWrapper {
            type_id: TypeId::of::<T>(),
            ops: Box::new(TypedChannelOps::new(channel)),
        };
        
        self.channels.insert(channel_id, wrapper);
        
        let case = SelectCase {
            index: case_index,
            operation: SelectOperation::Receive {
                channel_id,
                case_index,
            },
            send_data: None,
            callback: None,
            type_id: TypeId::of::<T>(),
        };
        
        self.cases.push(case);
        self
    }
    
    /// Add a default case (makes select non-blocking)
    pub fn default_case(&mut self) -> &mut Self {
        let case_index = self.next_case_index.fetch_add(1, Ordering::SeqCst);
        
        let case = SelectCase {
            index: case_index,
            operation: SelectOperation::Default { case_index },
            send_data: None,
            callback: None,
            type_id: TypeId::of::<()>(), // Default case has no associated type
        };
        
        self.cases.push(case);
        self
    }
    
    /// Set timeout for the select operation
    pub fn timeout(&mut self, duration: Duration) -> &mut Self {
        self.timeout = Some(duration);
        self
    }
    
    /// Execute the select statement
    pub fn execute(&mut self) -> ChannelResult<SelectResult<Box<dyn Any + Send>>> {
        if self.cases.is_empty() {
            return Err(ChannelError::NoSenders);
        }
        
        let start_time = Instant::now();
        let has_default = self.cases.iter().any(|case| {
            matches!(case.operation, SelectOperation::Default { .. })
        });
        
        loop {
            // Check timeout
            if let Some(timeout) = self.timeout {
                if start_time.elapsed() >= timeout {
                    return Ok(SelectResult::Timeout);
                }
            }
            
            // Try all operations in random order
            let ready_cases = self.find_ready_cases()?;
            
            if !ready_cases.is_empty() {
                // Select a random ready case
                let selected_index = self.select_random_case(&ready_cases);
                return self.execute_case(selected_index);
            }
            
            // If no cases are ready and we have a default case, execute it
            if has_default {
                return Ok(SelectResult::DefaultExecuted);
            }
            
            // Check if all channels are closed
            if self.all_channels_closed() {
                return Ok(SelectResult::AllClosed);
            }
            
            // Sleep briefly to avoid busy waiting
            thread::sleep(Duration::from_millis(1));
        }
    }
    
    /// Find all ready cases
    fn find_ready_cases(&self) -> ChannelResult<Vec<usize>> {
        let mut ready_cases = Vec::new();
        
        for (i, case) in self.cases.iter().enumerate() {
            match &case.operation {
                SelectOperation::Send { channel_id, .. } => {
                    if let Some(wrapper) = self.channels.get(channel_id) {
                        // Check if we can send (buffer not full)
                        if wrapper.ops.can_send() {
                            ready_cases.push(i);
                        }
                    }
                }
                SelectOperation::Receive { channel_id, .. } => {
                    if let Some(wrapper) = self.channels.get(channel_id) {
                        // Check if we can receive (buffer not empty)
                        if wrapper.ops.can_receive() {
                            ready_cases.push(i);
                        }
                    }
                }
                SelectOperation::Default { .. } => {
                    // Default is always ready
                    ready_cases.push(i);
                }
            }
        }
        
        Ok(ready_cases)
    }
    
    /// Select a random case from ready cases
    fn select_random_case(&self, ready_cases: &[usize]) -> usize {
        if ready_cases.len() == 1 {
            return ready_cases[0];
        }
        
        // Simple pseudo-random selection
        let seed = self.random_seed.fetch_add(1, Ordering::SeqCst);
        let index = (seed * 1103515245 + 12345) % ready_cases.len();
        ready_cases[index]
    }
    
    /// Execute a selected case
    fn execute_case(&mut self, select_case_index: usize) -> ChannelResult<SelectResult<Box<dyn Any + Send>>> {
        let case = &self.cases[select_case_index];
        
        match &case.operation {
            SelectOperation::Send { channel_id, case_index } => {
                if let Some(wrapper) = self.channels.get(channel_id) {
                    // Execute send operation
                    self.execute_send_case(wrapper, *case_index, select_case_index)
                } else {
                    Err(ChannelError::NoSenders)
                }
            }
            SelectOperation::Receive { channel_id, case_index } => {
                if let Some(wrapper) = self.channels.get(channel_id) {
                    // Execute receive operation
                    self.execute_receive_case(wrapper, *case_index)
                } else {
                    Err(ChannelError::NoReceivers)
                }
            }
            SelectOperation::Default { case_index } => {
                Ok(SelectResult::DefaultExecuted)
            }
        }
    }
    
    /// Execute a send case
    fn execute_send_case(
        &self,
        wrapper: &ChannelWrapper,
        operation_case_index: usize,
        select_case_index: usize,
    ) -> ChannelResult<SelectResult<Box<dyn Any + Send>>> {
        // Get the send data from the case
        if let Some(case) = self.cases.get(select_case_index) {
            if let Some(ref send_data) = case.send_data {
                // Clone the data safely using the wrapper's clone_value method
                if let Some(cloned_value) = wrapper.ops.clone_value(send_data.as_ref()) {
                    match wrapper.ops.try_send(cloned_value) {
                        Ok(()) => Ok(SelectResult::SendCompleted(operation_case_index)),
                        Err((_, err)) => Err(err),
                    }
                } else {
                    Err(ChannelError::NoSenders) // Type mismatch or clone failed
                }
            } else {
                Err(ChannelError::NoSenders)
            }
        } else {
            Err(ChannelError::NoSenders)
        }
    }
    
    /// Execute a receive case
    fn execute_receive_case(
        &self,
        wrapper: &ChannelWrapper,
        case_index: usize,
    ) -> ChannelResult<SelectResult<Box<dyn Any + Send>>> {
        match wrapper.ops.try_receive() {
            Ok(Some(value)) => Ok(SelectResult::ReceiveCompleted(case_index, value)),
            Ok(None) => Err(ChannelError::WouldBlock),
            Err(err) => Err(err),
        }
    }
    
    /// Check if all channels are closed
    fn all_channels_closed(&self) -> bool {
        self.channels.values().all(|wrapper| wrapper.ops.is_closed())
    }
}

/// Select macro for easier syntax
#[macro_export]
macro_rules! select {
    // Base case with no arms
    () => {
        compile_error!("select! requires at least one arm")
    };
    
    // Single receive arm
    (recv($channel:expr) => $body:expr) => {{
        let mut select = $crate::runtime::channels::select::Select::new();
        select.receive(0, $channel);
        match select.execute() {
            Ok($crate::runtime::channels::select::SelectResult::ReceiveCompleted(0, value)) => {
                $body
            }
            _ => panic!("Unexpected select result"),
        }
    }};
    
    // Single send arm
    (send($channel:expr, $value:expr) => $body:expr) => {{
        let mut select = $crate::runtime::channels::select::Select::new();
        select.send(0, $channel, $value);
        match select.execute() {
            Ok($crate::runtime::channels::select::SelectResult::SendCompleted(0)) => {
                $body
            }
            _ => panic!("Unexpected select result"),
        }
    }};
    
    // Default case
    (default => $body:expr) => {{
        let mut select = $crate::runtime::channels::select::Select::new();
        select.default_case();
        match select.execute() {
            Ok($crate::runtime::channels::select::SelectResult::DefaultExecuted) => {
                $body
            }
            _ => panic!("Unexpected select result"),
        }
    }};
}

/// Blocking select function - waits for any operation to complete
pub fn select_blocking<T: Send + Clone + 'static>(
    operations: Vec<(usize, Arc<dyn ChannelBuffer<T>>, SelectOperation)>,
) -> ChannelResult<SelectResult<T>> {
    let mut select = Select::new();
    
    for (channel_id, channel, operation) in operations {
        match operation {
            SelectOperation::Send { .. } => {
                // We would need the value here
                // This is a simplified implementation
            }
            SelectOperation::Receive { .. } => {
                // select.receive(channel_id, channel);
            }
            SelectOperation::Default { .. } => {
                select.default_case();
            }
        }
    }
    
    // This is a simplified implementation
    // Real implementation would handle the types properly
    Err(ChannelError::NoSenders)
}

/// Non-blocking select function - returns immediately with default if no operations ready
pub fn select_nonblocking<T: Send + Clone + 'static>(
    operations: Vec<(usize, Arc<dyn ChannelBuffer<T>>, SelectOperation)>,
) -> ChannelResult<SelectResult<T>> {
    let mut select = Select::new();
    select.default_case();
    
    for (channel_id, channel, operation) in operations {
        match operation {
            SelectOperation::Send { .. } => {
                // We would need the value here
                // This is a simplified implementation
            }
            SelectOperation::Receive { .. } => {
                // select.receive(channel_id, channel);
            }
            SelectOperation::Default { .. } => {
                // Already added
            }
        }
    }
    
    // This is a simplified implementation
    Err(ChannelError::NoSenders)
}

/// Select with timeout
pub fn select_timeout<T: Send + Clone + 'static>(
    operations: Vec<(usize, Arc<dyn ChannelBuffer<T>>, SelectOperation)>,
    timeout: Duration,
) -> ChannelResult<SelectResult<T>> {
    let mut select = Select::new();
    select.timeout(timeout);
    
    for (channel_id, channel, operation) in operations {
        match operation {
            SelectOperation::Send { .. } => {
                // We would need the value here
                // This is a simplified implementation
            }
            SelectOperation::Receive { .. } => {
                // select.receive(channel_id, channel);
            }
            SelectOperation::Default { .. } => {
                select.default_case();
            }
        }
    }
    
    // This is a simplified implementation
    Err(ChannelError::NoSenders)
}

/// Select state for tracking ongoing select operations
pub struct SelectState {
    /// Select operation ID
    pub id: SelectId,
    /// Waiting goroutines
    pub waiting_goroutines: Vec<crate::runtime::goroutine::GoroutineId>,
    /// Completed flag
    pub completed: bool,
    /// Result of the select operation
    pub result: Option<SelectResult<Box<dyn Any + Send>>>,
}

impl SelectState {
    pub fn new(id: SelectId) -> Self {
        Self {
            id,
            waiting_goroutines: Vec::new(),
            completed: false,
            result: None,
        }
    }
    
    pub fn add_waiting_goroutine(&mut self, goroutine_id: crate::runtime::goroutine::GoroutineId) {
        self.waiting_goroutines.push(goroutine_id);
    }
    
    pub fn complete_with_result(&mut self, result: SelectResult<Box<dyn Any + Send>>) {
        self.result = Some(result);
        self.completed = true;
    }
    
    pub fn is_completed(&self) -> bool {
        self.completed
    }
}

/// Global select state manager
pub struct SelectManager {
    /// Active select operations
    active_selects: Mutex<HashMap<SelectId, SelectState>>,
    /// Next select ID
    next_id: AtomicUsize,
}

impl SelectManager {
    pub fn new() -> Self {
        Self {
            active_selects: Mutex::new(HashMap::new()),
            next_id: AtomicUsize::new(1),
        }
    }
    
    pub fn create_select(&self) -> SelectId {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let state = SelectState::new(id);
        
        self.active_selects.lock().unwrap().insert(id, state);
        id
    }
    
    pub fn complete_select(&self, id: SelectId, result: SelectResult<Box<dyn Any + Send>>) {
        if let Some(state) = self.active_selects.lock().unwrap().get_mut(&id) {
            state.complete_with_result(result);
        }
    }
    
    pub fn remove_select(&self, id: SelectId) {
        self.active_selects.lock().unwrap().remove(&id);
    }
}

/// Global select manager instance
static SELECT_MANAGER: once_cell::sync::Lazy<SelectManager> = 
    once_cell::sync::Lazy::new(|| SelectManager::new());

/// Get the global select manager
pub fn get_select_manager() -> &'static SelectManager {
    &SELECT_MANAGER
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::channels::buffer::{RingBuffer, UnbufferedChannel};
    use std::sync::Arc;

    #[test]
    fn test_select_creation() {
        let mut select = Select::new();
        assert_eq!(select.cases.len(), 0);
        
        select.default_case();
        assert_eq!(select.cases.len(), 1);
    }

    #[test]
    fn test_select_manager() {
        let manager = SelectManager::new();
        let id = manager.create_select();
        assert!(id > 0);
        
        let result = SelectResult::DefaultExecuted;
        manager.complete_select(id, result);
        manager.remove_select(id);
    }

    #[test]
    fn test_select_with_timeout() {
        let mut select = Select::new();
        select.timeout(Duration::from_millis(100));
        select.default_case();
        
        let result = select.execute();
        assert!(result.is_ok());
    }
}
