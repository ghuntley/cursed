/// Channel selection and multiplexing for CURSED channels
/// Provides Go-like select statement functionality for non-blocking operations on multiple channels

use std::sync::{Arc, Mutex, Condvar, atomic::{AtomicBool, Ordering}};
use std::time::{Duration, Instant};
use std::collections::VecDeque;
use std::fmt;
use tracing::{debug, info, warn, error, instrument, field};
use rand::{Rng, SeedableRng};

use crate::runtime::channels::{ChannelError, ChannelResult, SendResult, ReceiveResult};
use crate::error::CursedError;

/// Unique identifier for select operations
pub type SelectId = u64;

/// Individual case in a select operation
#[derive(Debug, Clone)]
pub struct SelectCase<T> {
    /// Unique identifier for this case
    pub case_id: u64,
    /// Channel identifier this case operates on
    pub channel_id: u64,
    /// Operation to perform
    pub operation: SelectOperation<T>,
    /// Priority for this case (higher = more priority)
    pub priority: i32,
}

/// Types of operations in select cases
#[derive(Debug, Clone)]
pub enum SelectOperation<T> {
    /// Send operation with value
    Send(T),
    /// Receive operation
    Receive,
    /// Default case (fallback when no other cases ready)
    Default,
}

/// Result of a select operation
#[derive(Debug, Clone)]
pub struct SelectResult<T> {
    /// ID of the case that succeeded
    pub case_id: u64,
    /// Channel ID that was operated on
    pub channel_id: u64,
    /// Result of the operation
    pub result: SelectResultValue<T>,
    /// Time when the operation completed
    pub completion_time: Instant,
}

/// Value result from select operation
#[derive(Debug, Clone)]
pub enum SelectResultValue<T> {
    /// Successfully sent value
    Sent,
    /// Successfully received value
    Received(T),
    /// Default case executed
    Default,
    /// Channel was closed during operation
    Closed,
    /// Operation would have blocked
    WouldBlock,
}

/// Builder for constructing select operations
#[derive(Debug)]
pub struct SelectBuilder<T> {
    cases: Vec<SelectCase<T>>,
    next_case_id: u64,
    randomize: bool,
    timeout: Option<Duration>,
}

impl<T> Default for SelectBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SelectBuilder<T> {
    /// Create a new select builder
    pub fn new() -> Self {
        Self {
            cases: Vec::new(),
            next_case_id: 1,
            randomize: true,
            timeout: None,
        }
    }

    /// Add a send case
    #[instrument(skip(self, value))]
    pub fn send(mut self, channel_id: u64, value: T) -> Self {
        let case = SelectCase {
            case_id: self.next_case_id,
            channel_id,
            operation: SelectOperation::Send(value),
            priority: 0,
        };
        self.cases.push(case);
        self.next_case_id += 1;
        debug!(case_id = self.next_case_id - 1, channel_id, "Added send case");
        self
    }

    /// Add a receive case
    #[instrument(skip(self))]
    pub fn receive(mut self, channel_id: u64) -> Self {
        let case = SelectCase {
            case_id: self.next_case_id,
            channel_id,
            operation: SelectOperation::Receive,
            priority: 0,
        };
        self.cases.push(case);
        self.next_case_id += 1;
        debug!(case_id = self.next_case_id - 1, channel_id, "Added receive case");
        self
    }

    /// Add a default case
    #[instrument(skip(self))]
    pub fn default(mut self) -> Self {
        let case = SelectCase {
            case_id: self.next_case_id,
            channel_id: 0, // Default case has no channel
            operation: SelectOperation::Default,
            priority: -1000, // Default case has lowest priority
        };
        self.cases.push(case);
        self.next_case_id += 1;
        debug!(case_id = self.next_case_id - 1, "Added default case");
        self
    }

    /// Set priority for the last added case
    pub fn priority(mut self, priority: i32) -> Self {
        if let Some(last_case) = self.cases.last_mut() {
            last_case.priority = priority;
            debug!(case_id = last_case.case_id, priority, "Set case priority");
        }
        self
    }

