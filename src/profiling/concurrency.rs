use crate::error::CursedError;
// Concurrency profiling for goroutines and channel analysis

use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};
use std::time::{Duration, Instant, SystemTime};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument, warn};

/// State transition for concurrency profiling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    pub from_state: String,
    pub to_state: String,
    pub timestamp: Instant,
    pub duration: Duration,
    pub context: HashMap<String, String>,
}

impl StateTransition {
    pub fn new(from_state: String, to_state: String) -> Self {
        Self {
            from_state,
            to_state,
            timestamp: Instant::now(),
            duration: Duration::ZERO,
            context: HashMap::new(),
        }
    }
    
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }
    
    pub fn with_context(mut self, context: HashMap<String, String>) -> Self {
        self.context = context;
        self
    }
}

// use crate::profiling::core::{DataCollector, CollectorStats, ProfilerError};

/// Concurrency profiler for goroutines and channels
#[derive(Debug)]
pub struct ConcurrencyProfiler {
    collecting: Arc<Mutex<bool>>,
    data: Arc<RwLock<ConcurrencyProfileData>>,
    stats: Arc<RwLock<CollectorStats>>,
    goroutine_tracker: GoroutineTracker,
    channel_tracker: ChannelTracker,
}

impl ConcurrencyProfiler {
    pub fn new() -> Self {
        Self {
            collecting: Arc::new(Mutex::new(false)),
            data: Arc::new(RwLock::new(ConcurrencyProfileData::new())),
            stats: Arc::new(RwLock::new(CollectorStats::default())),
            goroutine_tracker: GoroutineTracker::new(),
            channel_tracker: ChannelTracker::new(),
        }
    }
    
    #[instrument(skip(self))]
    pub fn track_goroutine_spawn(
        &self,
        goroutine_id: u64,
        parent_id: Option<u64>,
        stack_trace: Vec<String>,
    ) -> crate::error::Result<()> {
        if !self.is_collecting() {
            return Ok(());
        }
        
        let event = GoroutineEvent {
            event_type: GoroutineEventType::Spawn,
            goroutine_id,
            parent_id,
            timestamp: Instant::now(),
            stack_trace,
            metadata: HashMap::new(),
        };
        
        self.goroutine_tracker.track_spawn(goroutine_id, parent_id)?;
        
        if let Ok(mut data) = self.data.write() {
            data.add_goroutine_event(event);
        }
        
        self.update_stats(|stats| stats.data_points += 1);
        Ok(())
    }
    
    #[instrument(skip(self))]
    pub fn track_goroutine_completion(
        &self,
        goroutine_id: u64,
        completion_type: GoroutineCompletionType,
    ) -> crate::error::Result<()> {
        if !self.is_collecting() {
            return Ok(());
        }
        
        let event = GoroutineEvent {
            event_type: GoroutineEventType::Complete(completion_type),
            goroutine_id,
            parent_id: None,
            timestamp: Instant::now(),
            stack_trace: Vec::new(),
            metadata: HashMap::new(),
        };
        
        self.goroutine_tracker.track_completion(goroutine_id)?;
        
        if let Ok(mut data) = self.data.write() {
            data.add_goroutine_event(event);
        }
        
        self.update_stats(|stats| stats.data_points += 1);
        Ok(())
    }
    
    #[instrument(skip(self))]
    pub fn track_goroutine_yield(&self, goroutine_id: u64) -> crate::error::Result<()> {
        if !self.is_collecting() {
            return Ok(());
        }
        
        let event = GoroutineEvent {
            event_type: GoroutineEventType::Yield,
            goroutine_id,
            parent_id: None,
            timestamp: Instant::now(),
            stack_trace: Vec::new(),
            metadata: HashMap::new(),
        };
        
        if let Ok(mut data) = self.data.write() {
            data.add_goroutine_event(event);
        }
        
        Ok(())
    }
    
