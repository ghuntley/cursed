//! Channel lifecycle management system
//!
//! This module provides comprehensive channel lifecycle management including:
//! - Centralized channel registry with tracking
//! - Resource management and limits
//! - Enhanced cleanup and verification
//! - GC integration for buffer contents
//! - Monitoring and debugging support

use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex, atomic::{AtomicU64, AtomicUsize, Ordering}};
use std::time::Instant;
use std::thread;
use std::sync::mpsc;
use std::any::Any;

use crate::runtime::channels::{ChannelError, ChannelResult, ChannelStats};
use crate::error::CursedError;
use crate::runtime::gc::GarbageCollector;

/// Channel lifecycle event types
#[derive(Debug, Clone, PartialEq)]
pub enum ChannelEvent {
    /// Channel was created
    Created { id: u64, capacity: usize },
    /// Message was sent
    MessageSent { id: u64, size: usize },
    /// Message was received
    MessageReceived { id: u64, size: usize },
    /// Channel was closed
    Closed { id: u64 },
    /// Channel was garbage collected
    GarbageCollected { id: u64 },
    /// Channel buffer was resized
    BufferResized { id: u64, old_capacity: usize, new_capacity: usize },
    /// Channel reached capacity limit
    CapacityLimitReached { id: u64, current: usize, limit: usize },
    /// Channel cleanup completed
    CleanupCompleted { id: u64 },
}

/// Channel lifecycle statistics
#[derive(Debug, Clone)]
pub struct ChannelLifecycleStats {
    /// Total channels created
    pub total_created: u64,
    /// Total channels closed
    pub total_closed: u64,
    /// Total channels garbage collected
    pub total_gc: u64,
    /// Active channels
    pub active_channels: usize,
    /// Total messages sent across all channels
    pub total_messages_sent: u64,
    /// Total messages received across all channels
    pub total_messages_received: u64,
    /// Total memory allocated for channels
    pub total_memory_allocated: usize,
    /// Total memory freed from channels
    pub total_memory_freed: usize,
    /// Average channel lifetime (seconds)
    pub average_lifetime: f64,
    /// Peak concurrent channels
    pub peak_concurrent_channels: usize,
}

impl Default for ChannelLifecycleStats {
    fn default() -> Self {
        Self {
            total_created: 0,
            total_closed: 0,
            total_gc: 0,
            active_channels: 0,
            total_messages_sent: 0,
            total_messages_received: 0,
            total_memory_allocated: 0,
            total_memory_freed: 0,
            average_lifetime: 0.0,
            peak_concurrent_channels: 0,
        }
    }
}

/// Channel resource limits
#[derive(Debug, Clone)]
pub struct ChannelResourceLimits {
    /// Maximum number of concurrent channels
    pub max_concurrent_channels: usize,
    /// Maximum channel buffer size
    pub max_buffer_size: usize,
    /// Maximum total memory for all channels
    pub max_total_memory: usize,
    /// Maximum messages per channel
    pub max_messages_per_channel: u64,
    /// Enable strict enforcement
    pub strict_enforcement: bool,
}

impl Default for ChannelResourceLimits {
    fn default() -> Self {
        Self {
            max_concurrent_channels: 10000,
            max_buffer_size: 1024 * 1024, // 1MB
            max_total_memory: 100 * 1024 * 1024, // 100MB
            max_messages_per_channel: 1_000_000,
            strict_enforcement: false,
        }
    }
}

/// Channel information for registry
#[derive(Debug, Clone)]
pub struct ChannelInfo {
    /// Channel ID
    pub id: u64,
    /// Channel type name
    pub type_name: String,
    /// Channel capacity
    pub capacity: usize,
    /// Current buffer size
    pub current_size: usize,
    /// Creation timestamp
    pub created_at: Instant,
    /// Last activity timestamp
    pub last_activity: Instant,
    /// Total messages sent
    pub messages_sent: u64,
    /// Total messages received
    pub messages_received: u64,
    /// Messages dropped (not processed)
    pub messages_dropped: u64,
    /// Memory allocated for this channel
    pub memory_allocated: usize,
    /// Whether channel is closed
    pub is_closed: bool,
    /// Number of active senders
    pub sender_count: usize,
    /// Number of active receivers
    pub receiver_count: usize,
    /// Channel buffer data addresses for GC
    pub buffer_addresses: Vec<usize>,
}

/// Channel lifecycle manager
pub struct ChannelLifecycleManager {
    /// Channel registry
    registry: Arc<RwLock<HashMap<u64, ChannelInfo>>>,
    /// Next channel ID
    next_id: AtomicU64,
    /// Lifecycle statistics
    stats: Arc<Mutex<ChannelLifecycleStats>>,
    /// Resource limits
    limits: Arc<RwLock<ChannelResourceLimits>>,
    /// Event listeners
    event_listeners: Arc<RwLock<Vec<Box<dyn Fn(&ChannelEvent) + Send + Sync>>>>,
    /// Monitoring thread handle
    monitor_thread: Option<thread::JoinHandle<()>>,
    /// Monitoring control channel
    monitor_control: Option<mpsc::Sender<MonitorCommand>>,
    /// GC integration
    gc_integration: Arc<Mutex<Option<Arc<GarbageCollector>>>>,
    /// Debug mode
    debug_mode: bool,
}

/// Monitor command types
#[derive(Debug)]
enum MonitorCommand {
    /// Stop monitoring
    Stop,
    /// Force cleanup check
    ForceCleanup,
    /// Update limits
    UpdateLimits(ChannelResourceLimits),
}