    /// Set timeout for select operation
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        debug!(?timeout, "Set select timeout");
        self
    }

    /// Disable randomization (use priority order)
    pub fn no_randomize(mut self) -> Self {
        self.randomize = false;
        debug!("Disabled case randomization");
        self
    }

    /// Build the select operation
    pub fn build(self) -> SelectOperation<T> {
        debug!(
            cases_count = self.cases.len(),
            randomize = self.randomize,
            ?self.timeout,
            "Built select operation"
        );
        SelectOperation::Default // Placeholder - will be replaced with actual select execution
    }

    /// Get the number of cases
    pub fn case_count(&self) -> usize {
        self.cases.len()
    }

    /// Check if timeout is set
    pub fn has_timeout(&self) -> bool {
        self.timeout.is_some()
    }

    /// Get the timeout duration
    pub fn get_timeout(&self) -> Option<Duration> {
        self.timeout
    }

    /// Get case priority by index
    pub fn get_case_priority(&self, index: usize) -> Option<i32> {
        self.cases.get(index).map(|case| case.priority)
    }
}

/// Handle for managing long-running select operations
#[derive(Debug)]
pub struct SelectHandle {
    /// Unique identifier for this select operation
    pub select_id: SelectId,
    /// Cancellation flag
    cancelled: Arc<AtomicBool>,
    /// Completion notification
    completion: Arc<(Mutex<bool>, Condvar)>,
}

impl SelectHandle {
    /// Create a new select handle
    pub fn new(select_id: SelectId) -> Self {
        Self {
            select_id,
            cancelled: Arc::new(AtomicBool::new(false)),
            completion: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }

    /// Cancel the select operation
    #[instrument(skip(self))]
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
        let (lock, cvar) = &*self.completion;
        let mut completed = lock.lock().unwrap();
        *completed = true;
        cvar.notify_all();
        info!(select_id = self.select_id, "Select operation cancelled");
    }

    /// Check if the operation is cancelled
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    /// Wait for completion or cancellation
    #[instrument(skip(self))]
    pub fn wait(&self) -> ChannelResult<()> {
        let (lock, cvar) = &*self.completion;
        let mut completed = lock.lock().unwrap();
        
        while !*completed && !self.is_cancelled() {
            completed = cvar.wait(completed).unwrap();
        }
        
        if self.is_cancelled() {
            debug!(select_id = self.select_id, "Select operation was cancelled");
            Err(ChannelError::Timeout) // Use timeout as cancellation error
        } else {
            debug!(select_id = self.select_id, "Select operation completed");
            Ok(())
        }
    }

    /// Signal completion
    pub(crate) fn complete(&self) {
        let (lock, cvar) = &*self.completion;
        let mut completed = lock.lock().unwrap();
        *completed = true;
        cvar.notify_all();
    }
}

