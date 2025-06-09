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
    pub base_error: DatabaseError,
    pub sql_state: Option<String>,
    pub error_code: Option<i32>,
    pub severity: ErrorSeverity,
    pub category: ErrorCategory,
    pub recovery_strategy: RecoveryStrategy,
    pub context: ErrorContext,
    pub occurred_at: Instant,
    pub retry_count: u32,
}

/// fr fr Error severity levels for handling prioritization
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    /// Low priority errors that don't affect functionality
    Info,
    /// Warnings that might indicate issues
    Warning,
    /// Errors that affect current operation but system remains stable
    Error,
    /// Critical errors that might affect system stability
    Critical,
    /// Fatal errors that require immediate attention
    Fatal,
}

/// fr fr Error categories for specific handling strategies
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorCategory {
    /// Connection-related errors
    ConnectionError {
        connection_id: Option<String>,
        is_timeout: bool,
        is_refused: bool,
    },
    /// SQL syntax or semantic errors
    SqlError {
        query: Option<String>,
        position: Option<u32>,
        constraint_name: Option<String>,
    },
    /// Transaction-related errors
    TransactionError {
        transaction_id: Option<String>,
        isolation_level: Option<String>,
        deadlock_detected: bool,
    },
    /// Data integrity violations
    IntegrityError {
        constraint_type: ConstraintType,
        constraint_name: Option<String>,
        column_name: Option<String>,
    },
    /// Authentication and authorization errors
    SecurityError {
        user: Option<String>,
        required_permission: Option<String>,
    },
    /// Resource exhaustion errors
    ResourceError {
        resource_type: ResourceType,
        current_usage: Option<u64>,
        limit: Option<u64>,
    },
    /// Configuration or setup errors
    ConfigurationError {
        parameter: Option<String>,
        expected_value: Option<String>,
    },
    /// Unknown or unclassified errors
    UnknownError,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintType {
    PrimaryKey,
    ForeignKey,
    Unique,
    NotNull,
    Check,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceType {
    Memory,
    DiskSpace,
    Connections,
    Locks,
    Buffers,
}

/// fr fr Recovery strategies for different error types
#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryStrategy {
    /// Retry the operation with exponential backoff
    Retry {
        max_attempts: u32,
        base_delay: Duration,
        max_delay: Duration,
    },
    /// Fallback to alternative approach
    Fallback {
        fallback_type: FallbackType,
    },
    /// Circuit breaker pattern
    CircuitBreaker {
        failure_threshold: u32,
        timeout: Duration,
    },
    /// Manual intervention required
    ManualIntervention {
        escalation_required: bool,
    },
    /// No recovery possible
    NoRecovery,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FallbackType {
    ReadOnlyMode,
    CachedData,
    AlternativeDataSource,
    DefaultValues,
}

/// fr fr Error context for detailed debugging information
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub operation_id: Option<String>,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub query: Option<String>,
    pub parameters: Vec<SqlValue>,
    pub connection_info: Option<ConnectionInfo>,
    pub stack_trace: Vec<String>,
    pub additional_data: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub connection_id: String,
    pub database_name: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub connection_time: Instant,
    pub last_activity: Instant,
}

/// fr fr SQL error analyzer for automatic categorization
#[derive(Debug)]
pub struct SqlErrorAnalyzer {
    error_patterns: HashMap<String, ErrorCategory>,
    sql_state_mappings: HashMap<String, ErrorCategory>,
    error_code_mappings: HashMap<i32, ErrorCategory>,
    vendor_specific_patterns: HashMap<String, Vec<ErrorPattern>>,
}

#[derive(Debug, Clone)]
pub struct ErrorPattern {
    pub pattern: String,
    pub category: ErrorCategory,
    pub severity: ErrorSeverity,
    pub recovery_strategy: RecoveryStrategy,
}

impl SqlErrorAnalyzer {
    /// slay Create new SQL error analyzer with comprehensive patterns
    #[instrument]
    pub fn new() -> Self {
        info!("Creating SQL error analyzer");
        
        let mut analyzer = Self {
            error_patterns: HashMap::new(),
            sql_state_mappings: HashMap::new(),
            error_code_mappings: HashMap::new(),
            vendor_specific_patterns: HashMap::new(),
        };
        
        analyzer.initialize_error_patterns();
        analyzer.initialize_sql_state_mappings();
        analyzer.initialize_vendor_patterns();
        
        analyzer
    }

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
            category = ?category,
            severity = ?severity,
            recovery = ?recovery_strategy,
            "Error analysis completed"
        );
        
        EnhancedSqlError {
            base_error: error,
            sql_state,
            error_code,
            severity,
            category,
            recovery_strategy,
            context,
            occurred_at: Instant::now(),
            retry_count: 0,
        }
    }

    /// lowkey Initialize common error patterns
    fn initialize_error_patterns(&mut self) {
        let patterns = vec![
            ("connection.*refused", ErrorCategory::ConnectionError {
                connection_id: None,
                is_timeout: false,
                is_refused: true,
            }),
            ("connection.*timeout", ErrorCategory::ConnectionError {
                connection_id: None,
                is_timeout: true,
                is_refused: false,
            }),
            ("deadlock.*detected", ErrorCategory::TransactionError {
                transaction_id: None,
                isolation_level: None,
                deadlock_detected: true,
            }),
            ("unique.*constraint", ErrorCategory::IntegrityError {
                constraint_type: ConstraintType::Unique,
                constraint_name: None,
                column_name: None,
            }),
            ("foreign.*key.*constraint", ErrorCategory::IntegrityError {
                constraint_type: ConstraintType::ForeignKey,
                constraint_name: None,
                column_name: None,
            }),
            ("syntax.*error", ErrorCategory::SqlError {
                query: None,
                position: None,
                constraint_name: None,
            }),
            ("permission.*denied", ErrorCategory::SecurityError {
                user: None,
                required_permission: None,
            }),
            ("out.*of.*memory", ErrorCategory::ResourceError {
                resource_type: ResourceType::Memory,
                current_usage: None,
                limit: None,
            }),
        ];
        
        for (pattern, category) in patterns {
            self.error_patterns.insert(pattern.to_string(), category);
        }
    }

    /// periodt Initialize SQL state code mappings
    fn initialize_sql_state_mappings(&mut self) {
        let mappings = vec![
            ("08001", ErrorCategory::ConnectionError {
                connection_id: None,
                is_timeout: false,
                is_refused: true,
            }),
            ("08006", ErrorCategory::ConnectionError {
                connection_id: None,
                is_timeout: false,
                is_refused: false,
            }),
            ("23000", ErrorCategory::IntegrityError {
                constraint_type: ConstraintType::Check,
                constraint_name: None,
                column_name: None,
            }),
            ("23001", ErrorCategory::IntegrityError {
                constraint_type: ConstraintType::NotNull,
                constraint_name: None,
                column_name: None,
            }),
            ("23505", ErrorCategory::IntegrityError {
                constraint_type: ConstraintType::Unique,
                constraint_name: None,
                column_name: None,
            }),
            ("23503", ErrorCategory::IntegrityError {
                constraint_type: ConstraintType::ForeignKey,
                constraint_name: None,
                column_name: None,
            }),
            ("40001", ErrorCategory::TransactionError {
                transaction_id: None,
                isolation_level: None,
                deadlock_detected: true,
            }),
            ("42601", ErrorCategory::SqlError {
                query: None,
                position: None,
                constraint_name: None,
            }),
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
                pattern: "remaining connection slots are reserved".to_string(),
                category: ErrorCategory::ResourceError {
                    resource_type: ResourceType::Connections,
                    current_usage: None,
                    limit: None,
                },
                severity: ErrorSeverity::Critical,
                recovery_strategy: RecoveryStrategy::CircuitBreaker {
                    failure_threshold: 3,
                    timeout: Duration::from_secs(30),
                },
            },
            ErrorPattern {
                pattern: "database.*does not exist".to_string(),
                category: ErrorCategory::ConfigurationError {
                    parameter: Some("database_name".to_string()),
                    expected_value: None,
                },
                severity: ErrorSeverity::Fatal,
                recovery_strategy: RecoveryStrategy::ManualIntervention {
                    escalation_required: true,
                },
            },
        ];
        
        self.vendor_specific_patterns.insert("postgresql".to_string(), postgres_patterns);
        
        // MySQL patterns
        let mysql_patterns = vec![
            ErrorPattern {
                pattern: "Too many connections".to_string(),
                category: ErrorCategory::ResourceError {
                    resource_type: ResourceType::Connections,
                    current_usage: None,
                    limit: None,
                },
                severity: ErrorSeverity::Critical,
                recovery_strategy: RecoveryStrategy::Retry {
                    max_attempts: 5,
                    base_delay: Duration::from_millis(500),
                    max_delay: Duration::from_secs(10),
                },
            },
        ];
        
        self.vendor_specific_patterns.insert("mysql".to_string(), mysql_patterns);
    }

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
    }

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
                connection_id: None,
                is_timeout: false,
                is_refused: false,
            },
            DatabaseErrorKind::QueryError => ErrorCategory::SqlError {
                query: None,
                position: None,
                constraint_name: None,
            },
            DatabaseErrorKind::TransactionError => ErrorCategory::TransactionError {
                transaction_id: None,
                isolation_level: None,
                deadlock_detected: false,
            },
            _ => ErrorCategory::UnknownError,
        }
    }

    /// facts Determine error severity
    fn determine_severity(&self, category: &ErrorCategory, error: &DatabaseError) -> ErrorSeverity {
        match category {
            ErrorCategory::ConnectionError { is_refused: true, .. } => ErrorSeverity::Critical,
            ErrorCategory::TransactionError { deadlock_detected: true, .. } => ErrorSeverity::Warning,
            ErrorCategory::IntegrityError { .. } => ErrorSeverity::Error,
            ErrorCategory::SecurityError { .. } => ErrorSeverity::Critical,
            ErrorCategory::ResourceError { resource_type: ResourceType::Memory, .. } => ErrorSeverity::Fatal,
            ErrorCategory::ConfigurationError { .. } => ErrorSeverity::Fatal,
            _ => ErrorSeverity::Error,
        }
    }

    /// periodt Determine recovery strategy
    fn determine_recovery_strategy(&self, category: &ErrorCategory, severity: &ErrorSeverity) -> RecoveryStrategy {
        match (category, severity) {
            (ErrorCategory::ConnectionError { is_timeout: true, .. }, _) => {
                RecoveryStrategy::Retry {
                    max_attempts: 3,
                    base_delay: Duration::from_millis(100),
                    max_delay: Duration::from_secs(5),
                }
            }
            (ErrorCategory::TransactionError { deadlock_detected: true, .. }, _) => {
                RecoveryStrategy::Retry {
                    max_attempts: 5,
                    base_delay: Duration::from_millis(50),
                    max_delay: Duration::from_secs(2),
                }
            }
            (ErrorCategory::ResourceError { resource_type: ResourceType::Connections, .. }, _) => {
                RecoveryStrategy::CircuitBreaker {
                    failure_threshold: 5,
                    timeout: Duration::from_secs(30),
                }
            }
            (_, ErrorSeverity::Fatal) => {
                RecoveryStrategy::ManualIntervention {
                    escalation_required: true,
                }
            }
            (ErrorCategory::IntegrityError { .. }, _) => {
                RecoveryStrategy::NoRecovery
            }
            _ => {
                RecoveryStrategy::Retry {
                    max_attempts: 1,
                    base_delay: Duration::from_millis(100),
                    max_delay: Duration::from_secs(1),
                }
            }
        }
    }
}

