use crate::error::CursedError;

// Placeholder macros for logging when log crate is not available
macro_rules! trace {
    ($($arg:tt)*) => {
        #[cfg(feature = "logging")]
        log::trace!($($arg)*);
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(feature = "logging")]
        log::debug!($($arg)*);
macro_rules! info {
    ($($arg:tt)*) => {
        #[cfg(feature = "logging")]
        log::info!($($arg)*);
macro_rules! warn {
    ($($arg:tt)*) => {
        #[cfg(feature = "logging")]
        log::warn!($($arg)*);
macro_rules! error {
    ($($arg:tt)*) => {
        #[cfg(feature = "logging")]
        log::error!($($arg)*);
}
/// Channel selection and multiplexing for CURSED channels
/// Provides Go-like select statement functionality for non-blocking operations on multiple channels

use std::sync::{Arc, Mutex, Condvar, atomic::{AtomicBool, Ordering}};
use std::time::{Duration, Instant};
use std::collections::VecDeque;
use std::fmt;
use tracing::{debug, info, warn, error, instrument, field};
use rand::{Rng, SeedableRng};

use crate::runtime::channels::{ChannelError, ChannelResult, SendResult, ReceiveResult, ChannelSender, ChannelReceiver};

/// Unique identifier for select operations
pub type SelectId = u64;

/// Individual case in a select operation
#[derive(Debug, Clone)]
pub struct SelectCase<T> {
    /// Unique identifier for this case
    /// Channel identifier this case operates on (for logging purposes)
    /// Operation to perform
    /// Priority for this case (higher = more priority)
    /// Actual channel sender for send operations
    /// Actual channel receiver for receive operations
/// Types of operations in select cases
#[derive(Debug, Clone)]
pub enum SelectOperation<T> {
    /// Send operation with value
    /// Receive operation
    /// Default case (fallback when no other cases ready)
/// Result of a select operation
#[derive(Debug, Clone)]
pub struct SelectResult<T> {
    /// ID of the case that succeeded
    /// Channel ID that was operated on
    /// Result of the operation
    /// Time when the operation completed
/// Value result from select operation
#[derive(Debug, Clone)]
pub enum SelectResultValue<T> {
    /// Successfully sent value
    /// Successfully received value
    /// Default case executed
    /// Channel was closed during operation
    /// Operation would have blocked
/// Builder for constructing select operations
#[derive(Debug)]
pub struct SelectBuilder<T> {
impl<T> Default for SelectBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SelectBuilder<T> {
    /// Create a new select builder
    pub fn new() -> Self {
        Self {
        }
    }

    /// Add a send case
    #[instrument(skip(self, value, sender))]
    pub fn send(mut self, channel_id: u64, value: T, sender: ChannelSender<T>) -> Self {
        let case = SelectCase {
        self.cases.push(case);
        self.next_case_id += 1;
        debug!(case_id = self.next_case_id - 1, channel_id, "Added send case");
        self
    /// Add a receive case
    #[instrument(skip(self, receiver))]
    pub fn receive(mut self, channel_id: u64, receiver: ChannelReceiver<T>) -> Self {
        let case = SelectCase {
        self.cases.push(case);
        self.next_case_id += 1;
        debug!(case_id = self.next_case_id - 1, channel_id, "Added receive case");
        self
    /// Add a default case
    #[instrument(skip(self))]
    pub fn default(mut self) -> Self {
        let case = SelectCase {
            channel_id: 0, // Default case has no channel
            priority: -1000, // Default case has lowest priority
        self.cases.push(case);
        self.next_case_id += 1;
        debug!(case_id = self.next_case_id - 1, "Added default case");
        self
    /// Set priority for the last added case
    pub fn priority(mut self, priority: i32) -> Self {
        if let Some(last_case) = self.cases.last_mut() {
            last_case.priority = priority;
            debug!(case_id = last_case.case_id, priority, "Set case priority");
        }
        self
    /// Set timeout for select operation
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        debug!(?timeout, "Set select timeout");
        self
    /// Disable randomization (use priority order)
    pub fn no_randomize(mut self) -> Self {
        self.randomize = false;
        debug!("Disabled case randomization");
        self
    /// Build the select operation and return the cases
    pub fn build(self) -> (Vec<SelectCase<T>>, bool, Option<Duration>) {
        debug!(
            "Built select operation"
        );
        (self.cases, self.randomize, self.timeout)
    /// Get the number of cases
    pub fn case_count(&self) -> usize {
        self.cases.len()
    /// Check if timeout is set
    pub fn has_timeout(&self) -> bool {
        self.timeout.is_some()
    /// Get the timeout duration
    pub fn get_timeout(&self) -> Option<Duration> {
        self.timeout
    /// Get case priority by index
    pub fn get_case_priority(&self, index: usize) -> Option<i32> {
        self.cases.get(index).map(|case| case.priority)
    }
}

/// Handle for managing long-running select operations
#[derive(Debug)]
pub struct SelectHandle {
    /// Unique identifier for this select operation
    /// Cancellation flag
    /// Completion notification
impl SelectHandle {
    /// Create a new select handle
    pub fn new(select_id: SelectId) -> Self {
        Self {
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
    /// Check if the operation is cancelled
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    /// Wait for completion or cancellation
    #[instrument(skip(self))]
    pub fn wait(&self) -> ChannelResult<()> {
        let (lock, cvar) = &*self.completion;
        let mut completed = lock.lock().unwrap();
        
        while !*completed && !self.is_cancelled() {
            completed = cvar.wait(completed).unwrap();
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
    /// Random seed for fair selection
    /// Active select operations
    /// Phantom data to hold the type parameter
impl<T> Default for ChannelSelector<T> 
where 
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ChannelSelector<T> 
where 
{
    /// Create a new channel selector
    pub fn new() -> Self {
        Self {
        }
    }

    /// Try all cases once and return immediately (non-blocking)
    #[instrument(skip(self, cases))]
    pub fn select_nonblocking(&mut self, mut cases: Vec<SelectCase<T>>) -> ChannelResult<SelectResult<T>> {
        let select_id = self.next_select_id;
        self.next_select_id += 1;

        debug!(
            "Starting non-blocking select operation"
        );

        // Randomize case order for fairness if needed
        if cases.len() > 1 {
            self.randomize_cases(&mut cases);
        // Separate default cases from regular cases
        let (default_cases, regular_cases): (Vec<_>, Vec<_>) = cases.iter()
            .partition(|c| matches!(c.operation, SelectOperation::Default));

        // Try regular cases first
        for case in &regular_cases {
            match self.try_case(case) {
                Ok(result) => {
                    info!(
                        "Non-blocking select succeeded"
                    );
                    return Ok(result);
                }
                Err(ChannelError::WouldBlock) => {
                    debug!(
                        "Case would block, trying next"
                    );
                    continue;
                }
                Err(err) => {
                    debug!(
                        "Case failed"
                    );
                    continue;
                }
            }
        // If no regular case succeeded, try default case
        if let Some(default_case) = default_cases.first() {
            let result = SelectResult {
            info!(select_id, "Non-blocking select using default case");
            return Ok(result);
        warn!(select_id, "Non-blocking select would block on all cases");
        Err(ChannelError::WouldBlock)
    /// Block until one case can proceed
    #[instrument(skip(self, cases))]
    pub fn select_blocking(&mut self, mut cases: Vec<SelectCase<T>>) -> ChannelResult<SelectResult<T>> {
        let select_id = self.next_select_id;
        self.next_select_id += 1;

        let handle = SelectHandle::new(select_id);
        self.active_selects.push_back(handle);

        debug!(
            "Starting blocking select operation"
        );

        // First try non-blocking
        if let Ok(result) = self.select_nonblocking(cases.clone()) {
            info!(select_id, "Blocking select succeeded immediately");
            return Ok(result);
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
                            "Blocking select succeeded"
                        );
                        if let Some(handle) = self.active_selects.back() {
                            handle.complete();
                        }
                        return Ok(result);
                    }
                }
            }

            // Brief sleep to avoid busy waiting
            std::thread::sleep(Duration::from_micros(100));
        }
    }

    /// Block with timeout until one case can proceed
    #[instrument(skip(self, cases))]
    pub fn select_timeout(
    ) -> ChannelResult<SelectResult<T>> {
        let select_id = self.next_select_id;
        self.next_select_id += 1;

        let start_time = Instant::now();
        let handle = SelectHandle::new(select_id);
        self.active_selects.push_back(handle);

        debug!(
            "Starting timeout select operation"
        );

        // First try non-blocking
        if let Ok(result) = self.select_nonblocking(cases.clone()) {
            info!(select_id, "Timeout select succeeded immediately");
            return Ok(result);
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
                            "Timeout select succeeded"
                        );
                        if let Some(handle) = self.active_selects.back() {
                            handle.complete();
                        }
                        return Ok(result);
                    }
                }
            }

            // Brief sleep to avoid busy waiting
            std::thread::sleep(Duration::from_micros(100));
        warn!(select_id, ?timeout, "Timeout select operation timed out");
        if let Some(handle) = self.active_selects.back() {
            handle.complete();
        }
        Err(ChannelError::Timeout)
    /// Try a single select case
    #[instrument(skip(self, case))]
    fn try_case(&self, case: &SelectCase<T>) -> ChannelResult<SelectResult<T>> {
        match &case.operation {
            SelectOperation::Send(value) => {
                debug!(
                    "Attempting send operation"
                );
                
                if let Some(sender) = &case.sender {
                    // Clone the value for the try_send operation
                    match sender.try_send(value.clone()) {
                        SendResult::Sent => {
                            debug!(case_id = case.case_id, "Send succeeded");
                            Ok(SelectResult {
                            })
                        }
                        SendResult::Closed(_) => {
                            debug!(case_id = case.case_id, "Channel closed during send");
                            Ok(SelectResult {
                            })
                        }
                        SendResult::WouldBlock(_) => {
                            trace!(case_id = case.case_id, "Send would block");
                            Err(ChannelError::WouldBlock)
                        }
                    }
                } else {
                    error!(case_id = case.case_id, "Send case missing sender");
                    Err(ChannelError::InvalidState)
                }
            }
            SelectOperation::Receive => {
                debug!(
                    "Attempting receive operation"
                );
                
                if let Some(receiver) = &case.receiver {
                    match receiver.try_receive() {
                        ReceiveResult::Received(value) => {
                            debug!(case_id = case.case_id, "Receive succeeded");
                            Ok(SelectResult {
                            })
                        }
                        ReceiveResult::Closed => {
                            debug!(case_id = case.case_id, "Channel closed during receive");
                            Ok(SelectResult {
                            })
                        }
                        ReceiveResult::WouldBlock => {
                            trace!(case_id = case.case_id, "Receive would block");
                            Err(ChannelError::WouldBlock)
                        }
                    }
                } else {
                    error!(case_id = case.case_id, "Receive case missing receiver");
                    Err(ChannelError::InvalidState)
                }
            }
            SelectOperation::Default => {
                debug!(case_id = case.case_id, "Executing default case");
                Ok(SelectResult {
                })
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
            // Shuffle this priority group
            for k in i..j {
                let swap_idx = i + rng.gen_range(0..(j - i));
                cases.swap(k, swap_idx);
            i = j;
        // Update seed for next randomization
        self.random_seed = rand::random();


    /// Clean up completed select operations
    #[instrument(skip(self))]
    pub fn cleanup_completed(&mut self) {
        let initial_count = self.active_selects.len();
        self.active_selects.retain(|handle| !handle.is_cancelled());
        let cleaned_count = initial_count - self.active_selects.len();
        
        if cleaned_count > 0 {
            debug!(
                "Cleaned up completed select operations"
            );
        }
    }

    /// Get statistics about active select operations
    pub fn get_stats(&self) -> SelectStats {
        SelectStats {
        }
    }

    /// Add a handle for testing (internal use)
    pub fn add_handle_for_test(&mut self, handle: SelectHandle) {
        self.active_selects.push_back(handle);
    /// Get active selects count for testing
    pub fn active_selects_count(&self) -> usize {
        self.active_selects.len()
    /// Execute a select operation using a builder
    #[instrument(skip(self, builder))]
    pub fn execute_select(&mut self, builder: SelectBuilder<T>) -> ChannelResult<SelectResult<T>> {
        let (cases, randomize, timeout) = builder.build();
        
        if cases.is_empty() {
            warn!("Attempted to execute select with no cases");
            return Err(ChannelError::InvalidState);
        match timeout {
        }
    }

    /// Execute a non-blocking select operation using a builder
    #[instrument(skip(self, builder))]
    pub fn execute_select_nonblocking(&mut self, builder: SelectBuilder<T>) -> ChannelResult<SelectResult<T>> {
        let (cases, _randomize, _timeout) = builder.build();
        
        if cases.is_empty() {
            warn!("Attempted to execute non-blocking select with no cases");
            return Err(ChannelError::InvalidState);
        self.select_nonblocking(cases)
    }
}

/// Statistics about select operations
#[derive(Debug, Clone)]
pub struct SelectStats {
    /// Number of active select operations
    /// Next select ID to be assigned
/// FFI-friendly functions for LLVM integration
pub mod ffi {
    use super::*;
    use std::ffi::c_void;

    /// Create a new select builder (FFI-safe)
    #[no_mangle]
    pub extern "C" fn cursed_select_builder_new() -> *mut c_void {
        let builder = Box::new(SelectBuilder::<i64>::new());
        Box::into_raw(builder) as *mut c_void
    /// Add send case to builder (FFI-safe)
    /// NOTE: This is a simplified FFI interface. In practice, you'd need to pass
    /// the actual channel sender handle, which is complex to do safely via FFI.
    /// This serves as a placeholder for LLVM integration.
    #[no_mangle]
    pub extern "C" fn cursed_select_builder_add_send(
    ) -> *mut c_void {
        if builder.is_null() {
            return std::ptr::null_mut();
        // NOTE: This is incomplete - we need the actual ChannelSender<i64>
        // In real usage, this would be passed from the LLVM compiled code
        // For now, this returns null to indicate the limitation
        std::ptr::null_mut()
    /// Add receive case to builder (FFI-safe)
    /// NOTE: This is a simplified FFI interface. Similar limitation as send case.
    #[no_mangle]
    pub extern "C" fn cursed_select_builder_add_receive(
    ) -> *mut c_void {
        if builder.is_null() {
            return std::ptr::null_mut();
        // NOTE: This is incomplete - we need the actual ChannelReceiver<i64>
        // In real usage, this would be passed from the LLVM compiled code
        // For now, this returns null to indicate the limitation
        std::ptr::null_mut()
    /// Execute non-blocking select (FFI-safe)
    #[no_mangle]
    pub extern "C" fn cursed_select_nonblocking(
    ) -> i32 {
        if selector.is_null() || cases.is_null() {
            return -1; // CursedError
        unsafe {
            let selector = &mut *(selector as *mut ChannelSelector<i64>);
            let cases_slice = std::slice::from_raw_parts(cases, cases_len);
            let cases_vec = cases_slice.to_vec();

            match selector.select_nonblocking(cases_vec) {
                Ok(_result) => 0, // Success
                Err(_) => -1,     // CursedError
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

