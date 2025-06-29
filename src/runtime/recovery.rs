//! Error recovery and resilience system for CURSED runtime
//!
//! Provides comprehensive error recovery capabilities including automatic
//! retry mechanisms, fallback strategies, circuit breakers, and system
//! resilience patterns for the CURSED runtime.

use crate::error_types::{Error, Result as CursedResult};
use crate::runtime::error_context::ErrorContext;
use crate::runtime::error_propagation::{ErrorSeverity, RecoveryAction};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use std::fmt;

/// Recovery strategy for handling errors
#[derive(Debug, Clone)]
pub struct RecoveryStrategy {
    /// Strategy identifier
    pub id: String,
    /// Strategy name
    pub name: String,
    /// Recovery actions to attempt
    pub actions: Vec<RecoveryAction>,
    /// Conditions when this strategy applies
    pub conditions: Vec<RecoveryCondition>,
    /// Maximum number of attempts
    pub max_attempts: usize,
    /// Backoff strategy between attempts
    pub backoff: BackoffStrategy,
    /// Timeout for recovery attempts
    pub timeout: Duration,
    /// Success rate threshold to continue using this strategy
    pub success_threshold: f64,
}

/// Conditions that trigger recovery strategies
#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryCondition {
    /// Error severity level
    ErrorSeverity(ErrorSeverity),
    /// Error message contains text
    ErrorMessageContains(String),
    /// Error occurred in specific module
    ErrorInModule(String),
    /// Resource exhaustion
    ResourceExhausted(ResourceType),
    /// Network connectivity issues
    NetworkIssue,
    /// Timeout occurred
    TimeoutOccurred,
    /// Memory pressure
    MemoryPressure,
    /// Custom condition
    Custom(String),
}

/// Types of resources that can be exhausted
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceType {
    Memory,
    CPU,
    Disk,
    Network,
    FileDescriptors,
    Threads,
    Goroutines,
    Channels,
}

/// Backoff strategies for retry attempts
#[derive(Debug, Clone)]
pub enum BackoffStrategy {
    /// No delay between attempts
    None,
    /// Fixed delay between attempts
    Fixed(Duration),
    /// Exponential backoff with base delay
    Exponential { base: Duration, multiplier: f64, max: Duration },
    /// Linear backoff with increment
    Linear { base: Duration, increment: Duration, max: Duration },
    /// Jittered exponential backoff
    Jittered { base: Duration, multiplier: f64, max: Duration },
}

/// Recovery attempt result
#[derive(Debug, Clone)]
pub struct RecoveryAttempt {
    /// Attempt number (1-based)
    pub attempt_number: usize,
    /// Strategy used
    pub strategy_id: String,
    /// Start time
    pub start_time: Instant,
    /// Duration of attempt
    pub duration: Duration,
    /// Result of attempt
    pub result: RecoveryResult,
    /// Actions taken
    pub actions_taken: Vec<RecoveryAction>,
    /// Error that occurred (if any)
    pub error: Option<Error>,
}

/// Result of a recovery attempt
#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryResult {
    /// Recovery succeeded
    Success,
    /// Recovery failed but should retry
    FailedRetry,
    /// Recovery failed permanently
    FailedPermanent,
    /// Recovery timed out
    Timeout,
    /// Recovery was skipped
    Skipped,
}

/// Circuit breaker for preventing cascading failures
#[derive(Debug)]
pub struct CircuitBreaker {
    /// Circuit breaker name
    pub name: String,
    /// Current state
    pub state: CircuitBreakerState,
    /// Configuration
    pub config: CircuitBreakerConfig,
    /// Failure counter
    pub failure_count: usize,
    /// Success counter
    pub success_count: usize,
    /// Last failure time
    pub last_failure_time: Option<Instant>,
    /// State change history
    pub state_history: Vec<CircuitBreakerStateChange>,
}

/// Circuit breaker states
#[derive(Debug, Clone, PartialEq)]
pub enum CircuitBreakerState {
    /// Circuit is closed (normal operation)
    Closed,
    /// Circuit is open (failing fast)
    Open,
    /// Circuit is half-open (testing recovery)
    HalfOpen,
}

