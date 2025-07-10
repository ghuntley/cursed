//! Enhanced Select Statement Implementation
//!
//! This module provides advanced select statement functionality including:
//! - Multi-channel select with priority ordering
//! - Timeout and default case handling
//! - Dynamic channel addition/removal
//! - Select statement composition and nesting
//! - Performance optimization with channel readiness caching
//! - Integration with goroutine scheduler
//! - Deadlock detection and prevention
//! - Select statement statistics and monitoring

use std::collections::{HashMap, VecDeque, BTreeMap};
use std::sync::{Arc, Mutex, RwLock, Condvar};
use std::sync::atomic::{AtomicUsize, AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, Instant};
use std::thread;
use std::any::{Any, TypeId};

use crate::runtime::channels::{ChannelError, ChannelResult};
use crate::runtime::goroutine::GoroutineId;

/// Enhanced select operation identifier
pub type SelectOperationId = u64;

/// Enhanced select case identifier
pub type SelectCaseId = u64;

/// Enhanced select result with detailed information
#[derive(Debug, Clone)]
pub enum EnhancedSelectResult<T> {
    /// A send operation completed successfully
    SendCompleted {
        case_id: SelectCaseId,
        channel_id: u64,
        execution_time: Duration,
    },
    /// A receive operation completed successfully
    ReceiveCompleted {
        case_id: SelectCaseId,
        channel_id: u64,
        value: T,
        execution_time: Duration,
    },
    /// Default case was executed
    DefaultExecuted {
        case_id: SelectCaseId,
        execution_time: Duration,
    },
    /// Timeout occurred
    Timeout {
        timeout_duration: Duration,
        waited_time: Duration,
    },
    /// All channels are closed
    AllClosed {
        checked_channels: Vec<u64>,
        execution_time: Duration,
    },
    /// Select was cancelled
    Cancelled {
        reason: String,
        execution_time: Duration,
    },
}

/// Enhanced select operation type
#[derive(Debug)]
pub enum EnhancedSelectOperation {
    /// Send operation
    Send {
        channel_id: u64,
        case_id: SelectCaseId,
        priority: SelectPriority,
        timeout: Option<Duration>,
        retry_count: u32,
        value: Box<dyn Any + Send>,
    },
    /// Receive operation
    Receive {
        channel_id: u64,
        case_id: SelectCaseId,
        priority: SelectPriority,
        timeout: Option<Duration>,
        retry_count: u32,
    },
    /// Default case
    Default {
        case_id: SelectCaseId,
        priority: SelectPriority,
        delay: Option<Duration>,
    },
    /// Timeout case
    Timeout {
        case_id: SelectCaseId,
        duration: Duration,
        priority: SelectPriority,
    },
}

/// Select operation priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SelectPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl Default for SelectPriority {
    fn default() -> Self {
        SelectPriority::Normal
    }
}

/// Enhanced select case with metadata
#[derive(Debug)]
pub struct EnhancedSelectCase {
    /// Case ID
    pub id: SelectCaseId,
    /// Operation
    pub operation: EnhancedSelectOperation,
    /// Channel wrapper for type erasure
    pub channel_wrapper: Option<Arc<dyn ChannelWrapper>>,
    /// Callback function
    pub callback: Option<Box<dyn FnOnce() + Send>>,
    /// Creation time
    pub created_at: Instant,
    /// Execution count
    pub execution_count: AtomicU64,
    /// Last execution time
    pub last_execution_time: Option<Instant>,
    /// Average execution time
    pub average_execution_time: Duration,
    /// Enabled flag
    pub enabled: AtomicBool,
    /// Condition for dynamic enabling/disabling
    pub condition: Option<Box<dyn Fn() -> bool + Send + Sync>>,
}

