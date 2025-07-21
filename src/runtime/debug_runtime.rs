//! CURSED Debug Runtime System
//!
//! Provides comprehensive runtime debugging capabilities including:
//! - Performance monitoring and profiling
//! - Runtime debugger with breakpoint management
//! - Variable inspection and value analysis
//! - Stack frame tracking and management
//! - Real-time debugging for CURSED programs
//! - Goroutine debugging integration
//! - Memory profiling and analysis

use crate::error_types::CursedError;
use crate::runtime::debug_manager::{DebugManager, Breakpoint as DebugManagerBreakpoint, StackFrame as DebugManagerStackFrame, VariableDebugInfo};
use crate::runtime::goroutine::GoroutineId;
use std::collections::{HashMap, BTreeMap, VecDeque};
use std::path::PathBuf;
use std::sync::{Arc, RwLock, Mutex};
use std::time::{SystemTime, Duration, Instant};
use std::thread;

/// Performance monitoring for runtime analysis
#[derive(Debug, Clone)]
pub struct PerformanceMonitor {
    /// Unique monitor ID
    id: u64,
    /// Monitor name/description
    name: String,
    /// Performance metrics collection
    metrics: Arc<RwLock<PerformanceMetrics>>,
    /// Configuration settings
    config: PerformanceConfig,
    /// Active measurement sessions
    active_sessions: Arc<Mutex<HashMap<String, MeasurementSession>>>,
    /// Historical performance data
    history: Arc<RwLock<Vec<PerformanceSnapshot>>>,
    /// Sampling interval for continuous monitoring
    sampling_interval: Duration,
    /// Whether monitoring is currently active
    is_active: bool,
}

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// CPU usage percentage (0.0 - 100.0)
    pub cpu_usage: f64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Memory peak usage in bytes
    pub memory_peak: u64,
    /// Number of allocations
    pub allocation_count: u64,
    /// Total allocation size
    pub total_allocated: u64,
    /// Garbage collection metrics
    pub gc_metrics: GcMetrics,
    /// Goroutine metrics
    pub goroutine_metrics: GoroutineMetrics,
    /// Function call metrics
    pub function_metrics: HashMap<String, FunctionMetrics>,
    /// I/O operation metrics
    pub io_metrics: IoMetrics,
    /// Timestamp of last update
    pub last_updated: SystemTime,
}

#[derive(Debug, Clone, Default)]
pub struct GcMetrics {
    /// Number of GC cycles
    pub cycles: u64,
    /// Total time spent in GC (microseconds)
    pub total_time: u64,
    /// Average GC pause time (microseconds)
    pub avg_pause_time: f64,
    /// Bytes freed in last cycle
    pub last_freed: u64,
    /// Total bytes freed
    pub total_freed: u64,
}

#[derive(Debug, Clone, Default)]
pub struct GoroutineMetrics {
    /// Total number of goroutines created
    pub total_created: u64,
    /// Currently active goroutines
    pub active_count: u32,
    /// Goroutines waiting for I/O
    pub waiting_io: u32,
    /// Goroutines waiting for channels
    pub waiting_channels: u32,
    /// Average goroutine lifetime (milliseconds)
    pub avg_lifetime: f64,
}

#[derive(Debug, Clone, Default)]
pub struct FunctionMetrics {
    /// Number of times called
    pub call_count: u64,
    /// Total execution time (microseconds)
    pub total_time: u64,
    /// Average execution time (microseconds)
    pub avg_time: f64,
    /// Minimum execution time (microseconds)
    pub min_time: u64,
    /// Maximum execution time (microseconds)
    pub max_time: u64,
}

#[derive(Debug, Clone, Default)]
pub struct IoMetrics {
    /// Number of read operations
    pub read_ops: u64,
    /// Number of write operations
    pub write_ops: u64,
    /// Bytes read
    pub bytes_read: u64,
    /// Bytes written
    pub bytes_written: u64,
    /// Network connections opened
    pub net_connections: u32,
}

#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// Enable CPU monitoring
    pub monitor_cpu: bool,
    /// Enable memory monitoring
    pub monitor_memory: bool,
    /// Enable GC monitoring
    pub monitor_gc: bool,
    /// Enable goroutine monitoring
    pub monitor_goroutines: bool,
    /// Enable function profiling
    pub profile_functions: bool,
    /// Enable I/O monitoring
    pub monitor_io: bool,
    /// Sampling rate (samples per second)
    pub sampling_rate: f64,
    /// Maximum history entries to keep
    pub max_history: usize,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            monitor_cpu: true,
            monitor_memory: true,
            monitor_gc: true,
            monitor_goroutines: true,
            profile_functions: true,
            monitor_io: true,
            sampling_rate: 10.0, // 10 samples per second
            max_history: 1000,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MeasurementSession {
    /// Session name
    pub name: String,
    /// Start time
    pub start_time: Instant,
    /// End time (if finished)
    pub end_time: Option<Instant>,
    /// Custom labels/tags
    pub labels: HashMap<String, String>,
    /// Measurements taken during session
    pub measurements: Vec<PerformanceSnapshot>,
}

#[derive(Debug, Clone)]
pub struct PerformanceSnapshot {
    /// Timestamp of snapshot
    pub timestamp: SystemTime,
    /// Performance metrics at this point
    pub metrics: PerformanceMetrics,
    /// Session context (if any)
    pub session: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new(name: String) -> Self {
        Self {
            id: Self::generate_id(),
            name,
            metrics: Arc::new(RwLock::new(Self::default_metrics())),
            config: PerformanceConfig::default(),
            active_sessions: Arc::new(Mutex::new(HashMap::new())),
            history: Arc::new(RwLock::new(Vec::new())),
            sampling_interval: Duration::from_millis(100),
            is_active: false,
        }
    }

