//! Tests for slice runtime functionality
//!
//! This module provides comprehensive tests for the slice runtime system,
//! including creation, growth, memory management, and utility functions.

use std::ffi::c_void;
use cursed::runtime::slice_runtime::{SliceRuntime, SliceHeader, SliceConfiguration, SliceStatistics};
use cursed::runtime::slice_utils::*;

#[path = "common.rs"]
mod common;

#[test]
fn test_slice_header_basic_operations() {
    common::tracing::setup();
    
    // Test empty header
    let header = SliceHeader::new();
    assert!(header.ptr.is_null());
    assert_eq!(header.len, 0);
    assert_eq!(header.capacity, 0);
    assert!(header.is_empty());
    assert!(header.is_valid());
    assert_eq!(header.remaining_capacity(), 0);
    
    // Test header with parameters
    let test_ptr = Box::into_raw(Box::new(42i32)) as *mut c_void;
    let header = SliceHeader::with_params(test_ptr, 5, 10);
    assert_eq!(header.ptr, test_ptr);
    assert_eq!(header.len, 5);
    assert_eq!(header.capacity, 10);
    assert!(!header.is_empty());
    assert!(header.is_valid());
    assert_eq!(header.remaining_capacity(), 5);
    
    // Cleanup
    unsafe { drop(Box::from_raw(test_ptr as *mut i32)); }
}

#[test]
fn test_slice_runtime_creation() {
    common::tracing::setup();
    
    let runtime = SliceRuntime::new();
    assert_eq!(runtime.slice_count(), 0);
    
    let stats = runtime.get_statistics();
    assert_eq!(stats.slices_created, 0);
    assert_eq!(stats.allocations, 0);
    assert_eq!(stats.bytes_allocated, 0);
}

#[test]
fn test_slice_runtime_with_custom_config() {
    common::tracing::setup();
    
    let config = SliceConfiguration {
        default_capacity: 16,
        growth_factor: 1.5,
        max_capacity: 1024,
        bounds_checking: true,
        zero_memory: false,
        auto_shrink: true,
        shrink_threshold: 0.5,
    };
    
    let runtime = SliceRuntime::with_config(config.clone());
    
    // Create a slice and verify it uses the custom config
    let header = runtime.create_slice(4, None).unwrap();
    assert_eq!(header.capacity, 16); // Uses default_capacity from config
    assert!(header.is_valid());
}

#[test]
fn test_slice_creation_and_deallocation() {
    common::tracing::setup();
    
    let runtime = SliceRuntime::new();
    
    // Create a slice
    let result = runtime.create_slice(4, Some(10));
    assert!(result.is_ok());
    
    let mut header = result.unwrap();
    assert!(!header.ptr.is_null());
    assert_eq!(header.len, 0);
    assert_eq!(header.capacity, 10);
    assert!(header.is_valid());
    
    // Check statistics
    let stats = runtime.get_statistics();
    assert_eq!(stats.slices_created, 1);
    assert_eq!(stats.allocations, 1);
    assert_eq!(stats.bytes_allocated, 40); // 4 bytes * 10 capacity
    
    // Deallocate the slice
    runtime.deallocate_slice(&mut header, 4);
    assert!(header.ptr.is_null());
    assert_eq!(header.len, 0);
    assert_eq!(header.capacity, 0);
    
    // Check final statistics
    let final_stats = runtime.get_statistics();
    assert_eq!(final_stats.deallocations, 1);
    assert_eq!(final_stats.bytes_deallocated, 40);
}