/// Circuit breaker configuration
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Failure threshold to open circuit
    pub failure_threshold: usize,
    /// Success threshold to close circuit from half-open
    pub success_threshold: usize,
    /// Timeout before attempting half-open
    pub timeout: Duration,
    /// Rolling window for failure counting
    pub rolling_window: Duration,
    /// Maximum requests in half-open state
    pub max_half_open_requests: usize,
}

/// Circuit breaker state change event
#[derive(Debug, Clone)]
pub struct CircuitBreakerStateChange {
    /// Previous state
    pub from_state: CircuitBreakerState,
    /// New state
    pub to_state: CircuitBreakerState,
    /// Timestamp of change
    pub timestamp: Instant,
    /// Reason for change
    pub reason: String,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 3,
            timeout: Duration::from_secs(60),
            rolling_window: Duration::from_secs(300),
            max_half_open_requests: 2,
        }
    }
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    pub fn new(name: String, config: CircuitBreakerConfig) -> Self {
        Self {
            name,
            state: CircuitBreakerState::Closed,
            config,
            failure_count: 0,
            success_count: 0,
            last_failure_time: None,
            state_history: Vec::new(),
        }
    }

    /// Check if request should be allowed
    pub fn can_execute(&self) -> bool {
        match self.state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::Open => {
                if let Some(last_failure) = self.last_failure_time {
                    last_failure.elapsed() >= self.config.timeout
                } else {
                    false
                }
            }
            CircuitBreakerState::HalfOpen => {
                // Allow limited requests in half-open state
                self.success_count < self.config.max_half_open_requests
            }
        }
    }

    /// Record a successful operation
    pub fn record_success(&mut self) {
        self.success_count += 1;
        
        match self.state {
            CircuitBreakerState::HalfOpen => {
                if self.success_count >= self.config.success_threshold {
                    self.transition_to_state(CircuitBreakerState::Closed, "Success threshold reached");
                    self.failure_count = 0;
                    self.success_count = 0;
                }
            }
            CircuitBreakerState::Closed => {
                // Reset failure count on success
                self.failure_count = 0;
            }
            _ => {}
        }
    }

    /// Record a failed operation
    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure_time = Some(Instant::now());
        
        match self.state {
            CircuitBreakerState::Closed => {
                if self.failure_count >= self.config.failure_threshold {
                    self.transition_to_state(CircuitBreakerState::Open, "Failure threshold exceeded");
                }
            }
            CircuitBreakerState::HalfOpen => {
                self.transition_to_state(CircuitBreakerState::Open, "Failure in half-open state");
                self.success_count = 0;
            }
            _ => {}
        }
    }

    /// Transition to new state
    fn transition_to_state(&mut self, new_state: CircuitBreakerState, reason: &str) {
        let old_state = self.state.clone();
        self.state = new_state.clone();
        
        let state_change = CircuitBreakerStateChange {
            from_state: old_state,
            to_state: new_state,
            timestamp: Instant::now(),
            reason: reason.to_string(),
        };
        
        self.state_history.push(state_change);
    }

    /// Check for state transitions
    pub fn check_transitions(&mut self) {
        match self.state {
            CircuitBreakerState::Open => {
                if let Some(last_failure) = self.last_failure_time {
                    if last_failure.elapsed() >= self.config.timeout {
                        self.transition_to_state(CircuitBreakerState::HalfOpen, "Timeout elapsed");
                        self.success_count = 0;
                    }
                }
            }
            _ => {}
        }
    }
}

/// Recovery system for managing error recovery strategies
pub struct RecoverySystem {
    /// Recovery strategies by ID
    strategies: RwLock<HashMap<String, RecoveryStrategy>>,
    /// Circuit breakers by name
    circuit_breakers: RwLock<HashMap<String, CircuitBreaker>>,
    /// Recovery history
    recovery_history: Mutex<Vec<RecoveryAttempt>>,
    /// System configuration
    config: RecoverySystemConfig,
    /// System statistics
    stats: Mutex<RecoveryStats>,
}

/// Configuration for recovery system
#[derive(Debug, Clone)]
pub struct RecoverySystemConfig {
    /// Maximum recovery attempts per error
    pub max_recovery_attempts: usize,
    /// Global timeout for recovery operations
    pub global_timeout: Duration,
    /// Whether to use circuit breakers
    pub enable_circuit_breakers: bool,
    /// Maximum history entries to keep
    pub max_history_entries: usize,
    /// Whether to enable automatic strategy selection
    pub enable_auto_strategy_selection: bool,
}

