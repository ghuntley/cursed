use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, Instant};
use std::thread;

use crate::error::CursedError;

/// Event identifier
pub type EventId = u64;

/// Event priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl Default for EventPriority {
    fn default() -> Self {
        EventPriority::Normal
    }
}

/// Event callback type
pub type EventCallback = Box<dyn FnOnce() + Send + 'static>;

/// Event loop event
pub struct Event {
    pub id: EventId,
    pub priority: EventPriority,
    pub callback: EventCallback,
    pub created_at: Instant,
    pub scheduled_for: Option<Instant>,
}

impl Event {
    pub fn new<F>(id: EventId, callback: F) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        Self {
            id,
            priority: EventPriority::default(),
            callback: Box::new(callback),
            created_at: Instant::now(),
            scheduled_for: None,
        }
    }

    pub fn with_priority<F>(id: EventId, callback: F, priority: EventPriority) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        let mut event = Self::new(id, callback);
        event.priority = priority;
        event
    }

    pub fn scheduled<F>(id: EventId, callback: F, when: Instant) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        let mut event = Self::new(id, callback);
        event.scheduled_for = Some(when);
        event
    }

    pub fn is_ready(&self) -> bool {
        match self.scheduled_for {
            Some(when) => Instant::now() >= when,
            None => true,
        }
    }
}

/// Event loop configuration
#[derive(Debug, Clone)]
pub struct EventLoopConfig {
    pub max_events_per_tick: usize,
    pub tick_duration: Duration,
    pub enable_priority_scheduling: bool,
    pub enable_event_batching: bool,
    pub max_batch_size: usize,
    pub enable_metrics: bool,
}

impl Default for EventLoopConfig {
    fn default() -> Self {
        Self {
            max_events_per_tick: 1000,
            tick_duration: Duration::from_millis(1),
            enable_priority_scheduling: true,
            enable_event_batching: true,
            max_batch_size: 100,
            enable_metrics: true,
        }
    }
}

/// Event loop statistics
#[derive(Debug, Default, Clone)]
pub struct EventLoopStats {
    pub events_scheduled: u64,
    pub events_executed: u64,
    pub events_dropped: u64,
    pub total_ticks: u64,
    pub average_events_per_tick: f64,
    pub max_events_in_tick: usize,
    pub total_execution_time: Duration,
    pub average_tick_duration: Duration,
    pub uptime: Duration,
    pub started_at: Option<Instant>,
}

/// High-performance event loop for async I/O and callbacks
pub struct EventLoop {
    config: EventLoopConfig,
    next_event_id: AtomicU64,
    events: Arc<Mutex<HashMap<EventId, Event>>>,
    ready_events: Arc<Mutex<Vec<EventId>>>,
    high_priority_events: Arc<Mutex<Vec<EventId>>>,
    stats: Arc<Mutex<EventLoopStats>>,
    running: AtomicBool,
    shutdown: AtomicBool,
    thread_handle: Mutex<Option<thread::JoinHandle<()>>>,
}

impl EventLoop {
    /// Create a new event loop
    pub fn new() -> Result<Self, CursedError> {
        Self::with_config(EventLoopConfig::default())
    }

    /// Create a new event loop with custom configuration
    pub fn with_config(config: EventLoopConfig) -> Result<Self, CursedError> {
        let mut stats = EventLoopStats::default();
        stats.started_at = Some(Instant::now());

        Ok(Self {
            config,
            next_event_id: AtomicU64::new(1),
            events: Arc::new(Mutex::new(HashMap::new())),
            ready_events: Arc::new(Mutex::new(Vec::new())),
            high_priority_events: Arc::new(Mutex::new(Vec::new())),
            stats: Arc::new(Mutex::new(stats)),
            running: AtomicBool::new(false),
            shutdown: AtomicBool::new(false),
            thread_handle: Mutex::new(None),
        })
    }

    /// Start the event loop
    pub fn start(&self) -> Result<(), CursedError> {
        if self.running.swap(true, Ordering::SeqCst) {
            return Err(CursedError::runtime_error("Event loop is already running"));
        }

        let events = self.events.clone();
        let ready_events = self.ready_events.clone();
        let high_priority_events = self.high_priority_events.clone();
        let stats = self.stats.clone();
        let config = self.config.clone();
        let running = self.running.clone();
        let shutdown = self.shutdown.clone();

        let handle = thread::spawn(move || {
            Self::run_loop(
                events,
                ready_events,
                high_priority_events,
                stats,
                config,
                running,
                shutdown,
            );
        });

        *self.thread_handle.lock().unwrap() = Some(handle);
        Ok(())
    }

