use crate::error::CursedError;
/// IPC-side Process Coordination for CURSED
/// 
/// This module provides IPC-aware process coordination, ensuring that IPC
/// resources are properly managed in the context of process lifecycles.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock, Weak};
use std::time::{Duration, Instant};

// use crate::stdlib::ipc::error::{IpcError, IpcResult, invalid_operation};
// use crate::stdlib::ipc::shared_memory::SharedMemorySegment;
// use crate::stdlib::ipc::named_pipes::NamedPipe;
// use crate::stdlib::ipc::message_queues::MessageQueue;
// use crate::stdlib::ipc::traits::IpcResource;

/// IPC Process Registry manages process-aware IPC resources
pub struct IpcProcessRegistry {
    /// Process-to-resource mappings
    process_resources: Arc<RwLock<HashMap<u32, ProcessIpcBinding>>>,
    /// Resource-to-process reverse mappings
    resource_processes: Arc<RwLock<HashMap<String, HashSet<u32>>>>,
    /// Resource cleanup callbacks
    cleanup_callbacks: Arc<Mutex<HashMap<String, Box<dyn Fn() + Send + Sync>>>>,
    /// Configuration
    config: ProcessRegistryConfig,
}

impl IpcProcessRegistry {
    /// Create new IPC process registry
    pub fn new(config: ProcessRegistryConfig) -> Self {
        Self {
            process_resources: Arc::new(RwLock::new(HashMap::new())),
            resource_processes: Arc::new(RwLock::new(HashMap::new())),
            cleanup_callbacks: Arc::new(Mutex::new(HashMap::new())),
            config,
        }
    }

    /// Register a process with the IPC system
    pub fn register_process(&self, process_id: u32) -> IpcResult<ProcessIpcBinding> {
        let binding = ProcessIpcBinding::new(process_id, self.config.max_resources_per_process);
        
        if let Ok(mut process_map) = self.process_resources.write() {
            process_map.insert(process_id, binding.clone());
        }

        Ok(binding)
    }

    /// Unregister a process and cleanup its IPC resources
    pub fn unregister_process(&self, process_id: u32) -> IpcResult<()> {
        // Get process binding
        let binding = if let Ok(mut process_map) = self.process_resources.write() {
            process_map.remove(&process_id)
        } else {
            None
        };

        if let Some(binding) = binding {
            // Clean up all resources associated with this process
            self.cleanup_process_resources(process_id, &binding)?;
        }

        Ok(())
    }

    /// Bind an IPC resource to a process
    pub fn bind_resource_to_process(
        &self,
        process_id: u32,
        resource_name: &str,
        resource_type: IpcResourceType,
    ) -> IpcResult<()> {
        // Check if process is registered
        let binding_exists = if let Ok(process_map) = self.process_resources.read() {
            process_map.contains_key(&process_id)
        } else {
            false
        };

        if !binding_exists {
            return Err(invalid_operation(
                "bind_resource",
                &format!("Process {} not registered", process_id)
            ));
        }

        // Add resource to process
        if let Ok(mut process_map) = self.process_resources.write() {
            if let Some(binding) = process_map.get_mut(&process_id) {
                binding.add_resource(resource_name.to_string(), resource_type)?;
            }
        }

        // Add process to resource reverse mapping
        if let Ok(mut resource_map) = self.resource_processes.write() {
            resource_map
                .entry(resource_name.to_string())
                .or_insert_with(HashSet::new)
                .insert(process_id);
        }

        Ok(())
    }