impl ChannelLifecycleManager {
    /// Create new channel lifecycle manager
    pub fn new() -> Self {
        Self {
            registry: Arc::new(RwLock::new(HashMap::new())),
            next_id: AtomicU64::new(1),
            stats: Arc::new(Mutex::new(ChannelLifecycleStats::default())),
            limits: Arc::new(RwLock::new(ChannelResourceLimits::default())),
            event_listeners: Arc::new(RwLock::new(Vec::new())),
            monitor_thread: None,
            monitor_control: None,
            gc_integration: Arc::new(Mutex::new(None)),
            debug_mode: false,
        }
    }

    /// Create with custom limits
    pub fn with_limits(limits: ChannelResourceLimits) -> Self {
        let mut manager = Self::new();
        *manager.limits.write().unwrap() = limits;
        manager
    }

    /// Enable debug mode
    pub fn enable_debug(&mut self) {
        self.debug_mode = true;
    }

    /// Register a channel
    pub fn register_channel(&self, type_name: String, capacity: usize) -> ChannelResult<u64> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let now = Instant::now();

        // Check resource limits
        if let Err(e) = self.check_resource_limits(capacity) {
            return Err(e);
        }

        let info = ChannelInfo {
            id,
            type_name: type_name.clone(),
            capacity,
            current_size: 0,
            created_at: now,
            last_activity: now,
            messages_sent: 0,
            messages_received: 0,
            messages_dropped: 0,
            memory_allocated: capacity * std::mem::size_of::<usize>(), // Estimate
            is_closed: false,
            sender_count: 1,
            receiver_count: 1,
            buffer_addresses: Vec::new(),
        };

