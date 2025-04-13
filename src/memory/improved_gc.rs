//! Improved Garbage Collector Implementation
//!
//! This module contains improved implementations for the garbage collector,
//! including proper cycle detection, incremental collection, and object finalization.
//! These implementations are designed to replace the previous implementations in gc.rs.

use std::collections::VecDeque;
use std::time::{Duration, Instant};
use std::sync::Arc;

use crate::memory::{Gc, Tag, Traceable, Visitor};
use crate::memory::gc::{GarbageCollector, GcObject, GcStateInner, MarkState};
use crate::memory::mark_sweep::{WHITE, GRAY, BLACK, CollectionStats, CollectionResult};
use crate::memory::deadlock_detector;

/// Helper for collecting references during object traversal
pub(crate) struct ReferenceCollector<'a> {
    gc: &'a GarbageCollector,
    found_references: &'a mut Vec<usize>,
}

impl<'a> Visitor for ReferenceCollector<'a> {
    fn visit<T: Traceable + 'static>(&mut self, obj: std::ptr::NonNull<T>) {
        let addr = obj.as_ptr() as usize;
        self.found_references.push(addr);
    }
}

impl GarbageCollector {
    /// Process all references from a given object by using its Traceable implementation
    pub(crate) fn process_object_references(&self, addr: usize) -> Vec<usize> {
        // Read object information
        let state_opt = deadlock_detector::try_read_with_timeout(
            &self.inner,
            Duration::from_secs(1),
            "process_object_references (get object data)"
        );
        
        if state_opt.is_none() {
            return Vec::new();
        }
        
        let state = state_opt.unwrap();
        // Get the object being traced
        let obj = match state.objects.get(&addr) {
            Some(obj) => obj,
            None => {
                return Vec::new();
            }
        };
        
        // Create a visitor that will collect all references
        let mut found_refs = Vec::new();
        let mut visitor = ReferenceCollector {
            gc: self,
            found_references: &mut found_refs,
        };
        
        // Use the object's trace method to find all references
        if let Some(obj_data) = obj.data.as_ref() {
            let traceable_obj = unsafe { &*(obj_data.as_ptr() as *const dyn Traceable) };
            traceable_obj.trace(&mut visitor);
        }
        
        found_refs
    }
    
    /// Mark an object as gray and add it to the processing queue
    pub(crate) fn mark_object(&self, addr: usize) {
        let state_opt = deadlock_detector::try_write_with_timeout(
            &self.inner,
            Duration::from_secs(1),
            "mark_object"
        );
        
        if state_opt.is_none() {
            return;
        }
        
        let mut state = state_opt.unwrap();
        if let Some(obj) = state.objects.get_mut(&addr) {
            if obj.mark_state == WHITE {
                obj.mark_state = GRAY;
                state.gray_objects.push_back(addr);
            }
        }
    }
    