    /// Unbind an IPC resource from a process
    pub fn unbind_resource_from_process(
        &self,
        process_id: u32,
        resource_name: &str,
    ) -> IpcResult<()> {
        // Remove from process binding
        if let Ok(mut process_map) = self.process_resources.write() {
            if let Some(binding) = process_map.get_mut(&process_id) {
                binding.remove_resource(resource_name);
            }
        }

        // Remove from resource reverse mapping
        if let Ok(mut resource_map) = self.resource_processes.write() {
            if let Some(processes) = resource_map.get_mut(resource_name) {
                processes.remove(&process_id);
                
                // If no processes are using this resource, remove it
                if processes.is_empty() {
                    resource_map.remove(resource_name);
                    
                    // Execute cleanup callback if registered
                    if let Ok(callbacks) = self.cleanup_callbacks.lock() {
                        if let Some(callback) = callbacks.get(resource_name) {
                            callback();
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Get processes using a specific resource
    pub fn get_resource_processes(&self, resource_name: &str) -> Vec<u32> {
        self.resource_processes
            .read()
            .ok()
            .and_then(|map| map.get(resource_name).cloned())
            .unwrap_or_default()
            .into_iter()
            .collect()
    }

    /// Get resources used by a specific process
    pub fn get_process_resources(&self, process_id: u32) -> Vec<(String, IpcResourceType)> {
        self.process_resources
            .read()
            .ok()
            .and_then(|map| map.get(&process_id).map(|b| b.get_resources()))
            .unwrap_or_default()
    }

    /// Register cleanup callback for a resource
    pub fn register_cleanup_callback<F>(&self, resource_name: &str, callback: F) -> IpcResult<()>
    where
        F: Fn() + Send + Sync + 'static,
    {
        if let Ok(mut callbacks) = self.cleanup_callbacks.lock() {
            callbacks.insert(resource_name.to_string(), Box::new(callback));
        }
        Ok(())
    }

    /// Check if a resource is in use by any process
    pub fn is_resource_in_use(&self, resource_name: &str) -> bool {
        self.resource_processes
            .read()
            .ok()
            .map(|map| map.contains_key(resource_name))
            .unwrap_or(false)
    }

    /// Get registry statistics
    pub fn get_statistics(&self) -> RegistryStatistics {
        let mut stats = RegistryStatistics::new();

        if let Ok(process_map) = self.process_resources.read() {
            stats.active_processes = process_map.len();
            
            for binding in process_map.values() {
                let resources = binding.get_resources();
                stats.total_resources += resources.len();
                
                for (_, resource_type) in resources {
                    match resource_type {
                        IpcResourceType::NamedPipe => stats.pipes += 1,
                        IpcResourceType::SharedMemory => stats.shared_memory += 1,
                        IpcResourceType::MessageQueue => stats.message_queues += 1,
                        IpcResourceType::Socket => stats.sockets += 1,
                    }
                }
            }
        }

        if let Ok(resource_map) = self.resource_processes.read() {
            stats.unique_resources = resource_map.len();
        }

        stats
    }

    /// Force cleanup of orphaned resources
    pub fn cleanup_orphaned_resources(&self) -> IpcResult<usize> {
        let mut cleaned_count = 0;

        // Get all resources
        let resources_to_check = if let Ok(resource_map) = self.resource_processes.read() {
            resource_map.keys().cloned().collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        // Check each resource for orphaned state
        for resource_name in resources_to_check {
            let processes = self.get_resource_processes(&resource_name);
            
            // If no processes are using this resource, it's orphaned
            if processes.is_empty() {
                if let Ok(mut resource_map) = self.resource_processes.write() {
                    resource_map.remove(&resource_name);
                }
                
                // Execute cleanup callback
                if let Ok(callbacks) = self.cleanup_callbacks.lock() {
                    if let Some(callback) = callbacks.get(&resource_name) {
                        callback();
                    }
                }
                
                cleaned_count += 1;
            }
        }

        Ok(cleaned_count)
    }

    fn cleanup_process_resources(&self, process_id: u32, binding: &ProcessIpcBinding) -> IpcResult<()> {
        let resources = binding.get_resources();
        
        for (resource_name, _resource_type) in resources {
            self.unbind_resource_from_process(process_id, &resource_name)?;
        }

        Ok(())
    }
}

/// Process IPC binding tracks resources for a single process
#[derive(Debug, Clone)]
pub struct ProcessIpcBinding {
    process_id: u32,
    resources: HashMap<String, IpcResourceType>,
    created_at: Instant,
    max_resources: usize,
}

impl ProcessIpcBinding {
    fn new(process_id: u32, max_resources: usize) -> Self {
        Self {
            process_id,
            resources: HashMap::new(),
            created_at: Instant::now(),
            max_resources,
        }
    }

    fn add_resource(&mut self, name: String, resource_type: IpcResourceType) -> IpcResult<()> {
        if self.resources.len() >= self.max_resources {
            return Err(invalid_operation(
                "add_resource",
                &format!("Process {} has reached maximum resource limit", self.process_id)
            ));
        }

        self.resources.insert(name, resource_type);
        Ok(())
    }

    fn remove_resource(&mut self, name: &str) {
        self.resources.remove(name);
    }

    fn get_resources(&self) -> Vec<(String, IpcResourceType)> {
        self.resources.iter().map(|(k, v)| (k.clone(), *v)).collect()
    }

    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }

    pub fn has_resource(&self, name: &str) -> bool {
        self.resources.contains_key(name)
    }

    pub fn get_resource_type(&self, name: &str) -> Option<IpcResourceType> {
        self.resources.get(name).copied()
    }
}

/// IPC resource types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpcResourceType {
    NamedPipe,
    SharedMemory,
    MessageQueue,
    Socket,
}

impl IpcResourceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            IpcResourceType::NamedPipe => "named_pipe",
            IpcResourceType::SharedMemory => "shared_memory",
            IpcResourceType::MessageQueue => "message_queue",
            IpcResourceType::Socket => "socket",
        }
    }
}

/// Process-aware IPC resource manager
pub struct ProcessAwareIpcManager {
    registry: Arc<IpcProcessRegistry>,
    shared_memory_manager: Arc<Mutex<HashMap<String, Arc<SharedMemorySegment<u8>>>>>,
    named_pipe_manager: Arc<Mutex<HashMap<String, Arc<NamedPipe>>>>,
    message_queue_manager: Arc<Mutex<HashMap<String, Arc<MessageQueue>>>>,
}

impl ProcessAwareIpcManager {
    /// Create new process-aware IPC manager
    pub fn new(registry: Arc<IpcProcessRegistry>) -> Self {
        Self {
            registry,
            shared_memory_manager: Arc::new(Mutex::new(HashMap::new())),
            named_pipe_manager: Arc::new(Mutex::new(HashMap::new())),
            message_queue_manager: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create shared memory for a process
    pub fn create_shared_memory_for_process(
        &self,
        process_id: u32,
        name: &str,
        size: usize,
    ) -> IpcResult<Arc<SharedMemorySegment<u8>>> {
        // Create the shared memory segment
        let segment = Arc::new(SharedMemorySegment::create(name, size)?);
        
        // Register with process
        self.registry.bind_resource_to_process(process_id, name, IpcResourceType::SharedMemory)?;
        
        // Store in manager
        if let Ok(mut manager) = self.shared_memory_manager.lock() {
            manager.insert(name.to_string(), segment.clone());
        }

        // Register cleanup callback
        let manager_weak = Arc::downgrade(&self.shared_memory_manager);
        let name_owned = name.to_string();
        self.registry.register_cleanup_callback(name, move || {
            if let Some(manager) = manager_weak.upgrade() {
                if let Ok(mut map) = manager.lock() {
                    map.remove(&name_owned);
                }
            }
        })?;

        Ok(segment)
    }

    /// Create named pipe for a process
    pub fn create_named_pipe_for_process(
        &self,
        process_id: u32,
        name: &str,
    ) -> IpcResult<Arc<NamedPipe>> {
        // Create the named pipe
        let pipe = Arc::new(NamedPipe::create(name)?);
        
        // Register with process
        self.registry.bind_resource_to_process(process_id, name, IpcResourceType::NamedPipe)?;
        
        // Store in manager
        if let Ok(mut manager) = self.named_pipe_manager.lock() {
            manager.insert(name.to_string(), pipe.clone());
        }

        // Register cleanup callback
        let manager_weak = Arc::downgrade(&self.named_pipe_manager);
        let name_owned = name.to_string();
        self.registry.register_cleanup_callback(name, move || {
            if let Some(manager) = manager_weak.upgrade() {
                if let Ok(mut map) = manager.lock() {
                    map.remove(&name_owned);
                }
            }
        })?;

        Ok(pipe)
    }

    /// Create message queue for a process
    pub fn create_message_queue_for_process(
        &self,
        process_id: u32,
        name: &str,
        max_messages: usize,
    ) -> IpcResult<Arc<MessageQueue>> {
        // Create the message queue
        let queue = Arc::new(MessageQueue::create(name, max_messages)?);
        
        // Register with process
        self.registry.bind_resource_to_process(process_id, name, IpcResourceType::MessageQueue)?;
        
        // Store in manager
        if let Ok(mut manager) = self.message_queue_manager.lock() {
            manager.insert(name.to_string(), queue.clone());
        }

        // Register cleanup callback
        let manager_weak = Arc::downgrade(&self.message_queue_manager);
        let name_owned = name.to_string();
        self.registry.register_cleanup_callback(name, move || {
            if let Some(manager) = manager_weak.upgrade() {
                if let Ok(mut map) = manager.lock() {
                    map.remove(&name_owned);
                }
            }
        })?;

        Ok(queue)
    }

    /// Get shared memory by name
    pub fn get_shared_memory(&self, name: &str) -> Option<Arc<SharedMemorySegment<u8>>> {
        self.shared_memory_manager
            .lock()
            .ok()
            .and_then(|map| map.get(name).cloned())
    }

    /// Get named pipe by name
    pub fn get_named_pipe(&self, name: &str) -> Option<Arc<NamedPipe>> {
        self.named_pipe_manager
            .lock()
            .ok()
            .and_then(|map| map.get(name).cloned())
    }

    /// Get message queue by name
    pub fn get_message_queue(&self, name: &str) -> Option<Arc<MessageQueue>> {
        self.message_queue_manager
            .lock()
            .ok()
            .and_then(|map| map.get(name).cloned())
    }

    /// Cleanup all resources for a process
    pub fn cleanup_process(&self, process_id: u32) -> IpcResult<()> {
        self.registry.unregister_process(process_id)
    }
}

/// Process registry configuration
#[derive(Debug, Clone)]
pub struct ProcessRegistryConfig {
    pub max_resources_per_process: usize,
    pub auto_cleanup_orphaned: bool,
    pub cleanup_check_interval: Duration,
}

impl Default for ProcessRegistryConfig {
    fn default() -> Self {
        Self {
            max_resources_per_process: 100,
            auto_cleanup_orphaned: true,
            cleanup_check_interval: Duration::from_secs(60),
        }
    }
}

/// Registry statistics
#[derive(Debug, Clone)]
pub struct RegistryStatistics {
    pub active_processes: usize,
    pub total_resources: usize,
    pub unique_resources: usize,
    pub pipes: usize,
    pub shared_memory: usize,
    pub message_queues: usize,
    pub sockets: usize,
}

impl RegistryStatistics {
    fn new() -> Self {
        Self {
            active_processes: 0,
            total_resources: 0,
            unique_resources: 0,
            pipes: 0,
            shared_memory: 0,
            message_queues: 0,
            sockets: 0,
        }
    }
}

/// Create a global IPC process registry
pub fn create_global_registry(config: ProcessRegistryConfig) -> Arc<IpcProcessRegistry> {
    Arc::new(IpcProcessRegistry::new(config))
}

/// Create a process-aware IPC manager
pub fn create_process_aware_manager(registry: Arc<IpcProcessRegistry>) -> ProcessAwareIpcManager {
    ProcessAwareIpcManager::new(registry)
}