#[test]
fn test_slice_growth() {
    common::tracing::setup();
    
    let runtime = SliceRuntime::new();
    let mut header = runtime.create_slice(4, Some(5)).unwrap();
    
    // Growth within current capacity should not reallocate
    let old_ptr = header.ptr;
    let result = runtime.grow_slice(&mut header, 4, 3);
    assert!(result.is_ok());
    assert_eq!(header.ptr, old_ptr); // Same pointer
    assert_eq!(header.capacity, 5);
    
    // Growth beyond capacity should reallocate
    let result = runtime.grow_slice(&mut header, 4, 10); // Force reallocation with large growth
    assert!(result.is_ok());
    // Pointer might be the same if system allocator reuses the same location
    // Just verify capacity increased beyond original
    println!("Original capacity: 5, new capacity: {}", header.capacity);
    assert!(header.capacity > 5); // Should be larger than original
    
    let stats = runtime.get_statistics();
    assert_eq!(stats.growths, 1); // One actual growth operation
}

#[test]
fn test_slice_shrinking() {
    common::tracing::setup();
    
    let config = SliceConfiguration {
        auto_shrink: true,
        shrink_threshold: 0.25,
        ..Default::default()
    };
    
    let runtime = SliceRuntime::with_config(config);
    let mut header = runtime.create_slice(4, Some(20)).unwrap();
    header.len = 3; // Simulate having only 3 elements
    
    // Should trigger shrinking since len (3) < capacity (20) * threshold (0.25) = 5
    let result = runtime.shrink_slice(&mut header, 4);
    assert!(result.is_ok());
    
    // Capacity should be reduced but not below minimum
    assert!(header.capacity < 20);
    assert!(header.capacity >= 8); // Default minimum capacity
}

#[test]
fn test_slice_bounds_checking() {
    common::tracing::setup();
    
    let runtime = SliceRuntime::new();
    let header = SliceRuntime::new().create_slice(4, Some(5)).unwrap();
    
    // Invalid indices (slice is empty, so no valid indices)
    assert!(!runtime.check_bounds(&header, 0));
    assert!(!runtime.check_bounds(&header, 5));
    
    // Test with non-empty slice
    let mut header = header;
    header.len = 3;
    assert!(runtime.check_bounds(&header, 0));
    assert!(runtime.check_bounds(&header, 2));
    assert!(!runtime.check_bounds(&header, 3));
    assert!(!runtime.check_bounds(&header, 10));
}

#[test]
fn test_slice_copy_operations() {
    common::tracing::setup();
    
    let runtime = SliceRuntime::new();
    
    // Create source slice with test data
    let mut src = runtime.create_slice(4, Some(5)).unwrap();
    let mut dst = runtime.create_slice(4, Some(5)).unwrap();
    
    unsafe {
        // Fill source with test data
        let src_data = [1i32, 2, 3, 4, 5];
        std::ptr::copy_nonoverlapping(
            src_data.as_ptr() as *const u8,
            src.ptr as *mut u8,
            20
        );
        src.len = 5;
        
        // Test copying a subset
        let copied = slice_copy(&src, &mut dst, 4, 1, 0, 3);
        assert_eq!(copied, 3);
        assert_eq!(dst.len, 3);
        
        // Verify copied data
        let dst_data = std::slice::from_raw_parts(dst.ptr as *const i32, 3);
        assert_eq!(dst_data, &[2, 3, 4]);
        
        // Test copying beyond bounds
        let copied = slice_copy(&src, &mut dst, 4, 3, 0, 10);
        assert_eq!(copied, 2); // Only 2 elements available from index 3
    }
}

#[test]
fn test_slice_fill_operations() {
    common::tracing::setup();
    
    let runtime = SliceRuntime::new();
    let mut slice = runtime.create_slice(4, Some(5)).unwrap();
    
    unsafe {
        let value = 42i32;
        let filled = slice_fill(&mut slice, 4, &value as *const i32 as *const c_void, 0, 3);
        
        assert_eq!(filled, 3);
        assert_eq!(slice.len, 3);
        
        // Verify filled data
        let data = std::slice::from_raw_parts(slice.ptr as *const i32, 3);
        assert_eq!(data, &[42, 42, 42]);
        
        // Test filling with different value
        let new_value = 99i32;
        let filled = slice_fill(&mut slice, 4, &new_value as *const i32 as *const c_void, 1, 2);
        assert_eq!(filled, 2);
        
        let data = std::slice::from_raw_parts(slice.ptr as *const i32, 3);
        assert_eq!(data, &[42, 99, 99]);
    }
}

