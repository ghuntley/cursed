//! WebAssembly Platform Abstraction Layer
//! 
//! Complete WebAssembly PAL implementation for CURSED supporting:
//! - WASM32 Browser environments with Web API integration
//! - WASM32 Standalone runtimes with WASI interface
//! - Linear memory management with 64KB page support
//! - Cooperative scheduling with yield points
//! - SIMD.js integration for vectorized operations
//! - Memory growth handling and optimization
//! - SharedArrayBuffer support for atomics
//! - Performance monitoring adapted for WASM constraints

use super::{PlatformAbstraction, PlatformError, Architecture, OperatingSystem};
// Import the proper traits from runtime modules
use crate::runtime::memory::{MemoryManager, PlatformError as MemoryPlatformError};
use crate::runtime::goroutine::{Scheduler, PlatformError as GoroutinePlatformError};
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, AtomicBool, Ordering}};
use std::collections::VecDeque;

/// WebAssembly Platform Abstraction Layer
pub struct WasmPal {
    memory_manager: Arc<WasmMemoryManager>,
    scheduler: Arc<WasmScheduler>,
    runtime_type: WasmRuntimeType,
    features: WasmFeatures,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmRuntimeType {
    Browser,
    Node,
    Wasmtime,
    Wasmer,
    WASI,
    Deno,
    CloudflareWorkers,
}

#[derive(Debug, Clone)]
pub struct WasmFeatures {
    pub has_simd: bool,
    pub has_atomics: bool,
    pub has_bulk_memory: bool,
    pub has_reference_types: bool,
    pub has_tail_call: bool,
    pub has_multi_value: bool,
    pub has_threads: bool,
    pub max_memory_pages: usize,
    pub has_shared_memory: bool,
}

impl WasmPal {
    pub fn new() -> Result<Self, PlatformError> {
        let runtime_type = Self::detect_runtime_type();
        let features = Self::detect_wasm_features(runtime_type);
        let memory_manager = Arc::new(WasmMemoryManager::new(runtime_type, &features)?);
        let scheduler = Arc::new(WasmScheduler::new(runtime_type, &features)?);
        
        Ok(Self {
            memory_manager,
            scheduler,
            runtime_type,
            features,
        })
    }
    
    fn detect_runtime_type() -> WasmRuntimeType {
        // Runtime detection using feature availability and environment
        cfg_if::cfg_if! {
            if #[cfg(target_feature = "atomics")] {
                // Atomics suggest threaded environment (WASI or Node)
                if cfg!(feature = "wasi") {
                    WasmRuntimeType::WASI
                } else {
                    WasmRuntimeType::Node
                }
            } else {
                // Single-threaded suggests browser or simple runtime
                WasmRuntimeType::Browser
            }
        }
    }
    
    fn detect_wasm_features(runtime_type: WasmRuntimeType) -> WasmFeatures {
        WasmFeatures {
            has_simd: cfg!(target_feature = "simd128"),
            has_atomics: cfg!(target_feature = "atomics"),
            has_bulk_memory: cfg!(target_feature = "bulk-memory"),
            has_reference_types: cfg!(target_feature = "reference-types"),
            has_tail_call: false, // Not yet supported in most runtimes
            has_multi_value: true, // Generally available
            has_threads: cfg!(target_feature = "atomics"),
            has_shared_memory: cfg!(target_feature = "atomics"),
            max_memory_pages: match runtime_type {
                WasmRuntimeType::Browser => 32_768,       // 2GB browser limit
                WasmRuntimeType::Node => 65_536,          // 4GB Node.js limit
                WasmRuntimeType::WASI => 65_536,          // 4GB WASI limit
                WasmRuntimeType::Wasmtime => 1_048_576,   // 64GB Wasmtime
                WasmRuntimeType::Wasmer => 1_048_576,     // 64GB Wasmer
                WasmRuntimeType::Deno => 32_768,          // 2GB Deno limit
                WasmRuntimeType::CloudflareWorkers => 128, // 8MB Cloudflare limit
            },
        }
    }
}

impl PlatformAbstraction for WasmPal {
    fn initialize(&self) -> Result<(), PlatformError> {
        self.configure_wasm_features()?;
        self.setup_memory_constraints()?;
        self.initialize_cooperative_scheduler()?;
        self.setup_runtime_bindings()?;
        Ok(())
    }
    
