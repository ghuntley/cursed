//! Runtime Stack Management for CURSED
//!
//! This module provides stack management functionality for the CURSED runtime,
//! including goroutine stack allocation, deallocation, and integration with
//! the garbage collection system.

use crate::error::CursedError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Default stack size for goroutines (2MB)
const DEFAULT_STACK_SIZE: usize = 2 * 1024 * 1024;

/// Minimum stack size (4KB)
const MIN_STACK_SIZE: usize = 4 * 1024;

/// Maximum stack size (16MB)
const MAX_STACK_SIZE: usize = 16 * 1024 * 1024;

/// Stack identifier type
pub type StackId = usize;

/// Stack segment information
#[derive(Debug, Clone)]
pub struct StackSegment {
    pub base: *mut u8,
    pub size: usize,
    pub used: usize,
    pub guard_page: Option<*mut u8>,
}

unsafe impl Send for StackSegment {}
unsafe impl Sync for StackSegment {}

/// Stack frame information for debugging and GC integration
#[derive(Debug, Clone)]
pub struct StackFrame {
    pub id: usize,
    pub function_name: String,
    pub locals: Vec<*mut u8>, // Pointers to local variables for GC
    pub stack_pointer: *mut u8,
    pub frame_pointer: *mut u8,
}

unsafe impl Send for StackFrame {}
unsafe impl Sync for StackFrame {}

/// Runtime stack manager for goroutines
pub struct RuntimeStack {
    /// Stack segments indexed by stack ID
    stacks: RwLock<HashMap<StackId, StackSegment>>,
    /// Stack frames for GC integration 
    frames: RwLock<HashMap<StackId, Vec<StackFrame>>>,
    /// Next available stack ID
    next_id: AtomicUsize,
    /// Stack allocation statistics
    stats: Mutex<StackStats>,
    /// Configuration
    config: StackConfig,
}

/// Stack configuration
#[derive(Debug, Clone)]
pub struct StackConfig {
    pub default_size: usize,
    pub min_size: usize,
    pub max_size: usize,
    pub enable_guard_pages: bool,
    pub enable_gc_integration: bool,
}

impl Default for StackConfig {
    fn default() -> Self {
        Self {
            default_size: DEFAULT_STACK_SIZE,
            min_size: MIN_STACK_SIZE,
            max_size: MAX_STACK_SIZE,
            enable_guard_pages: true,
            enable_gc_integration: true,
        }
    }
}

/// Stack allocation statistics
#[derive(Debug, Default, Clone)]
pub struct StackStats {
    pub total_allocated: usize,
    pub total_deallocated: usize,
    pub current_stacks: usize,
    pub peak_stacks: usize,
    pub total_memory_used: usize,
    pub guard_page_faults: usize,
}

impl RuntimeStack {
    /// Create a new runtime stack manager
    pub fn new() -> Self {
        Self::with_config(StackConfig::default())
    }

    /// Create a new runtime stack manager with custom configuration
    pub fn with_config(config: StackConfig) -> Self {
        Self {
            stacks: RwLock::new(HashMap::new()),
            frames: RwLock::new(HashMap::new()),
            next_id: AtomicUsize::new(1),
            stats: Mutex::new(StackStats::default()),
            config,
        }
    }

    /// Allocate a new stack segment
    pub fn allocate_stack(&self, size: Option<usize>) -> Result<StackId, CursedError> {
        let stack_size = size.unwrap_or(self.config.default_size);
        
        // Validate size
        if stack_size < self.config.min_size || stack_size > self.config.max_size {
            return Err(CursedError::runtime_error(&format!(
                "Invalid stack size: {} (must be between {} and {})",
                stack_size, self.config.min_size, self.config.max_size
            )));
        }

        let stack_id = self.next_id.fetch_add(1, Ordering::SeqCst);

        // Allocate stack memory
        let stack_base = self.allocate_memory(stack_size)?;
        
        // Set up guard page if enabled
        let guard_page = if self.config.enable_guard_pages {
            self.setup_guard_page(stack_base, stack_size)?
        } else {
            None
        };

        let segment = StackSegment {
            base: stack_base,
            size: stack_size,
            used: 0,
            guard_page,
        };

        // Store the stack segment
        {
            let mut stacks = self.stacks.write().unwrap();
            stacks.insert(stack_id, segment);
        }

        // Initialize frames for this stack
        {
            let mut frames = self.frames.write().unwrap();
            frames.insert(stack_id, Vec::new());
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_allocated += 1;
            stats.current_stacks += 1;
            stats.total_memory_used += stack_size;
            if stats.current_stacks > stats.peak_stacks {
                stats.peak_stacks = stats.current_stacks;
            }
        }

        Ok(stack_id)
    }

    /// Deallocate a stack segment
    pub fn deallocate_stack(&self, stack_id: StackId) -> Result<(), CursedError> {
        let segment = {
            let mut stacks = self.stacks.write().unwrap();
            stacks.remove(&stack_id)
        };

        if let Some(segment) = segment {
            // Clean up guard page
            if let Some(guard_page) = segment.guard_page {
                self.cleanup_guard_page(guard_page)?;
            }

            // Deallocate memory
            self.deallocate_memory(segment.base, segment.size)?;

            // Remove frames
            {
                let mut frames = self.frames.write().unwrap();
                frames.remove(&stack_id);
            }

            // Update statistics
            {
                let mut stats = self.stats.lock().unwrap();
                stats.total_deallocated += 1;
                stats.current_stacks = stats.current_stacks.saturating_sub(1);
                stats.total_memory_used = stats.total_memory_used.saturating_sub(segment.size);
            }

            Ok(())
        } else {
            Err(CursedError::runtime_error(&format!(
                "Invalid stack ID: {}", stack_id
            )))
        }
    }