#[test]
fn test_slice_zero_operations() {
    common::tracing::setup();
    
    let runtime = SliceRuntime::new();
    let mut slice = runtime.create_slice(4, Some(5)).unwrap();
    
    unsafe {
        // Fill with non-zero data first
        let data = [1i32, 2, 3, 4, 5];
        std::ptr::copy_nonoverlapping(
            data.as_ptr() as *const u8,
            slice.ptr as *mut u8,
            20
        );
        slice.len = 5;
        
        // Zero out middle elements
        let zeroed = slice_zero(&mut slice, 4, 1, 3);
        assert_eq!(zeroed, 3);
        
        // Verify zeroed data
        let result_data = std::slice::from_raw_parts(slice.ptr as *const i32, 5);
        assert_eq!(result_data, &[1, 0, 0, 0, 5]);
    }
}

#[test]
fn test_slice_comparison_operations() {
    common::tracing::setup();
    
    let runtime = SliceRuntime::new();
    
    unsafe {
        // Create two identical slices
        let mut slice1 = runtime.create_slice(4, Some(3)).unwrap();
        let mut slice2 = runtime.create_slice(4, Some(3)).unwrap();
        
        let data = [1i32, 2, 3];
        
        // Fill both slices with same data
        std::ptr::copy_nonoverlapping(
            data.as_ptr() as *const u8,
            slice1.ptr as *mut u8,
            12
        );
        slice1.len = 3;
        
        std::ptr::copy_nonoverlapping(
            data.as_ptr() as *const u8,
            slice2.ptr as *mut u8,
            12
        );
        slice2.len = 3;
        
        // Test equality
        assert!(slice_equals(&slice1, &slice2, 4));
        
        // Test comparison
        assert_eq!(slice_compare(&slice1, &slice2, 4), std::cmp::Ordering::Equal);
        
        // Modify one slice
        let new_data = [1i32, 2, 4]; // Changed last element
        std::ptr::copy_nonoverlapping(
            new_data.as_ptr() as *const u8,
            slice2.ptr as *mut u8,
            12
        );
        
        // Test inequality
        assert!(!slice_equals(&slice1, &slice2, 4));
        assert_eq!(slice_compare(&slice1, &slice2, 4), std::cmp::Ordering::Less);
    }
}

#[test]
fn test_slice_find_operations() {
    common::tracing::setup();
    
    let runtime = SliceRuntime::new();
    let mut slice = runtime.create_slice(4, Some(5)).unwrap();
    
    unsafe {
        let data = [10i32, 20, 30, 20, 40];
        std::ptr::copy_nonoverlapping(
            data.as_ptr() as *const u8,
            slice.ptr as *mut u8,
            20
        );
        slice.len = 5;
        
        // Test finding existing value
        let search_value = 20i32;
        let index = slice_find(&slice, 4, &search_value as *const i32 as *const c_void);
        assert_eq!(index, 1); // First occurrence at index 1
        
        // Test contains
        assert!(slice_contains(&slice, 4, &search_value as *const i32 as *const c_void));
        
        // Test finding non-existent value
        let not_found = 99i32;
        let index = slice_find(&slice, 4, &not_found as *const i32 as *const c_void);
        assert_eq!(index, usize::MAX);
        assert!(!slice_contains(&slice, 4, &not_found as *const i32 as *const c_void));
    }
}