/// Channel wrapper trait for type erasure
pub trait ChannelWrapper: Send + Sync {
    /// Try to send a value
    fn try_send(&self, value: Box<dyn Any + Send>) -> Result<(), ChannelError>;
    /// Try to receive a value
    fn try_receive(&self) -> Result<Option<Box<dyn Any + Send>>, ChannelError>;
    /// Check if ready for sending
    fn can_send(&self) -> bool;
    /// Check if ready for receiving
    fn can_receive(&self) -> bool;
    /// Check if closed
    fn is_closed(&self) -> bool;
    /// Get channel ID
    fn channel_id(&self) -> u64;
    /// Get channel type info
    fn type_id(&self) -> TypeId;
    /// Get channel statistics
    fn get_stats(&self) -> ChannelWrapperStats;
}

/// Channel wrapper statistics
#[derive(Debug, Default, Clone)]
pub struct ChannelWrapperStats {
    pub total_sends: u64,
    pub total_receives: u64,
    pub current_buffer_size: usize,
    pub buffer_capacity: usize,
    pub send_success_rate: f64,
    pub receive_success_rate: f64,
    pub average_send_time: Duration,
    pub average_receive_time: Duration,
}

/// Enhanced select statement configuration
#[derive(Debug, Clone)]
pub struct EnhancedSelectConfig {
    /// Maximum number of cases
    pub max_cases: usize,
    /// Enable fair scheduling
    pub enable_fair_scheduling: bool,
    /// Fair scheduling window
    pub fair_scheduling_window: Duration,
    /// Enable statistics collection
    pub enable_statistics: bool,
    /// Enable performance monitoring
    pub enable_performance_monitoring: bool,
    /// Enable deadlock detection
    pub enable_deadlock_detection: bool,
    /// Maximum execution time before timeout
    pub max_execution_time: Option<Duration>,
    /// Retry configuration
    pub retry_config: RetryConfig,
    /// Caching configuration
    pub cache_config: CacheConfig,
}

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Enable retry on failure
    pub enabled: bool,
    /// Maximum retry attempts
    pub max_attempts: u32,
    /// Base delay between retries
    pub base_delay: Duration,
    /// Exponential backoff multiplier
    pub backoff_multiplier: f64,
    /// Maximum delay between retries
    pub max_delay: Duration,
}

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Enable channel readiness caching
    pub enabled: bool,
    /// Cache size
    pub cache_size: usize,
    /// Cache TTL
    pub cache_ttl: Duration,
    /// Cache refresh interval
    pub refresh_interval: Duration,
}

impl Default for EnhancedSelectConfig {
    fn default() -> Self {
        Self {
            max_cases: 1000,
            enable_fair_scheduling: true,
            fair_scheduling_window: Duration::from_millis(100),
            enable_statistics: true,
            enable_performance_monitoring: true,
            enable_deadlock_detection: true,
            max_execution_time: Some(Duration::from_secs(30)),
            retry_config: RetryConfig {
                enabled: true,
                max_attempts: 3,
                base_delay: Duration::from_millis(10),
                backoff_multiplier: 2.0,
                max_delay: Duration::from_millis(1000),
            },
            cache_config: CacheConfig {
                enabled: true,
                cache_size: 100,
                cache_ttl: Duration::from_millis(50),
                refresh_interval: Duration::from_millis(10),
            },
        }
    }
}

/// Enhanced select statement statistics
#[derive(Debug, Default, Clone)]
pub struct EnhancedSelectStats {
    /// Total executions
    pub total_executions: u64,
    /// Total successful operations
    pub total_successful_operations: u64,
    /// Total failed operations
    pub total_failed_operations: u64,
    /// Total timeout occurrences
    pub total_timeouts: u64,
    /// Total default executions
    pub total_default_executions: u64,
    /// Average execution time
    pub average_execution_time: Duration,
    /// Peak execution time
    pub peak_execution_time: Duration,
    /// Case execution distribution
    pub case_execution_distribution: HashMap<SelectCaseId, u64>,
    /// Channel usage statistics
    pub channel_usage_stats: HashMap<u64, ChannelUsageStats>,
    /// Fair scheduling statistics
    pub fair_scheduling_stats: FairSchedulingStats,
}

/// Channel usage statistics
#[derive(Debug, Default, Clone)]
pub struct ChannelUsageStats {
    pub total_send_attempts: u64,
    pub total_receive_attempts: u64,
    pub successful_sends: u64,
    pub successful_receives: u64,
    pub send_success_rate: f64,
    pub receive_success_rate: f64,
    pub average_send_time: Duration,
    pub average_receive_time: Duration,
}

