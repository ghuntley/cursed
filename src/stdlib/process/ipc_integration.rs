use crate::error::CursedError;
/// Process-IPC Coordination System for CURSED
/// 
/// This module provides comprehensive coordination between process management
/// and IPC systems, ensuring seamless integration and automatic resource cleanup.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use std::thread;

// use crate::stdlib::process::error::{ProcessError, ProcessResult, communication_error, timeout_error};
// use crate::stdlib::process::communication::{ProcessCommunication, ProcessChannels, CommunicationConfig, IpcType};
// use crate::stdlib::process::core::{Process, ProcessState};
// use crate::stdlib::ipc::error::IpcResult;
// use crate::stdlib::ipc::shared_memory::SharedMemorySegment;
// use crate::stdlib::ipc::named_pipes::NamedPipe as IpcNamedPipe;
// use crate::stdlib::ipc::message_queues::MessageQueue as IpcMessageQueue;

/// IPC integration manager for process-IPC coordination
pub type IpcIntegration = ProcessIpcCoordinator;

/// Configuration options for IPC integration
#[derive(Debug, Clone)]
pub struct IpcOptions {
    /// Enable automatic cleanup
    pub auto_cleanup: bool,
    /// Cleanup interval
    pub cleanup_interval: Duration,
    /// Max resources per process
    pub max_resources_per_process: usize,
    /// Enable resource monitoring
    pub enable_monitoring: bool,
}

/// Process-IPC Coordinator manages IPC resources per process
pub struct ProcessIpcCoordinator {
    /// Active process-IPC mappings
    process_resources: Arc<RwLock<HashMap<u32, ProcessIpcResources>>>,
    /// Global resource registry
    resource_registry: Arc<Mutex<ResourceRegistry>>,
    /// Cleanup thread handle
    cleanup_thread: Option<thread::JoinHandle<()>>,
    /// Coordinator configuration
    config: CoordinatorConfig,
    /// Active flag for shutdown coordination
    active: Arc<Mutex<bool>>,
}

impl ProcessIpcCoordinator {
    /// Create new process-IPC coordinator
    pub fn new(config: CoordinatorConfig) -> Self {
        let coordinator = Self {
            process_resources: Arc::new(RwLock::new(HashMap::new())),
            resource_registry: Arc::new(Mutex::new(ResourceRegistry::new())),
            cleanup_thread: None,
            config,
            active: Arc::new(Mutex::new(true)),
        };
        
        coordinator
    }

    /// Start the coordinator with background cleanup
    pub fn start(&mut self) -> ProcessResult<()> {
        let process_resources = self.process_resources.clone();
        let resource_registry = self.resource_registry.clone();
        let active = self.active.clone();
        let cleanup_interval = self.config.cleanup_interval;

        let cleanup_handle = thread::spawn(move || {
            Self::cleanup_worker(process_resources, resource_registry, active, cleanup_interval);
        });

        self.cleanup_thread = Some(cleanup_handle);
        Ok(())
    }

    /// Stop the coordinator and cleanup resources
    pub fn stop(&mut self) -> ProcessResult<()> {
        // Signal shutdown
        if let Ok(mut active) = self.active.lock() {
            *active = false;
        }

        // Wait for cleanup thread to finish
        if let Some(handle) = self.cleanup_thread.take() {
            handle.join().map_err(|_| 
                communication_error("coordinator_stop", "Failed to join cleanup thread")
            )?;
        }

        // Clean up all remaining resources
        self.cleanup_all_resources()?;
        Ok(())
    }

    /// Register a process with IPC resources
    pub fn register_process(&self, process_id: u32, process: &Process) -> ProcessResult<ProcessIpcResources> {
        let mut resources = ProcessIpcResources::new(process_id, process.get_state());
        
        // Create default IPC resources based on process configuration
        self.create_default_resources(&mut resources, process)?;
        
        // Register in the coordinator
        if let Ok(mut process_map) = self.process_resources.write() {
            process_map.insert(process_id, resources.clone());
        }

        // Update global registry
        if let Ok(mut registry) = self.resource_registry.lock() {
            registry.register_process(process_id, &resources);
        }

        Ok(resources)
    }

    /// Unregister a process and cleanup its resources
    pub fn unregister_process(&self, process_id: u32) -> ProcessResult<()> {
        // Get process resources
        let resources = if let Ok(mut process_map) = self.process_resources.write() {
            process_map.remove(&process_id)
        } else {
            None
        };

        if let Some(resources) = resources {
            // Cleanup the process resources
            self.cleanup_process_resources(&resources)?;
            
            // Update global registry
            if let Ok(mut registry) = self.resource_registry.lock() {
                registry.unregister_process(process_id);
            }
        }

        Ok(())
    }