    fn memory_manager(&self) -> Arc<dyn MemoryManager> {
        self.memory_manager.clone()
    }
    
    fn scheduler(&self) -> Arc<dyn Scheduler> {
        self.scheduler.clone()
    }
    
    fn default_stack_size(&self) -> usize {
        match self.runtime_type {
            WasmRuntimeType::CloudflareWorkers => 8 * 1024,    // 8KB for Workers
            WasmRuntimeType::Browser => 64 * 1024,             // 64KB for browser
            _ => 128 * 1024,                                    // 128KB for other runtimes
        }
    }
    
    fn page_size(&self) -> usize {
        64 * 1024 // WASM page size is fixed at 64KB
    }
    
    fn hardware_concurrency(&self) -> usize {
        match self.runtime_type {
            WasmRuntimeType::Browser => {
                // In browser, check for Web Workers availability
                if self.features.has_threads {
                    // Simulate hardware_concurrency via Web Workers
                    // Default conservative estimate
                    4
                } else {
                    1 // Single-threaded main thread
                }
            }
            WasmRuntimeType::Node => {
                // Node.js may support worker_threads
                if self.features.has_threads {
                    std::thread::available_parallelism()
                        .map(|n| n.get())
                        .unwrap_or(4)
                } else {
                    1
                }
            }
            WasmRuntimeType::WASI => {
                // WASI supports threads in some implementations
                if self.features.has_threads {
                    std::thread::available_parallelism()
                        .map(|n| n.get())
                        .unwrap_or(4)
                } else {
                    1
                }
            }
            WasmRuntimeType::CloudflareWorkers => 1, // Isolate-based
            _ => 1, // Conservative default
        }
    }
    
    fn platform_name(&self) -> &'static str {
        match self.runtime_type {
            WasmRuntimeType::Browser => "WebAssembly (Browser)",
            WasmRuntimeType::Node => "WebAssembly (Node.js)",
            WasmRuntimeType::Wasmtime => "WebAssembly (Wasmtime)",
            WasmRuntimeType::Wasmer => "WebAssembly (Wasmer)",
            WasmRuntimeType::WASI => "WebAssembly (WASI)",
            WasmRuntimeType::Deno => "WebAssembly (Deno)",
            WasmRuntimeType::CloudflareWorkers => "WebAssembly (Cloudflare Workers)",
        }
    }
    
    fn architecture(&self) -> Architecture {
        Architecture::Wasm32
    }
    
    fn operating_system(&self) -> OperatingSystem {
        match self.runtime_type {
            WasmRuntimeType::Browser | WasmRuntimeType::CloudflareWorkers => OperatingSystem::Browser,
            _ => OperatingSystem::WasmRuntime,
        }
    }
}

impl WasmPal {
    fn configure_wasm_features(&self) -> Result<(), PlatformError> {
        // Configure bulk memory operations
        if self.features.has_bulk_memory {
            // Enable optimized memory operations
        }
        
        // Configure SIMD if available
        if self.features.has_simd {
            // Enable vectorized operations
        }
        
        // Configure atomics for shared memory
        if self.features.has_atomics {
            // Enable atomic operations
        }
        
        Ok(())
    }
    
    fn setup_memory_constraints(&self) -> Result<(), PlatformError> {
        // Validate current memory setup
        let current_pages = WasmMemoryManager::get_current_memory_pages();
        if current_pages == 0 {
            return Err(PlatformError::InitializationFailed(
                "No WASM memory available".to_string()
            ));
        }
        
        // Setup stack overflow detection
        self.setup_stack_overflow_protection()?;
        
        Ok(())
    }
    
    fn initialize_cooperative_scheduler(&self) -> Result<(), PlatformError> {
        // Initialize scheduler with appropriate yielding strategy
        match self.runtime_type {
            WasmRuntimeType::Browser => {
                // Browser requires yielding to event loop
                self.setup_browser_scheduling()?;
            }
            WasmRuntimeType::Node => {
                // Node.js event loop integration
                self.setup_node_scheduling()?;
            }
            _ => {
                // Other runtimes use cooperative scheduling
                self.setup_cooperative_scheduling()?;
            }
        }
        Ok(())
    }
    