/// fr fr Circuit breaker for preventing cascade failures
#[derive(Debug)]
pub struct DatabaseCircuitBreaker {
    state: Arc<RwLock<CircuitBreakerState>>,
    config: CircuitBreakerConfig,
    failure_history: Arc<Mutex<VecDeque<Instant>>>,
    success_history: Arc<Mutex<VecDeque<Instant>>>,
}

#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub success_threshold: u32,
    pub timeout: Duration,
    pub monitoring_window: Duration,
    pub half_open_max_calls: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CircuitBreakerState {
    Closed,   // Normal operation
    Open,     // Blocking calls
    HalfOpen, // Testing if service recovered
}

impl DatabaseCircuitBreaker {
    /// slay Create new circuit breaker with configuration
    #[instrument]
    pub fn new(config: CircuitBreakerConfig) -> Self {
        info!(
            failure_threshold = config.failure_threshold,
            timeout = ?config.timeout,
            "Creating database circuit breaker"
        );
        
        Self {
            state: Arc::new(RwLock::new(CircuitBreakerState::Closed)),
            config,
            failure_history: Arc::new(Mutex::new(VecDeque::new())),
            success_history: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// facts Execute operation through circuit breaker
    #[instrument(skip(self, operation))]
    pub async fn execute<F, T>(&self, operation: F) -> Result<T, CircuitBreakerError>
    where
        F: FnOnce() -> Result<T, DatabaseError>,
        T: Send,
    {
        let current_state = {
            self.state.read()
                .map(|s| s.clone())
                .map_err(|_| CircuitBreakerError::InternalError)?
        };

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
    }

    /// lowkey Record successful operation
    async fn record_success(&self) -> Result<(), CircuitBreakerError> {
        trace!("Recording successful operation");
        
        let now = Instant::now();
        
        // Add to success history
        if let Ok(mut successes) = self.success_history.lock() {
            successes.push_back(now);
            self.cleanup_old_entries(&mut successes, now);
        }
        
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
    }

    /// periodt Record failed operation
    async fn record_failure(&self) -> Result<(), CircuitBreakerError> {
        trace!("Recording failed operation");
        
        let now = Instant::now();
        
        // Add to failure history
        if let Ok(mut failures) = self.failure_history.lock() {
            failures.push_back(now);
            self.cleanup_old_entries(&mut failures, now);
        }
        
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
    }

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
    }

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
    }