    /// Create with custom configuration
    pub fn with_config(name: String, config: PerformanceConfig) -> Self {
        let sampling_interval = Duration::from_millis((1000.0 / config.sampling_rate) as u64);
        Self {
            id: Self::generate_id(),
            name,
            metrics: Arc::new(RwLock::new(Self::default_metrics())),
            config,
            active_sessions: Arc::new(Mutex::new(HashMap::new())),
            history: Arc::new(RwLock::new(Vec::new())),
            sampling_interval,
            is_active: false,
        }
    }

    /// Start monitoring
    pub fn start(&mut self) -> Result<(), CursedError> {
        if self.is_active {
            return Err(CursedError::Debug("Monitor already active".to_string()));
        }

        self.is_active = true;
        
        // Start background monitoring thread
        let metrics = self.metrics.clone();
        let history = self.history.clone();
        let config = self.config.clone();
        let interval = self.sampling_interval;

        thread::spawn(move || {
            while let Ok(current_metrics) = metrics.read() {
                if !current_metrics.cpu_usage.is_finite() {
                    break; // Monitor stopped
                }
                
                // Collect current metrics
                let snapshot = PerformanceSnapshot {
                    timestamp: SystemTime::now(),
                    metrics: current_metrics.clone(),
                    session: None,
                    metadata: HashMap::new(),
                };

                // Add to history
                if let Ok(mut hist) = history.write() {
                    hist.push(snapshot);
                    if hist.len() > config.max_history {
                        hist.remove(0);
                    }
                }

                thread::sleep(interval);
            }
        });

        Ok(())
    }

    /// Stop monitoring
    pub fn stop(&mut self) -> Result<(), CursedError> {
        if !self.is_active {
            return Err(CursedError::Debug("Monitor not active".to_string()));
        }

        self.is_active = false;
        
        // Signal stop to background thread by setting invalid value
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.cpu_usage = f64::INFINITY;
        }

        Ok(())
    }

    /// Start a measurement session
    pub fn start_session(&self, name: String, labels: HashMap<String, String>) -> Result<(), CursedError> {
        let session = MeasurementSession {
            name: name.clone(),
            start_time: Instant::now(),
            end_time: None,
            labels,
            measurements: Vec::new(),
        };

        if let Ok(mut sessions) = self.active_sessions.lock() {
            sessions.insert(name, session);
            Ok(())
        } else {
            Err(CursedError::Debug("Failed to acquire session lock".to_string()))
        }
    }

    /// End a measurement session
    pub fn end_session(&self, name: &str) -> Result<MeasurementSession, CursedError> {
        if let Ok(mut sessions) = self.active_sessions.lock() {
            if let Some(mut session) = sessions.remove(name) {
                session.end_time = Some(Instant::now());
                Ok(session)
            } else {
                Err(CursedError::Debug(format!("Session '{}' not found", name)))
            }
        } else {
            Err(CursedError::Debug("Failed to acquire session lock".to_string()))
        }
    }

    /// Get current performance metrics
    pub fn get_metrics(&self) -> Result<PerformanceMetrics, CursedError> {
        self.metrics.read()
            .map(|m| m.clone())
            .map_err(|_| CursedError::Debug("Failed to read metrics".to_string()))
    }

    /// Get performance history
    pub fn get_history(&self) -> Result<Vec<PerformanceSnapshot>, CursedError> {
        self.history.read()
            .map(|h| h.clone())
            .map_err(|_| CursedError::Debug("Failed to read history".to_string()))
    }

    /// Update CPU metrics
    pub fn update_cpu_usage(&self, usage: f64) -> Result<(), CursedError> {
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.cpu_usage = usage;
            metrics.last_updated = SystemTime::now();
            Ok(())
        } else {
            Err(CursedError::Debug("Failed to update CPU metrics".to_string()))
        }
    }

    /// Update memory metrics
    pub fn update_memory_usage(&self, current: u64, peak: u64) -> Result<(), CursedError> {
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.memory_usage = current;
            metrics.memory_peak = peak;
            metrics.last_updated = SystemTime::now();
            Ok(())
        } else {
            Err(CursedError::Debug("Failed to update memory metrics".to_string()))
        }
    }

    /// Record function call metrics
    pub fn record_function_call(&self, function_name: String, execution_time: Duration) -> Result<(), CursedError> {
        if !self.config.profile_functions {
            return Ok(());
        }

        if let Ok(mut metrics) = self.metrics.write() {
            let time_micros = execution_time.as_micros() as u64;
            
            let function_metrics = metrics.function_metrics
                .entry(function_name)
                .or_insert_with(FunctionMetrics::default);
            
            function_metrics.call_count += 1;
            function_metrics.total_time += time_micros;
            function_metrics.avg_time = function_metrics.total_time as f64 / function_metrics.call_count as f64;
            
            if function_metrics.call_count == 1 || time_micros < function_metrics.min_time {
                function_metrics.min_time = time_micros;
            }
            if time_micros > function_metrics.max_time {
                function_metrics.max_time = time_micros;
            }

            metrics.last_updated = SystemTime::now();
            Ok(())
        } else {
            Err(CursedError::Debug("Failed to record function call".to_string()))
        }
    }

    fn generate_id() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64
    }

    fn default_metrics() -> PerformanceMetrics {
        PerformanceMetrics {
            cpu_usage: 0.0,
            memory_usage: 0,
            memory_peak: 0,
            allocation_count: 0,
            total_allocated: 0,
            gc_metrics: GcMetrics::default(),
            goroutine_metrics: GoroutineMetrics::default(),
            function_metrics: HashMap::new(),
            io_metrics: IoMetrics::default(),
            last_updated: SystemTime::now(),
        }
    }
}