#[test]
fn test_slice_element_access() {
    common::tracing::setup();
    
    let runtime = SliceRuntime::new();
    let mut slice = runtime.create_slice(4, Some(3)).unwrap();
    
    unsafe {
        let data = [10i32, 20, 30];
        std::ptr::copy_nonoverlapping(
            data.as_ptr() as *const u8,
            slice.ptr as *mut u8,
            12
        );
        slice.len = 3;
        
        // Test valid element access
        let elem_ptr = slice_get_element_ptr(&slice, 4, 1);
        assert!(!elem_ptr.is_null());
        let value = *(elem_ptr as *const i32);
        assert_eq!(value, 20);
        
        // Test out of bounds access
        let elem_ptr = slice_get_element_ptr(&slice, 4, 5);
        assert!(elem_ptr.is_null());
    }
}

#[test]
fn test_slice_length_operations() {
    common::tracing::setup();
    
    let runtime = SliceRuntime::new();
    let mut slice = runtime.create_slice(4, Some(10)).unwrap();
    
    // Test setting valid length
    assert!(slice_set_length(&mut slice, 5));
    assert_eq!(slice.len, 5);
    
    // Test setting length to capacity
    assert!(slice_set_length(&mut slice, 10));
    assert_eq!(slice.len, 10);
    
    // Test setting length beyond capacity
    assert!(!slice_set_length(&mut slice, 15));
    assert_eq!(slice.len, 10); // Should remain unchanged
    
    // Test truncating
    assert!(slice_set_length(&mut slice, 3));
    assert_eq!(slice.len, 3);
}

#[test]
fn test_slice_move_within() {
    common::tracing::setup();
    
    let runtime = SliceRuntime::new();
    let mut slice = runtime.create_slice(4, Some(5)).unwrap();
    
    unsafe {
        let data = [1i32, 2, 3, 4, 5];
        std::ptr::copy_nonoverlapping(
            data.as_ptr() as *const u8,
            slice.ptr as *mut u8,
            20
        );
        slice.len = 5;
        
        // Move elements within the slice
        let moved = slice_move_within(&mut slice, 4, 1, 3, 2);
        assert_eq!(moved, 2);
        
        // Verify the move (elements 2,3 moved to positions 3,4)
        let result_data = std::slice::from_raw_parts(slice.ptr as *const i32, 5);
        assert_eq!(result_data, &[1, 2, 3, 2, 3]);
    }
}

#[test]
fn test_slice_statistics() {
    common::tracing::setup();
    
    let runtime = SliceRuntime::new();
    
    // Create some slices
    let _slice1 = runtime.create_slice(4, Some(10)).unwrap();
    let _slice2 = runtime.create_slice(8, Some(5)).unwrap();
    
    let stats = runtime.get_statistics();
    assert_eq!(stats.slices_created, 2);
    assert_eq!(stats.allocations, 2);
    assert_eq!(stats.bytes_allocated, 80); // (4*10) + (8*5) = 80
    
    // Reset statistics
    runtime.reset_statistics();
    let reset_stats = runtime.get_statistics();
    assert_eq!(reset_stats.slices_created, 0);
    assert_eq!(reset_stats.allocations, 0);
    assert_eq!(reset_stats.bytes_allocated, 0);
}

#[test]
fn test_slice_error_conditions() {
    common::tracing::setup();
    
    let config = SliceConfiguration {
        max_capacity: 100,
        ..Default::default()
    };
    
    let runtime = SliceRuntime::with_config(config);
    
    // Test capacity limit
    let result = runtime.create_slice(4, Some(200));
    assert!(result.is_err());
    
    // Test overflow protection
    let result = runtime.create_slice(usize::MAX, Some(2));
    assert!(result.is_err());
}

#[test]
fn test_thread_safe_slice_runtime() {
    common::tracing::setup();
    
    use cursed::runtime::slice_runtime::{create_thread_safe_runtime, create_thread_safe_runtime_with_config};
    
    // Test creating thread-safe runtime
    let runtime = create_thread_safe_runtime();
    {
        let rt = runtime.read().unwrap();
        let slice = rt.create_slice(8, Some(3));
        assert!(slice.is_ok());
    }
    
    // Test with custom config
    let config = SliceConfiguration::default();
    let _runtime_with_config = create_thread_safe_runtime_with_config(config);
}