    /// Stop the event loop
    pub fn stop(&self) -> Result<(), CursedError> {
        if !self.running.swap(false, Ordering::SeqCst) {
            return Ok(()); // Already stopped
        }

        self.shutdown.store(true, Ordering::SeqCst);

        // Wait for thread to finish
        if let Some(handle) = self.thread_handle.lock().unwrap().take() {
            handle.join().map_err(|_| CursedError::runtime_error("Failed to join event loop thread"))?;
        }

        Ok(())
    }

    /// Schedule an event for immediate execution
    pub fn schedule<F>(&self, callback: F) -> EventId
    where
        F: FnOnce() + Send + 'static,
    {
        self.schedule_with_priority(callback, EventPriority::Normal)
    }

    /// Schedule an event with specific priority
    pub fn schedule_with_priority<F>(&self, callback: F, priority: EventPriority) -> EventId
    where
        F: FnOnce() + Send + 'static,
    {
        let event_id = self.next_event_id.fetch_add(1, Ordering::SeqCst);
        let event = Event::with_priority(event_id, callback, priority);

        // Store the event
        self.events.lock().unwrap().insert(event_id, event);

        // Queue for execution
        match priority {
            EventPriority::High | EventPriority::Critical => {
                self.high_priority_events.lock().unwrap().push(event_id);
            }
            _ => {
                self.ready_events.lock().unwrap().push(event_id);
            }
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.events_scheduled += 1;
        }

        event_id
    }

    /// Schedule an event for execution at a specific time
    pub fn schedule_at<F>(&self, callback: F, when: Instant) -> EventId
    where
        F: FnOnce() + Send + 'static,
    {
        let event_id = self.next_event_id.fetch_add(1, Ordering::SeqCst);
        let event = Event::scheduled(event_id, callback, when);

        self.events.lock().unwrap().insert(event_id, event);

        // Don't queue immediately - will be queued when ready
        {
            let mut stats = self.stats.lock().unwrap();
            stats.events_scheduled += 1;
        }

        event_id
    }

    /// Schedule an event for execution after a delay
    pub fn schedule_after<F>(&self, callback: F, delay: Duration) -> EventId
    where
        F: FnOnce() + Send + 'static,
    {
        let when = Instant::now() + delay;
        self.schedule_at(callback, when)
    }

    /// Cancel a scheduled event
    pub fn cancel_event(&self, event_id: EventId) -> bool {
        let removed = self.events.lock().unwrap().remove(&event_id).is_some();
        
        if removed {
            // Remove from ready queues if present
            self.ready_events.lock().unwrap().retain(|&id| id != event_id);
            self.high_priority_events.lock().unwrap().retain(|&id| id != event_id);
            
            let mut stats = self.stats.lock().unwrap();
            stats.events_dropped += 1;
        }
        
        removed
    }

    /// Get event loop statistics
    pub fn get_stats(&self) -> EventLoopStats {
        let mut stats = self.stats.lock().unwrap().clone();
        
        if let Some(started_at) = stats.started_at {
            stats.uptime = started_at.elapsed();
        }

        if stats.total_ticks > 0 {
            stats.average_events_per_tick = stats.events_executed as f64 / stats.total_ticks as f64;
            stats.average_tick_duration = stats.total_execution_time / stats.total_ticks as u32;
        }

        stats
    }

    /// Check if event loop is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    // Private methods

    fn run_loop(
        events: Arc<Mutex<HashMap<EventId, Event>>>,
        ready_events: Arc<Mutex<Vec<EventId>>>,
        high_priority_events: Arc<Mutex<Vec<EventId>>>,
        stats: Arc<Mutex<EventLoopStats>>,
        config: EventLoopConfig,
        running: AtomicBool,
        shutdown: AtomicBool,
    ) {
        let mut tick_timer = Instant::now();
        
        while running.load(Ordering::SeqCst) && !shutdown.load(Ordering::SeqCst) {
            let tick_start = Instant::now();
            let mut events_processed = 0;

            // Check for ready scheduled events
            Self::update_ready_events(&events, &ready_events);

            // Process high priority events first
            let high_priority_batch = {
                let mut queue = high_priority_events.lock().unwrap();
                let batch_size = config.max_batch_size.min(queue.len());
                queue.drain(..batch_size).collect::<Vec<_>>()
            };

            for event_id in high_priority_batch {
                if Self::execute_event(event_id, &events) {
                    events_processed += 1;
                }
                
                if events_processed >= config.max_events_per_tick {
                    break;
                }
            }

            // Process regular events if we haven't hit the limit
            if events_processed < config.max_events_per_tick {
                let regular_batch = {
                    let mut queue = ready_events.lock().unwrap();
                    let remaining_capacity = config.max_events_per_tick - events_processed;
                    let batch_size = config.max_batch_size.min(queue.len()).min(remaining_capacity);
                    queue.drain(..batch_size).collect::<Vec<_>>()
                };

                for event_id in regular_batch {
                    if Self::execute_event(event_id, &events) {
                        events_processed += 1;
                    }
                }
            }

            let tick_duration = tick_start.elapsed();

            // Update statistics
            {
                let mut stats_guard = stats.lock().unwrap();
                stats_guard.total_ticks += 1;
                stats_guard.events_executed += events_processed as u64;
                stats_guard.total_execution_time += tick_duration;
                
                if events_processed > stats_guard.max_events_in_tick {
                    stats_guard.max_events_in_tick = events_processed;
                }
            }

            // Sleep for the remainder of the tick duration
            let elapsed = tick_timer.elapsed();
            if elapsed < config.tick_duration {
                thread::sleep(config.tick_duration - elapsed);
            }
            tick_timer = Instant::now();
        }
    }