    /// highkey Transition to open state
    async fn transition_to_open(&self) -> Result<(), CircuitBreakerError> {
        warn!("Circuit breaker transitioning to OPEN state");
        
        if let Ok(mut state) = self.state.write() {
            *state = CircuitBreakerState::Open;
        }
        
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
    }

    /// facts Transition to closed state
    async fn transition_to_closed(&self) -> Result<(), CircuitBreakerError> {
        info!("Circuit breaker transitioning to CLOSED state");
        
        if let Ok(mut state) = self.state.write() {
            *state = CircuitBreakerState::Closed;
        }
        
        Ok(())
    }

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
            state,
            recent_failures,
            recent_successes,
            failure_threshold: self.config.failure_threshold,
            success_threshold: self.config.success_threshold,
            timeout: self.config.timeout,
        }
    }
}

/// fr fr Circuit breaker error types
#[derive(Debug)]
pub enum CircuitBreakerError {
    CircuitOpen,
    OperationFailed(DatabaseError),
    InternalError,
}

impl fmt::Display for CircuitBreakerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CircuitBreakerError::CircuitOpen => write!(f, "Circuit breaker is open"),
            CircuitBreakerError::OperationFailed(err) => write!(f, "Operation failed: {}", err),
            CircuitBreakerError::InternalError => write!(f, "Circuit breaker internal error"),
        }
    }
}