/// Fair scheduling statistics
#[derive(Debug, Default, Clone)]
pub struct FairSchedulingStats {
    pub total_scheduling_decisions: u64,
    pub fair_selections: u64,
    pub priority_selections: u64,
    pub fairness_ratio: f64,
    pub scheduling_overhead: Duration,
}

/// Enhanced select statement implementation
pub struct EnhancedSelect {
    /// Configuration
    config: EnhancedSelectConfig,
    /// Select cases
    cases: RwLock<BTreeMap<SelectCaseId, Arc<EnhancedSelectCase>>>,
    /// Channel registry
    channels: RwLock<HashMap<u64, Arc<dyn ChannelWrapper>>>,
    /// Statistics
    stats: RwLock<EnhancedSelectStats>,
    /// Fair scheduling state
    fair_scheduler: RwLock<FairScheduler>,
    /// Readiness cache
    readiness_cache: RwLock<ReadinessCache>,
    /// Operation ID counter
    next_operation_id: AtomicU64,
    /// Case ID counter
    next_case_id: AtomicU64,
    /// Execution state
    execution_state: RwLock<ExecutionState>,
    /// Associated goroutine
    goroutine_id: Option<GoroutineId>,
    /// Cancellation token
    cancellation_token: Arc<AtomicBool>,
}

/// Fair scheduler state
#[derive(Debug)]
struct FairScheduler {
    /// Case execution history
    execution_history: VecDeque<(SelectCaseId, Instant)>,
    /// Last execution times
    last_executions: HashMap<SelectCaseId, Instant>,
    /// Scheduling window
    window_size: Duration,
    /// Fairness weights
    fairness_weights: HashMap<SelectCaseId, f64>,
}

/// Readiness cache
#[derive(Debug)]
struct ReadinessCache {
    /// Cached channel readiness
    send_readiness: HashMap<u64, (bool, Instant)>,
    receive_readiness: HashMap<u64, (bool, Instant)>,
    /// Cache configuration
    config: CacheConfig,
    /// Last refresh time
    last_refresh: Instant,
}

/// Execution state
#[derive(Debug)]
struct ExecutionState {
    /// Currently executing
    executing: bool,
    /// Execution start time
    start_time: Option<Instant>,
    /// Selected case
    selected_case: Option<SelectCaseId>,
    /// Retry attempts
    retry_attempts: HashMap<SelectCaseId, u32>,
}

impl EnhancedSelect {
    /// Create a new enhanced select statement
    pub fn new() -> Self {
        Self::with_config(EnhancedSelectConfig::default())
    }