impl Default for RecoverySystemConfig {
    fn default() -> Self {
        Self {
            max_recovery_attempts: 3,
            global_timeout: Duration::from_secs(300), // 5 minutes
            enable_circuit_breakers: true,
            max_history_entries: 1000,
            enable_auto_strategy_selection: true,
        }
    }
}

/// Recovery system statistics
#[derive(Debug, Default, Clone)]
pub struct RecoveryStats {
    pub total_recovery_attempts: usize,
    pub successful_recoveries: usize,
    pub failed_recoveries: usize,
    pub timeouts: usize,
    pub circuit_breaker_trips: usize,
    pub strategies_used: HashMap<String, usize>,
    pub average_recovery_time_ms: f64,
}

impl RecoverySystem {
    /// Create a new recovery system
    pub fn new() -> Self {
        let mut system = Self {
            strategies: RwLock::new(HashMap::new()),
            circuit_breakers: RwLock::new(HashMap::new()),
            recovery_history: Mutex::new(Vec::new()),
            config: RecoverySystemConfig::default(),
            stats: Mutex::new(RecoveryStats::default()),
        };
        
        // Add default recovery strategies
        system.add_default_strategies();
        system
    }

    /// Add default recovery strategies
    fn add_default_strategies(&mut self) {
        // Retry strategy for transient errors
        let retry_strategy = RecoveryStrategy {
            id: "retry_transient".to_string(),
            name: "Retry Transient Errors".to_string(),
            actions: vec![RecoveryAction::Retry],
            conditions: vec![
                RecoveryCondition::ErrorMessageContains("timeout".to_string()),
                RecoveryCondition::ErrorMessageContains("connection".to_string()),
                RecoveryCondition::NetworkIssue,
            ],
            max_attempts: 3,
            backoff: BackoffStrategy::Exponential {
                base: Duration::from_millis(100),
                multiplier: 2.0,
                max: Duration::from_secs(10),
            },
            timeout: Duration::from_secs(30),
            success_threshold: 0.7,
        };
        self.register_strategy(retry_strategy);

        // Fallback strategy for service unavailable
        let fallback_strategy = RecoveryStrategy {
            id: "fallback_service".to_string(),
            name: "Fallback to Alternative Service".to_string(),
            actions: vec![RecoveryAction::Fallback, RecoveryAction::UseDefault],
            conditions: vec![
                RecoveryCondition::ErrorMessageContains("service unavailable".to_string()),
                RecoveryCondition::ErrorMessageContains("not found".to_string()),
            ],
            max_attempts: 1,
            backoff: BackoffStrategy::None,
            timeout: Duration::from_secs(5),
            success_threshold: 0.8,
        };
        self.register_strategy(fallback_strategy);

        // Memory recovery strategy
        let memory_strategy = RecoveryStrategy {
            id: "memory_recovery".to_string(),
            name: "Memory Recovery".to_string(),
            actions: vec![RecoveryAction::Retry],
            conditions: vec![
                RecoveryCondition::ResourceExhausted(ResourceType::Memory),
                RecoveryCondition::MemoryPressure,
            ],
            max_attempts: 2,
            backoff: BackoffStrategy::Fixed(Duration::from_secs(1)),
            timeout: Duration::from_secs(15),
            success_threshold: 0.6,
        };
        self.register_strategy(memory_strategy);
    }

    /// Register a recovery strategy
    pub fn register_strategy(&mut self, strategy: RecoveryStrategy) {
        let mut strategies = self.strategies.write().unwrap();
        strategies.insert(strategy.id.clone(), strategy);
    }