    #[instrument(skip(self))]
    pub fn track_channel_operation(
        &self,
        channel_id: u64,
        operation: ChannelOperation,
        goroutine_id: u64,
        duration: Option<Duration>,
    ) -> crate::error::Result<()> {
        if !self.is_collecting() {
            return Ok(());
        }
        
        let event = ChannelEvent {
            channel_id,
            operation,
            goroutine_id,
            timestamp: Instant::now(),
            duration,
            buffer_size: self.channel_tracker.get_buffer_size(channel_id),
            blocked_goroutines: self.channel_tracker.get_blocked_count(channel_id),
        };
        
        self.channel_tracker.track_operation(&event)?;
        
        if let Ok(mut data) = self.data.write() {
            data.add_channel_event(event);
        }
        
        self.update_stats(|stats| stats.data_points += 1);
        Ok(())
    }
    
    #[instrument(skip(self))]
    pub fn track_scheduler_event(&self, event: SchedulerEvent) -> crate::error::Result<()> {
        if !self.is_collecting() {
            return Ok(());
        }
        
        if let Ok(mut data) = self.data.write() {
            data.add_scheduler_event(event);
        }
        
        Ok(())
    }
    
    fn update_stats<F>(&self, updater: F)
    where
        F: FnOnce(&mut CollectorStats),
    {
        if let Ok(mut stats) = self.stats.write() {
            updater(&mut stats);
        }
    }
    
    pub fn get_goroutine_timeline(&self) -> Vec<GoroutineTimeline> {
        if let Ok(data) = self.data.read() {
            data.generate_goroutine_timeline()
        } else {
            Vec::new()
        }
    }
    
    pub fn get_channel_analysis(&self) -> ChannelAnalysis {
        if let Ok(data) = self.data.read() {
            data.analyze_channels()
        } else {
            ChannelAnalysis::default()
        }
    }
    
    pub fn detect_deadlocks(&self) -> Vec<DeadlockDetection> {
        if let Ok(data) = self.data.read() {
            data.detect_deadlocks()
        } else {
            Vec::new()
        }
    }
    
    pub fn analyze_scheduler_performance(&self) -> SchedulerAnalysis {
        if let Ok(data) = self.data.read() {
            data.analyze_scheduler()
        } else {
            SchedulerAnalysis::default()
        }
    }
}

impl DataCollector for ConcurrencyProfiler {
    #[instrument(skip(self))]
    fn start_collection(&mut self) -> crate::error::Result<()> {
        if self.is_collecting() {
            return Err(ProfilerError::ConfigError("Concurrency profiler already collecting".to_string()));
        }
        
        *self.collecting.lock().unwrap() = true;
        self.goroutine_tracker.start_tracking()?;
        self.channel_tracker.start_tracking()?;
        
        info!("Started concurrency profiling");
        Ok(())
    }
    
    #[instrument(skip(self))]
    fn stop_collection(&mut self) -> crate::error::Result<()> {
        if !self.is_collecting() {
            return Err(ProfilerError::ConfigError("Concurrency profiler not collecting".to_string()));
        }
        
        *self.collecting.lock().unwrap() = false;
        self.goroutine_tracker.stop_tracking()?;
        self.channel_tracker.stop_tracking()?;
        
        let profile_data = self.data.read().unwrap().clone();
        match bincode::serialize(&profile_data) {
            Ok(data) => {
                if let Ok(mut stats) = self.stats.write() {
                    stats.bytes_collected = data.len() as u64;
                }
                info!("Concurrency profiling stopped, collected {} goroutine events and {} channel events",
                      profile_data.goroutine_events.len(), profile_data.channel_events.len());
                Ok(data)
            }
            Err(e) => Err(ProfilerError::SerializationError(e.to_string())),
        }
    }
    
    fn is_collecting(&self) -> bool {
        *self.collecting.lock().unwrap()
    }
    
    fn get_stats(&self) -> CollectorStats {
        self.stats.read().unwrap().clone()
    }
}