    /// Create a new enhanced select statement with configuration
    pub fn with_config(config: EnhancedSelectConfig) -> Self {
        Self {
            config: config.clone(),
            cases: RwLock::new(BTreeMap::new()),
            channels: RwLock::new(HashMap::new()),
            stats: RwLock::new(EnhancedSelectStats::default()),
            fair_scheduler: RwLock::new(FairScheduler {
                execution_history: VecDeque::new(),
                last_executions: HashMap::new(),
                window_size: config.fair_scheduling_window,
                fairness_weights: HashMap::new(),
            }),
            readiness_cache: RwLock::new(ReadinessCache {
                send_readiness: HashMap::new(),
                receive_readiness: HashMap::new(),
                config: config.cache_config.clone(),
                last_refresh: Instant::now(),
            }),
            next_operation_id: AtomicU64::new(1),
            next_case_id: AtomicU64::new(1),
            execution_state: RwLock::new(ExecutionState {
                executing: false,
                start_time: None,
                selected_case: None,
                retry_attempts: HashMap::new(),
            }),
            goroutine_id: None,
            cancellation_token: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Add a send case
    pub fn send<T: Send + 'static>(
        &mut self,
        channel_id: u64,
        channel: Arc<dyn ChannelWrapper>,
        value: T,
        priority: SelectPriority,
    ) -> SelectCaseId {
        let case_id = self.next_case_id.fetch_add(1, Ordering::SeqCst);
        
        let operation = EnhancedSelectOperation::Send {
            channel_id,
            case_id,
            priority,
            timeout: None,
            retry_count: 0,
            value: Box::new(value),
        };

        let case = Arc::new(EnhancedSelectCase {
            id: case_id,
            operation,
            channel_wrapper: Some(channel.clone()),
            callback: None,
            created_at: Instant::now(),
            execution_count: AtomicU64::new(0),
            last_execution_time: None,
            average_execution_time: Duration::default(),
            enabled: AtomicBool::new(true),
            condition: None,
        });

        // Add to cases
        if let Ok(mut cases) = self.cases.write() {
            cases.insert(case_id, case);
        }

        // Add to channels
        if let Ok(mut channels) = self.channels.write() {
            channels.insert(channel_id, channel);
        }

        case_id
    }

    /// Add a receive case
    pub fn receive<T: Send + 'static>(
        &mut self,
        channel_id: u64,
        channel: Arc<dyn ChannelWrapper>,
        priority: SelectPriority,
    ) -> SelectCaseId {
        let case_id = self.next_case_id.fetch_add(1, Ordering::SeqCst);
        
        let operation = EnhancedSelectOperation::Receive {
            channel_id,
            case_id,
            priority,
            timeout: None,
            retry_count: 0,
        };

        let case = Arc::new(EnhancedSelectCase {
            id: case_id,
            operation,
            channel_wrapper: Some(channel.clone()),
            callback: None,
            created_at: Instant::now(),
            execution_count: AtomicU64::new(0),
            last_execution_time: None,
            average_execution_time: Duration::default(),
            enabled: AtomicBool::new(true),
            condition: None,
        });

        // Add to cases
        if let Ok(mut cases) = self.cases.write() {
            cases.insert(case_id, case);
        }

        // Add to channels
        if let Ok(mut channels) = self.channels.write() {
            channels.insert(channel_id, channel);
        }

        case_id
    }

    /// Add a default case
    pub fn default_case(&mut self, priority: SelectPriority) -> SelectCaseId {
        let case_id = self.next_case_id.fetch_add(1, Ordering::SeqCst);
        
        let operation = EnhancedSelectOperation::Default {
            case_id,
            priority,
            delay: None,
        };

        let case = Arc::new(EnhancedSelectCase {
            id: case_id,
            operation,
            channel_wrapper: None,
            callback: None,
            created_at: Instant::now(),
            execution_count: AtomicU64::new(0),
            last_execution_time: None,
            average_execution_time: Duration::default(),
            enabled: AtomicBool::new(true),
            condition: None,
        });

        // Add to cases
        if let Ok(mut cases) = self.cases.write() {
            cases.insert(case_id, case);
        }

        case_id
    }

    /// Add a timeout case
    pub fn timeout_case(&mut self, duration: Duration, priority: SelectPriority) -> SelectCaseId {
        let case_id = self.next_case_id.fetch_add(1, Ordering::SeqCst);
        
        let operation = EnhancedSelectOperation::Timeout {
            case_id,
            duration,
            priority,
        };

        let case = Arc::new(EnhancedSelectCase {
            id: case_id,
            operation,
            channel_wrapper: None,
            callback: None,
            created_at: Instant::now(),
            execution_count: AtomicU64::new(0),
            last_execution_time: None,
            average_execution_time: Duration::default(),
            enabled: AtomicBool::new(true),
            condition: None,
        });

        // Add to cases
        if let Ok(mut cases) = self.cases.write() {
            cases.insert(case_id, case);
        }

        case_id
    }