    /// Improved mark phase implementation that properly handles cycles
    pub(crate) fn improved_mark_phase(&self, start_time: Instant, timeout: Duration) -> Result<(), String> {
        let verbose = self.inner.read().unwrap().verbose;
        
        // Reset mark state and prepare gray objects queue
        let roots = {
            let mut state = deadlock_detector::try_write_with_timeout(
                &self.inner,
                Duration::from_secs(5),
                "improved_mark_phase (reset objects)"
            ).unwrap_or_else(|| {
                panic!("Failed to acquire write lock in mark phase initialization");
            });
            
            // Reset all objects to white
            for obj in state.objects.values_mut() {
                obj.mark_state = WHITE;
            }
            
            // Clear the gray objects queue
            state.gray_objects.clear();
            
            // Copy roots to avoid borrowing issues
            state.roots.iter().cloned().collect::<Vec<usize>>()
        };
        
        // Mark all roots
        for &root_addr in &roots {
            self.mark_object(root_addr);
        }
        
        // Process gray objects until queue is empty
        let mut processed_count = 0;
        loop {
            // Check for timeout
            if start_time.elapsed() >= timeout {
                if verbose {
                    println!("GC: Mark phase timed out after {:?}, processed {} objects", 
                           start_time.elapsed(), processed_count);
                }
                return Err("Mark phase timed out".to_string());
            }
            
            // Get the next gray object
            let current_addr = {
                let state_opt = deadlock_detector::try_write_with_timeout(
                    &self.inner,
                    Duration::from_secs(1),
                    "improved_mark_phase (get next gray)"
                );
                
                if state_opt.is_none() {
                    continue;
                }
                
                let mut state = state_opt.unwrap();
                if state.gray_objects.is_empty() {
                    break;
                }
                
                state.gray_objects.pop_front()
            };
            
            // Process the gray object
            if let Some(addr) = current_addr {
                // Mark the object as black
                {
                    let state_opt = deadlock_detector::try_write_with_timeout(
                        &self.inner,
                        Duration::from_secs(1),
                        "improved_mark_phase (mark black)"
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
                let references = self.process_object_references(addr);
                for ref_addr in references {
                    self.mark_object(ref_addr);
                }
            } else {
                break; // Queue is empty
            }
        }
        
        if verbose {
            println!("GC: Mark phase complete, processed {} objects", processed_count);
        }
        Ok(())
    }
    
    /// Improved sweep phase that includes object finalization
    pub(crate) fn improved_sweep_phase(&self, start_time: Instant, timeout: Duration) -> Result<CollectionStats, String> {
        let verbose = self.inner.read().unwrap().verbose;
        let mut stats = CollectionStats::default();
        
        // Get initial stats
        {
            let state_opt = deadlock_detector::try_read_with_timeout(
                &self.inner,
                Duration::from_secs(1),
                "improved_sweep_phase (initial stats)"
            );
            
            if let Some(state) = state_opt {
                stats.initial_objects = state.objects.len();
            }
        }
        
        // Get unreachable objects
        let objects_to_sweep = {
            let state_opt = deadlock_detector::try_read_with_timeout(
                &self.inner,
                Duration::from_secs(1),
                "improved_sweep_phase (find unreachable)"
            );
            
            if state_opt.is_none() {
                return Err("Failed to acquire lock to find unreachable objects".to_string());
            }
            
            let state = state_opt.unwrap();
            let mut unreachable = Vec::new();
            
            for (&addr, obj) in &state.objects {
                if obj.mark_state == WHITE {
                    unreachable.push(addr);
                }
            }
            
            unreachable
        };
        
        if verbose {
            println!("GC: Found {} unreachable objects to sweep", objects_to_sweep.len());
        }
        
        // Finalize and remove unreachable objects
        for addr in objects_to_sweep {
            // Check for timeout
            if start_time.elapsed() >= timeout {
                if verbose {
                    println!("GC: Sweep phase timed out after {:?}", start_time.elapsed());
                }
                return Err("Sweep phase timed out".to_string());
            }
            
            // Finalize the object if needed
            {
                let state_opt = deadlock_detector::try_read_with_timeout(
                    &self.inner,
                    Duration::from_secs(1),
                    "improved_sweep_phase (finalize)"
                );
                
                if state_opt.is_none() {
                    continue;
                }
                
                let state = state_opt.unwrap();
                if let Some(obj) = state.objects.get(&addr) {
                    if let Some(obj_data) = obj.data.as_ref() {
                        // Finalize the object
                        let traceable_obj = unsafe { &mut *(obj_data.as_ptr() as *mut dyn Traceable) };
                        traceable_obj.finalize();
                    }
                }
            }
            
            // Remove the object
            {
                let state_opt = deadlock_detector::try_write_with_timeout(
                    &self.inner,
                    Duration::from_secs(1),
                    "improved_sweep_phase (remove)"
                );
                
                if state_opt.is_none() {
                    continue;
                }
                
                let mut state = state_opt.unwrap();
                if let Some(obj) = state.objects.remove(&addr) {
                    stats.bytes_freed += obj.size;
                    stats.objects_freed += 1;
                }
            }
        }
        
        // Get final stats
        {
            let state_opt = deadlock_detector::try_read_with_timeout(
                &self.inner,
                Duration::from_secs(1),
                "improved_sweep_phase (final stats)"
            );
            
            if let Some(state) = state_opt {
                stats.final_objects = state.objects.len();
            }
        }
        
        if verbose {
            println!("GC: Sweep phase complete, freed {} objects ({} bytes)", 
                   stats.objects_freed, stats.bytes_freed);
        }
        
        Ok(stats)
    }
    
    /// Incremental collection step - processes a limited number of objects
    pub fn collect_garbage_incremental(&self) -> CollectionResult {
        let start_time = Instant::now();
        let (timeout, step_size) = {
            let state = self.inner.read().unwrap();
            (Duration::from_millis(state.options.incremental_time_budget_ms), 
             state.options.incremental_step_size)
        };
        
        // Get the current gray objects queue
        let gray_count = {
            let state_opt = deadlock_detector::try_read_with_timeout(
                &self.inner,
                Duration::from_secs(1),
                "collect_garbage_incremental (check gray queue)"
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
                let mut state = deadlock_detector::try_write_with_timeout(
                    &self.inner,
                    Duration::from_secs(5),
                    "collect_garbage_incremental (reset objects)"
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
                self.mark_object(root_addr);
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
                let state_opt = deadlock_detector::try_write_with_timeout(
                    &self.inner,
                    Duration::from_secs(1),
                    "collect_garbage_incremental (get next gray)"
                );
                
                if state_opt.is_none() {
                    continue;
                }
                
                let mut state = state_opt.unwrap();
                if state.gray_objects.is_empty() {
                    // If no more gray objects, we're done with this incremental step
                    // In a future step, we'll start sweeping
                    state.collection_state = 1; // Mark that we're ready to sweep
                    break;
                }
                
                state.gray_objects.pop_front()
            };
            
            // Process the gray object
            if let Some(addr) = current_addr {
                // Mark the object as black
                {
                    let state_opt = deadlock_detector::try_write_with_timeout(
                        &self.inner,
                        Duration::from_secs(1),
                        "collect_garbage_incremental (mark black)"
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
                let references = self.process_object_references(addr);
                for ref_addr in references {
                    self.mark_object(ref_addr);
                }
            } else {
                break; // Queue is empty
            }
        }
        
        // Update stats
        {
            let state_opt = deadlock_detector::try_read_with_timeout(
                &self.inner,
                Duration::from_secs(1),
                "collect_garbage_incremental (update stats)"
            );
            
            if let Some(state) = state_opt {
                stats.final_objects = state.objects.len();
            }
        }
        
        stats.mark_time_ms = start_time.elapsed().as_millis();
        stats.total_time_ms = stats.mark_time_ms;
        
        CollectionResult::Success(stats)
    }
    
    /// Improved mark-and-sweep implementation with cycle detection
    pub fn improved_mark_and_sweep(&self, timeout: Duration) -> CollectionResult {
        let start_time = Instant::now();
        let verbose = self.inner.read().unwrap().verbose;
        
        // Initialize statistics
        let mut stats = CollectionStats::default();
        
        // 1. Mark Phase - identify all reachable objects
        if let Err(e) = self.improved_mark_phase(start_time, timeout) {
            return CollectionResult::Timeout {
                stats,
                phase: "mark".to_string(),
            };
        }
        
        stats.mark_time_ms = start_time.elapsed().as_millis();
        if verbose {
            println!("GC: Mark phase completed in {}ms", stats.mark_time_ms);
        }
        
        let sweep_start = Instant::now();
        
        // 2. Sweep Phase - free unreachable objects
        match self.improved_sweep_phase(start_time, timeout) {
            Ok(sweep_stats) => {
                stats.final_objects = sweep_stats.final_objects;
                stats.objects_freed = sweep_stats.objects_freed;
                stats.bytes_freed = sweep_stats.bytes_freed;
            },
            Err(_) => {
                return CollectionResult::Timeout {
                    stats,
                    phase: "sweep".to_string(),
                };
            }
        }
        
        stats.sweep_time_ms = sweep_start.elapsed().as_millis();
        stats.total_time_ms = start_time.elapsed().as_millis();
        
        if verbose {
            println!("GC: Sweep phase completed in {}ms", stats.sweep_time_ms);
            println!("GC: Total collection time: {}ms", stats.total_time_ms);
            println!("GC: Objects before: {}, after: {}, freed: {}", 
                     stats.initial_objects, stats.final_objects, stats.objects_freed);
        }
        
        CollectionResult::Success(stats)
    }
}