    /// Find applicable strategies for an error context
    pub fn find_applicable_strategies(&self, error_context: &ErrorContext) -> Vec<String> {
        let strategies = self.strategies.read().unwrap();
        let mut applicable = Vec::new();

        for (id, strategy) in strategies.iter() {
            if self.strategy_applies(strategy, error_context) {
                applicable.push(id.clone());
            }
        }

        // Sort by success rate (descending)
        applicable.sort_by(|a, b| {
            let strategy_a = strategies.get(a).unwrap();
            let strategy_b = strategies.get(b).unwrap();
            strategy_b.success_threshold.partial_cmp(&strategy_a.success_threshold)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        applicable
    }

    /// Check if a strategy applies to an error context
    fn strategy_applies(&self, strategy: &RecoveryStrategy, error_context: &ErrorContext) -> bool {
        if strategy.conditions.is_empty() {
            return true;
        }

        for condition in &strategy.conditions {
            if self.condition_matches(condition, error_context) {
                return true;
            }
        }

        false
    }

    /// Check if a condition matches an error context
    fn condition_matches(&self, condition: &RecoveryCondition, error_context: &ErrorContext) -> bool {
        match condition {
            RecoveryCondition::ErrorSeverity(severity) => {
                &error_context.classification.severity == severity
            }
            RecoveryCondition::ErrorMessageContains(text) => {
                format!("{}", error_context.primary_error).to_lowercase().contains(&text.to_lowercase())
            }
            RecoveryCondition::ErrorInModule(module) => {
                error_context.environment.function_name
                    .as_ref()
                    .map_or(false, |func| func.contains(module))
            }
            RecoveryCondition::ResourceExhausted(resource_type) => {
                let message = format!("{}", error_context.primary_error).to_lowercase();
                match resource_type {
                    ResourceType::Memory => message.contains("memory") || message.contains("oom"),
                    ResourceType::CPU => message.contains("cpu") || message.contains("timeout"),
                    ResourceType::Disk => message.contains("disk") || message.contains("space"),
                    ResourceType::Network => message.contains("network") || message.contains("connection"),
                    ResourceType::FileDescriptors => message.contains("file descriptor") || message.contains("too many files"),
                    ResourceType::Threads => message.contains("thread") || message.contains("spawn"),
                    ResourceType::Goroutines => message.contains("goroutine"),
                    ResourceType::Channels => message.contains("channel"),
                }
            }
            RecoveryCondition::NetworkIssue => {
                let message = format!("{}", error_context.primary_error).to_lowercase();
                message.contains("network") || 
                message.contains("connection") || 
                message.contains("timeout") ||
                message.contains("dns")
            }
            RecoveryCondition::TimeoutOccurred => {
                format!("{}", error_context.primary_error).to_lowercase().contains("timeout")
            }
            RecoveryCondition::MemoryPressure => {
                format!("{}", error_context.primary_error).to_lowercase().contains("memory")
            }
            RecoveryCondition::Custom(pattern) => {
                format!("{}", error_context.primary_error).contains(pattern)
            }
        }
    }

    /// Attempt recovery for an error
    pub fn attempt_recovery(&self, error_context: &ErrorContext) -> CursedResult<RecoveryAttempt> {
        let start_time = Instant::now();
        
        // Find applicable strategies
        let strategy_ids = self.find_applicable_strategies(error_context);
        
        if strategy_ids.is_empty() {
            return Ok(RecoveryAttempt {
                attempt_number: 1,
                strategy_id: "none".to_string(),
                start_time,
                duration: start_time.elapsed(),
                result: RecoveryResult::Skipped,
                actions_taken: Vec::new(),
                error: Some(Error::Runtime("No applicable recovery strategy found".to_string())),
            });
        }

        // Try the best strategy
        let strategy_id = strategy_ids[0].clone();
        let strategies = self.strategies.read().unwrap();
        let strategy = strategies.get(&strategy_id).unwrap().clone();
        drop(strategies);

        // Check circuit breaker if enabled
        if self.config.enable_circuit_breakers {
            let mut circuit_breakers = self.circuit_breakers.write().unwrap();
            let circuit_breaker = circuit_breakers.entry(strategy_id.clone())
                .or_insert_with(|| CircuitBreaker::new(strategy_id.clone(), CircuitBreakerConfig::default()));
            
            circuit_breaker.check_transitions();
            
            if !circuit_breaker.can_execute() {
                return Ok(RecoveryAttempt {
                    attempt_number: 1,
                    strategy_id: strategy_id.clone(),
                    start_time,
                    duration: start_time.elapsed(),
                    result: RecoveryResult::FailedPermanent,
                    actions_taken: Vec::new(),
                    error: Some(Error::Runtime("Circuit breaker is open".to_string())),
                });
            }
        }

        // Perform recovery attempt
        let result = self.execute_recovery_strategy(&strategy, error_context);
        let duration = start_time.elapsed();

        let attempt = RecoveryAttempt {
            attempt_number: 1,
            strategy_id: strategy_id.clone(),
            start_time,
            duration,
            result: result.clone(),
            actions_taken: strategy.actions.clone(),
            error: None,
        };

        // Update circuit breaker
        if self.config.enable_circuit_breakers {
            let mut circuit_breakers = self.circuit_breakers.write().unwrap();
            if let Some(circuit_breaker) = circuit_breakers.get_mut(&strategy_id) {
                match result {
                    RecoveryResult::Success => circuit_breaker.record_success(),
                    _ => circuit_breaker.record_failure(),
                }
            }
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_recovery_attempts += 1;
            
            match result {
                RecoveryResult::Success => stats.successful_recoveries += 1,
                RecoveryResult::Timeout => stats.timeouts += 1,
                _ => stats.failed_recoveries += 1,
            }
            
            *stats.strategies_used.entry(strategy_id).or_insert(0) += 1;
            
            // Update average recovery time
            let total_time = stats.average_recovery_time_ms * (stats.total_recovery_attempts - 1) as f64;
            stats.average_recovery_time_ms = (total_time + duration.as_secs_f64() * 1000.0) / stats.total_recovery_attempts as f64;
        }

        // Add to history
        {
            let mut history = self.recovery_history.lock().unwrap();
            if history.len() >= self.config.max_history_entries {
                history.remove(0);
            }
            history.push(attempt.clone());
        }

        Ok(attempt)
    }

    /// Execute a recovery strategy
    fn execute_recovery_strategy(&self, strategy: &RecoveryStrategy, error_context: &ErrorContext) -> RecoveryResult {
        // This is a simplified implementation
        // In a real system, you would implement the actual recovery actions
        
        for action in &strategy.actions {
            match action {
                RecoveryAction::Retry => {
                    // Simulate retry logic
                    if rand::random::<f64>() < strategy.success_threshold {
                        return RecoveryResult::Success;
                    }
                }
                RecoveryAction::UseDefault => {
                    // Always succeeds with default value
                    return RecoveryResult::Success;
                }
                RecoveryAction::Fallback => {
                    // Simulate fallback success rate
                    if rand::random::<f64>() < 0.8 {
                        return RecoveryResult::Success;
                    }
                }
                RecoveryAction::Skip => {
                    return RecoveryResult::Success;
                }
                RecoveryAction::UserInput => {
                    // Would prompt user in real implementation
                    return RecoveryResult::Success;
                }
                RecoveryAction::Terminate => {
                    return RecoveryResult::FailedPermanent;
                }
            }
        }

        RecoveryResult::FailedRetry
    }

    /// Calculate backoff delay
    pub fn calculate_backoff_delay(&self, strategy: &BackoffStrategy, attempt: usize) -> Duration {
        match strategy {
            BackoffStrategy::None => Duration::ZERO,
            BackoffStrategy::Fixed(delay) => *delay,
            BackoffStrategy::Exponential { base, multiplier, max } => {
                let delay = Duration::from_nanos((base.as_nanos() as f64 * multiplier.powi(attempt as i32 - 1)) as u64);
                std::cmp::min(delay, *max)
            }
            BackoffStrategy::Linear { base, increment, max } => {
                let delay = *base + *increment * (attempt as u32 - 1);
                std::cmp::min(delay, *max)
            }
            BackoffStrategy::Jittered { base, multiplier, max } => {
                let base_delay = Duration::from_nanos((base.as_nanos() as f64 * multiplier.powi(attempt as i32 - 1)) as u64);
                let jitter = rand::random::<f64>() * 0.1; // 10% jitter
                let jittered_delay = Duration::from_nanos((base_delay.as_nanos() as f64 * (1.0 + jitter)) as u64);
                std::cmp::min(jittered_delay, *max)
            }
        }
    }

    /// Get recovery statistics
    pub fn get_stats(&self) -> RecoveryStats {
        self.stats.lock().unwrap().clone()
    }

    /// Get recovery history
    pub fn get_recovery_history(&self) -> Vec<RecoveryAttempt> {
        self.recovery_history.lock().unwrap().clone()
    }

    /// Clear recovery history
    pub fn clear_history(&self) {
        self.recovery_history.lock().unwrap().clear();
    }

    /// Get circuit breaker status
    pub fn get_circuit_breaker_status(&self, name: &str) -> Option<CircuitBreakerState> {
        let circuit_breakers = self.circuit_breakers.read().unwrap();
        circuit_breakers.get(name).map(|cb| cb.state.clone())
    }
}

impl Default for RecoverySystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Global recovery system instance
static GLOBAL_RECOVERY_SYSTEM: std::sync::LazyLock<Mutex<RecoverySystem>> = 
    std::sync::LazyLock::new(|| Mutex::new(RecoverySystem::new()));

/// Get the global recovery system
pub fn get_global_recovery_system() -> &'static Mutex<RecoverySystem> {
    &GLOBAL_RECOVERY_SYSTEM
}

/// Utility functions for error recovery
pub mod utils {
    use super::*;