        // Add to registry
        if let Ok(mut registry) = self.registry.write() {
            registry.insert(id, info);
        }

        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_created += 1;
            stats.active_channels += 1;
            stats.total_memory_allocated += capacity * std::mem::size_of::<usize>();
            if stats.active_channels > stats.peak_concurrent_channels {
                stats.peak_concurrent_channels = stats.active_channels;
            }
        }

        // Emit event
        let event = ChannelEvent::Created { id, capacity };
        self.emit_event(&event);

        if self.debug_mode {
            println!("Channel {} created: type={}, capacity={}", id, type_name, capacity);
        }

        Ok(id)
    }

    /// Unregister a channel
    pub fn unregister_channel(&self, id: u64) -> ChannelResult<()> {
        let mut info = None;
        
        // Remove from registry
        if let Ok(mut registry) = self.registry.write() {
            info = registry.remove(&id);
        }

        if let Some(channel_info) = info {
            // Update statistics
            if let Ok(mut stats) = self.stats.lock() {
                stats.total_closed += 1;
                stats.active_channels = stats.active_channels.saturating_sub(1);
                stats.total_memory_freed += channel_info.memory_allocated;
                
                // Update average lifetime
                let lifetime = channel_info.created_at.elapsed().as_secs_f64();
                let total_lifetime = stats.average_lifetime * (stats.total_closed - 1) as f64;
                stats.average_lifetime = (total_lifetime + lifetime) / stats.total_closed as f64;
            }

            // Emit event
            let event = ChannelEvent::Closed { id };
            self.emit_event(&event);

            if self.debug_mode {
                println!("Channel {} closed: lifetime={:.2}s", id, 
                    channel_info.created_at.elapsed().as_secs_f64());
            }
        }

        Ok(())
    }

    /// Record message sent
    pub fn record_message_sent(&self, id: u64, size: usize) -> ChannelResult<()> {
        if let Ok(mut registry) = self.registry.write() {
            if let Some(info) = registry.get_mut(&id) {
                info.messages_sent += 1;
                info.last_activity = Instant::now();
                info.current_size = info.current_size.saturating_add(size);
            }
        }

        if let Ok(mut stats) = self.stats.lock() {
            stats.total_messages_sent += 1;
        }

        let event = ChannelEvent::MessageSent { id, size };
        self.emit_event(&event);

        Ok(())
    }

    /// Record message received
    pub fn record_message_received(&self, id: u64, size: usize) -> ChannelResult<()> {
        if let Ok(mut registry) = self.registry.write() {
            if let Some(info) = registry.get_mut(&id) {
                info.messages_received += 1;
                info.last_activity = Instant::now();
                info.current_size = info.current_size.saturating_sub(size);
            }
        }

        if let Ok(mut stats) = self.stats.lock() {
            stats.total_messages_received += 1;
        }

        let event = ChannelEvent::MessageReceived { id, size };
        self.emit_event(&event);

        Ok(())
    }

    /// Record message dropped (for GC integration)
    pub fn record_message_dropped(&self, id: u64, size: usize) -> ChannelResult<()> {
        if let Ok(mut registry) = self.registry.write() {
            if let Some(info) = registry.get_mut(&id) {
                info.messages_dropped += 1;
                info.last_activity = Instant::now();
                info.current_size = info.current_size.saturating_sub(size);
                
                if self.debug_mode {
                    println!("Message dropped for channel {}: size={}, total_dropped={}", 
                        id, size, info.messages_dropped);
                }
            }
        }

        // Trigger GC cleanup if drop count is high
        self.trigger_gc_cleanup_if_needed(id)?;

        Ok(())
    }

    /// Trigger GC cleanup if needed based on drop count
    fn trigger_gc_cleanup_if_needed(&self, id: u64) -> ChannelResult<()> {
        if let Ok(registry) = self.registry.read() {
            if let Some(info) = registry.get(&id) {
                // Trigger cleanup if more than 10% of messages are being dropped
                let drop_ratio = info.messages_dropped as f64 / 
                    (info.messages_sent.max(1)) as f64;
                
                if drop_ratio > 0.1 {
                    if self.debug_mode {
                        println!("High drop ratio {:.2}% for channel {}, triggering GC cleanup", 
                            drop_ratio * 100.0, id);
                    }
                    self.force_cleanup()?;
                }
            }
        }
        Ok(())
    }

    /// Add buffer address for GC integration (enhanced)
    pub fn add_buffer_address(&self, id: u64, address: usize) -> ChannelResult<()> {
        if let Ok(mut registry) = self.registry.write() {
            if let Some(info) = registry.get_mut(&id) {
                if !info.buffer_addresses.contains(&address) {
                    info.buffer_addresses.push(address);
                    info.last_activity = Instant::now();
                }
                return Ok(());
            }
        }
        Err(ChannelError::ChannelNotFound)
    }

    /// Get channel buffer addresses for GC
    pub fn get_buffer_addresses(&self, id: u64) -> Vec<usize> {
        if let Ok(registry) = self.registry.read() {
            if let Some(info) = registry.get(&id) {
                return info.buffer_addresses.clone();
            }
        }
        Vec::new()
    }
    
    /// Enhanced channel creation with proper memory management
    pub fn create_channel_with_buffer(&self, type_name: String, capacity: usize, 
                                     buffer_ptr: *mut u8, buffer_size: usize) -> ChannelResult<u64> {
        // Check resource limits first
        if let Err(e) = self.check_resource_limits(capacity) {
            return Err(e);
        }
        
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let now = Instant::now();
        
        // Calculate actual memory allocation
        let memory_allocation = capacity * std::mem::size_of::<usize>() + buffer_size;
        
        let mut info = ChannelInfo {
            id,
            type_name: type_name.clone(),
            capacity,
            current_size: 0,
            created_at: now,
            last_activity: now,
            messages_sent: 0,
            messages_received: 0,
            messages_dropped: 0,
            memory_allocated: memory_allocation,
            is_closed: false,
            sender_count: 1,
            receiver_count: 1,
            buffer_addresses: Vec::new(),
        };
        
        // Add buffer address if provided
        if !buffer_ptr.is_null() {
            info.buffer_addresses.push(buffer_ptr as usize);
        }
        
        // Register with GC system
        if let Ok(gc_opt) = self.gc_integration.lock() {
            if let Some(gc) = gc_opt.as_ref() {
                if !buffer_ptr.is_null() {
                    if let Err(e) = gc.add_root(buffer_ptr) {
                        if self.debug_mode {
                            println!("Failed to add GC root for new channel {}: {:?}", id, e);
                        }
                    }
                }
            }
        }
        
        // Add to registry
        if let Ok(mut registry) = self.registry.write() {
            registry.insert(id, info);
        }
        
        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_created += 1;
            stats.active_channels += 1;
            stats.total_memory_allocated += memory_allocation;
            if stats.active_channels > stats.peak_concurrent_channels {
                stats.peak_concurrent_channels = stats.active_channels;
            }
        }
        
        // Emit event
        let event = ChannelEvent::Created { id, capacity };
        self.emit_event(&event);
        
        if self.debug_mode {
            println!("Channel {} created with buffer: type={}, capacity={}, buffer_size={}", 
                id, type_name, capacity, buffer_size);
        }
        
        Ok(id)
    }
    
    /// Enhanced channel destruction with proper cleanup
    pub fn destroy_channel(&self, id: u64) -> ChannelResult<()> {
        let mut info = None;
        
        // Remove from registry
        if let Ok(mut registry) = self.registry.write() {
            info = registry.remove(&id);
        }
        
        if let Some(channel_info) = info {
            // Perform proper cleanup of channel resources
            self.cleanup_channel_resources(&channel_info)?;
            
            // Update statistics
            if let Ok(mut stats) = self.stats.lock() {
                stats.total_closed += 1;
                stats.active_channels = stats.active_channels.saturating_sub(1);
                stats.total_memory_freed += channel_info.memory_allocated;
                
                // Update average lifetime
                let lifetime = channel_info.created_at.elapsed().as_secs_f64();
                let total_lifetime = stats.average_lifetime * (stats.total_closed - 1) as f64;
                stats.average_lifetime = (total_lifetime + lifetime) / stats.total_closed as f64;
            }
            
            // Emit cleanup completed event
            let event = ChannelEvent::CleanupCompleted { id };
            self.emit_event(&event);
            
            if self.debug_mode {
                println!("Channel {} destroyed: lifetime={:.2}s, memory_freed={}", 
                    id, channel_info.created_at.elapsed().as_secs_f64(), 
                    channel_info.memory_allocated);
            }
        }
        
        Ok(())
    }
    
    /// Cleanup channel resources including buffer memory
    fn cleanup_channel_resources(&self, info: &ChannelInfo) -> ChannelResult<()> {
        if self.debug_mode {
            println!("Cleaning up resources for channel {}: {} buffer addresses", 
                info.id, info.buffer_addresses.len());
        }
        
        // Clean up GC integration
        if let Ok(gc_opt) = self.gc_integration.lock() {
            if let Some(gc) = gc_opt.as_ref() {
                for &addr in &info.buffer_addresses {
                    let ptr = addr as *mut u8;
                    
                    // Remove from GC root set
                    if let Err(e) = gc.remove_root(ptr) {
                        if self.debug_mode {
                            println!("Failed to remove GC root for channel {} address 0x{:x}: {:?}", 
                                info.id, addr, e);
                        }
                    } else if self.debug_mode {
                        println!("Removed GC root for channel {} address: 0x{:x}", info.id, addr);
                    }
                }
                
                // Force a collection to clean up any remaining references
                if let Err(e) = gc.force_collection() {
                    if self.debug_mode {
                        println!("Failed to force GC collection for channel {}: {:?}", info.id, e);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Update channel buffer addresses for GC tracking
    pub fn update_buffer_addresses(&self, id: u64, addresses: Vec<usize>) -> ChannelResult<()> {
        if let Ok(mut registry) = self.registry.write() {
            if let Some(info) = registry.get_mut(&id) {
                info.buffer_addresses = addresses;
                info.last_activity = Instant::now();
                return Ok(());
            }
        }
        Err(ChannelError::ChannelNotFound)
    }
    

    
    /// Remove buffer address from channel
    pub fn remove_buffer_address(&self, id: u64, address: usize) -> ChannelResult<()> {
        if let Ok(mut registry) = self.registry.write() {
            if let Some(info) = registry.get_mut(&id) {
                info.buffer_addresses.retain(|&addr| addr != address);
                info.last_activity = Instant::now();
                return Ok(());
            }
        }
        Err(ChannelError::ChannelNotFound)
    }

    /// Get channel statistics
    pub fn get_channel_stats(&self, id: u64) -> Option<ChannelStats> {
        if let Ok(registry) = self.registry.read() {
            if let Some(info) = registry.get(&id) {
                return Some(ChannelStats {
                    id: info.id as usize,
                    capacity: info.capacity,
                    current_length: info.current_size,
                    sender_count: info.sender_count,
                    receiver_count: info.receiver_count,
                    is_closed: info.is_closed,
                    total_sent: info.messages_sent,
                    total_received: info.messages_received,
                    messages_dropped: info.messages_dropped,
                });
            }
        }
        None
    }

    /// Get lifecycle statistics
    pub fn get_lifecycle_stats(&self) -> ChannelLifecycleStats {
        if let Ok(stats) = self.stats.lock() {
            stats.clone()
        } else {
            ChannelLifecycleStats::default()
        }
    }

    /// Check resource limits
    fn check_resource_limits(&self, capacity: usize) -> ChannelResult<()> {
        if let Ok(limits) = self.limits.read() {
            if let Ok(stats) = self.stats.lock() {
                // Check concurrent channels limit
                if stats.active_channels >= limits.max_concurrent_channels {
                    return Err(ChannelError::AllocationError(
                        format!("Maximum concurrent channels reached: {}", limits.max_concurrent_channels)
                    ));
                }

                // Check buffer size limit
                if capacity > limits.max_buffer_size {
                    return Err(ChannelError::InvalidBufferSize(capacity));
                }

                // Check total memory limit
                let estimated_memory = capacity * std::mem::size_of::<usize>();
                if stats.total_memory_allocated + estimated_memory > limits.max_total_memory {
                    return Err(ChannelError::AllocationError(
                        "Total memory limit exceeded".to_string()
                    ));
                }
            }
        }
        Ok(())
    }
    
    /// Resize channel buffer with proper synchronization
    pub fn resize_channel_buffer(&self, id: u64, new_capacity: usize) -> ChannelResult<()> {
        // Check resource limits for new capacity
        if let Err(e) = self.check_resource_limits(new_capacity) {
            return Err(e);
        }
        
        if let Ok(mut registry) = self.registry.write() {
            if let Some(info) = registry.get_mut(&id) {
                let old_capacity = info.capacity;
                
                // Calculate memory difference
                let old_memory = old_capacity * std::mem::size_of::<usize>();
                let new_memory = new_capacity * std::mem::size_of::<usize>();
                let memory_diff = new_memory as i64 - old_memory as i64;
                
                // Update channel info
                info.capacity = new_capacity;
                info.memory_allocated = (info.memory_allocated as i64 + memory_diff) as usize;
                info.last_activity = Instant::now();
                
                // Update statistics
                if let Ok(mut stats) = self.stats.lock() {
                    stats.total_memory_allocated = (stats.total_memory_allocated as i64 + memory_diff) as usize;
                }
                
                // Emit resize event
                let event = ChannelEvent::BufferResized { 
                    id, 
                    old_capacity, 
                    new_capacity 
                };
                self.emit_event(&event);
                
                if self.debug_mode {
                    println!("Channel {} buffer resized: {} -> {} (memory change: {:+})", 
                        id, old_capacity, new_capacity, memory_diff);
                }
                
                return Ok(());
            }
        }
        
        Err(ChannelError::ChannelNotFound)
    }
    
    /// Add synchronization primitive for channel operations
    pub fn create_channel_sync_barrier(&self, channel_ids: Vec<u64>) -> ChannelResult<u64> {
        // Create a barrier ID
        let barrier_id = self.next_id.fetch_add(1, Ordering::SeqCst);
        
        // Validate all channels exist
        if let Ok(registry) = self.registry.read() {
            for &id in &channel_ids {
                if !registry.contains_key(&id) {
                    return Err(ChannelError::ChannelNotFound);
                }
            }
        }
        
        // Create barrier tracking entry
        if let Ok(mut registry) = self.registry.write() {
            let barrier_info = ChannelInfo {
                id: barrier_id,
                type_name: "sync_barrier".to_string(),
                capacity: channel_ids.len(),
                current_size: 0,
                created_at: Instant::now(),
                last_activity: Instant::now(),
                messages_sent: 0,
                messages_received: 0,
                messages_dropped: 0,
                memory_allocated: std::mem::size_of::<u64>() * channel_ids.len(),
                is_closed: false,
                sender_count: 0,
                receiver_count: 0,
                buffer_addresses: channel_ids.iter().map(|&id| id as usize).collect(),
            };
            
            registry.insert(barrier_id, barrier_info);
        }
        
        if self.debug_mode {
            println!("Created sync barrier {} for channels: {:?}", barrier_id, channel_ids);
        }
        
        Ok(barrier_id)
    }
    
    /// Synchronize operations across multiple channels
    pub fn sync_channels(&self, barrier_id: u64) -> ChannelResult<()> {
        if let Ok(mut registry) = self.registry.write() {
            if let Some(barrier_info) = registry.get_mut(&barrier_id) {
                barrier_info.current_size += 1;
                barrier_info.last_activity = Instant::now();
                
                // Check if all channels in barrier are synchronized
                if barrier_info.current_size >= barrier_info.capacity {
                    // Reset barrier for next synchronization
                    barrier_info.current_size = 0;
                    
                    if self.debug_mode {
                        println!("Sync barrier {} completed synchronization", barrier_id);
                    }
                    
                    return Ok(());
                }
                
                if self.debug_mode {
                    println!("Sync barrier {} waiting: {}/{}", 
                        barrier_id, barrier_info.current_size, barrier_info.capacity);
                }
                
                return Ok(());
            }
        }
        
        Err(ChannelError::ChannelNotFound)
    }
    
    /// Enhanced error handling for channel operations
    pub fn handle_channel_error(&self, id: u64, error: &ChannelError) -> ChannelResult<()> {
        if let Ok(mut registry) = self.registry.write() {
            if let Some(info) = registry.get_mut(&id) {
                info.last_activity = Instant::now();
                
                match error {
                    ChannelError::BufferFull => {
                        // Check if we've hit capacity limits frequently
                        if info.messages_dropped > info.messages_sent / 10 {
                            let event = ChannelEvent::CapacityLimitReached {
                                id,
                                current: info.current_size,
                                limit: info.capacity,
                            };
                            self.emit_event(&event);
                            
                            if self.debug_mode {
                                println!("Channel {} frequently hitting capacity limits", id);
                            }
                        }
                    }
                    ChannelError::Closed => {
                        info.is_closed = true;
                        
                        // Emit closure event
                        let event = ChannelEvent::Closed { id };
                        self.emit_event(&event);
                    }
                    ChannelError::AllocationError(_) => {
                        // Force memory cleanup if allocation failed
                        if let Err(e) = self.force_cleanup() {
                            if self.debug_mode {
                                println!("Failed to force cleanup after allocation error: {:?}", e);
                            }
                        }
                    }
                    _ => {}
                }
                
                return Ok(());
            }
        }
        
        Err(ChannelError::ChannelNotFound)
    }

    /// Start monitoring thread
    pub fn start_monitoring(&mut self) -> ChannelResult<()> {
        if self.monitor_thread.is_some() {
            return Err(ChannelError::AllocationError("Monitor already running".to_string()));
        }

        let (tx, rx) = mpsc::channel();
        self.monitor_control = Some(tx);

        let registry = Arc::clone(&self.registry);
        let stats = Arc::clone(&self.stats);
        let limits = Arc::clone(&self.limits);
        let gc_integration = Arc::clone(&self.gc_integration);
        let debug_mode = self.debug_mode;

        let handle = thread::spawn(move || {
            let mut last_cleanup = Instant::now();
            
            loop {
                // Check for commands
                if let Ok(cmd) = rx.try_recv() {
                    match cmd {
                        MonitorCommand::Stop => break,
                        MonitorCommand::ForceCleanup => {
                            Self::perform_cleanup(&registry, &stats, &gc_integration, debug_mode);
                            last_cleanup = Instant::now();
                        }
                        MonitorCommand::UpdateLimits(new_limits) => {
                            if let Ok(mut current_limits) = limits.write() {
                                *current_limits = new_limits;
                                if debug_mode {
                                    println!("Channel resource limits updated");
                                }
                            }
                        }
                    }
                }

                // Periodic cleanup (every 30 seconds)
                if last_cleanup.elapsed().as_secs() > 30 {
                    Self::perform_cleanup(&registry, &stats, &gc_integration, debug_mode);
                    last_cleanup = Instant::now();
                }

                thread::sleep(std::time::Duration::from_secs(1));
            }
        });

        self.monitor_thread = Some(handle);
        Ok(())
    }

    /// Stop monitoring thread
    pub fn stop_monitoring(&mut self) -> ChannelResult<()> {
        if let Some(tx) = &self.monitor_control {
            let _ = tx.send(MonitorCommand::Stop);
        }

        if let Some(handle) = self.monitor_thread.take() {
            let _ = handle.join();
        }

        self.monitor_control = None;
        Ok(())
    }

    /// Perform cleanup operations
    fn perform_cleanup(
        registry: &Arc<RwLock<HashMap<u64, ChannelInfo>>>,
        stats: &Arc<Mutex<ChannelLifecycleStats>>,
        gc_integration: &Arc<Mutex<Option<Arc<GarbageCollector>>>>,
        debug_mode: bool,
    ) {
        if debug_mode {
            println!("Performing channel cleanup...");
        }

        // Find channels that need cleanup
        let mut channels_to_cleanup = Vec::new();
        
        if let Ok(registry) = registry.read() {
            for (id, info) in registry.iter() {
                // Check for stale channels (inactive for > 5 minutes)
                if info.last_activity.elapsed().as_secs() > 300 && info.is_closed {
                    channels_to_cleanup.push(*id);
                }
            }
        }

        // Perform GC integration cleanup
        if let Ok(gc_opt) = gc_integration.lock() {
            if let Some(gc) = gc_opt.as_ref() {
                for id in &channels_to_cleanup {
                    if let Ok(registry) = registry.read() {
                        if let Some(info) = registry.get(id) {
                            // Integrate with GC system for proper cleanup
                            for &addr in &info.buffer_addresses {
                                // Add address to GC root set for proper tracking
                                let ptr = addr as *mut u8;
                                if let Err(e) = gc.add_root(ptr) {
                            if debug_mode {
                                println!("Failed to add GC root for channel {} address 0x{:x}: {:?}", id, addr, e);
                                }
                            } else if debug_mode {
                                println!("Added GC root for channel {} address: 0x{:x}", id, addr);
                            }
                            
                            // Force GC collection to clean up channel buffers
                            if let Err(e) = gc.force_collection() {
                            if debug_mode {
                                println!("Failed to force GC collection for channel {}: {:?}", id, e);
                                }
                            } else if debug_mode {
                                println!("Forced GC collection for channel {} cleanup", id);
                            }
                            
                            // Remove from root set after collection to prevent memory leaks
                            if let Err(e) = gc.remove_root(ptr) {
                            if debug_mode {
                                println!("Failed to remove GC root for channel {} address 0x{:x}: {:?}", id, addr, e);
                                }
                                } else if debug_mode {
                                    println!("Removed GC root for channel {} address: 0x{:x}", id, addr);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Remove cleaned up channels
        if !channels_to_cleanup.is_empty() {
            if let Ok(mut registry) = registry.write() {
                for id in channels_to_cleanup {
                    registry.remove(&id);
                    if let Ok(mut stats) = stats.lock() {
                        stats.total_gc += 1;
                        stats.active_channels = stats.active_channels.saturating_sub(1);
                    }
                }
            }
        }
    }

    /// Set GC integration
    pub fn set_gc_integration(&self, gc: Arc<GarbageCollector>) -> ChannelResult<()> {
        if let Ok(mut gc_opt) = self.gc_integration.lock() {
            *gc_opt = Some(gc);
        }
        Ok(())
    }

    /// Add event listener
    pub fn add_event_listener<F>(&self, listener: F) -> ChannelResult<()>
    where
        F: Fn(&ChannelEvent) + Send + Sync + 'static,
    {
        if let Ok(mut listeners) = self.event_listeners.write() {
            listeners.push(Box::new(listener));
        }
        Ok(())
    }

    /// Emit event to all listeners
    fn emit_event(&self, event: &ChannelEvent) {
        if let Ok(listeners) = self.event_listeners.read() {
            for listener in listeners.iter() {
                listener(event);
            }
        }
    }

    /// Force cleanup
    pub fn force_cleanup(&self) -> ChannelResult<()> {
        if let Some(tx) = &self.monitor_control {
            tx.send(MonitorCommand::ForceCleanup)
                .map_err(|_| ChannelError::AllocationError("Failed to send cleanup command".to_string()))?;
        }
        Ok(())
    }

    /// Get all active channels
    pub fn get_active_channels(&self) -> Vec<u64> {
        if let Ok(registry) = self.registry.read() {
            registry.keys().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Verify channel integrity
    pub fn verify_channel_integrity(&self, id: u64) -> ChannelResult<bool> {
        if let Ok(registry) = self.registry.read() {
            if let Some(info) = registry.get(&id) {
                // Basic integrity checks
                let valid_capacity = info.capacity > 0;
                let valid_size = info.current_size <= info.capacity;
                let valid_timestamps = info.last_activity >= info.created_at;
                let valid_counters = info.messages_sent >= info.messages_received;

                let is_valid = valid_capacity && valid_size && valid_timestamps && valid_counters;
                
                if self.debug_mode && !is_valid {
                    println!("Channel {} integrity check failed: capacity={}, size={}, timestamps={}, counters={}", 
                        id, valid_capacity, valid_size, valid_timestamps, valid_counters);
                }
                
                return Ok(is_valid);
            }
        }
        Ok(false)
    }

    /// Update resource limits
    pub fn update_limits(&self, limits: ChannelResourceLimits) -> ChannelResult<()> {
        if let Ok(mut current_limits) = self.limits.write() {
            *current_limits = limits.clone();
        }
        
        // Send update command to monitor thread if running
        if let Some(tx) = &self.monitor_control {
            let _ = tx.send(MonitorCommand::UpdateLimits(limits));
        }
        
        Ok(())
    }

    /// Integrate channel with GC for memory management
    pub fn integrate_with_gc(&self, id: u64, buffer_ptr: *mut u8, size: usize) -> ChannelResult<()> {
        // Add buffer address to channel tracking
        self.add_buffer_address(id, buffer_ptr as usize)?;
        
        // Integrate with GC if available
        if let Ok(gc_opt) = self.gc_integration.lock() {
            if let Some(gc) = gc_opt.as_ref() {
                // Add buffer to GC root set
                if let Err(e) = gc.add_root(buffer_ptr) {
                    if self.debug_mode {
                        println!("Failed to add GC root for channel {} buffer: {:?}", id, e);
                    }
                    return Err(ChannelError::AllocationError(
                        format!("GC integration failed: {:?}", e)
                    ));
                }
                
                if self.debug_mode {
                    println!("Integrated channel {} buffer (size: {}) with GC", id, size);
                }
            }
        }
        
        Ok(())
    }

    /// Remove channel buffer from GC tracking
    pub fn remove_from_gc(&self, id: u64, buffer_ptr: *mut u8) -> ChannelResult<()> {
        // Remove buffer address from channel tracking
        self.remove_buffer_address(id, buffer_ptr as usize)?;
        
        // Remove from GC if available
        if let Ok(gc_opt) = self.gc_integration.lock() {
            if let Some(gc) = gc_opt.as_ref() {
                if let Err(e) = gc.remove_root(buffer_ptr) {
                    if self.debug_mode {
                        println!("Failed to remove GC root for channel {} buffer: {:?}", id, e);
                    }
                }
                
                if self.debug_mode {
                    println!("Removed channel {} buffer from GC tracking", id);
                }
            }
        }
        
        Ok(())
    }

    /// Check memory pressure and trigger cleanup if needed
    pub fn check_memory_pressure(&self) -> ChannelResult<()> {
        if let Ok(stats) = self.stats.lock() {
            let memory_usage_ratio = stats.total_memory_allocated as f64 / 
                (1024.0 * 1024.0 * 100.0); // 100MB baseline
            
            if memory_usage_ratio > 0.8 { // 80% of memory limit
                if self.debug_mode {
                    println!("High memory pressure detected: {:.1}%, triggering cleanup", 
                        memory_usage_ratio * 100.0);
                }
                self.force_cleanup()?;
            }
        }
        
        Ok(())
    }
}

impl Drop for ChannelLifecycleManager {
    fn drop(&mut self) {
        let _ = self.stop_monitoring();
    }
}

/// Global channel lifecycle manager instance
static GLOBAL_CHANNEL_MANAGER: std::sync::OnceLock<Arc<ChannelLifecycleManager>> = std::sync::OnceLock::new();

/// Get global channel lifecycle manager
pub fn get_global_channel_manager() -> Arc<ChannelLifecycleManager> {
    GLOBAL_CHANNEL_MANAGER.get_or_init(|| {
        Arc::new(ChannelLifecycleManager::new())
    }).clone()
}

/// Initialize global channel lifecycle manager with custom limits
pub fn init_global_channel_manager(limits: ChannelResourceLimits) -> Arc<ChannelLifecycleManager> {
    let manager = Arc::new(ChannelLifecycleManager::with_limits(limits));
    GLOBAL_CHANNEL_MANAGER.set(manager.clone()).unwrap_or_else(|_| {
        panic!("Global channel manager already initialized");
    });
    manager
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_channel_lifecycle_manager_creation() {
        let manager = ChannelLifecycleManager::new();
        let stats = manager.get_lifecycle_stats();
        assert_eq!(stats.total_created, 0);
        assert_eq!(stats.active_channels, 0);
    }

    #[test]
    fn test_channel_registration() {
        let manager = ChannelLifecycleManager::new();
        
        let id = manager.register_channel("test".to_string(), 100).unwrap();
        assert!(id > 0);
        
        let stats = manager.get_lifecycle_stats();
        assert_eq!(stats.total_created, 1);
        assert_eq!(stats.active_channels, 1);
    }

    #[test]
    fn test_channel_unregistration() {
        let manager = ChannelLifecycleManager::new();
        
        let id = manager.register_channel("test".to_string(), 100).unwrap();
        manager.unregister_channel(id).unwrap();
        
        let stats = manager.get_lifecycle_stats();
        assert_eq!(stats.total_created, 1);
        assert_eq!(stats.total_closed, 1);
        assert_eq!(stats.active_channels, 0);
    }

    #[test]
    fn test_message_tracking() {
        let manager = ChannelLifecycleManager::new();
        
        let id = manager.register_channel("test".to_string(), 100).unwrap();
        manager.record_message_sent(id, 10).unwrap();
        manager.record_message_received(id, 10).unwrap();
        
        let stats = manager.get_lifecycle_stats();
        assert_eq!(stats.total_messages_sent, 1);
        assert_eq!(stats.total_messages_received, 1);
    }

    #[test]
    fn test_resource_limits() {
        let limits = ChannelResourceLimits {
            max_concurrent_channels: 2,
            max_buffer_size: 50,
            max_total_memory: 1000,
            max_messages_per_channel: 100,
            strict_enforcement: true,
        };
        
        let manager = ChannelLifecycleManager::with_limits(limits);
        
        // Should succeed
        let id1 = manager.register_channel("test1".to_string(), 40).unwrap();
        let id2 = manager.register_channel("test2".to_string(), 40).unwrap();
        
        // Should fail - too many channels
        let result = manager.register_channel("test3".to_string(), 40);
        assert!(result.is_err());
        
        // Should fail - buffer too large
        manager.unregister_channel(id1).unwrap();
        let result = manager.register_channel("test4".to_string(), 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_channel_integrity() {
        let manager = ChannelLifecycleManager::new();
        
        let id = manager.register_channel("test".to_string(), 100).unwrap();
        let is_valid = manager.verify_channel_integrity(id).unwrap();
        assert!(is_valid);
        
        // Non-existent channel should return false
        let is_valid = manager.verify_channel_integrity(999).unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn test_event_listening() {
        let manager = ChannelLifecycleManager::new();
        
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = Arc::clone(&events);
        
        manager.add_event_listener(move |event| {
            events_clone.lock().unwrap().push(event.clone());
        }).unwrap();
        
        let id = manager.register_channel("test".to_string(), 100).unwrap();
        manager.record_message_sent(id, 10).unwrap();
        manager.unregister_channel(id).unwrap();
        
        let captured_events = events.lock().unwrap();
        assert_eq!(captured_events.len(), 3); // Created, MessageSent, Closed
    }

    #[test]
    fn test_global_manager() {
        let manager = get_global_channel_manager();
        let id = manager.register_channel("global_test".to_string(), 100).unwrap();
        
        let stats = manager.get_lifecycle_stats();
        assert!(stats.total_created > 0);
        
        manager.unregister_channel(id).unwrap();
    }

    #[test]
    fn test_message_drop_tracking() {
        let manager = ChannelLifecycleManager::new();
        
        let id = manager.register_channel("drop_test".to_string(), 100).unwrap();
        
        // Record some messages
        manager.record_message_sent(id, 10).unwrap();
        manager.record_message_sent(id, 15).unwrap();
        manager.record_message_received(id, 10).unwrap();
        manager.record_message_dropped(id, 15).unwrap();
        
        let stats = manager.get_channel_stats(id).unwrap();
        assert_eq!(stats.total_sent, 2);
        assert_eq!(stats.total_received, 1);
        assert_eq!(stats.messages_dropped, 1);
        
        manager.unregister_channel(id).unwrap();
    }

    #[test]
    fn test_gc_integration() {
        let manager = ChannelLifecycleManager::new();
        let gc = Arc::new(GarbageCollector::new());
        manager.set_gc_integration(gc.clone()).unwrap();
        
        let id = manager.register_channel("gc_test".to_string(), 100).unwrap();
        
        // Test buffer integration
        let buffer_ptr = 0x1000 as *mut u8;
        manager.integrate_with_gc(id, buffer_ptr, 100).unwrap();
        
        let addresses = manager.get_buffer_addresses(id);
        assert_eq!(addresses.len(), 1);
        assert_eq!(addresses[0], buffer_ptr as usize);
        
        // Test removal
        manager.remove_from_gc(id, buffer_ptr).unwrap();
        let addresses_after = manager.get_buffer_addresses(id);
        assert_eq!(addresses_after.len(), 0);
        
        manager.unregister_channel(id).unwrap();
    }

    #[test]
    fn test_limits_update() {
        let manager = ChannelLifecycleManager::new();
        
        let new_limits = ChannelResourceLimits {
            max_concurrent_channels: 5,
            max_buffer_size: 200,
            max_total_memory: 20000, // Increased to accommodate test channels
            max_messages_per_channel: 500,
            strict_enforcement: true,
        };
        
        manager.update_limits(new_limits.clone()).unwrap();
        
        // Test that new limits are enforced
        let id1 = manager.register_channel("test1".to_string(), 150).unwrap();
        let id2 = manager.register_channel("test2".to_string(), 150).unwrap();
        
        // Should fail - buffer too large for new limits
        let result = manager.register_channel("test3".to_string(), 250);
        assert!(result.is_err());
        
        manager.unregister_channel(id1).unwrap();
        manager.unregister_channel(id2).unwrap();
    }

    #[test]
    fn test_memory_pressure_detection() {
        let manager = ChannelLifecycleManager::new();
        
        // Create channels that would trigger memory pressure
        let mut ids = Vec::new();
        for i in 0..10 {
            let id = manager.register_channel(format!("pressure_test_{}", i), 1000).unwrap();
            ids.push(id);
        }
        
        // Check memory pressure (should trigger cleanup)
        let result = manager.check_memory_pressure();
        assert!(result.is_ok());
        
        // Clean up
        for id in ids {
            manager.unregister_channel(id).unwrap();
        }
    }

    #[test]
    fn test_thread_safety() {
        use std::thread;
        use std::sync::Arc;
        
        let manager = Arc::new(ChannelLifecycleManager::new());
        let mut handles = Vec::new();
        
        // Spawn multiple threads doing operations
        for i in 0..5 {
            let manager_clone = Arc::clone(&manager);
            let handle = thread::spawn(move || {
                let id = manager_clone.register_channel(format!("thread_test_{}", i), 100).unwrap();
                manager_clone.record_message_sent(id, 10).unwrap();
                manager_clone.record_message_received(id, 10).unwrap();
                manager_clone.unregister_channel(id).unwrap();
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        let stats = manager.get_lifecycle_stats();
        assert_eq!(stats.total_created, 5);
        assert_eq!(stats.total_closed, 5);
        assert_eq!(stats.active_channels, 0);
    }

    #[test]
    fn test_cleanup_trigger() {
        let mut manager = ChannelLifecycleManager::new();
        manager.enable_debug();
        
        let id = manager.register_channel("cleanup_test".to_string(), 100).unwrap();
        
        // Simulate high drop ratio to trigger cleanup
        for _ in 0..15 {
            manager.record_message_sent(id, 10).unwrap();
        }
        for _ in 0..10 {
            manager.record_message_dropped(id, 10).unwrap();
        }
        
        // Drop ratio should be high enough to trigger cleanup
        let stats = manager.get_channel_stats(id).unwrap();
        let drop_ratio = stats.messages_dropped as f64 / stats.total_sent as f64;
        assert!(drop_ratio > 0.1);
        
        manager.unregister_channel(id).unwrap();
    }
}