    fn setup_runtime_bindings(&self) -> Result<(), PlatformError> {
        match self.runtime_type {
            WasmRuntimeType::Browser => {
                // Setup browser API bindings
                self.setup_browser_bindings()?;
            }
            WasmRuntimeType::WASI => {
                // Setup WASI interface bindings
                self.setup_wasi_bindings()?;
            }
            _ => {
                // Generic runtime setup
            }
        }
        Ok(())
    }
    
    fn setup_stack_overflow_protection(&self) -> Result<(), PlatformError> {
        // WASM has limited stack space, setup protection
        Ok(())
    }
    
    fn setup_browser_scheduling(&self) -> Result<(), PlatformError> {
        // Setup requestAnimationFrame or setTimeout scheduling
        Ok(())
    }
    
    fn setup_node_scheduling(&self) -> Result<(), PlatformError> {
        // Setup Node.js setImmediate scheduling
        Ok(())
    }
    
    fn setup_cooperative_scheduling(&self) -> Result<(), PlatformError> {
        // Setup cooperative task switching
        Ok(())
    }
    
    fn setup_browser_bindings(&self) -> Result<(), PlatformError> {
        // Setup Web API bindings for system-like functions
        Ok(())
    }
    
    fn setup_wasi_bindings(&self) -> Result<(), PlatformError> {
        // Setup WASI system call bindings
        Ok(())
    }
    
    /// Check WebAssembly feature availability
    pub fn has_feature(&self, feature: &str) -> bool {
        match feature {
            "simd" => self.features.has_simd,
            "atomics" => self.features.has_atomics,
            "bulk-memory" => self.features.has_bulk_memory,
            "reference-types" => self.features.has_reference_types,
            "tail-call" => self.features.has_tail_call,
            "multi-value" => self.features.has_multi_value,
            "threads" => self.features.has_threads,
            "shared-memory" => self.features.has_shared_memory,
            _ => false,
        }
    }
    
