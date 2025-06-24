use crate::error::Error;
/// Signal actions for common signal handling patterns
use std::process;
use std::sync::Arc;
use crate::stdlib::signal_boost::core::BoostSignal;
use crate::stdlib::signal_boost::error::{SignalBoostError, SignalBoostResult};

/// A signal action function that returns whether the signal was handled
pub type SignalAction = Arc<dyn Fn(BoostSignal) -> bool + Send + Sync>;

/// Create a signal action from a function
pub fn create_action<F>(action: F) -> SignalAction
where
    F: Fn(BoostSignal) -> bool + Send + Sync + 'static,
{
    Arc::new(action)
}

/// Ignore the signal (do nothing)
pub fn ignore_action() -> SignalAction {
    create_action(|signal| {
        tracing::debug!("Ignoring signal: {}", signal);
        true
    })
}

/// Exit the program immediately
pub fn exit_action() -> SignalAction {
    create_action(|signal| {
        tracing::info!("Exiting due to signal: {}", signal);
        process::exit(0);
    })
}

/// Exit the program with a specific exit code
pub fn exit_with_code_action(code: i32) -> SignalAction {
    create_action(move |signal| {
        tracing::info!("Exiting with code {} due to signal: {}", code, signal);
        process::exit(code);
    })
}

/// Log the signal using tracing
pub fn log_action(level: LogLevel) -> SignalAction {
    create_action(move |signal| {
        match level {
            LogLevel::Trace => tracing::trace!("Signal received: {}", signal),
            LogLevel::Debug => tracing::debug!("Signal received: {}", signal),
            LogLevel::Info => tracing::info!("Signal received: {}", signal),
            LogLevel::Warn => tracing::warn!("Signal received: {}", signal),
            LogLevel::Error => tracing::error!("Signal received: {}", signal),
        }
        true
    })
}

/// Log level for signal logging
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Custom action using a logger-like interface
pub fn custom_log_action<F>(logger: F) -> SignalAction
where
    F: Fn(&str) + Send + Sync + 'static,
{
    create_action(move |signal| {
        logger(&format!("Signal received: {}", signal));
        true
    })
}

/// "Shook" action - logs that the signal shook the application
pub fn shook_action() -> SignalAction {
    create_action(|signal| {
        tracing::warn!("Application shook by signal: {} 😱", signal);
        true
    })
}

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
}

/// Conditional action - only execute if predicate is true
pub fn conditional_action<P>(predicate: P, action: SignalAction) -> SignalAction
where
    P: Fn(BoostSignal) -> bool + Send + Sync + 'static,
{
    create_action(move |signal| {
        if predicate(signal) {
            action(signal)
        } else {
            false
        }
    })
}

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
            },
            _ => {
                *last = Some(now);
                action(signal)
            }
        }
    })
}

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
}

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
}

/// Retry action - retry the action if it fails
pub fn retry_action(action: SignalAction, max_retries: usize, retry_delay: std::time::Duration) -> SignalAction {
    create_action(move |signal| {
        for attempt in 0..=max_retries {
            if action(signal) {
                return true;
            }
            
            if attempt < max_retries {
                tracing::warn!("Signal action failed, retrying in {:?} (attempt {}/{})", 
                              retry_delay, attempt + 1, max_retries);
                std::thread::sleep(retry_delay);
            }
        }
        
        tracing::error!("Signal action failed after {} retries for signal {}", max_retries, signal);
        false
    })
}

/// Memory pressure action - only execute if memory usage is below threshold
pub fn memory_pressure_action(action: SignalAction, max_memory_mb: usize) -> SignalAction {
    create_action(move |signal| {
        // Simple memory check (in a real implementation, you'd check actual memory usage)
        let estimated_memory = get_estimated_memory_usage();
        
        if estimated_memory < max_memory_mb {
            action(signal)
        } else {
            tracing::warn!("Skipping signal action due to memory pressure: {}MB > {}MB", 
                          estimated_memory, max_memory_mb);
            false
        }
    })
}

/// Get estimated memory usage (simplified implementation)
fn get_estimated_memory_usage() -> usize {
    // In a real implementation, this would check actual memory usage
    // For now, we'll return a dummy value
    use std::sync::atomic::{AtomicUsize, Ordering};
    static SIMULATED_MEMORY: AtomicUsize = AtomicUsize::new(50);
    SIMULATED_MEMORY.load(Ordering::SeqCst)
}

/// Thread-safe action - ensure only one instance runs at a time
pub fn thread_safe_action(action: SignalAction) -> SignalAction {
    use std::sync::Mutex;
    
    let mutex = Arc::new(Mutex::new(()));
    
    create_action(move |signal| {
        match mutex.try_lock() {
            Ok(_guard) => {
                tracing::debug!("Executing thread-safe action for signal {}", signal);
                action(signal)
            },
            Err(_) => {
                tracing::warn!("Thread-safe action already running for signal {}", signal);
                false
            }
        }
    })
}

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
}

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
            },
            Err(err) => {
                tracing::error!("Failed to open file {}: {}", file_path, err);
                false
            }
        }
    })
}