    fn update_ready_events(
        events: &Arc<Mutex<HashMap<EventId, Event>>>,
        ready_events: &Arc<Mutex<Vec<EventId>>>,
    ) {
        let now = Instant::now();
        let mut events_to_queue = Vec::new();
        
        // Find scheduled events that are now ready
        {
            let events_guard = events.lock().unwrap();
            for (event_id, event) in events_guard.iter() {
                if let Some(scheduled_for) = event.scheduled_for {
                    if now >= scheduled_for {
                        events_to_queue.push(*event_id);
                    }
                }
            }
        }

        // Move ready events to the ready queue
        if !events_to_queue.is_empty() {
            let mut ready_queue = ready_events.lock().unwrap();
            ready_queue.extend(events_to_queue);
        }
    }

    fn execute_event(
        event_id: EventId,
        events: &Arc<Mutex<HashMap<EventId, Event>>>,
    ) -> bool {
        // Remove and execute the event
        let event = {
            let mut events_guard = events.lock().unwrap();
            events_guard.remove(&event_id)
        };

        if let Some(event) = event {
            // Execute the callback
            (event.callback)();
            true
        } else {
            false
        }
    }
}

/// Global event loop instance
static GLOBAL_EVENT_LOOP: once_cell::sync::OnceCell<Arc<EventLoop>> = once_cell::sync::OnceCell::new();

/// Initialize the global event loop
pub fn initialize_global_event_loop() -> Result<(), CursedError> {
    initialize_global_event_loop_with_config(EventLoopConfig::default())
}

/// Initialize the global event loop with custom configuration
pub fn initialize_global_event_loop_with_config(config: EventLoopConfig) -> Result<(), CursedError> {
    let event_loop = Arc::new(EventLoop::with_config(config)?);
    
    GLOBAL_EVENT_LOOP
        .set(event_loop.clone())
        .map_err(|_| CursedError::runtime_error("Global event loop already initialized"))?;

    event_loop.start()?;
    Ok(())
}

/// Get the global event loop
pub fn get_global_event_loop() -> Option<Arc<EventLoop>> {
    GLOBAL_EVENT_LOOP.get().cloned()
}

/// Shutdown the global event loop
pub fn shutdown_global_event_loop() -> Result<(), CursedError> {
    if let Some(event_loop) = get_global_event_loop() {
        event_loop.stop()
    } else {
        Ok(())
    }
}

/// Schedule an event on the global event loop
pub fn schedule<F>(callback: F) -> Result<EventId, CursedError>
where
    F: FnOnce() + Send + 'static,
{
    get_global_event_loop()
        .map(|event_loop| event_loop.schedule(callback))
        .ok_or_else(|| CursedError::runtime_error("Global event loop not initialized"))
}

/// Schedule an event with priority on the global event loop
pub fn schedule_with_priority<F>(callback: F, priority: EventPriority) -> Result<EventId, CursedError>
where
    F: FnOnce() + Send + 'static,
{
    get_global_event_loop()
        .map(|event_loop| event_loop.schedule_with_priority(callback, priority))
        .ok_or_else(|| CursedError::runtime_error("Global event loop not initialized"))
}

/// Schedule an event after a delay on the global event loop
pub fn schedule_after<F>(callback: F, delay: Duration) -> Result<EventId, CursedError>
where
    F: FnOnce() + Send + 'static,
{
    get_global_event_loop()
        .map(|event_loop| event_loop.schedule_after(callback, delay))
        .ok_or_else(|| CursedError::runtime_error("Global event loop not initialized"))
}