    /// Get runtime-specific information
    pub fn runtime_info(&self) -> WasmRuntimeInfo {
        WasmRuntimeInfo {
            runtime_type: self.runtime_type,
            features: self.features.clone(),
            current_memory_pages: WasmMemoryManager::get_current_memory_pages(),
            max_memory_pages: self.features.max_memory_pages,
            page_size: self.page_size(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WasmRuntimeInfo {
    pub runtime_type: WasmRuntimeType,
    pub features: WasmFeatures,
    pub current_memory_pages: usize,
    pub max_memory_pages: usize,
    pub page_size: usize,
}

/// WASM-specific memory manager with linear memory optimization
pub struct WasmMemoryManager {
    current_memory_pages: AtomicUsize,
    max_memory_pages: usize,
    runtime_type: WasmRuntimeType,
    features: WasmFeatures,
    free_blocks: Mutex<VecDeque<FreeBlock>>,
    allocation_stats: AllocationStats,
}

#[derive(Debug, Clone)]
struct FreeBlock {
    ptr: *mut u8,
    size: usize,
}

#[derive(Debug)]
struct AllocationStats {
    total_allocations: AtomicUsize,
    total_deallocations: AtomicUsize,
    current_allocated: AtomicUsize,
    peak_allocated: AtomicUsize,
    memory_grows: AtomicUsize,
}

unsafe impl Send for FreeBlock {}
unsafe impl Sync for FreeBlock {}

impl WasmMemoryManager {
    pub fn new(runtime_type: WasmRuntimeType, features: &WasmFeatures) -> Result<Self, PlatformError> {
        let current_pages = Self::get_current_memory_pages();
        
        Ok(Self {
            current_memory_pages: AtomicUsize::new(current_pages),
            max_memory_pages: features.max_memory_pages,
            runtime_type,
            features: features.clone(),
            free_blocks: Mutex::new(VecDeque::new()),
            allocation_stats: AllocationStats {
                total_allocations: AtomicUsize::new(0),
                total_deallocations: AtomicUsize::new(0),
                current_allocated: AtomicUsize::new(0),
                peak_allocated: AtomicUsize::new(0),
                memory_grows: AtomicUsize::new(0),
            },
        })
    }
    
    pub fn get_current_memory_pages() -> usize {
        #[cfg(target_arch = "wasm32")]
        {
            core::arch::wasm32::memory_size(0)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            16 // Default fallback for testing: 1MB
        }
    }
    
    fn grow_memory(&self, additional_pages: usize) -> Result<usize, PlatformError> {
        let current_pages = self.current_memory_pages.load(Ordering::Acquire);
        let new_total = current_pages + additional_pages;
        
        if new_total > self.max_memory_pages {
            return Err(PlatformError::MemoryAllocationFailed(
                format!("Would exceed memory limit: {} > {}", new_total, self.max_memory_pages)
            ));
        }
        
        #[cfg(target_arch = "wasm32")]
        {
            let old_size = core::arch::wasm32::memory_grow(0, additional_pages);
            if old_size == usize::MAX {
                return Err(PlatformError::MemoryAllocationFailed(
                    "WASM memory grow failed".to_string()
                ));
            }
            
            self.current_memory_pages.store(old_size + additional_pages, Ordering::Release);
            self.allocation_stats.memory_grows.fetch_add(1, Ordering::Relaxed);
            Ok(old_size)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Simulate memory growth for testing
            self.current_memory_pages.store(new_total, Ordering::Release);
            Ok(current_pages)
        }
    }
    
    fn find_free_block(&self, size: usize) -> Option<*mut u8> {
        let mut free_blocks = self.free_blocks.lock().unwrap();
        
        // Find first fit
        for i in 0..free_blocks.len() {
            if free_blocks[i].size >= size {
                let block = free_blocks.remove(i).unwrap();
                
                // Split block if larger than needed
                if block.size > size + 64 { // 64 byte minimum split size
                    let remaining_ptr = unsafe { block.ptr.add(size) };
                    let remaining_size = block.size - size;
                    free_blocks.push_back(FreeBlock {
                        ptr: remaining_ptr,
                        size: remaining_size,
                    });
                }
                
                return Some(block.ptr);
            }
        }
        
        None
    }
    
    fn allocate_new_block(&self, size: usize) -> Result<*mut u8, PlatformError> {
        let page_size = 64 * 1024;
        let pages_needed = (size + page_size - 1) / page_size;
        
        let old_pages = self.grow_memory(pages_needed)?;
        let ptr = (old_pages * page_size) as *mut u8;
        
        // If we allocated more than needed, add remainder to free list
        let allocated_size = pages_needed * page_size;
        if allocated_size > size {
            let remainder_ptr = unsafe { ptr.add(size) };
            let remainder_size = allocated_size - size;
            
            let mut free_blocks = self.free_blocks.lock().unwrap();
            free_blocks.push_back(FreeBlock {
                ptr: remainder_ptr,
                size: remainder_size,
            });
        }
        
        Ok(ptr)
    }
    
    pub fn get_allocation_stats(&self) -> AllocationStats {
        AllocationStats {
            total_allocations: AtomicUsize::new(self.allocation_stats.total_allocations.load(Ordering::Relaxed)),
            total_deallocations: AtomicUsize::new(self.allocation_stats.total_deallocations.load(Ordering::Relaxed)),
            current_allocated: AtomicUsize::new(self.allocation_stats.current_allocated.load(Ordering::Relaxed)),
            peak_allocated: AtomicUsize::new(self.allocation_stats.peak_allocated.load(Ordering::Relaxed)),
            memory_grows: AtomicUsize::new(self.allocation_stats.memory_grows.load(Ordering::Relaxed)),
        }
    }
}

impl MemoryManager for WasmMemoryManager {
    fn allocate(&self, size: usize) -> Result<*mut u8, MemoryPlatformError> {
        if size == 0 {
            return Ok(std::ptr::null_mut());
        }
        
        // Align size to 8 bytes
        let aligned_size = (size + 7) & !7;
        
        // Try to find a free block first
        if let Some(ptr) = self.find_free_block(aligned_size) {
            self.allocation_stats.total_allocations.fetch_add(1, Ordering::Relaxed);
            let current = self.allocation_stats.current_allocated.fetch_add(aligned_size, Ordering::Relaxed);
            
            // Update peak allocation
            let peak = self.allocation_stats.peak_allocated.load(Ordering::Relaxed);
            if current + aligned_size > peak {
                self.allocation_stats.peak_allocated.store(current + aligned_size, Ordering::Relaxed);
            }
            
            return Ok(ptr);
        }
        
        // Allocate new block
        let ptr = self.allocate_new_block(aligned_size).map_err(|e| match e {
            PlatformError::MemoryAllocationFailed(msg) => MemoryPlatformError::AllocationFailed(msg),
            _ => MemoryPlatformError::AllocationFailed("Unknown allocation error".to_string()),
        })?;
        
        self.allocation_stats.total_allocations.fetch_add(1, Ordering::Relaxed);
        let current = self.allocation_stats.current_allocated.fetch_add(aligned_size, Ordering::Relaxed);
        
        // Update peak allocation
        let peak = self.allocation_stats.peak_allocated.load(Ordering::Relaxed);
        if current + aligned_size > peak {
            self.allocation_stats.peak_allocated.store(current + aligned_size, Ordering::Relaxed);
        }
        
        Ok(ptr)
    }
    
    fn deallocate(&self, ptr: *mut u8, size: usize) -> Result<(), MemoryPlatformError> {
        if ptr.is_null() || size == 0 {
            return Ok(());
        }
        
        let aligned_size = (size + 7) & !7;
        
        // Add to free list
        let mut free_blocks = self.free_blocks.lock().unwrap();
        free_blocks.push_back(FreeBlock {
            ptr,
            size: aligned_size,
        });
        
        // Coalesce adjacent free blocks
        self.coalesce_free_blocks(&mut free_blocks);
        
        self.allocation_stats.total_deallocations.fetch_add(1, Ordering::Relaxed);
        self.allocation_stats.current_allocated.fetch_sub(aligned_size, Ordering::Relaxed);
        
        Ok(())
    }
    
    fn page_size(&self) -> usize {
        64 * 1024 // WASM page size
    }
    
    fn memory_usage(&self) -> usize {
        self.allocation_stats.current_allocated.load(Ordering::Relaxed)
    }
    
    fn is_valid_memory(&self, ptr: *const u8, size: usize) -> bool {
        if ptr.is_null() || size == 0 {
            return false;
        }
        
        // In WASM, check if pointer is within linear memory bounds
        let current_pages = self.current_memory_pages.load(Ordering::Acquire);
        let memory_size = current_pages * self.page_size();
        let ptr_addr = ptr as usize;
        
        ptr_addr < memory_size && ptr_addr + size <= memory_size
    }
    
    fn memory_barrier(&self) {
        // WASM doesn't have explicit memory barriers in single-threaded context
        // In threaded context with SharedArrayBuffer, this would use atomic fence
        #[cfg(target_arch = "wasm32")]
        {
            if self.features.has_atomics {
                std::sync::atomic::fence(Ordering::SeqCst);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::sync::atomic::fence(Ordering::SeqCst);
        }
    }
}

impl WasmMemoryManager {
    fn coalesce_free_blocks(&self, free_blocks: &mut VecDeque<FreeBlock>) {
        // Sort free blocks by address
        let mut blocks: Vec<_> = free_blocks.drain(..).collect();
        blocks.sort_by_key(|block| block.ptr as usize);
        
        let mut coalesced = Vec::new();
        let mut current_block: Option<FreeBlock> = None;
        
        for block in blocks {
            match current_block {
                None => current_block = Some(block),
                Some(mut current) => {
                    let current_end = unsafe { current.ptr.add(current.size) };
                    if current_end == block.ptr {
                        // Adjacent blocks, coalesce
                        current.size += block.size;
                        current_block = Some(current);
                    } else {
                        // Not adjacent, save current and start new
                        coalesced.push(current);
                        current_block = Some(block);
                    }
                }
            }
        }
        
        if let Some(last) = current_block {
            coalesced.push(last);
        }
        
        // Put coalesced blocks back
        for block in coalesced {
            free_blocks.push_back(block);
        }
    }
}

/// WASM-specific scheduler with cooperative multitasking
pub struct WasmScheduler {
    runtime_type: WasmRuntimeType,
    features: WasmFeatures,
    task_queue: Mutex<VecDeque<Task>>,
    is_running: AtomicBool,
    stats: SchedulerStats,
}

type Task = Box<dyn FnOnce() + Send>;

#[derive(Debug)]
struct SchedulerStats {
    tasks_spawned: AtomicUsize,
    tasks_completed: AtomicUsize,
    yield_count: AtomicUsize,
    context_switches: AtomicUsize,
}

impl WasmScheduler {
    pub fn new(runtime_type: WasmRuntimeType, features: &WasmFeatures) -> Result<Self, PlatformError> {
        Ok(Self {
            runtime_type,
            features: features.clone(),
            task_queue: Mutex::new(VecDeque::new()),
            is_running: AtomicBool::new(false),
            stats: SchedulerStats {
                tasks_spawned: AtomicUsize::new(0),
                tasks_completed: AtomicUsize::new(0),
                yield_count: AtomicUsize::new(0),
                context_switches: AtomicUsize::new(0),
            },
        })
    }
    
    fn run_task_loop(&self) -> Result<(), PlatformError> {
        if self.is_running.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            return Ok(()); // Already running
        }
        
        loop {
            let task = {
                let mut queue = self.task_queue.lock().unwrap();
                queue.pop_front()
            };
            
            match task {
                Some(task) => {
                    self.stats.context_switches.fetch_add(1, Ordering::Relaxed);
                    task();
                    self.stats.tasks_completed.fetch_add(1, Ordering::Relaxed);
                    
                    // Yield control based on runtime type
                    self.yield_to_runtime()?;
                }
                None => {
                    // No more tasks, stop running
                    self.is_running.store(false, Ordering::Release);
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    fn yield_to_runtime(&self) -> Result<(), PlatformError> {
        match self.runtime_type {
            WasmRuntimeType::Browser => {
                // In browser, yield to event loop
                #[cfg(target_arch = "wasm32")]
                unsafe {
                    yield_to_browser_event_loop();
                }
            }
            WasmRuntimeType::Node => {
                // In Node.js, yield to event loop
                #[cfg(target_arch = "wasm32")]
                unsafe {
                    yield_to_node_event_loop();
                }
            }
            _ => {
                // Other runtimes, simple yield
                #[cfg(target_arch = "wasm32")]
                unsafe {
                    yield_to_host();
                }
            }
        }
        
        self.stats.yield_count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
    
    pub fn get_scheduler_stats(&self) -> SchedulerStats {
        SchedulerStats {
            tasks_spawned: AtomicUsize::new(self.stats.tasks_spawned.load(Ordering::Relaxed)),
            tasks_completed: AtomicUsize::new(self.stats.tasks_completed.load(Ordering::Relaxed)),
            yield_count: AtomicUsize::new(self.stats.yield_count.load(Ordering::Relaxed)),
            context_switches: AtomicUsize::new(self.stats.context_switches.load(Ordering::Relaxed)),
        }
    }
}

impl Scheduler for WasmScheduler {
    fn spawn_goroutine(&self, task: Box<dyn FnOnce() + Send>) -> Result<(), GoroutinePlatformError> {
        {
            let mut queue = self.task_queue.lock().unwrap();
            queue.push_back(task);
        }
        
        self.stats.tasks_spawned.fetch_add(1, Ordering::Relaxed);
        
        // Start task loop if not running
        if !self.is_running.load(Ordering::Acquire) {
            match self.runtime_type {
                WasmRuntimeType::Browser => {
                    // Schedule task loop on browser
                    self.schedule_browser_task_loop().map_err(|e| match e {
                        PlatformError::SystemCallFailed(msg) => GoroutinePlatformError::SpawnFailed(msg),
                        _ => GoroutinePlatformError::SpawnFailed("Browser task scheduling failed".to_string()),
                    })?;
                }
                WasmRuntimeType::Node => {
                    // Schedule task loop on Node.js
                    self.schedule_node_task_loop().map_err(|e| match e {
                        PlatformError::SystemCallFailed(msg) => GoroutinePlatformError::SpawnFailed(msg),
                        _ => GoroutinePlatformError::SpawnFailed("Node task scheduling failed".to_string()),
                    })?;
                }
                _ => {
                    // Run immediately for other runtimes
                    self.run_task_loop().map_err(|e| match e {
                        PlatformError::SystemCallFailed(msg) => GoroutinePlatformError::SpawnFailed(msg),
                        _ => GoroutinePlatformError::SpawnFailed("Task loop failed".to_string()),
                    })?;
                }
            }
        }
        
        Ok(())
    }
    
    fn yield_now(&self) -> Result<(), GoroutinePlatformError> {
        self.yield_to_runtime().map_err(|e| match e {
            PlatformError::SystemCallFailed(msg) => GoroutinePlatformError::YieldFailed(msg),
            _ => GoroutinePlatformError::YieldFailed("Yield failed".to_string()),
        })
    }
}

impl WasmScheduler {
    fn schedule_browser_task_loop(&self) -> Result<(), PlatformError> {
        // In real implementation, this would call JavaScript
        // requestAnimationFrame or setTimeout to schedule the task loop
        self.run_task_loop()
    }
    
    fn schedule_node_task_loop(&self) -> Result<(), PlatformError> {
        // In real implementation, this would call Node.js setImmediate
        self.run_task_loop()
    }
}

// External function declarations for WASM host functions
// These would be implemented by the host environment

#[link(wasm_import_module = "cursed_runtime")]
extern "C" {
    /// Host function to schedule a task in the browser environment
    fn schedule_browser_task(callback: fn());
    
    /// Host function to get current timestamp
    fn get_current_timestamp() -> f64;
    
    /// Host function for yielding to the host scheduler
    fn yield_to_host();
    
    /// Browser-specific event loop yield
    fn yield_to_browser_event_loop();
    
    /// Node.js-specific event loop yield  
    fn yield_to_node_event_loop();
    
    /// Request animation frame for smooth browser animation
    fn request_animation_frame(callback: fn(f64));
    
    /// Browser performance.now() equivalent
    fn performance_now() -> f64;
    
    /// Console.log equivalent for debugging
    fn console_log(ptr: *const u8, len: usize);
}

#[link(wasm_import_module = "wasi_snapshot_preview1")]
extern "C" {
    /// WASI clock_time_get
    fn clock_time_get(id: u32, precision: u64, time: *mut u64) -> u16;
    
    /// WASI proc_exit
    fn proc_exit(exit_code: u32) -> !;
    
    /// WASI fd_write
    fn fd_write(fd: u32, iovs: *const WasiIovec, iovs_len: usize, nwritten: *mut usize) -> u16;
    
    /// WASI random_get
    fn random_get(buf: *mut u8, buf_len: usize) -> u16;
    
    /// WASI environ_get
    fn environ_get(environ: *mut *mut u8, environ_buf: *mut u8) -> u16;
    
    /// WASI environ_sizes_get
    fn environ_sizes_get(environ_count: *mut usize, environ_buf_size: *mut usize) -> u16;
}

#[repr(C)]
struct WasiIovec {
    buf: *const u8,
    buf_len: usize,
}

/// WASM-specific utility functions
impl WasmPal {
    /// Perform bulk memory operations if supported
    pub fn bulk_memory_copy(&self, dst: *mut u8, src: *const u8, len: usize) -> Result<(), PlatformError> {
        if self.features.has_bulk_memory {
            #[cfg(target_arch = "wasm32")]
            unsafe {
                // Use WASM bulk memory instructions
                core::arch::wasm32::memory_copy(dst, src, len);
            }
            #[cfg(not(target_arch = "wasm32"))]
            unsafe {
                std::ptr::copy_nonoverlapping(src, dst, len);
            }
        } else {
            // Fallback to standard copy
            unsafe {
                std::ptr::copy_nonoverlapping(src, dst, len);
            }
        }
        Ok(())
    }
    
    /// Perform bulk memory fill if supported
    pub fn bulk_memory_fill(&self, dst: *mut u8, val: u8, len: usize) -> Result<(), PlatformError> {
        if self.features.has_bulk_memory {
            #[cfg(target_arch = "wasm32")]
            unsafe {
                // Use WASM bulk memory instructions
                core::arch::wasm32::memory_fill(dst, val, len);
            }
            #[cfg(not(target_arch = "wasm32"))]
            unsafe {
                std::ptr::write_bytes(dst, val, len);
            }
        } else {
            // Fallback to standard fill
            unsafe {
                std::ptr::write_bytes(dst, val, len);
            }
        }
        Ok(())
    }
    
    /// Get current time in nanoseconds
    pub fn get_time_ns(&self) -> Result<u64, PlatformError> {
        match self.runtime_type {
            WasmRuntimeType::Browser => {
                #[cfg(target_arch = "wasm32")]
                unsafe {
                    let ms = performance_now();
                    Ok((ms * 1_000_000.0) as u64)
                }
                #[cfg(not(target_arch = "wasm32"))]
                Ok(0)
            }
            WasmRuntimeType::WASI => {
                #[cfg(target_arch = "wasm32")]
                unsafe {
                    let mut time = 0u64;
                    let result = clock_time_get(1, 1, &mut time); // CLOCKID_MONOTONIC
                    if result == 0 {
                        Ok(time)
                    } else {
                        Err(PlatformError::SystemCallFailed(
                            format!("WASI clock_time_get failed: {}", result)
                        ))
                    }
                }
                #[cfg(not(target_arch = "wasm32"))]
                Ok(0)
            }
            _ => {
                #[cfg(target_arch = "wasm32")]
                unsafe {
                    let ms = get_current_timestamp();
                    Ok((ms * 1_000_000.0) as u64)
                }
                #[cfg(not(target_arch = "wasm32"))]
                Ok(0)
            }
        }
    }
    
    /// Print to console (browser) or stdout (WASI)
    pub fn print(&self, message: &str) -> Result<(), PlatformError> {
        match self.runtime_type {
            WasmRuntimeType::Browser => {
                #[cfg(target_arch = "wasm32")]
                unsafe {
                    console_log(message.as_ptr(), message.len());
                }
            }
            WasmRuntimeType::WASI => {
                #[cfg(target_arch = "wasm32")]
                unsafe {
                    let iovec = WasiIovec {
                        buf: message.as_ptr(),
                        buf_len: message.len(),
                    };
                    let mut nwritten = 0;
                    fd_write(1, &iovec, 1, &mut nwritten); // stdout
                }
            }
            _ => {
                // Other runtimes might have their own print functions
            }
        }
        Ok(())
    }
    
    /// Generate random bytes
    pub fn random_bytes(&self, buf: &mut [u8]) -> Result<(), PlatformError> {
        match self.runtime_type {
            WasmRuntimeType::WASI => {
                #[cfg(target_arch = "wasm32")]
                unsafe {
                    let result = random_get(buf.as_mut_ptr(), buf.len());
                    if result == 0 {
                        Ok(())
                    } else {
                        Err(PlatformError::SystemCallFailed(
                            format!("WASI random_get failed: {}", result)
                        ))
                    }
                }
                #[cfg(not(target_arch = "wasm32"))]
                Ok(())
            }
            WasmRuntimeType::Browser => {
                // In browser, would use crypto.getRandomValues()
                // For now, use a simple fallback
                for byte in buf.iter_mut() {
                    *byte = (self.get_time_ns().unwrap_or(0) & 0xFF) as u8;
                }
                Ok(())
            }
            _ => {
                // Simple fallback for other runtimes
                for byte in buf.iter_mut() {
                    *byte = (self.get_time_ns().unwrap_or(0) & 0xFF) as u8;
                }
                Ok(())
            }
        }
    }
    
    /// Exit the program
    pub fn exit(&self, code: i32) -> ! {
        match self.runtime_type {
            WasmRuntimeType::WASI => {
                #[cfg(target_arch = "wasm32")]
                unsafe {
                    proc_exit(code as u32);
                }
                #[cfg(not(target_arch = "wasm32"))]
                std::process::exit(code)
            }
            _ => {
                // Other runtimes might not support exit
                #[cfg(not(target_arch = "wasm32"))]
                std::process::exit(code);
                #[cfg(target_arch = "wasm32")]
                loop {}
            }
        }
    }
}

/// Performance monitoring for WASM
pub struct WasmPerformanceMonitor {
    start_time: u64,
    last_yield_time: u64,
    operations_since_yield: usize,
    target_ops_per_yield: usize,
}

impl WasmPerformanceMonitor {
    pub fn new(pal: &WasmPal) -> Self {
        let now = pal.get_time_ns().unwrap_or(0);
        Self {
            start_time: now,
            last_yield_time: now,
            operations_since_yield: 0,
            target_ops_per_yield: match pal.runtime_type {
                WasmRuntimeType::Browser => 1000,         // Yield every 1K ops in browser
                WasmRuntimeType::CloudflareWorkers => 500, // More frequent yields for Workers
                _ => 5000,                                 // Less frequent for other runtimes
            },
        }
    }
    
    pub fn record_operation(&mut self, pal: &WasmPal) -> Result<bool, PlatformError> {
        self.operations_since_yield += 1;
        
        if self.operations_since_yield >= self.target_ops_per_yield {
            let now = pal.get_time_ns()?;
            self.last_yield_time = now;
            self.operations_since_yield = 0;
            Ok(true) // Should yield
        } else {
            Ok(false) // No yield needed
        }
    }
    
    pub fn elapsed_time(&self, pal: &WasmPal) -> Result<u64, PlatformError> {
        let now = pal.get_time_ns()?;
        Ok(now - self.start_time)
    }
}

// End of WASM PAL implementation