/// Concurrency profiling data collection
#[derive(Debug, Clone, Serialize)]
pub struct ConcurrencyProfileData {
    #[serde(skip)]
    pub goroutine_events: Vec<GoroutineEvent>,
    #[serde(skip)]
    pub channel_events: Vec<ChannelEvent>,
    #[serde(skip)]
    pub scheduler_events: Vec<SchedulerEvent>,
    pub start_time: SystemTime,
}

impl ConcurrencyProfileData {
    pub fn new() -> Self {
        Self {
            goroutine_events: Vec::new(),
            channel_events: Vec::new(),
            scheduler_events: Vec::new(),
            start_time: SystemTime::now(),
        }
    }
    
    pub fn add_goroutine_event(&mut self, event: GoroutineEvent) {
        self.goroutine_events.push(event);
    }
    
    pub fn add_channel_event(&mut self, event: ChannelEvent) {
        self.channel_events.push(event);
    }
    
    pub fn add_scheduler_event(&mut self, event: SchedulerEvent) {
        self.scheduler_events.push(event);
    }
    
    pub fn generate_goroutine_timeline(&self) -> Vec<GoroutineTimeline> {
        let mut timelines: HashMap<u64, GoroutineTimeline> = HashMap::new();
        
        for event in &self.goroutine_events {
            let timeline = timelines
                .entry(event.goroutine_id)
                .or_insert_with(|| GoroutineTimeline::new(event.goroutine_id));
            
            match &event.event_type {
                GoroutineEventType::Spawn => {
                    timeline.spawn_time = Some(event.timestamp);
                    timeline.parent_id = event.parent_id;
                }
                GoroutineEventType::Complete(_) => {
                    timeline.completion_time = Some(event.timestamp);
                    if let Some(spawn_time) = timeline.spawn_time {
                        timeline.lifetime = Some(event.timestamp.duration_since(spawn_time));
                    }
                }
                GoroutineEventType::Yield => {
                    timeline.yield_count += 1;
                }
                GoroutineEventType::Block => {
                    timeline.block_count += 1;
                }
                GoroutineEventType::Resume => {
                    timeline.resume_count += 1;
                }
            }
        }
        
        timelines.into_values().collect()
    }
    
    pub fn analyze_channels(&self) -> ChannelAnalysis {
        let mut channel_stats: HashMap<u64, ChannelStats> = HashMap::new();
        let mut operation_patterns = Vec::new();
        
        for event in &self.channel_events {
            let stats = channel_stats
                .entry(event.channel_id)
                .or_insert_with(ChannelStats::default);
            
            match &event.operation {
                ChannelOperation::Send(_) => {
                    stats.send_count += 1;
                    if let Some(duration) = event.duration {
                        stats.total_send_time += duration;
                    }
                }
                ChannelOperation::Receive => {
                    stats.receive_count += 1;
                    if let Some(duration) = event.duration {
                        stats.total_receive_time += duration;
                    }
                }
                ChannelOperation::Close => {
                    stats.close_count += 1;
                }
                ChannelOperation::Select => {
                    stats.select_count += 1;
                }
            }
            
            if event.blocked_goroutines > 0 {
                stats.max_blocked_goroutines = 
                    std::cmp::max(stats.max_blocked_goroutines, event.blocked_goroutines);
            }
            
            operation_patterns.push(ChannelOperationPattern {
                timestamp: event.timestamp,
                channel_id: event.channel_id,
                operation_type: event.operation.clone(),
                blocking_time: event.duration,
                queue_length: event.buffer_size,
            });
        }
        
        let total_channels = channel_stats.len() as u64;
        let total_operations = self.channel_events.len() as u64;
        
        ChannelAnalysis {
            channel_stats,
            operation_patterns,
            total_channels,
            total_operations,
        }
    }
    