    /// Get IPC resources for a process
    pub fn get_process_resources(&self, process_id: u32) -> Option<ProcessIpcResources> {
        self.process_resources.read()
            .ok()
            .and_then(|map| map.get(&process_id).cloned())
    }

    /// Create communication channel between processes
    pub fn create_inter_process_channel(
        &self,
        process1_id: u32,
        process2_id: u32,
        channel_type: IpcType,
    ) -> ProcessResult<InterProcessChannel> {
        let channel_name = format!("channel_{}_{}", process1_id, process2_id);
        
        let channel = match channel_type {
            IpcType::Pipe => {
                let pipe = IpcNamedPipe::create(&channel_name)?;
                InterProcessChannel::Pipe {
                    name: channel_name.clone(),
                    pipe: Arc::new(Mutex::new(pipe)),
                }
            }
            IpcType::SharedMemory => {
                let segment = SharedMemorySegment::create(&channel_name, self.config.default_memory_size)?;
                InterProcessChannel::SharedMemory {
                    name: channel_name.clone(),
                    segment: Arc::new(Mutex::new(segment)),
                }
            }
            IpcType::MessageQueue => {
                let queue = IpcMessageQueue::create(&channel_name, self.config.default_queue_size)?;
                InterProcessChannel::MessageQueue {
                    name: channel_name.clone(),
                    queue: Arc::new(Mutex::new(queue)),
                }
            }
            IpcType::Auto => {
                // Default to pipe for auto selection
                let pipe = IpcNamedPipe::create(&channel_name)?;
                InterProcessChannel::Pipe {
                    name: channel_name.clone(),
                    pipe: Arc::new(Mutex::new(pipe)),
                }
            }
        };

        // Register channel with both processes
        self.register_channel_with_processes(process1_id, process2_id, &channel)?;

        Ok(channel)
    }

    /// Monitor process lifecycle and handle IPC cleanup
    pub fn monitor_process_lifecycle(&self, process_id: u32, process: &Process) -> ProcessResult<()> {
        let resources = self.get_process_resources(process_id);
        if resources.is_none() {
            return Err(communication_error(
                "monitor_lifecycle", 
                &format!("Process {} not registered", process_id)
            ));
        }

        // Monitor process state changes
        let current_state = process.get_state();
        if let Ok(mut process_map) = self.process_resources.write() {
            if let Some(resources) = process_map.get_mut(&process_id) {
                resources.update_state(current_state);
                
                // If process is terminated, mark for cleanup
                if matches!(current_state, ProcessState::Terminated) {
                    resources.mark_for_cleanup();
                }
            }
        }

        Ok(())
    }

    /// Get cross-process communication statistics
    pub fn get_statistics(&self) -> CoordinatorStatistics {
        let mut stats = CoordinatorStatistics::new();
        
        if let Ok(process_map) = self.process_resources.read() {
            stats.active_processes = process_map.len();
            
            for resources in process_map.values() {
                stats.total_pipes += resources.pipes.len();
                stats.total_shared_memory += resources.shared_memory.len();
                stats.total_message_queues += resources.message_queues.len();
                stats.total_channels += resources.channels.len();
            }
        }

        if let Ok(registry) = self.resource_registry.lock() {
            stats.total_resources = registry.total_resources();
            stats.cleanup_operations = registry.cleanup_count;
        }

        stats
    }

    fn create_default_resources(&self, resources: &mut ProcessIpcResources, process: &Process) -> ProcessResult<()> {
        let process_id = process.id();
        
        // Create default pipe
        let pipe_name = format!("process_pipe_{}", process_id);
        match IpcNamedPipe::create(&pipe_name) {
            Ok(pipe) => {
                resources.add_pipe(pipe_name.clone(), Arc::new(Mutex::new(pipe)));
            }
            Err(_) => {
                // Log but don't fail - pipes might not be available on all platforms
                eprintln!("Warning: Could not create default pipe for process {}", process_id);
            }
        }

        // Create default shared memory if configured
        if self.config.create_default_shared_memory {
            let memory_name = format!("process_memory_{}", process_id);
            match SharedMemorySegment::create(&memory_name, self.config.default_memory_size) {
                Ok(segment) => {
                    resources.add_shared_memory(memory_name.clone(), Arc::new(Mutex::new(segment)));
                }
                Err(_) => {
                    eprintln!("Warning: Could not create default shared memory for process {}", process_id);
                }
            }
        }

        Ok(())
    }