/// Main runtime debugger interface
pub struct RuntimeDebugger {
    /// Unique debugger ID
    id: u64,
    /// Debugger name
    name: String,
    /// Debug manager integration
    debug_manager: DebugManager,
    /// Performance monitor
    performance_monitor: PerformanceMonitor,
    /// Variable inspector
    variable_inspector: VariableInspection,
    /// Stack frame tracker
    stack_tracker: StackTracker,
    /// Breakpoint manager
    breakpoint_manager: BreakpointManager,
    /// Debugger configuration
    config: RuntimeDebuggerConfig,
    /// Current debugging session
    current_session: Option<DebugSession>,
    /// Event listeners
    event_listeners: Vec<Box<dyn DebugEventListener>>,
    /// Debugging state
    state: DebuggerState,
}

#[derive(Debug, Clone)]
pub struct RuntimeDebuggerConfig {
    /// Enable automatic variable inspection
    pub auto_inspect_variables: bool,
    /// Enable stack frame tracking
    pub track_stack_frames: bool,
    /// Enable performance monitoring
    pub monitor_performance: bool,
    /// Enable goroutine debugging
    pub debug_goroutines: bool,
    /// Maximum call stack depth to track
    pub max_stack_depth: usize,
    /// Debug output verbosity
    pub verbosity: DebugVerbosity,
    /// Enable real-time updates
    pub real_time_updates: bool,
}