/// Core select functionality
pub struct ChannelSelector<T> {
    /// Next select operation ID
    next_select_id: u64,
    /// Random seed for fair selection
    random_seed: u64,
    /// Active select operations
    active_selects: VecDeque<SelectHandle>,
    /// Phantom data to hold the type parameter
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Default for ChannelSelector<T> 
where 
    T: Clone + Send + Sync + fmt::Debug + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ChannelSelector<T> 
where 
    T: Clone + Send + Sync + fmt::Debug + 'static,
{
    /// Create a new channel selector
    pub fn new() -> Self {
        Self {
            next_select_id: 1,
            random_seed: rand::random(),
            active_selects: VecDeque::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Try all cases once and return immediately (non-blocking)
    #[instrument(skip(self, cases))]
    pub fn select_nonblocking(&mut self, mut cases: Vec<SelectCase<T>>) -> ChannelResult<SelectResult<T>> {
        let select_id = self.next_select_id;
        self.next_select_id += 1;

        debug!(
            select_id,
            cases_count = cases.len(),
            "Starting non-blocking select operation"
        );

        // Randomize case order for fairness if needed
        if cases.len() > 1 {
            self.randomize_cases(&mut cases);
        }

        // Separate default cases from regular cases
        let (default_cases, regular_cases): (Vec<_>, Vec<_>) = cases.iter()
            .partition(|c| matches!(c.operation, SelectOperation::Default));

        // Try regular cases first
        for case in &regular_cases {
            match self.try_case(case) {
                Ok(result) => {
                    info!(
                        select_id,
                        case_id = case.case_id,
                        channel_id = case.channel_id,
                        "Non-blocking select succeeded"
                    );
                    return Ok(result);
                }
                Err(ChannelError::WouldBlock) => {
                    debug!(
                        case_id = case.case_id,
                        channel_id = case.channel_id,
                        "Case would block, trying next"
                    );
                    continue;
                }
                Err(err) => {
                    debug!(
                        case_id = case.case_id,
                        channel_id = case.channel_id,
                        error = ?err,
                        "Case failed"
                    );
                    continue;
                }
            }
        }

        // If no regular case succeeded, try default case
        if let Some(default_case) = default_cases.first() {
            let result = SelectResult {
                case_id: default_case.case_id,
                channel_id: 0,
                result: SelectResultValue::Default,
                completion_time: Instant::now(),
            };
            info!(select_id, "Non-blocking select using default case");
            return Ok(result);
        }

        warn!(select_id, "Non-blocking select would block on all cases");
        Err(ChannelError::WouldBlock)
    }

    /// Block until one case can proceed
    #[instrument(skip(self, cases))]
    pub fn select_blocking(&mut self, mut cases: Vec<SelectCase<T>>) -> ChannelResult<SelectResult<T>> {
        let select_id = self.next_select_id;
        self.next_select_id += 1;

        let handle = SelectHandle::new(select_id);
        self.active_selects.push_back(handle);

        debug!(
            select_id,
            cases_count = cases.len(),
            "Starting blocking select operation"
        );

        // First try non-blocking
        if let Ok(result) = self.select_nonblocking(cases.clone()) {
            info!(select_id, "Blocking select succeeded immediately");
            return Ok(result);
        }

        // If no immediate success, enter blocking loop
        loop {
            // Check for cancellation
            if let Some(handle) = self.active_selects.back() {
                if handle.is_cancelled() {
                    warn!(select_id, "Blocking select was cancelled");
                    return Err(ChannelError::Timeout);
                }
            }

            // Randomize for fairness
            self.randomize_cases(&mut cases);

            // Try cases again
            for case in &cases {
                match self.try_case(case) {
                    Ok(result) => {
                        info!(
                            select_id,
                            case_id = case.case_id,
                            channel_id = case.channel_id,
                            "Blocking select succeeded"
                        );
                        if let Some(handle) = self.active_selects.back() {
                            handle.complete();
                        }
                        return Ok(result);
                    }
                    Err(ChannelError::WouldBlock) => continue,
                    Err(_) => continue,
                }
            }

            // Brief sleep to avoid busy waiting
            std::thread::sleep(Duration::from_micros(100));
        }
    }

    /// Block with timeout until one case can proceed
    #[instrument(skip(self, cases))]
    pub fn select_timeout(
        &mut self,
        mut cases: Vec<SelectCase<T>>,
        timeout: Duration,
    ) -> ChannelResult<SelectResult<T>> {
        let select_id = self.next_select_id;
        self.next_select_id += 1;

        let start_time = Instant::now();
        let handle = SelectHandle::new(select_id);
        self.active_selects.push_back(handle);

        debug!(
            select_id,
            cases_count = cases.len(),
            ?timeout,
            "Starting timeout select operation"
        );

        // First try non-blocking
        if let Ok(result) = self.select_nonblocking(cases.clone()) {
            info!(select_id, "Timeout select succeeded immediately");
            return Ok(result);
        }

        // Enter timeout loop
        while start_time.elapsed() < timeout {
            // Check for cancellation
            if let Some(handle) = self.active_selects.back() {
                if handle.is_cancelled() {
                    warn!(select_id, "Timeout select was cancelled");
                    return Err(ChannelError::Timeout);
                }
            }

            // Randomize for fairness
            self.randomize_cases(&mut cases);

            // Try cases
            for case in &cases {
                match self.try_case(case) {
                    Ok(result) => {
                        info!(
                            select_id,
                            case_id = case.case_id,
                            channel_id = case.channel_id,
                            elapsed = ?start_time.elapsed(),
                            "Timeout select succeeded"
                        );
                        if let Some(handle) = self.active_selects.back() {
                            handle.complete();
                        }
                        return Ok(result);
                    }
                    Err(ChannelError::WouldBlock) => continue,
                    Err(_) => continue,
                }
            }

            // Brief sleep to avoid busy waiting
            std::thread::sleep(Duration::from_micros(100));
        }

        warn!(select_id, ?timeout, "Timeout select operation timed out");
        if let Some(handle) = self.active_selects.back() {
            handle.complete();
        }
        Err(ChannelError::Timeout)
    }

    /// Try a single select case
    #[instrument(skip(self, case))]
    fn try_case(&self, case: &SelectCase<T>) -> ChannelResult<SelectResult<T>> {
        match &case.operation {
            SelectOperation::Send(_value) => {
                // Simulate channel send operation
                // In a real implementation, this would call the actual channel send
                debug!(
                    case_id = case.case_id,
                    channel_id = case.channel_id,
                    "Attempting send operation"
                );
                
                // Placeholder logic - would integrate with actual channel
                if self.simulate_channel_ready(case.channel_id) {
                    Ok(SelectResult {
                        case_id: case.case_id,
                        channel_id: case.channel_id,
                        result: SelectResultValue::Sent,
                        completion_time: Instant::now(),
                    })
                } else {
                    Err(ChannelError::WouldBlock)
                }
            }
            SelectOperation::Receive => {
                // Simulate channel receive operation
                debug!(
                    case_id = case.case_id,
                    channel_id = case.channel_id,
                    "Attempting receive operation"
                );
                
                // Placeholder logic - would integrate with actual channel
                if self.simulate_channel_ready(case.channel_id) {
                    // Simulate received value - would be actual channel data
                    Ok(SelectResult {
                        case_id: case.case_id,
                        channel_id: case.channel_id,
                        result: SelectResultValue::Default, // Placeholder
                        completion_time: Instant::now(),
                    })
                } else {
                    Err(ChannelError::WouldBlock)
                }
            }
            SelectOperation::Default => {
                debug!(case_id = case.case_id, "Executing default case");
                Ok(SelectResult {
                    case_id: case.case_id,
                    channel_id: 0,
                    result: SelectResultValue::Default,
                    completion_time: Instant::now(),
                })
            }
        }
    }

    /// Randomize case order for fairness
    fn randomize_cases(&mut self, cases: &mut [SelectCase<T>]) {
        // Sort by priority first, then randomize within same priority
        cases.sort_by_key(|case| std::cmp::Reverse(case.priority));
        
        // Create a local RNG from our seed
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.random_seed);
        
        // Fisher-Yates shuffle for each priority group
        let mut i = 0;
        while i < cases.len() {
            let current_priority = cases[i].priority;
            let mut j = i;
            
            // Find end of same priority group
            while j < cases.len() && cases[j].priority == current_priority {
                j += 1;
            }
            
            // Shuffle this priority group
            for k in i..j {
                let swap_idx = i + rng.gen_range(0..(j - i));
                cases.swap(k, swap_idx);
            }
            
            i = j;
        }
        
        // Update seed for next randomization
        self.random_seed = rand::random();
    }

    /// Simulate channel readiness (placeholder for actual channel integration)
    fn simulate_channel_ready(&self, channel_id: u64) -> bool {
        // Simple simulation - would be replaced with actual channel state checking
        (channel_id % 3) == 0 // Simulate 1/3 of channels being ready
    }

    /// Clean up completed select operations
    #[instrument(skip(self))]
    pub fn cleanup_completed(&mut self) {
        let initial_count = self.active_selects.len();
        self.active_selects.retain(|handle| !handle.is_cancelled());
        let cleaned_count = initial_count - self.active_selects.len();
        
        if cleaned_count > 0 {
            debug!(
                cleaned_count,
                remaining_count = self.active_selects.len(),
                "Cleaned up completed select operations"
            );
        }
    }

    /// Get statistics about active select operations
    pub fn get_stats(&self) -> SelectStats {
        SelectStats {
            active_selects: self.active_selects.len(),
            next_select_id: self.next_select_id,
        }
    }

    /// Add a handle for testing (internal use)
    pub fn add_handle_for_test(&mut self, handle: SelectHandle) {
        self.active_selects.push_back(handle);
    }

    /// Get active selects count for testing
    pub fn active_selects_count(&self) -> usize {
        self.active_selects.len()
    }
}

/// Statistics about select operations
#[derive(Debug, Clone)]
pub struct SelectStats {
    /// Number of active select operations
    pub active_selects: usize,
    /// Next select ID to be assigned
    pub next_select_id: u64,
}

/// FFI-friendly functions for LLVM integration
pub mod ffi {
    use super::*;
    use std::ffi::c_void;

    /// Create a new select builder (FFI-safe)
    #[no_mangle]
    pub extern "C" fn cursed_select_builder_new() -> *mut c_void {
        let builder = Box::new(SelectBuilder::<i64>::new());
        Box::into_raw(builder) as *mut c_void
    }

    /// Add send case to builder (FFI-safe)
    #[no_mangle]
    pub extern "C" fn cursed_select_builder_add_send(
        builder: *mut c_void,
        channel_id: u64,
        value: i64,
    ) -> *mut c_void {
        if builder.is_null() {
            return std::ptr::null_mut();
        }

        unsafe {
            let builder_box = Box::from_raw(builder as *mut SelectBuilder<i64>);
            let new_builder = builder_box.send(channel_id, value);
            Box::into_raw(Box::new(new_builder)) as *mut c_void
        }
    }

    /// Add receive case to builder (FFI-safe)
    #[no_mangle]
    pub extern "C" fn cursed_select_builder_add_receive(
        builder: *mut c_void,
        channel_id: u64,
    ) -> *mut c_void {
        if builder.is_null() {
            return std::ptr::null_mut();
        }

        unsafe {
            let builder_box = Box::from_raw(builder as *mut SelectBuilder<i64>);
            let new_builder = builder_box.receive(channel_id);
            Box::into_raw(Box::new(new_builder)) as *mut c_void
        }
    }

    /// Execute non-blocking select (FFI-safe)
    #[no_mangle]
    pub extern "C" fn cursed_select_nonblocking(
        selector: *mut c_void,
        cases: *const SelectCase<i64>,
        cases_len: usize,
    ) -> i32 {
        if selector.is_null() || cases.is_null() {
            return -1; // Error
        }

        unsafe {
            let selector = &mut *(selector as *mut ChannelSelector<i64>);
            let cases_slice = std::slice::from_raw_parts(cases, cases_len);
            let cases_vec = cases_slice.to_vec();

            match selector.select_nonblocking(cases_vec) {
                Ok(_result) => 0, // Success
                Err(_) => -1,     // Error
            }
        }
    }

    /// Cleanup FFI resources
    #[no_mangle]
    pub extern "C" fn cursed_select_builder_free(builder: *mut c_void) {
        if !builder.is_null() {
            unsafe {
                let _ = Box::from_raw(builder as *mut SelectBuilder<i64>);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_builder() {
        let builder = SelectBuilder::<i32>::new()
            .send(1, 42)
            .receive(2)
            .default()
            .timeout(Duration::from_millis(100));

        // Builder should have 3 cases
        assert_eq!(builder.cases.len(), 3);
        assert!(builder.timeout.is_some());
    }

    #[test]
    fn test_select_handle() {
        let handle = SelectHandle::new(1);
        assert!(!handle.is_cancelled());
        
        handle.cancel();
        assert!(handle.is_cancelled());
    }

    #[test]
    fn test_channel_selector() {
        let mut selector = ChannelSelector::<i32>::new();
        let stats = selector.get_stats();
        assert_eq!(stats.active_selects, 0);
        assert_eq!(stats.next_select_id, 1);
    }

    #[test]
    fn test_select_nonblocking_with_default() {
        let mut selector = ChannelSelector::<i32>::new();
        let cases = vec![
            SelectCase {
                case_id: 1,
                channel_id: 1,
                operation: SelectOperation::Send(42),
                priority: 0,
            },
            SelectCase {
                case_id: 2,
                channel_id: 0,
                operation: SelectOperation::Default,
                priority: -1000,
            },
        ];

        let result = selector.select_nonblocking(cases);
        assert!(result.is_ok());
    }
}