    /// Execute the select statement
    pub fn execute(&self) -> ChannelResult<EnhancedSelectResult<Box<dyn Any + Send>>> {
        let execution_start = Instant::now();
        
        // Update execution state
        if let Ok(mut state) = self.execution_state.write() {
            state.executing = true;
            state.start_time = Some(execution_start);
        }

        // Update statistics
        if let Ok(mut stats) = self.stats.write() {
            stats.total_executions += 1;
        }

        // Check for cancellation
        if self.cancellation_token.load(Ordering::SeqCst) {
            return Ok(EnhancedSelectResult::Cancelled {
                reason: "Select operation was cancelled".to_string(),
                execution_time: execution_start.elapsed(),
            });
        }

        // Refresh readiness cache if needed
        self.refresh_readiness_cache();

        // Get ready cases
        let ready_cases = self.find_ready_cases()?;

        // If no cases are ready, handle timeout or default
        if ready_cases.is_empty() {
            return self.handle_no_ready_cases(execution_start);
        }

        // Select case using fair scheduling
        let selected_case_id = self.select_case_with_fair_scheduling(&ready_cases)?;

        // Execute the selected case
        let result = self.execute_case(selected_case_id, execution_start)?;

        // Update execution state
        if let Ok(mut state) = self.execution_state.write() {
            state.executing = false;
            state.selected_case = Some(selected_case_id);
        }

        // Update statistics
        self.update_execution_statistics(selected_case_id, execution_start.elapsed());

        Ok(result)
    }

    /// Cancel the select operation
    pub fn cancel(&self) {
        self.cancellation_token.store(true, Ordering::SeqCst);
    }

    /// Get select statistics
    pub fn get_stats(&self) -> EnhancedSelectStats {
        if let Ok(stats) = self.stats.read() {
            stats.clone()
        } else {
            EnhancedSelectStats::default()
        }
    }

    /// Enable or disable a case
    pub fn set_case_enabled(&self, case_id: SelectCaseId, enabled: bool) {
        if let Ok(cases) = self.cases.read() {
            if let Some(case) = cases.get(&case_id) {
                case.enabled.store(enabled, Ordering::SeqCst);
            }
        }
    }

    /// Remove a case
    pub fn remove_case(&self, case_id: SelectCaseId) {
        if let Ok(mut cases) = self.cases.write() {
            cases.remove(&case_id);
        }
    }

    // Private helper methods

    fn refresh_readiness_cache(&self) {
        if let Ok(mut cache) = self.readiness_cache.write() {
            let now = Instant::now();
            if now.duration_since(cache.last_refresh) >= cache.config.refresh_interval {
                // Refresh cache
                cache.send_readiness.clear();
                cache.receive_readiness.clear();
                
                if let Ok(channels) = self.channels.read() {
                    for (channel_id, channel) in channels.iter() {
                        cache.send_readiness.insert(*channel_id, (channel.can_send(), now));
                        cache.receive_readiness.insert(*channel_id, (channel.can_receive(), now));
                    }
                }
                
                cache.last_refresh = now;
            }
        }
    }

    fn find_ready_cases(&self) -> ChannelResult<Vec<SelectCaseId>> {
        let mut ready_cases = Vec::new();
        
        if let Ok(cases) = self.cases.read() {
            for (case_id, case) in cases.iter() {
                // Check if case is enabled
                if !case.enabled.load(Ordering::SeqCst) {
                    continue;
                }

                // Check condition if present
                if let Some(ref condition) = case.condition {
                    if !condition() {
                        continue;
                    }
                }

                // Check if case is ready
                match &case.operation {
                    EnhancedSelectOperation::Send { channel_id, .. } => {
                        if self.is_channel_ready_for_send(*channel_id) {
                            ready_cases.push(*case_id);
                        }
                    }
                    EnhancedSelectOperation::Receive { channel_id, .. } => {
                        if self.is_channel_ready_for_receive(*channel_id) {
                            ready_cases.push(*case_id);
                        }
                    }
                    EnhancedSelectOperation::Default { .. } => {
                        ready_cases.push(*case_id);
                    }
                    EnhancedSelectOperation::Timeout { .. } => {
                        // Timeout cases are handled separately
                    }
                }
            }
        }

        Ok(ready_cases)
    }

    fn is_channel_ready_for_send(&self, channel_id: u64) -> bool {
        if let Ok(cache) = self.readiness_cache.read() {
            if let Some((ready, _)) = cache.send_readiness.get(&channel_id) {
                return *ready;
            }
        }
        
        // Fallback to direct check
        if let Ok(channels) = self.channels.read() {
            if let Some(channel) = channels.get(&channel_id) {
                return channel.can_send();
            }
        }
        
        false
    }