impl std::error::Error for CircuitBreakerError {}

/// fr fr Circuit breaker statistics
#[derive(Debug, Clone)]
pub struct CircuitBreakerStats {
    pub state: CircuitBreakerState,
    pub recent_failures: u32,
    pub recent_successes: u32,
    pub failure_threshold: u32,
    pub success_threshold: u32,
    pub timeout: Duration,
}

/// fr fr Error recovery coordinator for handling multiple recovery strategies
#[derive(Debug)]
pub struct ErrorRecoveryCoordinator {
    recovery_handlers: HashMap<ErrorCategory, Box<dyn RecoveryHandler>>,
    circuit_breakers: HashMap<String, Arc<DatabaseCircuitBreaker>>,
    error_analyzer: Arc<SqlErrorAnalyzer>,
}

/// fr fr Recovery handler trait for pluggable recovery strategies
pub trait RecoveryHandler: Send + Sync + std::fmt::Debug {
    fn can_handle(&self, error: &EnhancedSqlError) -> bool;
    fn recover(&self, error: &EnhancedSqlError) -> Result<RecoveryAction, DatabaseError>;
}

#[derive(Debug)]
pub enum RecoveryAction {
    Retry {
        delay: Duration,
        modified_operation: Option<String>,
    },
    Fallback {
        fallback_operation: String,
    },
    Escalate {
        escalation_level: EscalationLevel,
        message: String,
    },
    Abort,
}