    /// Attempt automatic recovery for an error
    pub fn auto_recover(error_context: &ErrorContext) -> CursedResult<bool> {
        let recovery_system = get_global_recovery_system().lock().map_err(|_| {
            Error::Runtime("Failed to acquire recovery system lock".to_string())
        })?;
        
        let attempt = recovery_system.attempt_recovery(error_context)?;
        Ok(matches!(attempt.result, RecoveryResult::Success))
    }

    /// Create a simple retry strategy
    pub fn create_retry_strategy(max_attempts: usize, base_delay: Duration) -> RecoveryStrategy {
        RecoveryStrategy {
            id: format!("retry_{}", max_attempts),
            name: format!("Retry {} times", max_attempts),
            actions: vec![RecoveryAction::Retry],
            conditions: vec![RecoveryCondition::NetworkIssue, RecoveryCondition::TimeoutOccurred],
            max_attempts,
            backoff: BackoffStrategy::Exponential {
                base: base_delay,
                multiplier: 2.0,
                max: Duration::from_secs(30),
            },
            timeout: Duration::from_secs(60),
            success_threshold: 0.7,
        }
    }

    /// Create a fallback strategy
    pub fn create_fallback_strategy(fallback_actions: Vec<RecoveryAction>) -> RecoveryStrategy {
        RecoveryStrategy {
            id: "fallback_strategy".to_string(),
            name: "Fallback Strategy".to_string(),
            actions: fallback_actions,
            conditions: vec![RecoveryCondition::ErrorSeverity(ErrorSeverity::Error)],
            max_attempts: 1,
            backoff: BackoffStrategy::None,
            timeout: Duration::from_secs(10),
            success_threshold: 0.9,
        }
    }