impl Default for RuntimeDebuggerConfig {
    fn default() -> Self {
        Self {
            auto_inspect_variables: true,
            track_stack_frames: true,
            monitor_performance: true,
            debug_goroutines: true,
            max_stack_depth: 100,
            verbosity: DebugVerbosity::Normal,
            real_time_updates: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugVerbosity {
    Silent,
    Minimal,
    Normal,
    Verbose,
    Debug,
}

#[derive(Debug, Clone)]
pub struct DebugSession {
    /// Session ID
    pub id: String,
    /// Session name
    pub name: String,
    /// Start time
    pub start_time: SystemTime,
    /// Target program/file
    pub target: String,
    /// Session metadata
    pub metadata: HashMap<String, String>,
    /// Collected debug events
    pub events: Vec<DebugEvent>,
    /// Total number of steps taken in this session
    pub total_steps: usize,
    /// Current call stack depth
    pub call_stack_depth: usize,
    /// Target depth for step out operation
    pub step_out_target_depth: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum DebugEvent {
    /// Stepping event
    Stepping {
        step_type: String,
        location: Option<SourceLocation>,
        timestamp: SystemTime,
    },
    /// Function entry event
    FunctionEntry {
        function_name: String,
        location: Option<SourceLocation>,
        timestamp: SystemTime,
    },
    /// Function exit event
    FunctionExit {
        function_name: String,
        location: Option<SourceLocation>,
        timestamp: SystemTime,
    },
    /// Legacy event structure for compatibility
    Legacy {
        /// Event timestamp
        timestamp: SystemTime,
        /// Event type
        event_type: DebugEventType,
        /// Event data
        data: HashMap<String, String>,
        /// Associated goroutine (if any)
        goroutine_id: Option<GoroutineId>,
        /// Source location (if available)
        location: Option<SourceLocation>,
    },
}

#[derive(Debug, Clone)]
pub enum DebugEventType {
    /// Breakpoint hit
    BreakpointHit,
    /// Variable value changed
    VariableChanged,
    /// Function entry
    FunctionEnter,
    /// Function exit
    FunctionExit,
    /// Goroutine created
    GoroutineCreated,
    /// Goroutine ended
    GoroutineEnded,
    /// Exception/panic occurred
    Exception,
    /// Performance threshold exceeded
    PerformanceAlert,
    /// Memory allocation
    MemoryAllocation,
    /// Custom debug event
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct SourceLocation {
    /// File path
    pub file: PathBuf,
    /// Line number
    pub line: u32,
    /// Column number
    pub column: u32,
    /// Function name (if known)
    pub function: Option<String>,
    /// Byte offset in the source file
    pub offset: usize,
}

pub trait DebugEventListener: Send + Sync {
    fn on_debug_event(&self, event: &DebugEvent);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebuggerState {
    /// Debugger is idle
    Idle,
    /// Currently debugging
    Active,
    /// Paused at breakpoint
    Paused,
    /// Stepping through code
    Stepping,
    /// Debugger encountered an error
    Error,
}

impl std::fmt::Debug for RuntimeDebugger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuntimeDebugger")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("config", &self.config)
            .field("current_session", &self.current_session)
            .field("state", &self.state)
            .field("event_listeners_count", &self.event_listeners.len())
            .finish()
    }
}

impl RuntimeDebugger {
    /// Create a new runtime debugger
    pub fn new(name: String) -> Self {
        let debug_manager = DebugManager::default();
        let performance_monitor = PerformanceMonitor::new(format!("{}_perf", name));
        let variable_inspector = VariableInspection::new();
        let stack_tracker = StackTracker::new();
        let breakpoint_manager = BreakpointManager::new();

        Self {
            id: Self::generate_id(),
            name,
            debug_manager,
            performance_monitor,
            variable_inspector,
            stack_tracker,
            breakpoint_manager,
            config: RuntimeDebuggerConfig::default(),
            current_session: None,
            event_listeners: Vec::new(),
            state: DebuggerState::Idle,
        }
    }

    /// Create debugger with custom configuration
    pub fn with_config(name: String, config: RuntimeDebuggerConfig) -> Self {
        let mut debugger = Self::new(name);
        debugger.config = config;
        debugger
    }

    /// Start a debugging session
    pub fn start_session(&mut self, target: String, session_name: String) -> Result<String, CursedError> {
        if self.current_session.is_some() {
            return Err(CursedError::Debug("Debug session already active".to_string()));
        }

        let session_id = format!("debug_{}_{}", self.id, SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs());

        let session = DebugSession {
            id: session_id.clone(),
            name: session_name,
            start_time: SystemTime::now(),
            target,
            metadata: HashMap::new(),
            events: Vec::new(),
            total_steps: 0,
            call_stack_depth: 0,
            step_out_target_depth: None,
        };

        self.current_session = Some(session);
        self.state = DebuggerState::Active;

        // Start performance monitoring if enabled
        if self.config.monitor_performance {
            self.performance_monitor.start()?;
        }

        // Emit session start event
        self.emit_event(DebugEvent::Legacy {
            timestamp: SystemTime::now(),
            event_type: DebugEventType::Custom("session_started".to_string()),
            data: HashMap::new(),
            goroutine_id: None,
            location: None,
        });

        Ok(session_id)
    }

    /// End the current debugging session
    pub fn end_session(&mut self) -> Result<DebugSession, CursedError> {
        if let Some(session) = self.current_session.take() {
            self.state = DebuggerState::Idle;
            
            // Stop performance monitoring
            if self.config.monitor_performance {
                let _ = self.performance_monitor.stop();
            }

            // Emit session end event
            self.emit_event(DebugEvent::Legacy {
                timestamp: SystemTime::now(),
                event_type: DebugEventType::Custom("session_ended".to_string()),
                data: HashMap::new(),
                goroutine_id: None,
                location: None,
            });

            Ok(session)
        } else {
            Err(CursedError::Debug("No active debug session".to_string()))
        }
    }

    /// Set a breakpoint
    pub fn set_breakpoint(&mut self, location: SourceLocation, condition: Option<String>) -> Result<u64, CursedError> {
        let breakpoint_id = self.breakpoint_manager.add_breakpoint(Breakpoint {
            id: 0, // Will be assigned by manager
            location: location.clone(),
            condition: condition.clone(),
            enabled: true,
            hit_count: 0,
            breakpoint_type: BreakpointType::Line,
        })?;

        // Also set in debug manager for compatibility
        let _ = self.debug_manager.set_breakpoint(location.file, location.line, condition);

        Ok(breakpoint_id)
    }

    /// Remove a breakpoint
    pub fn remove_breakpoint(&mut self, breakpoint_id: u64) -> Result<(), CursedError> {
        self.breakpoint_manager.remove_breakpoint(breakpoint_id)?;
        
        // Also remove from debug manager
        let _ = self.debug_manager.remove_breakpoint(breakpoint_id);
        
        Ok(())
    }

    /// Inspect a variable
    pub fn inspect_variable(&self, variable_name: &str, scope: &str) -> Result<VariableValue, CursedError> {
        self.variable_inspector.inspect_variable(variable_name, scope)
    }

    /// Get current stack trace
    pub fn get_stack_trace(&self) -> Result<Vec<RuntimeStackFrame>, CursedError> {
        self.stack_tracker.get_stack_trace()
    }

    /// Add an event listener
    pub fn add_event_listener(&mut self, listener: Box<dyn DebugEventListener>) {
        self.event_listeners.push(listener);
    }

    /// Get current debugger state
    pub fn get_state(&self) -> DebuggerState {
        self.state
    }

    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> Result<PerformanceMetrics, CursedError> {
        self.performance_monitor.get_metrics()
    }

    /// Pause execution at current location
    pub fn pause(&mut self) -> Result<(), CursedError> {
        if self.state != DebuggerState::Active {
            return Err(CursedError::Debug("Debugger not active".to_string()));
        }
        
        self.state = DebuggerState::Paused;
        Ok(())
    }

    /// Resume execution
    pub fn resume(&mut self) -> Result<(), CursedError> {
        if self.state != DebuggerState::Paused {
            return Err(CursedError::Debug("Debugger not paused".to_string()));
        }
        
        self.state = DebuggerState::Active;
        Ok(())
    }

    /// Step to next line
    pub fn step_next(&mut self) -> Result<(), CursedError> {
        self.state = DebuggerState::Stepping;
        
        // Basic stepping implementation:
        // 1. Get current execution location
        // 2. Set single-step mode
        // 3. Continue execution until next line
        
        // Get current location and stack info before mutable borrow
        let current_location = self.get_current_location();
        let stack_depth = if let Ok(stack_info) = self.stack_tracker.capture_stack_trace() {
            stack_info.frames.len()
        } else {
            0
        };
        
        if let Some(session) = &mut self.current_session {
            // Increment step counter
            session.total_steps += 1;
            session.call_stack_depth = stack_depth;
        }
        
        // Emit step event
        self.emit_event(DebugEvent::Stepping {
            step_type: "next".to_string(),
            location: current_location,
            timestamp: SystemTime::now(),
        });
        
        Ok(())
    }

    /// Step into function
    pub fn step_into(&mut self) -> Result<(), CursedError> {
        self.state = DebuggerState::Stepping;
        
        // Basic step into implementation:
        // 1. Check if current instruction is a function call
        // 2. If yes, step into the function
        // 3. If no, behave like step_next
        
        // Get current location and stack info before mutable borrow
        let current_location = self.get_current_location();
        let (new_depth, function_name) = if let Ok(stack_info) = self.stack_tracker.capture_stack_trace() {
            let depth = stack_info.frames.len();
            let fname = stack_info.frames.last()
                .map(|f| f.function_name.clone())
                .unwrap_or_else(|| "unknown".to_string());
            (depth, fname)
        } else {
            (0, "unknown".to_string())
        };
        
        let mut stepped_into_function = false;
        if let Some(session) = &mut self.current_session {
            // Increment step counter
            session.total_steps += 1;
            
            // Check if we stepped into a function
            if new_depth > session.call_stack_depth {
                session.call_stack_depth = new_depth;
                stepped_into_function = true;
            }
        }
        
        // Emit step into event
        self.emit_event(DebugEvent::Stepping {
            step_type: "into".to_string(),
            location: current_location.clone(),
            timestamp: SystemTime::now(),
        });
        
        // Emit function entry event if we stepped into a function
        if stepped_into_function {
            self.emit_event(DebugEvent::FunctionEntry {
                function_name,
                location: current_location,
                timestamp: SystemTime::now(),
            });
        }
        
        Ok(())
    }

    /// Step out of current function
    pub fn step_out(&mut self) -> Result<(), CursedError> {
        self.state = DebuggerState::Stepping;
        
        // Basic step out implementation:
        // 1. Continue execution until we return from current function
        // 2. Monitor stack depth to detect function exit
        
        // Get current location and stack info before mutable borrow
        let current_location = self.get_current_location();
        let function_name = if let Ok(stack_info) = self.stack_tracker.capture_stack_trace() {
            stack_info.frames.last()
                .map(|f| f.function_name.clone())
                .unwrap_or_else(|| "unknown".to_string())
        } else {
            "unknown".to_string()
        };
        
        let mut should_emit_events = false;
        if let Some(session) = &mut self.current_session {
            let current_depth = session.call_stack_depth;
            
            // Only step out if we're in a function (depth > 0)
            if current_depth > 0 {
                // Increment step counter
                session.total_steps += 1;
                
                // Mark that we want to step out
                session.step_out_target_depth = Some(current_depth.saturating_sub(1));
                should_emit_events = true;
            }
        }
        
        if should_emit_events {
            // Emit step out event
            self.emit_event(DebugEvent::Stepping {
                step_type: "out".to_string(),
                location: current_location.clone(),
                timestamp: SystemTime::now(),
            });
            
            // Emit function exit event
            self.emit_event(DebugEvent::FunctionExit {
                function_name,
                location: current_location,
                timestamp: SystemTime::now(),
            });
        }
        
        Ok(())
    }

    fn emit_event(&mut self, event: DebugEvent) {
        // Add to current session if active
        if let Some(ref mut session) = self.current_session {
            session.events.push(event.clone());
        }

        // Notify listeners
        for listener in &self.event_listeners {
            listener.on_debug_event(&event);
        }
    }

    fn generate_id() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64
    }

    /// Get current execution location
    fn get_current_location(&self) -> Option<SourceLocation> {
        // Try to get location from stack tracker
        if let Ok(stack_info) = self.stack_tracker.capture_stack_trace() {
            if let Some(frame) = stack_info.frames.last() {
                return Some(SourceLocation {
file: frame.source_file.clone(),
                    line: frame.line_number,
                    column: frame.column_number,
                    function: Some(frame.function_name.clone()),
                
                    offset: 0,
                });
            }
        }
        None
    }
}

/// Variable inspection and value analysis
#[derive(Debug, Clone)]
pub struct VariableInspection {
    /// Variable cache for fast lookup
    variable_cache: HashMap<String, VariableValue>,
    /// Inspection configuration
    config: InspectionConfig,
    /// Watch expressions
    watch_expressions: Vec<WatchExpression>,
    /// Variable access history
    access_history: VecDeque<VariableAccess>,
}

#[derive(Debug, Clone)]
pub struct InspectionConfig {
    /// Maximum depth for object inspection
    pub max_depth: usize,
    /// Maximum string length to display
    pub max_string_length: usize,
    /// Cache variable values
    pub enable_caching: bool,
    /// Track variable access patterns
    pub track_access: bool,
    /// Show private/internal fields
    pub show_private_fields: bool,
}

impl Default for InspectionConfig {
    fn default() -> Self {
        Self {
            max_depth: 10,
            max_string_length: 1000,
            enable_caching: true,
            track_access: true,
            show_private_fields: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VariableValue {
    /// Variable name
    pub name: String,
    /// Variable type
    pub type_name: String,
    /// String representation of value
    pub value: String,
    /// Nested values (for objects/structs)
    pub children: Vec<VariableValue>,
    /// Memory address (if available)
    pub memory_address: Option<u64>,
    /// Variable scope
    pub scope: String,
    /// Is the variable mutable?
    pub is_mutable: bool,
    /// Size in memory (bytes)
    pub size_bytes: Option<usize>,
    /// Last time this value was updated
    pub last_updated: SystemTime,
}

#[derive(Debug, Clone)]
pub struct WatchExpression {
    /// Unique watch ID
    pub id: u64,
    /// Expression to evaluate
    pub expression: String,
    /// Current value (if evaluable)
    pub current_value: Option<VariableValue>,
    /// Is the watch active?
    pub enabled: bool,
    /// Error message if evaluation failed
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VariableAccess {
    /// Variable name that was accessed
    pub variable_name: String,
    /// Type of access (read/write)
    pub access_type: AccessType,
    /// Timestamp of access
    pub timestamp: SystemTime,
    /// Source location of access
    pub location: Option<SourceLocation>,
    /// Value before access (for writes)
    pub old_value: Option<String>,
    /// Value after access
    pub new_value: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessType {
    Read,
    Write,
    Reference,
}

impl VariableInspection {
    /// Create a new variable inspector
    pub fn new() -> Self {
        Self {
            variable_cache: HashMap::new(),
            config: InspectionConfig::default(),
            watch_expressions: Vec::new(),
            access_history: VecDeque::new(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: InspectionConfig) -> Self {
        Self {
            variable_cache: HashMap::new(),
            config,
            watch_expressions: Vec::new(),
            access_history: VecDeque::new(),
        }
    }

    /// Inspect a variable by name and scope
    pub fn inspect_variable(&self, variable_name: &str, scope: &str) -> Result<VariableValue, CursedError> {
        let cache_key = format!("{}::{}", scope, variable_name);
        
        // Check cache first if enabled
        if self.config.enable_caching {
            if let Some(cached_value) = self.variable_cache.get(&cache_key) {
                return Ok(cached_value.clone());
            }
        }

        // Simplified variable lookup implementation
        // For now, return a basic implementation that works
        let variable_value = VariableValue {
            name: variable_name.to_string(),
            type_name: "unknown".to_string(),
            value: format!("Variable '{}' in scope '{}'", variable_name, scope),
            children: Vec::new(),
            memory_address: None,
            scope: scope.to_string(),
            is_mutable: true,
            size_bytes: None,
            last_updated: SystemTime::now(),
        };

        Ok(variable_value)
    }

    /// Add a watch expression
    pub fn add_watch(&mut self, expression: String) -> Result<u64, CursedError> {
        let watch_id = self.generate_watch_id();
        let watch = WatchExpression {
            id: watch_id,
            expression,
            current_value: None,
            enabled: true,
            error: None,
        };

        self.watch_expressions.push(watch);
        Ok(watch_id)
    }

    /// Remove a watch expression
    pub fn remove_watch(&mut self, watch_id: u64) -> Result<(), CursedError> {
        if let Some(pos) = self.watch_expressions.iter().position(|w| w.id == watch_id) {
            self.watch_expressions.remove(pos);
            Ok(())
        } else {
            Err(CursedError::Debug(format!("Watch expression {} not found", watch_id)))
        }
    }

    /// Get all watch expressions
    pub fn get_watches(&self) -> &[WatchExpression] {
        &self.watch_expressions
    }

    /// Update a variable value in cache
    pub fn update_variable(&mut self, scope: &str, variable_name: &str, new_value: VariableValue) {
        if self.config.enable_caching {
            let cache_key = format!("{}::{}", scope, variable_name);
            self.variable_cache.insert(cache_key, new_value);
        }
    }

    /// Get variable access history
    pub fn get_access_history(&self) -> &VecDeque<VariableAccess> {
        &self.access_history
    }

    /// Record variable access
    pub fn record_access(&mut self, access: VariableAccess) {
        if self.config.track_access {
            self.access_history.push_back(access);
            
            // Keep history bounded
            if self.access_history.len() > 1000 {
                self.access_history.pop_front();
            }
        }
    }

    /// Clear variable cache
    pub fn clear_cache(&mut self) {
        self.variable_cache.clear();
    }

    fn generate_watch_id(&self) -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64
    }
}

impl Default for VariableInspection {
    fn default() -> Self {
        Self::new()
    }
}

/// Stack frame information for debugging
#[derive(Debug, Clone)]
pub struct RuntimeStackFrame {
    /// Frame ID
    pub id: u64,
    /// Function name
    pub function_name: String,
    /// Source file
    pub source_file: PathBuf,
    /// Line number
    pub line_number: u32,
    /// Column number
    pub column_number: u32,
    /// Local variables in this frame
    pub local_variables: HashMap<String, VariableValue>,
    /// Function parameters
    pub parameters: Vec<VariableValue>,
    /// Frame pointer address
    pub frame_pointer: Option<u64>,
    /// Return address
    pub return_address: Option<u64>,
    /// Frame size in bytes
    pub frame_size: Option<usize>,
    /// Associated goroutine
    pub goroutine_id: Option<GoroutineId>,
    /// Frame creation timestamp
    pub created_at: SystemTime,
}

impl RuntimeStackFrame {
    /// Create a new stack frame
    pub fn new(function_name: String, source_file: PathBuf, line: u32, column: u32) -> Self {
        Self {
            id: Self::generate_id(),
            function_name,
            source_file,
            line_number: line,
            column_number: column,
            local_variables: HashMap::new(),
            parameters: Vec::new(),
            frame_pointer: None,
            return_address: None,
            frame_size: None,
            goroutine_id: None,
            created_at: SystemTime::now(),
        }
    }

    /// Add a local variable to this frame
    pub fn add_local_variable(&mut self, variable: VariableValue) {
        self.local_variables.insert(variable.name.clone(), variable);
    }

    /// Add a parameter to this frame
    pub fn add_parameter(&mut self, parameter: VariableValue) {
        self.parameters.push(parameter);
    }

    /// Get a local variable by name
    pub fn get_local_variable(&self, name: &str) -> Option<&VariableValue> {
        self.local_variables.get(name)
    }

    /// Set frame memory information
    pub fn set_memory_info(&mut self, frame_pointer: u64, return_address: u64, size: usize) {
        self.frame_pointer = Some(frame_pointer);
        self.return_address = Some(return_address);
        self.frame_size = Some(size);
    }

    /// Set associated goroutine
    pub fn set_goroutine(&mut self, goroutine_id: GoroutineId) {
        self.goroutine_id = Some(goroutine_id);
    }

    fn generate_id() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64
    }
}

/// Stack frame tracker
#[derive(Debug)]
pub struct StackTracker {
    /// Current call stack
    call_stack: Vec<RuntimeStackFrame>,
    /// Maximum stack depth to track
    max_depth: usize,
    /// Stack overflow threshold
    overflow_threshold: usize,
    /// Enable detailed tracking
    detailed_tracking: bool,
}

impl StackTracker {
    /// Create a new stack tracker
    pub fn new() -> Self {
        Self {
            call_stack: Vec::new(),
            max_depth: 1000,
            overflow_threshold: 10000,
            detailed_tracking: true,
        }
    }

    /// Push a new frame onto the stack
    pub fn push_frame(&mut self, frame: RuntimeStackFrame) -> Result<(), CursedError> {
        if self.call_stack.len() >= self.overflow_threshold {
            return Err(CursedError::Debug("Stack overflow detected".to_string()));
        }

        if self.call_stack.len() < self.max_depth {
            self.call_stack.push(frame);
        }
        
        Ok(())
    }

    /// Pop the top frame from the stack
    pub fn pop_frame(&mut self) -> Option<RuntimeStackFrame> {
        self.call_stack.pop()
    }

    /// Get the current stack trace
    pub fn get_stack_trace(&self) -> Result<Vec<RuntimeStackFrame>, CursedError> {
        Ok(self.call_stack.clone())
    }

    /// Get the current frame (top of stack)
    pub fn get_current_frame(&self) -> Option<&RuntimeStackFrame> {
        self.call_stack.last()
    }

    /// Get frame at specific depth
    pub fn get_frame_at_depth(&self, depth: usize) -> Option<&RuntimeStackFrame> {
        if depth < self.call_stack.len() {
            self.call_stack.get(self.call_stack.len() - 1 - depth)
        } else {
            None
        }
    }

    /// Get current stack depth
    pub fn get_depth(&self) -> usize {
        self.call_stack.len()
    }

    /// Clear the entire stack
    pub fn clear(&mut self) {
        self.call_stack.clear();
    }

    /// Set maximum tracking depth
    pub fn set_max_depth(&mut self, depth: usize) {
        self.max_depth = depth;
    }

    /// Capture current stack trace
    pub fn capture_stack_trace(&self) -> Result<StackTrace, CursedError> {
        let frames = self.call_stack.clone();
        Ok(StackTrace { frames })
    }
}

impl Default for StackTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Stack trace information
#[derive(Debug, Clone)]
pub struct StackTrace {
    /// Stack frames
    pub frames: Vec<RuntimeStackFrame>,
}

/// Breakpoint management system
#[derive(Debug, Clone)]
pub struct Breakpoint {
    /// Unique breakpoint ID
    pub id: u64,
    /// Source location
    pub location: SourceLocation,
    /// Optional condition for conditional breakpoints
    pub condition: Option<String>,
    /// Whether the breakpoint is enabled
    pub enabled: bool,
    /// Number of times this breakpoint has been hit
    pub hit_count: u32,
    /// Type of breakpoint
    pub breakpoint_type: BreakpointType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BreakpointType {
    /// Line breakpoint
    Line,
    /// Function entry breakpoint
    Function,
    /// Conditional breakpoint
    Conditional,
    /// Data watchpoint (breaks on variable change)
    Watchpoint,
    /// Exception breakpoint
    Exception,
}

#[derive(Debug)]
pub struct BreakpointManager {
    /// Active breakpoints
    breakpoints: HashMap<u64, Breakpoint>,
    /// Next breakpoint ID
    next_id: u64,
    /// Breakpoint hit statistics
    hit_statistics: HashMap<u64, BreakpointStats>,
}

#[derive(Debug, Clone)]
pub struct BreakpointStats {
    /// Total hits
    pub total_hits: u32,
    /// First hit time
    pub first_hit: Option<SystemTime>,
    /// Last hit time
    pub last_hit: Option<SystemTime>,
    /// Average time between hits
    pub avg_hit_interval: Option<Duration>,
}

impl BreakpointManager {
    /// Create a new breakpoint manager
    pub fn new() -> Self {
        Self {
            breakpoints: HashMap::new(),
            next_id: 1,
            hit_statistics: HashMap::new(),
        }
    }

    /// Add a new breakpoint
    pub fn add_breakpoint(&mut self, mut breakpoint: Breakpoint) -> Result<u64, CursedError> {
        let id = self.next_id;
        self.next_id += 1;
        
        breakpoint.id = id;
        self.breakpoints.insert(id, breakpoint);
        self.hit_statistics.insert(id, BreakpointStats {
            total_hits: 0,
            first_hit: None,
            last_hit: None,
            avg_hit_interval: None,
        });

        Ok(id)
    }

    /// Remove a breakpoint
    pub fn remove_breakpoint(&mut self, id: u64) -> Result<Breakpoint, CursedError> {
        if let Some(breakpoint) = self.breakpoints.remove(&id) {
            self.hit_statistics.remove(&id);
            Ok(breakpoint)
        } else {
            Err(CursedError::Debug(format!("Breakpoint {} not found", id)))
        }
    }

    /// Enable/disable a breakpoint
    pub fn set_breakpoint_enabled(&mut self, id: u64, enabled: bool) -> Result<(), CursedError> {
        if let Some(breakpoint) = self.breakpoints.get_mut(&id) {
            breakpoint.enabled = enabled;
            Ok(())
        } else {
            Err(CursedError::Debug(format!("Breakpoint {} not found", id)))
        }
    }

    /// Check if a breakpoint should trigger at the given location
    pub fn check_breakpoint(&mut self, file: &PathBuf, line: u32) -> Option<u64> {
        for (id, breakpoint) in &mut self.breakpoints {
            if breakpoint.enabled && 
               breakpoint.location.file == *file && 
               breakpoint.location.line == line {
                
                // Update hit count and statistics
                breakpoint.hit_count += 1;
                
                if let Some(stats) = self.hit_statistics.get_mut(id) {
                    let now = SystemTime::now();
                    stats.total_hits += 1;
                    
                    if stats.first_hit.is_none() {
                        stats.first_hit = Some(now);
                    }
                    
                    if let Some(last_hit) = stats.last_hit {
                        if let Ok(interval) = now.duration_since(last_hit) {
                            if let Some(avg_interval) = stats.avg_hit_interval {
                                stats.avg_hit_interval = Some(Duration::from_nanos(
                                ((avg_interval.as_nanos() + interval.as_nanos()) / 2) as u64
                                ));
                            } else {
                                stats.avg_hit_interval = Some(interval);
                            }
                        }
                    }
                    
                    stats.last_hit = Some(now);
                }

                return Some(*id);
            }
        }
        None
    }

    /// Get breakpoint by ID
    pub fn get_breakpoint(&self, id: u64) -> Option<&Breakpoint> {
        self.breakpoints.get(&id)
    }

    /// Get all breakpoints
    pub fn get_all_breakpoints(&self) -> Vec<&Breakpoint> {
        self.breakpoints.values().collect()
    }

    /// Get breakpoint statistics
    pub fn get_breakpoint_stats(&self, id: u64) -> Option<&BreakpointStats> {
        self.hit_statistics.get(&id)
    }

    /// Clear all breakpoints
    pub fn clear_all(&mut self) {
        self.breakpoints.clear();
        self.hit_statistics.clear();
    }
}

impl Default for BreakpointManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Legacy minimal implementation for backward compatibility
pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED debug runtime enabled".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_monitor_creation() {
        let mut monitor = PerformanceMonitor::new("test_monitor".to_string());
        assert!(!monitor.is_active);
        
        let start_result = monitor.start();
        assert!(start_result.is_ok());
        assert!(monitor.is_active);
    }

    #[test]
    fn test_runtime_debugger_session() {
        let mut debugger = RuntimeDebugger::new("test_debugger".to_string());
        assert_eq!(debugger.get_state(), DebuggerState::Idle);
        
        let session_id = debugger.start_session("test_program.csd".to_string(), "test_session".to_string());
        assert!(session_id.is_ok());
        assert_eq!(debugger.get_state(), DebuggerState::Active);
        
        let end_result = debugger.end_session();
        assert!(end_result.is_ok());
        assert_eq!(debugger.get_state(), DebuggerState::Idle);
    }

    #[test]
    fn test_variable_inspection() {
        let inspector = VariableInspection::new();
        let result = inspector.inspect_variable("test_var", "main");
        assert!(result.is_ok());
        
        let variable = result.unwrap();
        assert_eq!(variable.name, "test_var");
        assert_eq!(variable.scope, "main");
    }

    #[test]
    fn test_stack_frame_management() {
        let mut tracker = StackTracker::new();
        assert_eq!(tracker.get_depth(), 0);
        
        let frame = RuntimeStackFrame::new(
            "test_function".to_string(),
            PathBuf::from("test.csd"),
            10,
            5
        );
        
        let push_result = tracker.push_frame(frame);
        assert!(push_result.is_ok());
        assert_eq!(tracker.get_depth(), 1);
        
        let current_frame = tracker.get_current_frame();
        assert!(current_frame.is_some());
        assert_eq!(current_frame.unwrap().function_name, "test_function");
    }

    #[test]
    fn test_breakpoint_management() {
        let mut manager = BreakpointManager::new();
        
        let breakpoint = Breakpoint {
            id: 0, // Will be assigned by manager
            location: SourceLocation {
file: PathBuf::from("test.csd"),
                line: 10,
                column: 5,
                function: Some("test_function".to_string()),
            
                    offset: 0,
                },
            condition: None,
            enabled: true,
            hit_count: 0,
            breakpoint_type: BreakpointType::Line,
        };
        
        let bp_id = manager.add_breakpoint(breakpoint);
        assert!(bp_id.is_ok());
        
        let bp_id = bp_id.unwrap();
        let retrieved = manager.get_breakpoint(bp_id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().location.line, 10);
        
        // Test breakpoint triggering
        let hit_id = manager.check_breakpoint(&PathBuf::from("test.csd"), 10);
        assert_eq!(hit_id, Some(bp_id));
        
        let updated_bp = manager.get_breakpoint(bp_id).unwrap();
        assert_eq!(updated_bp.hit_count, 1);
    }

    #[test]
    fn test_watch_expressions() {
        let mut inspector = VariableInspection::new();
        
        let watch_id = inspector.add_watch("variable_name + 10".to_string());
        assert!(watch_id.is_ok());
        
        let watches = inspector.get_watches();
        assert_eq!(watches.len(), 1);
        assert_eq!(watches[0].expression, "variable_name + 10");
        
        let remove_result = inspector.remove_watch(watch_id.unwrap());
        assert!(remove_result.is_ok());
        
        let watches_after = inspector.get_watches();
        assert_eq!(watches_after.len(), 0);
    }
}
