/// fr fr Enhanced error handling with detailed SQL error categorization and recovery strategies
/// This module provides comprehensive error handling with circuit breaker patterns and retry policies

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use std::fmt;
use tracing::{instrument, debug, info, warn, error, trace};

use super::{DatabaseError, DatabaseErrorKind, SqlValue};

/// fr fr Enhanced SQL error with detailed categorization
#[derive(Debug, Clone)]
pub struct EnhancedSqlError {
/// fr fr CursedError severity levels for handling prioritization
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    /// Low priority errors that don't affect functionality
    /// Warnings that might indicate issues
    /// Errors that affect current operation but system remains stable
    /// Critical errors that might affect system stability
    /// Fatal errors that require immediate attention
/// fr fr CursedError categories for specific handling strategies
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorCategory {
    /// Connection-related errors
    ConnectionError {
    /// SQL syntax or semantic errors
    SqlError {
    /// Transaction-related errors
    TransactionError {
    /// Data integrity violations
    IntegrityError {
    /// Authentication and authorization errors
    SecurityError {
    /// Resource exhaustion errors
    ResourceError {
    /// Configuration or setup errors
    ConfigurationError {
    /// Unknown or unclassified errors
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintType {
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceType {
/// fr fr Recovery strategies for different error types
#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryStrategy {
    /// Retry the operation with exponential backoff
    Retry {
    /// Fallback to alternative approach
    Fallback {
    /// Circuit breaker pattern
    CircuitBreaker {
    /// Manual intervention required
    ManualIntervention {
    /// No recovery possible
#[derive(Debug, Clone, PartialEq)]
pub enum FallbackType {
/// fr fr CursedError context for detailed debugging information
#[derive(Debug, Clone)]
pub struct ErrorContext {
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
/// fr fr SQL error analyzer for automatic categorization
#[derive(Debug)]
pub struct SqlErrorAnalyzer {
#[derive(Debug, Clone)]
pub struct ErrorPattern {
impl SqlErrorAnalyzer {
    /// slay Create new SQL error analyzer with comprehensive patterns
    #[instrument]
    pub fn new() -> Self {
        info!("Creating SQL error analyzer");
        
        let mut analyzer = Self {
        
        analyzer.initialize_error_patterns();
        analyzer.initialize_sql_state_mappings();
        analyzer.initialize_vendor_patterns();
        
        analyzer
    /// facts Analyze database error and create enhanced error
    #[instrument(skip(self))]
    pub fn analyze_error(&self, error: DatabaseError, context: ErrorContext) -> EnhancedSqlError {
        debug!(error = ?error, "Analyzing database error");
        
        let sql_state = self.extract_sql_state(&error);
        let error_code = self.extract_error_code(&error);
        
        let category = self.categorize_error(&error, &sql_state, error_code);
        let severity = self.determine_severity(&category, &error);
        let recovery_strategy = self.determine_recovery_strategy(&category, &severity);
        
        debug!(
            "CursedError analysis completed"
        );
        
        EnhancedSqlError {
        }
    }

    /// lowkey Initialize common error patterns
    fn initialize_error_patterns(&mut self) {
        let patterns = vec![
            ("connection.*refused", ErrorCategory::ConnectionError {
            ("connection.*timeout", ErrorCategory::ConnectionError {
            ("deadlock.*detected", ErrorCategory::TransactionError {
            ("unique.*constraint", ErrorCategory::IntegrityError {
            ("foreign.*key.*constraint", ErrorCategory::IntegrityError {
            ("syntax.*error", ErrorCategory::SqlError {
            ("permission.*denied", ErrorCategory::SecurityError {
            ("out.*of.*memory", ErrorCategory::ResourceError {
        ];
        
        for (pattern, category) in patterns {
            self.error_patterns.insert(pattern.to_string(), category);
        }
    }

    /// periodt Initialize SQL state code mappings
    fn initialize_sql_state_mappings(&mut self) {
        let mappings = vec![
            ("08001", ErrorCategory::ConnectionError {
            ("08006", ErrorCategory::ConnectionError {
            ("23000", ErrorCategory::IntegrityError {
            ("23001", ErrorCategory::IntegrityError {
            ("23505", ErrorCategory::IntegrityError {
            ("23503", ErrorCategory::IntegrityError {
            ("40001", ErrorCategory::TransactionError {
            ("42601", ErrorCategory::SqlError {
        ];
        
        for (sql_state, category) in mappings {
            self.sql_state_mappings.insert(sql_state.to_string(), category);
        }
    }

    /// bestie Initialize vendor-specific error patterns
    fn initialize_vendor_patterns(&mut self) {
        // PostgreSQL patterns
        let postgres_patterns = vec![
            ErrorPattern {
                category: ErrorCategory::ResourceError {
                recovery_strategy: RecoveryStrategy::CircuitBreaker {
            ErrorPattern {
                category: ErrorCategory::ConfigurationError {
                recovery_strategy: RecoveryStrategy::ManualIntervention {
        ];
        
        self.vendor_specific_patterns.insert("postgresql".to_string(), postgres_patterns);
        
        // MySQL patterns
        let mysql_patterns = vec![
            ErrorPattern {
                category: ErrorCategory::ResourceError {
                recovery_strategy: RecoveryStrategy::Retry {
        ];
        
        self.vendor_specific_patterns.insert("mysql".to_string(), mysql_patterns);
    /// yolo Extract SQL state from error
    fn extract_sql_state(&self, error: &DatabaseError) -> Option<String> {
        // Extract SQL state from error message or metadata
        // This would be more sophisticated in a real implementation
        if error.message.contains("23505") {
            Some("23505".to_string())
        } else if error.message.contains("deadlock") {
            Some("40001".to_string())
        } else {
            None
        }
    }

    /// slay Extract error code from error
    fn extract_error_code(&self, error: &DatabaseError) -> Option<i32> {
        // Extract numeric error code from error
        // Implementation would depend on database driver
        None
    /// highkey Categorize error based on patterns and codes
    fn categorize_error(&self, error: &DatabaseError, sql_state: &Option<String>, error_code: Option<i32>) -> ErrorCategory {
        // Check SQL state first
        if let Some(state) = sql_state {
            if let Some(category) = self.sql_state_mappings.get(state) {
                return category.clone();
            }
        }
        
        // Check error code
        if let Some(code) = error_code {
            if let Some(category) = self.error_code_mappings.get(&code) {
                return category.clone();
            }
        }
        
        // Check error message patterns
        let message_lower = error.message.to_lowercase();
        for (pattern, category) in &self.error_patterns {
            if message_lower.contains(pattern) {
                return category.clone();
            }
        }
        
        // Check base error kind
        match error.kind {
            DatabaseErrorKind::ConnectionError => ErrorCategory::ConnectionError {
            DatabaseErrorKind::QueryError => ErrorCategory::SqlError {
            DatabaseErrorKind::TransactionError => ErrorCategory::TransactionError {
        }
    }

    /// facts Determine error severity
    fn determine_severity(&self, category: &ErrorCategory, error: &DatabaseError) -> ErrorSeverity {
        match category {
        }
    }

    /// periodt Determine recovery strategy
    fn determine_recovery_strategy(&self, category: &ErrorCategory, severity: &ErrorSeverity) -> RecoveryStrategy {
        match (category, severity) {
            (ErrorCategory::ConnectionError { is_timeout: true, .. }, _) => {
                RecoveryStrategy::Retry {
                }
            }
            (ErrorCategory::TransactionError { deadlock_detected: true, .. }, _) => {
                RecoveryStrategy::Retry {
                }
            }
            (ErrorCategory::ResourceError { resource_type: ResourceType::Connections, .. }, _) => {
                RecoveryStrategy::CircuitBreaker {
                }
            }
            (_, ErrorSeverity::Fatal) => {
                RecoveryStrategy::ManualIntervention {
                }
            }
            (ErrorCategory::IntegrityError { .. }, _) => {
                RecoveryStrategy::NoRecovery
            }
            _ => {
                RecoveryStrategy::Retry {
                }
            }
        }
    }
/// fr fr Circuit breaker for preventing cascade failures
#[derive(Debug)]
pub struct DatabaseCircuitBreaker {
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
#[derive(Debug, Clone, PartialEq)]
pub enum CircuitBreakerState {
    Closed,   // Normal operation
    Open,     // Blocking calls
    HalfOpen, // Testing if service recovered
impl DatabaseCircuitBreaker {
    /// slay Create new circuit breaker with configuration
    #[instrument]
    pub fn new(config: CircuitBreakerConfig) -> Self {
        info!(
            "Creating database circuit breaker"
        );
        
        Self {
        }
    }

    /// facts Execute operation through circuit breaker
    #[instrument(skip(self, operation))]
    pub async fn execute<F, T>(&self, operation: F) -> crate::error::Result<()>
    where
    {
        let current_state = {
            self.state.read()
                .map(|s| s.clone())
                .map_err(|_| CircuitBreakerError::InternalError)?

        match current_state {
            CircuitBreakerState::Open => {
                debug!("Circuit breaker is open, rejecting call");
                Err(CircuitBreakerError::CircuitOpen)
            }
            CircuitBreakerState::Closed | CircuitBreakerState::HalfOpen => {
                match operation() {
                    Ok(result) => {
                        self.record_success().await?;
                        Ok(result)
                    }
                    Err(error) => {
                        self.record_failure().await?;
                        Err(CircuitBreakerError::OperationFailed(error))
                    }
                }
            }
        }
    /// lowkey Record successful operation
    async fn record_success(&self) -> crate::error::Result<()> {
        trace!("Recording successful operation");
        
        let now = Instant::now();
        
        // Add to success history
        if let Ok(mut successes) = self.success_history.lock() {
            successes.push_back(now);
            self.cleanup_old_entries(&mut successes, now);
        // Check if we should transition from HalfOpen to Closed
        let current_state = self.state.read()
            .map(|s| s.clone())
            .map_err(|_| CircuitBreakerError::InternalError)?;
        
        if current_state == CircuitBreakerState::HalfOpen {
            let recent_successes = self.count_recent_successes(now);
            if recent_successes >= self.config.success_threshold {
                self.transition_to_closed().await?;
            }
        }
        
        Ok(())
    /// periodt Record failed operation
    async fn record_failure(&self) -> crate::error::Result<()> {
        trace!("Recording failed operation");
        
        let now = Instant::now();
        
        // Add to failure history
        if let Ok(mut failures) = self.failure_history.lock() {
            failures.push_back(now);
            self.cleanup_old_entries(&mut failures, now);
        // Check if we should open the circuit
        let current_state = self.state.read()
            .map(|s| s.clone())
            .map_err(|_| CircuitBreakerError::InternalError)?;
        
        if current_state == CircuitBreakerState::Closed {
            let recent_failures = self.count_recent_failures(now);
            if recent_failures >= self.config.failure_threshold {
                self.transition_to_open().await?;
            }
        }
        
        Ok(())
    /// bestie Clean up old entries outside monitoring window
    fn cleanup_old_entries(&self, entries: &mut VecDeque<Instant>, now: Instant) {
        let cutoff = now.saturating_duration_since(self.config.monitoring_window);
        while let Some(&front) = entries.front() {
            if front < cutoff {
                entries.pop_front();
            } else {
                break;
            }
        }
    /// yolo Count recent failures
    fn count_recent_failures(&self, now: Instant) -> u32 {
        self.failure_history.lock()
            .map(|failures| {
                let cutoff = now.saturating_duration_since(self.config.monitoring_window);
                failures.iter()
                    .filter(|&&time| time >= cutoff)
                    .count() as u32
            })
            .unwrap_or(0)
    /// slay Count recent successes
    fn count_recent_successes(&self, now: Instant) -> u32 {
        self.success_history.lock()
            .map(|successes| {
                let cutoff = now.saturating_duration_since(self.config.monitoring_window);
                successes.iter()
                    .filter(|&&time| time >= cutoff)
                    .count() as u32
            })
            .unwrap_or(0)
    /// highkey Transition to open state
    async fn transition_to_open(&self) -> crate::error::Result<()> {
        warn!("Circuit breaker transitioning to OPEN state");
        
        if let Ok(mut state) = self.state.write() {
            *state = CircuitBreakerState::Open;
        // Schedule transition to half-open
        let state_clone = Arc::clone(&self.state);
        let timeout = self.config.timeout;
        
        tokio::spawn(async move {
            tokio::time::sleep(timeout).await;
            
            if let Ok(mut state) = state_clone.write() {
                if *state == CircuitBreakerState::Open {
                    info!("Circuit breaker transitioning to HALF_OPEN state");
                    *state = CircuitBreakerState::HalfOpen;
                }
            }
        });
        
        Ok(())
    /// facts Transition to closed state
    async fn transition_to_closed(&self) -> crate::error::Result<()> {
        info!("Circuit breaker transitioning to CLOSED state");
        
        if let Ok(mut state) = self.state.write() {
            *state = CircuitBreakerState::Closed;
        Ok(())
    /// periodt Get circuit breaker statistics
    #[instrument(skip(self))]
    pub fn get_stats(&self) -> CircuitBreakerStats {
        let state = self.state.read()
            .map(|s| s.clone())
            .unwrap_or(CircuitBreakerState::Closed);
        
        let now = Instant::now();
        let recent_failures = self.count_recent_failures(now);
        let recent_successes = self.count_recent_successes(now);
        
        CircuitBreakerStats {
        }
    }
/// fr fr Circuit breaker error types
#[derive(Debug)]
pub enum CircuitBreakerError {
// impl fmt::Display for CircuitBreakerError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             CircuitBreakerError::CircuitOpen => write!(f, "Circuit breaker is open"),
//             CircuitBreakerError::OperationFailed(err) => write!(f, "Operation failed: {}", err),
//             CircuitBreakerError::InternalError => write!(f, "Circuit breaker internal error"),
//         }
//     }
// }

// impl std::error::CursedError for CircuitBreakerError {}
// 
/// fr fr Circuit breaker statistics
#[derive(Debug, Clone)]
pub struct CircuitBreakerStats {
/// fr fr CursedError recovery coordinator for handling multiple recovery strategies
#[derive(Debug)]
pub struct ErrorRecoveryCoordinator {
/// fr fr Recovery handler trait for pluggable recovery strategies
pub trait RecoveryHandler: Send + Sync + std::fmt::Debug {
    fn can_handle(&self, error: &EnhancedSqlError) -> bool;
    fn recover(&self, error: &EnhancedSqlError) -> crate::error::Result<()>;
#[derive(Debug)]
pub enum RecoveryAction {
    Retry {
    Fallback {
    Escalate {
#[derive(Debug, Clone)]
pub enum EscalationLevel {
impl ErrorRecoveryCoordinator {
    /// slay Create new error recovery coordinator
    #[instrument]
    pub fn new() -> Self {
        info!("Creating error recovery coordinator");
        Self {
        }
    }

    /// facts Handle database error with comprehensive recovery
    #[instrument(skip(self, error, context))]
    pub async fn handle_error(&self, error: DatabaseError, context: ErrorContext) -> crate::error::Result<()> {
        let enhanced_error = self.error_analyzer.analyze_error(error, context);
        
        info!(
            "Handling database error with recovery coordinator"
        );
        
        // Try circuit breaker if applicable
        if let Some(circuit_breaker) = self.get_circuit_breaker(&enhanced_error) {
            let stats = circuit_breaker.get_stats();
            if stats.state == CircuitBreakerState::Open {
                warn!("Circuit breaker is open, returning fallback action");
                return Ok(RecoveryAction::Fallback {
                });
            }
        }
        
        // Try recovery handlers
        for (category, handler) in &self.recovery_handlers {
            if std::mem::discriminant(category) == std::mem::discriminant(&enhanced_error.category) {
                if handler.can_handle(&enhanced_error) {
                    debug!(category = ?category, "Found matching recovery handler");
                    return handler.recover(&enhanced_error);
                }
            }
        // Default recovery based on strategy
        self.apply_default_recovery(&enhanced_error).await
    /// lowkey Get appropriate circuit breaker
    fn get_circuit_breaker(&self, error: &EnhancedSqlError) -> Option<&Arc<DatabaseCircuitBreaker>> {
        match &error.category {
            ErrorCategory::ConnectionError { .. } => {
                self.circuit_breakers.get("connection")
            }
            ErrorCategory::ResourceError { .. } => {
                self.circuit_breakers.get("resource")
            }
        }
    }

    /// periodt Apply default recovery strategy
    async fn apply_default_recovery(&self, error: &EnhancedSqlError) -> crate::error::Result<()> {
        match &error.recovery_strategy {
            RecoveryStrategy::Retry { max_attempts, base_delay, .. } => {
                if error.retry_count < *max_attempts {
                    let delay = *base_delay * 2_u32.pow(error.retry_count);
                    Ok(RecoveryAction::Retry {
                    })
                } else {
                    Ok(RecoveryAction::Escalate {
                    })
                }
            }
            RecoveryStrategy::Fallback { fallback_type } => {
                let fallback_op = match fallback_type {
                
                Ok(RecoveryAction::Fallback {
                })
            }
            RecoveryStrategy::ManualIntervention { escalation_required } => {
                let level = if *escalation_required {
                    EscalationLevel::Emergency
                } else {
                    EscalationLevel::Critical
                
                Ok(RecoveryAction::Escalate {
                })
            }
        }
    }
impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
        }
    }