    fn cleanup_process_resources(&self, resources: &ProcessIpcResources) -> ProcessResult<()> {
        // Close all pipes
        for (name, _pipe) in &resources.pipes {
            eprintln!("Cleaning up pipe: {}", name);
            // In a real implementation, we would properly close the pipe
        }

        // Close all shared memory segments
        for (name, _segment) in &resources.shared_memory {
            eprintln!("Cleaning up shared memory: {}", name);
            // In a real implementation, we would properly cleanup the segment
        }

        // Close all message queues
        for (name, _queue) in &resources.message_queues {
            eprintln!("Cleaning up message queue: {}", name);
            // In a real implementation, we would properly close the queue
        }

        // Close all inter-process channels
        for channel in &resources.channels {
            channel.close()?;
        }

        Ok(())
    }

    fn cleanup_all_resources(&self) -> ProcessResult<()> {
        if let Ok(process_map) = self.process_resources.read() {
            for resources in process_map.values() {
                self.cleanup_process_resources(resources)?;
            }
        }
        Ok(())
    }

    fn register_channel_with_processes(
        &self,
        process1_id: u32,
        process2_id: u32,
        channel: &InterProcessChannel,
    ) -> ProcessResult<()> {
        if let Ok(mut process_map) = self.process_resources.write() {
            // Add channel to first process
            if let Some(resources) = process_map.get_mut(&process1_id) {
                resources.add_channel(channel.clone());
            }
            
            // Add channel to second process
            if let Some(resources) = process_map.get_mut(&process2_id) {
                resources.add_channel(channel.clone());
            }
        }
        Ok(())
    }

    fn cleanup_worker(
        process_resources: Arc<RwLock<HashMap<u32, ProcessIpcResources>>>,
        resource_registry: Arc<Mutex<ResourceRegistry>>,
        active: Arc<Mutex<bool>>,
        cleanup_interval: Duration,
    ) {
        while Self::is_active(&active) {
            // Identify processes marked for cleanup
            let mut processes_to_cleanup = Vec::new();
            
            if let Ok(process_map) = process_resources.read() {
                for (process_id, resources) in process_map.iter() {
                    if resources.should_cleanup() {
                        processes_to_cleanup.push(*process_id);
                    }
                }
            }

            // Perform cleanup for marked processes
            for process_id in processes_to_cleanup {
                if let Ok(mut process_map) = process_resources.write() {
                    if let Some(resources) = process_map.remove(&process_id) {
                        // Cleanup resources (simplified for now)
                        eprintln!("Cleaning up resources for process {}", process_id);
                        
                        // Update registry
                        if let Ok(mut registry) = resource_registry.lock() {
                            registry.cleanup_count += 1;
                        }
                    }
                }
            }

            thread::sleep(cleanup_interval);
        }
    }

    fn is_active(active: &Arc<Mutex<bool>>) -> bool {
        active.lock().map(|a| *a).unwrap_or(false)
    }
}

/// Process IPC resources for a single process
#[derive(Debug, Clone)]
pub struct ProcessIpcResources {
    pub process_id: u32,
    pub state: ProcessState,
    pub pipes: HashMap<String, Arc<Mutex<IpcNamedPipe>>>,
    pub shared_memory: HashMap<String, Arc<Mutex<SharedMemorySegment<u8>>>>,
    pub message_queues: HashMap<String, Arc<Mutex<IpcMessageQueue>>>,
    pub channels: Vec<InterProcessChannel>,
    pub created_at: Instant,
    pub cleanup_marked: bool,
}

impl ProcessIpcResources {
    pub fn new(process_id: u32, state: ProcessState) -> Self {
        Self {
            process_id,
            state,
            pipes: HashMap::new(),
            shared_memory: HashMap::new(),
            message_queues: HashMap::new(),
            channels: Vec::new(),
            created_at: Instant::now(),
            cleanup_marked: false,
        }
    }

    pub fn add_pipe(&mut self, name: String, pipe: Arc<Mutex<IpcNamedPipe>>) {
        self.pipes.insert(name, pipe);
    }

    pub fn add_shared_memory(&mut self, name: String, segment: Arc<Mutex<SharedMemorySegment<u8>>>) {
        self.shared_memory.insert(name, segment);
    }

    pub fn add_message_queue(&mut self, name: String, queue: Arc<Mutex<IpcMessageQueue>>) {
        self.message_queues.insert(name, queue);
    }

    pub fn add_channel(&mut self, channel: InterProcessChannel) {
        self.channels.push(channel);
    }

    pub fn update_state(&mut self, new_state: ProcessState) {
        self.state = new_state;
    }

    pub fn mark_for_cleanup(&mut self) {
        self.cleanup_marked = true;
    }

    pub fn should_cleanup(&self) -> bool {
        self.cleanup_marked || matches!(self.state, ProcessState::Terminated)
    }