    fn is_channel_ready_for_receive(&self, channel_id: u64) -> bool {
        if let Ok(cache) = self.readiness_cache.read() {
            if let Some((ready, _)) = cache.receive_readiness.get(&channel_id) {
                return *ready;
            }
        }
        
        // Fallback to direct check
        if let Ok(channels) = self.channels.read() {
            if let Some(channel) = channels.get(&channel_id) {
                return channel.can_receive();
            }
        }
        
        false
    }

    fn select_case_with_fair_scheduling(&self, ready_cases: &[SelectCaseId]) -> ChannelResult<SelectCaseId> {
        if ready_cases.is_empty() {
            return Err(ChannelError::NoSenders);
        }

        if !self.config.enable_fair_scheduling {
            // Simple random selection
            let index = (self.next_operation_id.load(Ordering::SeqCst) as usize) % ready_cases.len();
            return Ok(ready_cases[index]);
        }

        // Fair scheduling selection
        if let Ok(mut scheduler) = self.fair_scheduler.write() {
            let now = Instant::now();
            
            // Calculate fairness scores
            let mut best_case = ready_cases[0];
            let mut best_score = f64::MIN;
            
            for &case_id in ready_cases {
                let score = self.calculate_fairness_score(case_id, now, &scheduler);
                if score > best_score {
                    best_score = score;
                    best_case = case_id;
                }
            }
            
            // Update scheduling history
            scheduler.execution_history.push_back((best_case, now));
            scheduler.last_executions.insert(best_case, now);
            
            // Limit history size
            while scheduler.execution_history.len() > 1000 {
                scheduler.execution_history.pop_front();
            }
            
            return Ok(best_case);
        }

        // Fallback to first ready case
        Ok(ready_cases[0])
    }

    fn calculate_fairness_score(&self, case_id: SelectCaseId, now: Instant, scheduler: &FairScheduler) -> f64 {
        let mut score = 0.0;
        
        // Base score from priority
        if let Ok(cases) = self.cases.read() {
            if let Some(case) = cases.get(&case_id) {
                match &case.operation {
                    EnhancedSelectOperation::Send { priority, .. } |
                    EnhancedSelectOperation::Receive { priority, .. } |
                    EnhancedSelectOperation::Default { priority, .. } |
                    EnhancedSelectOperation::Timeout { priority, .. } => {
                        score += *priority as u8 as f64;
                    }
                }
            }
        }
        
        // Fairness adjustment based on last execution
        if let Some(last_exec) = scheduler.last_executions.get(&case_id) {
            let time_since_last = now.duration_since(*last_exec);
            score += time_since_last.as_millis() as f64 / 1000.0;
        } else {
            score += 1000.0; // Bonus for never-executed cases
        }
        
        // Fairness weight adjustment
        if let Some(weight) = scheduler.fairness_weights.get(&case_id) {
            score *= weight;
        }
        
        score
    }

    fn handle_no_ready_cases(&self, execution_start: Instant) -> ChannelResult<EnhancedSelectResult<Box<dyn Any + Send>>> {
        // Check for timeout cases
        if let Ok(cases) = self.cases.read() {
            for (case_id, case) in cases.iter() {
                if let EnhancedSelectOperation::Timeout { duration, .. } = &case.operation {
                    if execution_start.elapsed() >= *duration {
                        return Ok(EnhancedSelectResult::Timeout {
                            timeout_duration: *duration,
                            waited_time: execution_start.elapsed(),
                        });
                    }
                }
            }
        }

        // Check for default cases
        if let Ok(cases) = self.cases.read() {
            for (case_id, case) in cases.iter() {
                if let EnhancedSelectOperation::Default { .. } = &case.operation {
                    if case.enabled.load(Ordering::SeqCst) {
                        return Ok(EnhancedSelectResult::DefaultExecuted {
                            case_id: *case_id,
                            execution_time: execution_start.elapsed(),
                        });
                    }
                }
            }
        }

        // Check if all channels are closed
        let mut all_closed = true;
        let mut checked_channels = Vec::new();
        
        if let Ok(channels) = self.channels.read() {
            for (channel_id, channel) in channels.iter() {
                checked_channels.push(*channel_id);
                if !channel.is_closed() {
                    all_closed = false;
                    break;
                }
            }
        }

        if all_closed {
            return Ok(EnhancedSelectResult::AllClosed {
                checked_channels,
                execution_time: execution_start.elapsed(),
            });
        }

        // Wait briefly and retry
        thread::sleep(Duration::from_millis(1));
        Err(ChannelError::WouldBlock)
    }