#[derive(Debug, Clone)]
pub enum EscalationLevel {
    Warning,
    Alert,
    Critical,
    Emergency,
}

impl ErrorRecoveryCoordinator {
    /// slay Create new error recovery coordinator
    #[instrument]
    pub fn new() -> Self {
        info!("Creating error recovery coordinator");
        Self {
            recovery_handlers: HashMap::new(),
            circuit_breakers: HashMap::new(),
            error_analyzer: Arc::new(SqlErrorAnalyzer::new()),
        }
    }

    /// facts Handle database error with comprehensive recovery
    #[instrument(skip(self, error, context))]
    pub async fn handle_error(&self, error: DatabaseError, context: ErrorContext) -> Result<RecoveryAction, DatabaseError> {
        let enhanced_error = self.error_analyzer.analyze_error(error, context);
        
        info!(
            category = ?enhanced_error.category,
            severity = ?enhanced_error.severity,
            "Handling database error with recovery coordinator"
        );
        
        // Try circuit breaker if applicable
        if let Some(circuit_breaker) = self.get_circuit_breaker(&enhanced_error) {
            let stats = circuit_breaker.get_stats();
            if stats.state == CircuitBreakerState::Open {
                warn!("Circuit breaker is open, returning fallback action");
                return Ok(RecoveryAction::Fallback {
                    fallback_operation: "read_from_cache".to_string(),
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
        }
        
        // Default recovery based on strategy
        self.apply_default_recovery(&enhanced_error).await
    }

    /// lowkey Get appropriate circuit breaker
    fn get_circuit_breaker(&self, error: &EnhancedSqlError) -> Option<&Arc<DatabaseCircuitBreaker>> {
        match &error.category {
            ErrorCategory::ConnectionError { .. } => {
                self.circuit_breakers.get("connection")
            }
            ErrorCategory::ResourceError { .. } => {
                self.circuit_breakers.get("resource")
            }
            _ => None,
        }
    }

    /// periodt Apply default recovery strategy
    async fn apply_default_recovery(&self, error: &EnhancedSqlError) -> Result<RecoveryAction, DatabaseError> {
        match &error.recovery_strategy {
            RecoveryStrategy::Retry { max_attempts, base_delay, .. } => {
                if error.retry_count < *max_attempts {
                    let delay = *base_delay * 2_u32.pow(error.retry_count);
                    Ok(RecoveryAction::Retry {
                        delay,
                        modified_operation: None,
                    })
                } else {
                    Ok(RecoveryAction::Escalate {
                        escalation_level: EscalationLevel::Alert,
                        message: "Maximum retry attempts exceeded".to_string(),
                    })
                }
            }
            RecoveryStrategy::Fallback { fallback_type } => {
                let fallback_op = match fallback_type {
                    FallbackType::ReadOnlyMode => "enable_read_only_mode",
                    FallbackType::CachedData => "read_from_cache",
                    FallbackType::AlternativeDataSource => "use_secondary_database",
                    FallbackType::DefaultValues => "return_default_values",
                };
                
                Ok(RecoveryAction::Fallback {
                    fallback_operation: fallback_op.to_string(),
                })
            }
            RecoveryStrategy::ManualIntervention { escalation_required } => {
                let level = if *escalation_required {
                    EscalationLevel::Emergency
                } else {
                    EscalationLevel::Critical
                };
                
                Ok(RecoveryAction::Escalate {
                    escalation_level: level,
                    message: format!("Manual intervention required for: {}", error.base_error.message),
                })
            }
            RecoveryStrategy::NoRecovery => Ok(RecoveryAction::Abort),
            _ => Ok(RecoveryAction::Abort),
        }
    }
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 3,
            timeout: Duration::from_secs(60),
            monitoring_window: Duration::from_secs(300),
            half_open_max_calls: 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_error_analyzer() {
        let analyzer = SqlErrorAnalyzer::new();
        
        let error = DatabaseError::connection_error("Connection refused");
        let context = ErrorContext {
            operation_id: Some("op_123".to_string()),
            user_id: None,
            session_id: None,
            query: None,
            parameters: Vec::from([]),
            connection_info: None,
            stack_trace: Vec::from([]),
            additional_data: HashMap::new(),
        };
        
        let enhanced = analyzer.analyze_error(error, context);
        
        match enhanced.category {
            ErrorCategory::ConnectionError { is_refused: true, .. } => {
                assert_eq!(enhanced.severity, ErrorSeverity::Critical);
            }
            _ => panic!("Expected connection error category"),
        }
    }

    #[traced_test]
    #[tokio::test]
    async fn test_circuit_breaker() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            success_threshold: 2,
            timeout: Duration::from_millis(100),
            monitoring_window: Duration::from_secs(60),
            half_open_max_calls: 1,
        };
        
        let circuit_breaker = DatabaseCircuitBreaker::new(config);
        
        // Simulate failures to trigger circuit opening
        for _ in 0..3 {
            let result = circuit_breaker.execute(|| {
                Err(DatabaseError::connection_error("Simulated failure"))
            }).await;
            
            assert!(result.is_err());
        }
        
        // Circuit should be open now
        let result = circuit_breaker.execute(|| {
            Ok("success")
        }).await;
        
        match result {
            Err(CircuitBreakerError::CircuitOpen) => {
                // Expected - circuit is open
            }
            _ => panic!("Expected circuit to be open"),
        }
        
        let stats = circuit_breaker.get_stats();
        assert_eq!(stats.state, CircuitBreakerState::Open);
        assert_eq!(stats.recent_failures, 3);
    }

    #[traced_test]
    #[test]
    fn test_error_categorization() {
        let analyzer = SqlErrorAnalyzer::new();
        
        let deadlock_error = DatabaseError::transaction_error("deadlock detected");
        let context = ErrorContext {
            operation_id: None,
            user_id: None,
            session_id: None,
            query: None,
            parameters: Vec::from([]),
            connection_info: None,
            stack_trace: Vec::from([]),
            additional_data: HashMap::new(),
        };
        
        let enhanced = analyzer.analyze_error(deadlock_error, context);
        
        match enhanced.category {
            ErrorCategory::TransactionError { deadlock_detected: true, .. } => {
                assert_eq!(enhanced.severity, ErrorSeverity::Warning);
                match enhanced.recovery_strategy {
                    RecoveryStrategy::Retry { max_attempts: 5, .. } => {
                        // Expected
                    }
                    _ => panic!("Expected retry strategy for deadlock"),
                }
            }
            _ => panic!("Expected transaction error category"),
        }
    }

    #[traced_test]
    #[tokio::test]
    async fn test_recovery_coordinator() {
        let coordinator = ErrorRecoveryCoordinator::new();
        
        let error = DatabaseError::connection_error("Connection timeout");
        let context = ErrorContext {
            operation_id: Some("test_op".to_string()),
            user_id: None,
            session_id: None,
            query: Some("SELECT * FROM users".to_string()),
            parameters: Vec::from([]),
            connection_info: None,
            stack_trace: Vec::from([]),
            additional_data: HashMap::new(),
        };
        
        let recovery_action = coordinator.handle_error(error, context).await
            .expect("Should handle error");
        
        match recovery_action {
            RecoveryAction::Retry { delay, .. } => {
                assert!(delay > Duration::ZERO);
            }
            _ => panic!("Expected retry action for connection timeout"),
        }
    }
}