    pub fn get_communication_channels(&self) -> ProcessChannels {
        let mut channels = ProcessChannels::new();
        
        // Add pipes
        for name in self.pipes.keys() {
            channels.add_pipe(name.clone());
        }

        // Add shared memory
        for name in self.shared_memory.keys() {
            channels.add_shared_memory(name.clone());
        }

        // Add message queues
        for name in self.message_queues.keys() {
            channels.add_message_queue(name.clone());
        }

        channels
    }
}

/// Inter-process communication channel
#[derive(Debug, Clone)]
pub enum InterProcessChannel {
    Pipe {
        name: String,
        pipe: Arc<Mutex<IpcNamedPipe>>,
    },
    SharedMemory {
        name: String,
        segment: Arc<Mutex<SharedMemorySegment<u8>>>,
    },
    MessageQueue {
        name: String,
        queue: Arc<Mutex<IpcMessageQueue>>,
    },
}

impl InterProcessChannel {
    pub fn name(&self) -> &str {
        match self {
            InterProcessChannel::Pipe { name, .. } => name,
            InterProcessChannel::SharedMemory { name, .. } => name,
            InterProcessChannel::MessageQueue { name, .. } => name,
        }
    }

    pub fn channel_type(&self) -> IpcType {
        match self {
            InterProcessChannel::Pipe { .. } => IpcType::Pipe,
            InterProcessChannel::SharedMemory { .. } => IpcType::SharedMemory,
            InterProcessChannel::MessageQueue { .. } => IpcType::MessageQueue,
        }
    }

    pub fn close(&self) -> ProcessResult<()> {
        match self {
            InterProcessChannel::Pipe { name, .. } => {
                eprintln!("Closing pipe channel: {}", name);
                // Real implementation would close the pipe
            }
            InterProcessChannel::SharedMemory { name, .. } => {
                eprintln!("Closing shared memory channel: {}", name);
                // Real implementation would cleanup shared memory
            }
            InterProcessChannel::MessageQueue { name, .. } => {
                eprintln!("Closing message queue channel: {}", name);
                // Real implementation would close the queue
            }
        }
        Ok(())
    }
}

/// Global resource registry
#[derive(Debug)]
struct ResourceRegistry {
    processes: HashSet<u32>,
    cleanup_count: usize,
}

impl ResourceRegistry {
    fn new() -> Self {
        Self {
            processes: HashSet::new(),
            cleanup_count: 0,
        }
    }

    fn register_process(&mut self, process_id: u32, _resources: &ProcessIpcResources) {
        self.processes.insert(process_id);
    }

    fn unregister_process(&mut self, process_id: u32) {
        self.processes.remove(&process_id);
    }

    fn total_resources(&self) -> usize {
        self.processes.len()
    }
}

/// Coordinator configuration
#[derive(Debug, Clone)]
pub struct CoordinatorConfig {
    pub cleanup_interval: Duration,
    pub default_memory_size: usize,
    pub default_queue_size: usize,
    pub create_default_shared_memory: bool,
    pub auto_cleanup_terminated: bool,
}

impl Default for CoordinatorConfig {
    fn default() -> Self {
        Self {
            cleanup_interval: Duration::from_secs(30),
            default_memory_size: 64 * 1024, // 64KB
            default_queue_size: 100,
            create_default_shared_memory: false,
            auto_cleanup_terminated: true,
        }
    }
}

/// Coordinator statistics
#[derive(Debug, Clone)]
pub struct CoordinatorStatistics {
    pub active_processes: usize,
    pub total_pipes: usize,
    pub total_shared_memory: usize,
    pub total_message_queues: usize,
    pub total_channels: usize,
    pub total_resources: usize,
    pub cleanup_operations: usize,
}

impl CoordinatorStatistics {
    fn new() -> Self {
        Self {
            active_processes: 0,
            total_pipes: 0,
            total_shared_memory: 0,
            total_message_queues: 0,
            total_channels: 0,
            total_resources: 0,
            cleanup_operations: 0,
        }
    }
}

/// Create a global process-IPC coordinator
pub fn create_global_coordinator(config: CoordinatorConfig) -> ProcessResult<ProcessIpcCoordinator> {
    let mut coordinator = ProcessIpcCoordinator::new(config);
    coordinator.start()?;
    Ok(coordinator)
}

/// Enhanced process communication with real IPC integration
pub fn create_enhanced_process_communication(
    process_id: u32,
    coordinator: &ProcessIpcCoordinator,
    config: CommunicationConfig,
) -> ProcessResult<ProcessCommunication> {
    // Get process resources from coordinator
    let resources = coordinator.get_process_resources(process_id);
    
    let channels = if let Some(resources) = resources {
        resources.get_communication_channels()
    } else {
        return Err(communication_error(
            "create_enhanced_communication",
            &format!("Process {} not registered with coordinator", process_id)
        ));
    };

    Ok(ProcessCommunication::new(process_id, channels))
}