    pub fn detect_deadlocks(&self) -> Vec<DeadlockDetection> {
        let mut potential_deadlocks = Vec::new();
        let mut goroutine_states: HashMap<u64, GoroutineState> = HashMap::new();
        
        // Track goroutine states through events
        for event in &self.goroutine_events {
            let state = goroutine_states
                .entry(event.goroutine_id)
                .or_insert(GoroutineState::Running);
            
            match &event.event_type {
                GoroutineEventType::Block => *state = GoroutineState::Blocked,
                GoroutineEventType::Resume => *state = GoroutineState::Running,
                GoroutineEventType::Complete(_) => *state = GoroutineState::Completed,
                _ => {}
            }
        }
        
        // Look for patterns that might indicate deadlocks
        let blocked_goroutines: Vec<_> = goroutine_states
            .iter()
            .filter(|(_, state)| matches!(state, GoroutineState::Blocked))
            .map(|(id, _)| *id)
            .collect();
        
        if blocked_goroutines.len() >= 2 {
            // Multiple goroutines blocked - potential deadlock
            potential_deadlocks.push(DeadlockDetection {
                detection_time: SystemTime::now(),
                involved_goroutines: blocked_goroutines.clone(),
                suspected_resources: self.find_suspected_resources(&blocked_goroutines),
                confidence: 0.7, // Heuristic confidence
                description: format!("Potential deadlock detected: {} goroutines blocked", blocked_goroutines.len()),
            });
        }
        
        potential_deadlocks
    }
    
    fn find_suspected_resources(&self, blocked_goroutines: &[u64]) -> Vec<u64> {
        let mut suspected_channels = Vec::new();
        
        // Find channels that blocked goroutines were waiting on
        for event in &self.channel_events {
            if blocked_goroutines.contains(&event.goroutine_id) {
                if matches!(event.operation, ChannelOperation::Send(_) | ChannelOperation::Receive) {
                    if let Some(duration) = event.duration {
                        if duration > Duration::from_millis(100) {
                            suspected_channels.push(event.channel_id);
                        }
                    }
                }
            }
        }
        
        suspected_channels
    }
    
    pub fn analyze_scheduler(&self) -> SchedulerAnalysis {
        let mut context_switches = 0;
        let mut total_scheduling_time = Duration::default();
        let mut goroutine_wait_times = Vec::new();
        
        for event in &self.scheduler_events {
            match &event.event_type {
                SchedulerEventType::ContextSwitch { .. } => {
                    context_switches += 1;
                    total_scheduling_time += event.duration;
                }
                SchedulerEventType::GoroutineQueued { wait_time, .. } => {
                    if let Some(wait) = wait_time {
                        goroutine_wait_times.push(*wait);
                    }
                }
                _ => {}
            }
        }
        
        let average_wait_time = if !goroutine_wait_times.is_empty() {
            goroutine_wait_times.iter().sum::<Duration>() / goroutine_wait_times.len() as u32
        } else {
            Duration::default()
        };
        
        SchedulerAnalysis {
            context_switches,
            average_scheduling_overhead: if context_switches > 0 {
                total_scheduling_time / context_switches as u32
            } else {
                Duration::default()
            },
            average_goroutine_wait_time: average_wait_time,
            scheduler_efficiency: Self::calculate_scheduler_efficiency(context_switches, &goroutine_wait_times),
        }
    }
    
    fn calculate_scheduler_efficiency(context_switches: u64, wait_times: &[Duration]) -> f64 {
        if context_switches == 0 || wait_times.is_empty() {
            return 1.0;
        }
        
        let average_wait = wait_times.iter().sum::<Duration>().as_secs_f64() / wait_times.len() as f64;
        let switch_rate = context_switches as f64;
        
        // Efficiency decreases with longer wait times and higher switch rates
        (1.0 / (1.0 + average_wait * 0.001 + switch_rate * 0.0001)).clamp(0.0, 1.0)
    }
}

/// Goroutine lifecycle event
#[derive(Debug, Clone, Serialize)]
pub struct GoroutineEvent {
    pub event_type: GoroutineEventType,
    pub goroutine_id: u64,
    pub parent_id: Option<u64>,
    #[serde(skip)]
    pub timestamp: Instant,
    pub stack_trace: Vec<String>,
    pub metadata: HashMap<String, String>,
}