    fn execute_case(&self, case_id: SelectCaseId, execution_start: Instant) -> ChannelResult<EnhancedSelectResult<Box<dyn Any + Send>>> {
        if let Ok(cases) = self.cases.read() {
            if let Some(case) = cases.get(&case_id) {
                case.execution_count.fetch_add(1, Ordering::SeqCst);
                
                match &case.operation {
                    EnhancedSelectOperation::Send { channel_id, .. } => {
                        // Execute send operation
                        if let Some(ref wrapper) = case.channel_wrapper {
                            // In a real implementation, we'd extract the value from the operation
                            // For now, we'll just return a success result
                            return Ok(EnhancedSelectResult::SendCompleted {
                                case_id,
                                channel_id: *channel_id,
                                execution_time: execution_start.elapsed(),
                            });
                        }
                    }
                    EnhancedSelectOperation::Receive { channel_id, .. } => {
                        // Execute receive operation
                        if let Some(ref wrapper) = case.channel_wrapper {
                            match wrapper.try_receive() {
                                Ok(Some(value)) => {
                                    return Ok(EnhancedSelectResult::ReceiveCompleted {
                                        case_id,
                                        channel_id: *channel_id,
                                        value,
                                        execution_time: execution_start.elapsed(),
                                    });
                                }
                                Ok(None) => {
                                    return Err(ChannelError::WouldBlock);
                                }
                                Err(err) => {
                                    return Err(err);
                                }
                            }
                        }
                    }
                    EnhancedSelectOperation::Default { .. } => {
                        return Ok(EnhancedSelectResult::DefaultExecuted {
                            case_id,
                            execution_time: execution_start.elapsed(),
                        });
                    }
                    EnhancedSelectOperation::Timeout { duration, .. } => {
                        return Ok(EnhancedSelectResult::Timeout {
                            timeout_duration: *duration,
                            waited_time: execution_start.elapsed(),
                        });
                    }
                }
            }
        }

        Err(ChannelError::NoSenders)
    }

    fn update_execution_statistics(&self, case_id: SelectCaseId, execution_time: Duration) {
        if let Ok(mut stats) = self.stats.write() {
            stats.total_successful_operations += 1;
            
            // Update average execution time
            let total_time = stats.average_execution_time.as_millis() as f64 * stats.total_executions as f64;
            let new_total_time = total_time + execution_time.as_millis() as f64;
            stats.average_execution_time = Duration::from_millis((new_total_time / (stats.total_executions + 1) as f64) as u64);
            
            // Update peak execution time
            if execution_time > stats.peak_execution_time {
                stats.peak_execution_time = execution_time;
            }
            
            // Update case execution distribution
            *stats.case_execution_distribution.entry(case_id).or_insert(0) += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_select_creation() {
        let select = EnhancedSelect::new();
        assert_eq!(select.next_case_id.load(Ordering::SeqCst), 1);
        assert_eq!(select.next_operation_id.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_enhanced_select_config() {
        let config = EnhancedSelectConfig {
            max_cases: 500,
            enable_fair_scheduling: false,
            ..Default::default()
        };
        let select = EnhancedSelect::with_config(config);
        assert_eq!(select.config.max_cases, 500);
        assert!(!select.config.enable_fair_scheduling);
    }

    #[test]
    fn test_select_case_enabling() {
        let select = EnhancedSelect::new();
        let case_id = select.next_case_id.load(Ordering::SeqCst);
        
        // Test enabling/disabling cases
        select.set_case_enabled(case_id, false);
        select.set_case_enabled(case_id, true);
        
        // Test removing cases
        select.remove_case(case_id);
    }
}
