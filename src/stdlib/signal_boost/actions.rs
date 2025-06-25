use crate::error::CursedError;
/// Signal actions for common signal handling patterns
use std::process;
use std::sync::Arc;
// use crate::stdlib::signal_boost::core::BoostSignal;
// use crate::stdlib::signal_boost::error::{SignalBoostError, SignalBoostResult};

/// A signal action function that returns whether the signal was handled
pub type SignalAction = Arc<dyn Fn(BoostSignal) -> bool + Send + Sync>;

/// Create a signal action from a function
pub fn create_action<F>(action: F) -> SignalAction
where
{
    Arc::new(action)
/// Ignore the signal (do nothing)
pub fn ignore_action() -> SignalAction {
    create_action(|signal| {
        tracing::debug!("Ignoring signal: {}", signal);
        true
    })
/// Exit the program immediately
pub fn exit_action() -> SignalAction {
    create_action(|signal| {
        tracing::info!("Exiting due to signal: {}", signal);
        process::exit(0);
    })
/// Exit the program with a specific exit code
pub fn exit_with_code_action(code: i32) -> SignalAction {
    create_action(move |signal| {
        tracing::info!("Exiting with code {} due to signal: {}", code, signal);
        process::exit(code);
    })
/// Log the signal using tracing
pub fn log_action(level: LogLevel) -> SignalAction {
    create_action(move |signal| {
        match level {
        }
        true
    })
/// Log level for signal logging
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
/// Custom action using a logger-like interface
pub fn custom_log_action<F>(logger: F) -> SignalAction
where
{
    create_action(move |signal| {
        logger(&format!("Signal received: {}", signal));
        true
    })
/// "Shook" action - logs that the signal shook the application
pub fn shook_action() -> SignalAction {
    create_action(|signal| {
        tracing::warn!("Application shook by signal: {} 😱", signal);
        true
    })
/// Chain multiple actions together
pub fn chain_actions(actions: Vec<SignalAction>) -> SignalAction {
    create_action(move |signal| {
        let mut all_handled = true;
        for action in &actions {
            if !action(signal) {
                all_handled = false;
            }
        }
        all_handled
    })
/// Conditional action - only execute if predicate is true
pub fn conditional_action<P>(predicate: P, action: SignalAction) -> SignalAction
where
{
    create_action(move |signal| {
        if predicate(signal) {
            action(signal)
        } else {
            false
        }
    })
/// Rate-limited action - only execute once per duration
pub fn rate_limited_action(action: SignalAction, duration: std::time::Duration) -> SignalAction {
    use std::sync::Mutex;
    use std::time::Instant;
    
    let last_execution = Arc::new(Mutex::new(None::<Instant>));
    
    create_action(move |signal| {
        let mut last = last_execution.lock().unwrap();
        let now = Instant::now();
        
        match *last {
            Some(last_time) if now.duration_since(last_time) < duration => {
                tracing::debug!("Rate limiting signal action for {}", signal);
                false
            _ => {
                *last = Some(now);
                action(signal)
            }
        }
    })
/// Counting action - tracks how many times a signal has been received
pub fn counting_action(action: SignalAction) -> (SignalAction, Arc<std::sync::atomic::AtomicUsize>) {
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);
    
    let counting = create_action(move |signal| {
        counter_clone.fetch_add(1, Ordering::SeqCst);
        action(signal)
    });
    
    (counting, counter)
/// Delayed action - execute after a delay
pub fn delayed_action(action: SignalAction, delay: std::time::Duration) -> SignalAction {
    create_action(move |signal| {
        let action_clone = Arc::clone(&action);
        std::thread::spawn(move || {
            std::thread::sleep(delay);
            action_clone(signal);
        });
        true
    })
/// Retry action - retry the action if it fails
pub fn retry_action(action: SignalAction, max_retries: usize, retry_delay: std::time::Duration) -> SignalAction {
    create_action(move |signal| {
        for attempt in 0..=max_retries {
            if action(signal) {
                return true;
            if attempt < max_retries {
                tracing::warn!("Signal action failed, retrying in {:?} (attempt {}/{})", 
                              retry_delay, attempt + 1, max_retries);
                std::thread::sleep(retry_delay);
            }
        }
        
        tracing::error!("Signal action failed after {} retries for signal {}", max_retries, signal);
        false
    })
/// Memory pressure action - only execute if memory usage is below threshold
pub fn memory_pressure_action(action: SignalAction, max_memory_mb: usize) -> SignalAction {
    create_action(move |signal| {
        // Simple memory check (in a real implementation, you'd check actual memory usage)
        let estimated_memory = get_estimated_memory_usage();
        
        if estimated_memory < max_memory_mb {
            action(signal)
        } else {
                          estimated_memory, max_memory_mb);
            false
        }
    })
/// Get estimated memory usage (simplified implementation)
fn get_estimated_memory_usage() -> usize {
    // In a real implementation, this would check actual memory usage
    // For now, we'll return a dummy value
    use std::sync::atomic::{AtomicUsize, Ordering};
    static SIMULATED_MEMORY: AtomicUsize = AtomicUsize::new(50);
    SIMULATED_MEMORY.load(Ordering::SeqCst)
/// Thread-safe action - ensure only one instance runs at a time
pub fn thread_safe_action(action: SignalAction) -> SignalAction {
    use std::sync::Mutex;
    
    let mutex = Arc::new(Mutex::new(()));
    
    create_action(move |signal| {
        match mutex.try_lock() {
            Ok(_guard) => {
                tracing::debug!("Executing thread-safe action for signal {}", signal);
                action(signal)
            Err(_) => {
                tracing::warn!("Thread-safe action already running for signal {}", signal);
                false
            }
        }
    })
/// Async action wrapper - execute action in background thread
pub fn async_action(action: SignalAction) -> SignalAction {
    create_action(move |signal| {
        let action_clone = Arc::clone(&action);
        std::thread::spawn(move || {
            tracing::debug!("Executing async action for signal {}", signal);
            action_clone(signal);
        });
        true
    })
/// File-writing action - write signal information to a file
pub fn file_writer_action(file_path: String) -> SignalAction {
    use std::fs::OpenOptions;
    use std::io::Write;
    
    create_action(move |signal| {
        match OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_path)
        {
            Ok(mut file) => {
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                
                if writeln!(file, "{}: Signal {} received", timestamp, signal).is_ok() {
                    tracing::debug!("Wrote signal {} to file {}", signal, file_path);
                    true
                } else {
                    tracing::error!("Failed to write signal {} to file {}", signal, file_path);
                    false
                }
            Err(err) => {
                tracing::error!("Failed to open file {}: {}", file_path, err);
                false
            }
        }
    })
/// Environment variable action - set an environment variable when signal is received
pub fn env_var_action(var_name: String, var_value: String) -> SignalAction {
    create_action(move |signal| {
        std::env::set_var(&var_name, &var_value);
                      var_name, var_value, signal);
        true
    })
/// Command execution action - run a command when signal is received
pub fn command_action(command: String, args: Vec<String>) -> SignalAction {
    create_action(move |signal| {
        tracing::info!("Executing command '{}' due to signal {}", command, signal);
        
        match std::process::Command::new(&command)
            .args(&args)
            .status()
        {
            Ok(status) => {
                if status.success() {
                    tracing::info!("Command '{}' executed successfully", command);
                    true
                } else {
                    tracing::error!("Command '{}' failed with status: {}", command, status);
                    false
                }
            Err(err) => {
                tracing::error!("Failed to execute command '{}': {}", command, err);
                false
            }
        }
    })
/// Network notification action - send HTTP request when signal is received
pub fn network_notification_action(url: String, timeout_secs: u64) -> SignalAction {
    create_action(move |signal| {
        tracing::info!("Sending network notification to {} for signal {}", url, signal);
        
        // In a real implementation, you would use an HTTP client
        // For now, we'll simulate the network call
        std::thread::spawn({
            let url = url.clone();
            move || {
                std::thread::sleep(std::time::Duration::from_millis(100)); // Simulate network delay
                tracing::info!("Network notification sent to {} (simulated)", url);
            }
        });
        
        true
    })