/// Environment variable action - set an environment variable when signal is received
pub fn env_var_action(var_name: String, var_value: String) -> SignalAction {
    create_action(move |signal| {
        std::env::set_var(&var_name, &var_value);
        tracing::info!("Set environment variable {}={} due to signal {}", 
                      var_name, var_value, signal);
        true
    })
}

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
            },
            Err(err) => {
                tracing::error!("Failed to execute command '{}': {}", command, err);
                false
            }
        }
    })
}

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::signal_boost::core::SIGINT;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;
    
    #[test]
    fn test_ignore_action() {
        let action = ignore_action();
        assert!(action(SIGINT));
    }
    
    #[test]
    fn test_log_action() {
        let action = log_action(LogLevel::Info);
        assert!(action(SIGINT));
    }
    
    #[test]
    fn test_custom_log_action() {
        let logged = Arc::new(AtomicBool::new(false));
        let logged_clone = Arc::clone(&logged);
        
        let action = custom_log_action(move |_msg| {
            logged_clone.store(true, Ordering::SeqCst);
        });
        
        assert!(action(SIGINT));
        assert!(logged.load(Ordering::SeqCst));
    }
    
    #[test]
    fn test_shook_action() {
        let action = shook_action();
        assert!(action(SIGINT));
    }
    
    #[test]
    fn test_chain_actions() {
        let called1 = Arc::new(AtomicBool::new(false));
        let called2 = Arc::new(AtomicBool::new(false));
        
        let called1_clone = Arc::clone(&called1);
        let called2_clone = Arc::clone(&called2);
        
        let action1 = create_action(move |_| {
            called1_clone.store(true, Ordering::SeqCst);
            true
        });
        
        let action2 = create_action(move |_| {
            called2_clone.store(true, Ordering::SeqCst);
            true
        });
        
        let chained = chain_actions(vec![action1, action2]);
        assert!(chained(SIGINT));
        assert!(called1.load(Ordering::SeqCst));
        assert!(called2.load(Ordering::SeqCst));
    }
    
    #[test]
    fn test_conditional_action() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        
        let action = create_action(move |_| {
            called_clone.store(true, Ordering::SeqCst);
            true
        });
        
        let conditional = conditional_action(|signal| signal == SIGINT, action);
        
        assert!(conditional(SIGINT));
        assert!(called.load(Ordering::SeqCst));
    }
    
    #[test]
    fn test_counting_action() {
        let base_action = create_action(|_| true);
        let (counting, counter) = counting_action(base_action);
        
        assert_eq!(counter.load(Ordering::SeqCst), 0);
        
        assert!(counting(SIGINT));
        assert_eq!(counter.load(Ordering::SeqCst), 1);
        
        assert!(counting(SIGINT));
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }
    
    #[test]
    fn test_rate_limited_action() {
        let called = Arc::new(AtomicUsize::new(0));
        let called_clone = Arc::clone(&called);
        
        let action = create_action(move |_| {
            called_clone.fetch_add(1, Ordering::SeqCst);
            true
        });
        
        let rate_limited = rate_limited_action(action, Duration::from_millis(100));
        
        assert!(rate_limited(SIGINT));
        assert_eq!(called.load(Ordering::SeqCst), 1);
        
        // Should be rate limited
        assert!(!rate_limited(SIGINT));
        assert_eq!(called.load(Ordering::SeqCst), 1);
        
        // Wait for rate limit to expire
        std::thread::sleep(Duration::from_millis(150));
        assert!(rate_limited(SIGINT));
        assert_eq!(called.load(Ordering::SeqCst), 2);
    }
    
    #[test]
    fn test_delayed_action() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        
        let action = create_action(move |_| {
            called_clone.store(true, Ordering::SeqCst);
            true
        });
        
        let delayed = delayed_action(action, Duration::from_millis(50));
        
        assert!(delayed(SIGINT));
        assert!(!called.load(Ordering::SeqCst)); // Should not be called immediately
        
        std::thread::sleep(Duration::from_millis(100));
        assert!(called.load(Ordering::SeqCst)); // Should be called after delay
    }
    
    #[test]
    fn test_thread_safe_action() {
        let action = create_action(|_| {
            std::thread::sleep(Duration::from_millis(50));
            true
        });
        
        let thread_safe = thread_safe_action(action);
        
        // First call should succeed
        let handle1 = {
            let ts = Arc::clone(&thread_safe);
            std::thread::spawn(move || ts(SIGINT))
        };
        
        std::thread::sleep(Duration::from_millis(10));
        
        // Second call should fail (already running)
        assert!(!thread_safe(SIGINT));
        
        // Wait for first to complete
        assert!(handle1.join().unwrap());
    }
    
    #[test]
    fn test_async_action() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        
        let action = create_action(move |_| {
            called_clone.store(true, Ordering::SeqCst);
            true
        });
        
        let async_action = async_action(action);
        
        assert!(async_action(SIGINT)); // Should return immediately
        
        // Give async thread time to execute
        std::thread::sleep(Duration::from_millis(10));
        assert!(called.load(Ordering::SeqCst));
    }
    
    #[test]
    fn test_env_var_action() {
        let action = env_var_action("TEST_SIGNAL_VAR".to_string(), "signal_received".to_string());
        
        assert!(action(SIGINT));
        assert_eq!(std::env::var("TEST_SIGNAL_VAR").unwrap(), "signal_received");
    }
    
    #[test]
    fn test_memory_pressure_action() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        
        let action = create_action(move |_| {
            called_clone.store(true, Ordering::SeqCst);
            true
        });
        
        // Should execute with high memory limit
        let memory_action = memory_pressure_action(action, 1000);
        assert!(memory_action(SIGINT));
        assert!(called.load(Ordering::SeqCst));
    }
}
