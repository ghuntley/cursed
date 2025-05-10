//! Cycle Detection for Garbage Collection
//!
//! This module provides an implementation of cycle detection for the garbage collector.
//! It focuses on identifying circular references in an object graph and ensuring
//! they are correctly collected when no longer reachable from the root set.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, error, info, instrument, trace, warn};

use crate::memory::{Gc, Tag, Traceable, Visitor};
use crate::memory::gc::{GarbageCollector, GcObject, MarkState};
use crate::memory::object_storage::global_object_storage;

/// Constants for mark states to improve readability
pub const WHITE: MarkState = MarkState::White; // Not visited
pub const GRAY: MarkState = MarkState::Gray;   // Visited but not all references processed
pub const BLACK: MarkState = MarkState::Black; // Fully processed

/// Result of a garbage collection cycle
#[derive(Debug)]
pub enum CollectionResult {
    /// Collection was completed successfully
    Success(CollectionStats),
    /// Collection timed out during processing
    Timeout {
        stats: CollectionStats,
        phase: String,
    },
    /// Collection encountered an error
    Error(String),
}

/// Statistics from a garbage collection cycle
#[derive(Debug, Default)]
pub struct CollectionStats {
    pub initial_objects: usize,
    pub final_objects: usize,
    pub objects_freed: usize,
    pub bytes_freed: usize,
    pub mark_time_ms: u128,
    pub sweep_time_ms: u128,
    pub total_time_ms: u128,
    pub marked: usize,
}

/// Collects references during object tracing
#[derive(Debug)]
pub struct ReferenceCollector {
    pub references: Vec<usize>,
}

impl ReferenceCollector {
    pub fn new() -> Self {
        Self {
            references: Vec::new(),
        }
    }
    
    pub fn collect_references(obj_id: usize) -> Vec<usize> {
        let mut collector = Self::new();
        
        // Get the object from global storage
        let storage = global_object_storage();
        if let Ok(storage_lock) = storage.read() {
            if let Some(obj_box) = storage_lock.get_dyn_traceable(obj_id) {
                // Trace the object to collect its references
                unsafe {
                    let obj = &*obj_box.as_ptr();
                    obj.trace(&mut collector);
                }
            }
        }
        
        collector.references
    }
}

impl Visitor for ReferenceCollector {
    fn visit(&mut self, obj: std::ptr::NonNull<dyn Traceable>) {
        // Cast through a thin pointer first to get the address
        let ptr = obj.as_ptr();
        let addr = ptr as *const () as usize;
        self.references.push(addr);
    }
    
    fn visit_ptr(&mut self, ptr: usize, _tag: Tag) {
        self.references.push(ptr);
    }
}

impl GarbageCollector {
    /// Mark phase implementation that properly handles cycles
    pub fn mark_phase_with_cycle_detection(&self, timeout_ms: u64) -> Result<CollectionStats, String> {
        let start_time = Instant::now();
        let timeout = Duration::from_millis(timeout_ms);
        let mut stats = CollectionStats::default();
        
        // Get the root set
        let roots = {
            if let Ok(state) = self.inner.read() {
                state.roots.clone()
            } else {
                return Err("Failed to acquire read lock for root set".to_string());
            }
        };
        
        debug!(roots_count = roots.len(), "Starting mark phase with cycle detection");
        
        // Initialize our marking state
        {
            if let Ok(mut state) = self.inner.write() {
                // Record initial object count
                stats.initial_objects = state.objects.len();
                
                // Reset all objects to white (unmarked)
                for obj in state.objects.values_mut() {
                    obj.mark_state = WHITE;
                }
                
                // Initialize our gray object queue
                state.gray_objects.clear();
                
                // Mark all roots as gray and add to queue
                for &root in &roots {
                    if let Some(obj) = state.objects.get_mut(&root) {
                        obj.mark_state = GRAY;
                        state.gray_objects.push_back(root);
                    }
                }
                
                debug!(gray_count = state.gray_objects.len(), "Initialized gray queue with roots");
            } else {
                return Err("Failed to acquire write lock for initialization".to_string());
            }
        }
        
        // Process gray objects until queue is empty
        let mut marked_count = 0;
        
        loop {
            // Check for timeout
            if start_time.elapsed() > timeout {
                warn!("Mark phase timed out after {:?}", start_time.elapsed());
                stats.mark_time_ms = start_time.elapsed().as_millis();
                return Err(format!("Mark phase timed out after {}ms", stats.mark_time_ms));
            }
            
            // Get the next gray object
            let current = {
                if let Ok(mut state) = self.inner.write() {
                    state.gray_objects.pop_front()
                } else {
                    return Err("Failed to acquire write lock for gray queue".to_string());
                }
            };
            
            // If no more gray objects, we're done
            if current.is_none() {
                debug!("No more gray objects, mark phase complete");
                break;
            }
            
            let current_id = current.unwrap();
            trace!(id = current_id, "Processing gray object");
            
            // Mark the current object black
            {
                if let Ok(mut state) = self.inner.write() {
                    if let Some(obj) = state.objects.get_mut(&current_id) {
                        obj.mark_state = BLACK;
                        marked_count += 1;
                    }
                } else {
                    return Err("Failed to acquire write lock for marking".to_string());
                }
            }
            
            // Get the object's references
            let references = ReferenceCollector::collect_references(current_id);
            trace!(id = current_id, refs_count = references.len(), "Found references");
            
            // Process each reference
            for &ref_id in &references {
                // Check if the reference is in our object set and unmarked
                let should_mark = {
                    if let Ok(state) = self.inner.read() {
                        match state.objects.get(&ref_id) {
                            Some(obj) if obj.mark_state == WHITE => true,
                            _ => false,
                        }
                    } else {
                        continue; // Skip if we can't get a lock
                    }
                };
                
                // Mark the reference if needed
                if should_mark {
                    trace!(from = current_id, to = ref_id, "Marking reference");
                    if let Ok(mut state) = self.inner.write() {
                        if let Some(obj) = state.objects.get_mut(&ref_id) {
                            obj.mark_state = GRAY;
                            state.gray_objects.push_back(ref_id);
                        }
                    }
                }
            }
        }
        
        debug!(marked = marked_count, "Mark phase complete");
        stats.marked = marked_count;
        stats.mark_time_ms = start_time.elapsed().as_millis();
        
        Ok(stats)
    }
    