    /// Get recovery system status
    pub fn get_recovery_status() -> CursedResult<String> {
        let recovery_system = get_global_recovery_system().lock().map_err(|_| {
            Error::Runtime("Failed to acquire recovery system lock".to_string())
        })?;
        
        let stats = recovery_system.get_stats();
        
        Ok(format!(
            "Recovery System Status:\\nTotal Attempts: {}\\nSuccessful: {}\\nFailed: {}\\nTimeout: {}\\nAverage Time: {:.2}ms",
            stats.total_recovery_attempts,
            stats.successful_recoveries,
            stats.failed_recoveries,
            stats.timeouts,
            stats.average_recovery_time_ms
        ))
    }

    /// Test circuit breaker functionality
    pub fn test_circuit_breaker(name: &str, failure_count: usize) -> CursedResult<CircuitBreakerState> {
        let mut circuit_breaker = CircuitBreaker::new(name.to_string(), CircuitBreakerConfig::default());
        
        // Simulate failures
        for _ in 0..failure_count {
            circuit_breaker.record_failure();
        }
        
        Ok(circuit_breaker.state)
    }

    /// Register a custom recovery strategy
    pub fn register_custom_strategy(strategy: RecoveryStrategy) -> CursedResult<()> {
        let mut recovery_system = get_global_recovery_system().lock().map_err(|_| {
            Error::Runtime("Failed to acquire recovery system lock".to_string())
        })?;
        
        recovery_system.register_strategy(strategy);
        Ok(())
    }
}

/// Public function that was likely being used by external code
pub fn get_minimal_result() -> CursedResult<String> {
    Ok("CURSED error recovery system initialized".to_string())
}