/// Types of goroutine events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoroutineEventType {
    Spawn,
    Complete(GoroutineCompletionType),
    Yield,
    Block,
    Resume,
}

/// How a goroutine completed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoroutineCompletionType {
    Normal,
    Panic(String),
    Canceled,
}

/// Channel operation event
#[derive(Debug, Clone, Serialize)]
pub struct ChannelEvent {
    pub channel_id: u64,
    pub operation: ChannelOperation,
    pub goroutine_id: u64,
    #[serde(skip)]
    pub timestamp: Instant,
    pub duration: Option<Duration>,
    pub buffer_size: usize,
    pub blocked_goroutines: u64,
}

/// Types of channel operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelOperation {
    Send(String), // Simplified - would contain actual data type info
    Receive,
    Close,
    Select,
}

/// Scheduler event for performance analysis
#[derive(Debug, Clone, Serialize)]
pub struct SchedulerEvent {
    pub event_type: SchedulerEventType,
    #[serde(skip)]
    pub timestamp: Instant,
    pub duration: Duration,
    pub processor_id: u32,
}

/// Types of scheduler events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchedulerEventType {
    ContextSwitch {
        from_goroutine: u64,
        to_goroutine: u64,
    },
    GoroutineQueued {
        goroutine_id: u64,
        wait_time: Option<Duration>,
    },
    ProcessorIdle {
        processor_id: u32,
    },
    ProcessorActive {
        processor_id: u32,
    },
}

/// Goroutine timeline visualization data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoroutineTimeline {
    pub goroutine_id: u64,
    pub parent_id: Option<u64>,
    #[serde(skip)]
    pub spawn_time: Option<Instant>,
    #[serde(skip)]
    pub completion_time: Option<Instant>,
    pub lifetime: Option<Duration>,
    pub yield_count: u64,
    pub block_count: u64,
    pub resume_count: u64,
}

impl GoroutineTimeline {
    pub fn new(goroutine_id: u64) -> Self {
        Self {
            goroutine_id,
            parent_id: None,
            spawn_time: None,
            completion_time: None,
            lifetime: None,
            yield_count: 0,
            block_count: 0,
            resume_count: 0,
        }
    }
}

/// Channel usage analysis
#[derive(Debug, Clone, Default, Serialize)]
pub struct ChannelAnalysis {
    pub channel_stats: HashMap<u64, ChannelStats>,
    #[serde(skip)]
    pub operation_patterns: Vec<ChannelOperationPattern>,
    pub total_channels: u64,
    pub total_operations: u64,
}

/// Statistics for individual channels
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChannelStats {
    pub send_count: u64,
    pub receive_count: u64,
    pub close_count: u64,
    pub select_count: u64,
    pub total_send_time: Duration,
    pub total_receive_time: Duration,
    pub max_blocked_goroutines: u64,
}

impl ChannelStats {
    pub fn throughput(&self) -> f64 {
        let total_ops = self.send_count + self.receive_count;
        let total_time = self.total_send_time + self.total_receive_time;
        
        if total_time.as_secs_f64() > 0.0 {
            total_ops as f64 / total_time.as_secs_f64()
        } else {
            0.0
        }
    }
}

/// Channel operation pattern for analysis
#[derive(Debug, Clone, Serialize)]
pub struct ChannelOperationPattern {
    #[serde(skip)]
    pub timestamp: Instant,
    pub channel_id: u64,
    pub operation_type: ChannelOperation,
    pub blocking_time: Option<Duration>,
    pub queue_length: usize,
}

/// Deadlock detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadlockDetection {
    pub detection_time: SystemTime,
    pub involved_goroutines: Vec<u64>,
    pub suspected_resources: Vec<u64>,
    pub confidence: f64,
    pub description: String,
}