    /// Sweep phase implementation that removes unmarked objects
    pub fn sweep_phase(&self, timeout_ms: u64) -> Result<CollectionStats, String> {
        let start_time = Instant::now();
        let timeout = Duration::from_millis(timeout_ms);
        let mut stats = CollectionStats::default();
        
        // Find all unmarked (white) objects
        let unmarked_objects = {
            if let Ok(state) = self.inner.read() {
                stats.initial_objects = state.objects.len();
                
                // Collect all WHITE objects
                state.objects.iter()
                    .filter_map(|(&id, obj)| {
                        if obj.mark_state == WHITE {
                            Some(id)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<usize>>()
            } else {
                return Err("Failed to acquire read lock for sweep phase".to_string());
            }
        };
        
        debug!(unmarked_count = unmarked_objects.len(), "Found unmarked objects");
        
        // Process all unmarked objects
        for &obj_id in &unmarked_objects {
            // Check for timeout
            if start_time.elapsed() > timeout {
                warn!("Sweep phase timed out after {:?}", start_time.elapsed());
                stats.sweep_time_ms = start_time.elapsed().as_millis();
                return Err(format!("Sweep phase timed out after {}ms", stats.sweep_time_ms));
            }
            
            // Finalize the object (release resources, close files, etc.)
            self.finalize_object(obj_id);
            
            // Remove the object from the GC's object map
            if let Ok(mut state) = self.inner.write() {
                if let Some(obj) = state.objects.remove(&obj_id) {
                    stats.objects_freed += 1;
                    stats.bytes_freed += obj.size;
                    trace!(id = obj_id, size = obj.size, "Removed object");
                }
            }
        }
        
        // Get final object count
        if let Ok(state) = self.inner.read() {
            stats.final_objects = state.objects.len();
        }
        
        stats.sweep_time_ms = start_time.elapsed().as_millis();
        debug!(freed = stats.objects_freed, bytes = stats.bytes_freed, "Sweep phase complete");
        
        Ok(stats)
    }
    
    /// Finalize an object (release resources)
    fn finalize_object(&self, obj_id: usize) {
        trace!(id = obj_id, "Finalizing object");
        
        // Get the object from global storage
        let storage = global_object_storage();
        if let Ok(storage_lock) = storage.read() {
            if let Some(obj_box) = storage_lock.get_dyn_traceable(obj_id) {
                // Call finalize on the object
                unsafe {
                    let obj = &mut *(obj_box.as_ptr() as *mut dyn Traceable);
                    obj.finalize();
                }
            }
        }
    }
    
    /// Incremental collection step - processes a limited number of objects
    pub fn collect_garbage_incremental_impl(&self) -> CollectionResult {
        let start_time = Instant::now();
        let (timeout, step_size) = {
            let state = self.inner.read().unwrap();
            (Duration::from_millis(state.options.incremental_time_budget_ms), 
             state.options.incremental_step_size)
        };
        
        // Get the current gray objects queue
        let gray_count = {
            let state_opt = crate::memory::deadlock_detector::try_read_with_timeout(
                &self.inner,
                Some(1000), // 1 second timeout
                Some("collect_garbage_incremental (check gray queue)")
            );
            
            if let Some(state) = state_opt {
                state.gray_objects.len()
            } else {
                0
            }
        };
        
        let mut stats = CollectionStats::default();
        
        // If no gray objects, start a new collection cycle
        if gray_count == 0 {
            // Initialize the mark phase - reset all objects to white and mark roots as gray
            let roots = {
                let mut state = crate::memory::deadlock_detector::try_write_with_timeout(
                    &self.inner,
                    Some(5000), // 5 seconds timeout
                    Some("collect_garbage_incremental (reset objects)")
                ).unwrap_or_else(|| {
                    panic!("Failed to acquire write lock in incremental collection initialization");
                });
                
                // Get initial stats
                stats.initial_objects = state.objects.len();
                
                // Reset all objects to white
                for obj in state.objects.values_mut() {
                    obj.mark_state = WHITE;
                }
                
                // Clear the gray objects queue
                state.gray_objects.clear();
                
                // Copy roots to avoid borrowing issues
                state.roots.iter().cloned().collect::<Vec<usize>>()
            };
            
            // Mark all roots as gray
            for &root_addr in &roots {
                // Use the mark_object helper to mark as gray and add to queue
                if let Ok(mut state) = self.inner.write() {
                    if let Some(obj) = state.objects.get_mut(&root_addr) {
                        obj.mark_state = GRAY;
                        state.gray_objects.push_back(root_addr);
                    }
                }
            }
        }
        
        // Process a limited number of gray objects
        let mut processed_count = 0;
        while processed_count < step_size {
            // Check for timeout
            if start_time.elapsed() >= timeout {
                stats.mark_time_ms = start_time.elapsed().as_millis();
                return CollectionResult::Timeout {
                    stats,
                    phase: "incremental mark".to_string(),
                };
            }
            
            // Get the next gray object
            let current_addr = {
                let state_opt = crate::memory::deadlock_detector::try_write_with_timeout(
                    &self.inner,
                    Some(1000), // 1 second timeout
                    Some("collect_garbage_incremental (get next gray)")
                );
                
                if state_opt.is_none() {
                    continue;
                }
                
                let mut state = state_opt.unwrap();
                if state.gray_objects.is_empty() {
                    // If no more gray objects, we're done with this incremental step
                    // In a future step, we'll start sweeping
                    break;
                }
                
                state.gray_objects.pop_front()
            };
            
            // Process the gray object
            if let Some(addr) = current_addr {
                // Mark the object as black
                {
                    let state_opt = crate::memory::deadlock_detector::try_write_with_timeout(
                        &self.inner,
                        Some(1000), // 1 second timeout
                        Some("collect_garbage_incremental (mark black)")
                    );
                    
                    if state_opt.is_none() {
                        continue;
                    }
                    
                    let mut state = state_opt.unwrap();
                    
                    if let Some(obj) = state.objects.get_mut(&addr) {
                        if obj.mark_state == GRAY {
                            obj.mark_state = BLACK;
                            processed_count += 1;
                        }
                    } else {
                        continue;
                    }
                }
                
                // Get references and mark them
                let references = ReferenceCollector::collect_references(addr);
                for ref_addr in references {
                    // Mark reference as gray and add to queue
                    if let Ok(mut state) = self.inner.write() {
                        if let Some(obj) = state.objects.get_mut(&ref_addr) {
                            if obj.mark_state == WHITE {
                                obj.mark_state = GRAY;
                                state.gray_objects.push_back(ref_addr);
                            }
                        }
                    }
                }
            } else {
                break; // Queue is empty
            }
        }
        
        // Update stats
        {
            let state_opt = crate::memory::deadlock_detector::try_read_with_timeout(
                &self.inner,
                Some(1000), // 1 second timeout
                Some("collect_garbage_incremental (update stats)")
            );
            
            if let Some(state) = state_opt {
                stats.final_objects = state.objects.len();
            }
        }
        
        stats.mark_time_ms = start_time.elapsed().as_millis();
        stats.total_time_ms = stats.mark_time_ms;
        
        CollectionResult::Success(stats)
    }
}