    /// Get stack pointer for a given stack
    pub fn get_stack_pointer(&self, stack_id: StackId) -> Result<*mut u8, CursedError> {
        let stacks = self.stacks.read().unwrap();
        if let Some(segment) = stacks.get(&stack_id) {
            // Stack grows downward, so pointer starts at top
            Ok(unsafe { segment.base.add(segment.size) })
        } else {
            Err(CursedError::runtime_error(&format!(
                "Invalid stack ID: {}", stack_id
            )))
        }
    }

    /// Push a new stack frame
    pub fn push_frame(&self, stack_id: StackId, frame: StackFrame) -> Result<(), CursedError> {
        let mut frames = self.frames.write().unwrap();
        if let Some(frame_stack) = frames.get_mut(&stack_id) {
            frame_stack.push(frame);
            Ok(())
        } else {
            Err(CursedError::runtime_error(&format!(
                "Invalid stack ID: {}", stack_id
            )))
        }
    }

    /// Pop the top stack frame
    pub fn pop_frame(&self, stack_id: StackId) -> Result<Option<StackFrame>, CursedError> {
        let mut frames = self.frames.write().unwrap();
        if let Some(frame_stack) = frames.get_mut(&stack_id) {
            Ok(frame_stack.pop())
        } else {
            Err(CursedError::runtime_error(&format!(
                "Invalid stack ID: {}", stack_id
            )))
        }
    }

    /// Get current stack frames for garbage collection
    pub fn get_gc_roots(&self, stack_id: StackId) -> Result<Vec<*mut u8>, CursedError> {
        if !self.config.enable_gc_integration {
            return Ok(Vec::new());
        }

        let frames = self.frames.read().unwrap();
        if let Some(frame_stack) = frames.get(&stack_id) {
            let mut roots = Vec::new();
            for frame in frame_stack {
                roots.extend_from_slice(&frame.locals);
            }
            Ok(roots)
        } else {
            Err(CursedError::runtime_error(&format!(
                "Invalid stack ID: {}", stack_id
            )))
        }
    }

    /// Get all GC roots from all stacks
    pub fn get_all_gc_roots(&self) -> Vec<*mut u8> {
        if !self.config.enable_gc_integration {
            return Vec::new();
        }

        let frames = self.frames.read().unwrap();
        let mut all_roots = Vec::new();
        
        for frame_stack in frames.values() {
            for frame in frame_stack {
                all_roots.extend_from_slice(&frame.locals);
            }
        }
        
        all_roots
    }

    /// Get stack statistics
    pub fn get_stats(&self) -> StackStats {
        self.stats.lock().unwrap().clone()
    }

    /// Check if a stack exists
    pub fn stack_exists(&self, stack_id: StackId) -> bool {
        let stacks = self.stacks.read().unwrap();
        stacks.contains_key(&stack_id)
    }

    /// Get information about a specific stack
    pub fn get_stack_info(&self, stack_id: StackId) -> Result<StackInfo, CursedError> {
        let stacks = self.stacks.read().unwrap();
        let frames = self.frames.read().unwrap();
        
        if let Some(segment) = stacks.get(&stack_id) {
            let frame_count = frames.get(&stack_id).map(|f| f.len()).unwrap_or(0);
            
            Ok(StackInfo {
                id: stack_id,
                size: segment.size,
                used: segment.used,
                frame_count,
                has_guard_page: segment.guard_page.is_some(),
            })
        } else {
            Err(CursedError::runtime_error(&format!(
                "Invalid stack ID: {}", stack_id
            )))
        }
    }

    // Private helper methods
    fn allocate_memory(&self, size: usize) -> Result<*mut u8, CursedError> {
        use std::alloc::{alloc, Layout};
        
        let layout = Layout::from_size_align(size, 4096)
            .map_err(|e| CursedError::runtime_error(&format!("Layout error: {}", e)))?;
        
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            Err(CursedError::runtime_error("Failed to allocate stack memory"))
        } else {
            Ok(ptr)
        }
    }

    fn deallocate_memory(&self, ptr: *mut u8, size: usize) -> Result<(), CursedError> {
        use std::alloc::{dealloc, Layout};
        
        let layout = Layout::from_size_align(size, 4096)
            .map_err(|e| CursedError::runtime_error(&format!("Layout error: {}", e)))?;
        
        unsafe { dealloc(ptr, layout) };
        Ok(())
    }

    fn setup_guard_page(&self, _stack_base: *mut u8, _size: usize) -> Result<Option<*mut u8>, CursedError> {
        // Simplified guard page setup - in a real implementation this would use mprotect
        // For now, just return None to indicate no guard page
        Ok(None)
    }

    fn cleanup_guard_page(&self, _guard_page: *mut u8) -> Result<(), CursedError> {
        // Simplified guard page cleanup
        Ok(())
    }
}

/// Stack information for debugging and monitoring
#[derive(Debug, Clone)]
pub struct StackInfo {
    pub id: StackId,
    pub size: usize,
    pub used: usize,
    pub frame_count: usize,
    pub has_guard_page: bool,
}

// Keep the existing MinimalImplementation for compatibility
pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED advanced features enabled".to_string())
}