/// Scheduler performance analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchedulerAnalysis {
    pub context_switches: u64,
    pub average_scheduling_overhead: Duration,
    pub average_goroutine_wait_time: Duration,
    pub scheduler_efficiency: f64,
}

/// Goroutine state tracking
#[derive(Debug, Clone, PartialEq, Eq)]
enum GoroutineState {
    Running,
    Blocked,
    Completed,
}

/// Goroutine tracking system
#[derive(Debug)]
struct GoroutineTracker {
    active_goroutines: Arc<RwLock<HashMap<u64, GoroutineInfo>>>,
    tracking: bool,
}

impl GoroutineTracker {
    fn new() -> Self {
        Self {
            active_goroutines: Arc::new(RwLock::new(HashMap::new())),
            tracking: false,
        }
    }
    
    fn start_tracking(&mut self) -> crate::error::Result<()> {
        self.tracking = true;
        Ok(())
    }
    
    fn stop_tracking(&mut self) -> crate::error::Result<()> {
        self.tracking = false;
        Ok(())
    }
    
    fn track_spawn(&self, goroutine_id: u64, parent_id: Option<u64>) -> crate::error::Result<()> {
        if !self.tracking {
            return Ok(());
        }
        
        let info = GoroutineInfo {
            id: goroutine_id,
            parent_id,
            spawn_time: Instant::now(),
            state: GoroutineState::Running,
        };
        
        if let Ok(mut goroutines) = self.active_goroutines.write() {
            goroutines.insert(goroutine_id, info);
        }
        
        Ok(())
    }
    
    fn track_completion(&self, goroutine_id: u64) -> crate::error::Result<()> {
        if !self.tracking {
            return Ok(());
        }
        
        if let Ok(mut goroutines) = self.active_goroutines.write() {
            goroutines.remove(&goroutine_id);
        }
        
        Ok(())
    }
}

/// Information about an active goroutine
#[derive(Debug, Clone)]
struct GoroutineInfo {
    id: u64,
    parent_id: Option<u64>,
    spawn_time: Instant,
    state: GoroutineState,
}

/// Channel tracking system
#[derive(Debug)]
struct ChannelTracker {
    active_channels: Arc<RwLock<HashMap<u64, ChannelInfo>>>,
    tracking: bool,
}

impl ChannelTracker {
    fn new() -> Self {
        Self {
            active_channels: Arc::new(RwLock::new(HashMap::new())),
            tracking: false,
        }
    }
    
    fn start_tracking(&mut self) -> crate::error::Result<()> {
        self.tracking = true;
        Ok(())
    }
    
    fn stop_tracking(&mut self) -> crate::error::Result<()> {
        self.tracking = false;
        Ok(())
    }
    
    fn track_operation(&self, event: &ChannelEvent) -> crate::error::Result<()> {
        if !self.tracking {
            return Ok(());
        }
        
        if let Ok(mut channels) = self.active_channels.write() {
            let info = channels
                .entry(event.channel_id)
                .or_insert_with(|| ChannelInfo::new(event.channel_id));
            
            info.last_operation = Some(event.timestamp);
            info.operation_count += 1;
        }
        
        Ok(())
    }
    
    fn get_buffer_size(&self, channel_id: u64) -> usize {
        if let Ok(channels) = self.active_channels.read() {
            channels.get(&channel_id)
                .map(|info| info.buffer_size)
                .unwrap_or(0)
        } else {
            0
        }
    }
    
    fn get_blocked_count(&self, _channel_id: u64) -> u64 {
        // In a real implementation, this would track blocked goroutines
        0
    }
}

/// Information about an active channel
#[derive(Debug, Clone)]
struct ChannelInfo {
    id: u64,
    buffer_size: usize,
    last_operation: Option<Instant>,
    operation_count: u64,
}

impl ChannelInfo {
    fn new(id: u64) -> Self {
        Self {
            id,
            buffer_size: 0,
            last_operation: None,
            operation_count: 0,
        }
    }
}

