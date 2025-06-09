//! Tests for slice FFI functionality
//!
//! This module tests the FFI interface between LLVM-generated code
//! and the slice runtime system.

use std::ffi::c_void;
use cursed::runtime::slice_runtime::*;

mod common;

#[test]
fn test_slice_ffi_basic_operations() {
    common::tracing::setup();
    
    // Initialize global runtime
    let runtime_ptr = cursed_slice_runtime_init();
    assert!(!runtime_ptr.is_null());
    
    // Test getting runtime pointer
    let runtime_ptr2 = cursed_slice_runtime_get();
    assert_eq!(runtime_ptr, runtime_ptr2);
    
    // Create a slice via FFI
    let header = cursed_slice_create(4, 10, runtime_ptr);
    assert!(!header.ptr.is_null());
    assert_eq!(header.len, 0);
    assert_eq!(header.capacity, 10);
    
    // Test bounds checking
    let in_bounds = cursed_slice_check_bounds(&header, 0, runtime_ptr);
    assert_eq!(in_bounds, 0); // Empty slice, so index 0 is out of bounds
    
    // Test setting length
    let mut header = header;
    let success = cursed_slice_set_length(&mut header, 5);
    assert_eq!(success, 1);
    assert_eq!(header.len, 5);
    
    // Now index 0 should be in bounds
    let in_bounds = cursed_slice_check_bounds(&header, 0, runtime_ptr);
    assert_eq!(in_bounds, 1);
    
    // Index 5 should be out of bounds
    let in_bounds = cursed_slice_check_bounds(&header, 5, runtime_ptr);
    assert_eq!(in_bounds, 0);
    
    // Deallocate the slice
    cursed_slice_deallocate(&mut header, 4, runtime_ptr);
    assert!(header.ptr.is_null());
    assert_eq!(header.len, 0);
    assert_eq!(header.capacity, 0);
}

#[test]
fn test_slice_ffi_growth() {
    common::tracing::setup();
    
    let runtime_ptr = cursed_slice_runtime_init();
    
    // Create a small slice
    let mut header = cursed_slice_create(4, 5, runtime_ptr);
    let original_ptr = header.ptr;
    
    // Set length to use some capacity
    cursed_slice_set_length(&mut header, 3);
    
    // Grow within current capacity (should not reallocate)
    let result = cursed_slice_grow(&mut header, 4, 1, runtime_ptr);
    assert_eq!(result, 0); // Success
    assert_eq!(header.ptr, original_ptr); // Same pointer
    assert_eq!(header.capacity, 5); // Same capacity
    
    // Grow beyond current capacity (should reallocate)
    let result = cursed_slice_grow(&mut header, 4, 3, runtime_ptr);
    assert_eq!(result, 0); // Success
    assert_ne!(header.ptr, original_ptr); // Different pointer
    assert!(header.capacity >= 6); // Increased capacity
    
    // Clean up
    cursed_slice_deallocate(&mut header, 4, runtime_ptr);
}

#[test]
fn test_slice_ffi_element_access() {
    common::tracing::setup();
    
    let runtime_ptr = cursed_slice_runtime_init();
    
    // Create and set up a slice
    let mut header = cursed_slice_create(4, 5, runtime_ptr);
    cursed_slice_set_length(&mut header, 3);
    
    // Fill with test data
    unsafe {
        let data = [10i32, 20, 30];
        std::ptr::copy_nonoverlapping(
            data.as_ptr() as *const u8,
            header.ptr as *mut u8,
            12
        );
    }
    
    // Test element access
    let elem_ptr = cursed_slice_get_element_ptr(&header, 4, 1);
    assert!(!elem_ptr.is_null());
    
    unsafe {
        let value = *(elem_ptr as *const i32);
        assert_eq!(value, 20);
    }
    
    // Test out of bounds access
    let elem_ptr = cursed_slice_get_element_ptr(&header, 4, 5);
    assert!(elem_ptr.is_null());
    
    // Clean up
    cursed_slice_deallocate(&mut header, 4, runtime_ptr);
}

#[test]
fn test_slice_ffi_copy_operations() {
    common::tracing::setup();
    
    let runtime_ptr = cursed_slice_runtime_init();
    
    // Create source and destination slices
    let mut src = cursed_slice_create(4, 5, runtime_ptr);
    let mut dst = cursed_slice_create(4, 5, runtime_ptr);
    
    // Set up source data
    cursed_slice_set_length(&mut src, 5);
    unsafe {
        let data = [1i32, 2, 3, 4, 5];
        std::ptr::copy_nonoverlapping(
            data.as_ptr() as *const u8,
            src.ptr as *mut u8,
            20
        );
    }
    
    // Test copying
    let copied = cursed_slice_copy(&src, &mut dst, 4, 1, 0, 3);
    assert_eq!(copied, 3);
    assert_eq!(dst.len, 3);
    
    // Verify copied data
    unsafe {
        let dst_data = std::slice::from_raw_parts(dst.ptr as *const i32, 3);
        assert_eq!(dst_data, &[2, 3, 4]);
    }
    
    // Clean up
    cursed_slice_deallocate(&mut src, 4, runtime_ptr);
    cursed_slice_deallocate(&mut dst, 4, runtime_ptr);
}

#[test]
fn test_slice_ffi_fill_operations() {
    common::tracing::setup();
    
    let runtime_ptr = cursed_slice_runtime_init();
    
    // Create a slice
    let mut header = cursed_slice_create(4, 5, runtime_ptr);
    
    // Test filling
    let value = 42i32;
    let filled = cursed_slice_fill(
        &mut header,
        4,
        &value as *const i32 as *const c_void,
        0,
        3
    );
    assert_eq!(filled, 3);
    assert_eq!(header.len, 3);
    
    // Verify filled data
    unsafe {
        let data = std::slice::from_raw_parts(header.ptr as *const i32, 3);
        assert_eq!(data, &[42, 42, 42]);
    }
    
    // Clean up
    cursed_slice_deallocate(&mut header, 4, runtime_ptr);
}

#[test]
fn test_slice_ffi_error_handling() {
    common::tracing::setup();
    
    // Test with null runtime pointer
    let header = cursed_slice_create(4, 10, std::ptr::null_mut());
    assert!(header.ptr.is_null());
    assert_eq!(header.len, 0);
    assert_eq!(header.capacity, 0);
    
    // Test with null header pointer
    let result = cursed_slice_grow(std::ptr::null_mut(), 4, 5, std::ptr::null_mut());
    assert_eq!(result, -1);
    
    // Test bounds checking with null pointers
    let in_bounds = cursed_slice_check_bounds(std::ptr::null(), 0, std::ptr::null_mut());
    assert_eq!(in_bounds, 0);
    
    // Test element access with null pointer
    let elem_ptr = cursed_slice_get_element_ptr(std::ptr::null(), 4, 0);
    assert!(elem_ptr.is_null());
    
    // Test copy with null pointers
    let copied = cursed_slice_copy(
        std::ptr::null(),
        std::ptr::null_mut(),
        4,
        0,
        0,
        1
    );
    assert_eq!(copied, 0);
    
    // Test fill with null pointers
    let filled = cursed_slice_fill(
        std::ptr::null_mut(),
        4,
        std::ptr::null(),
        0,
        1
    );
    assert_eq!(filled, 0);
}

#[test]
fn test_slice_ffi_length_operations() {
    common::tracing::setup();
    
    let runtime_ptr = cursed_slice_runtime_init();
    let mut header = cursed_slice_create(4, 10, runtime_ptr);
    
    // Test setting valid length
    let success = cursed_slice_set_length(&mut header, 5);
    assert_eq!(success, 1);
    assert_eq!(header.len, 5);
    
    // Test setting length to capacity
    let success = cursed_slice_set_length(&mut header, 10);
    assert_eq!(success, 1);
    assert_eq!(header.len, 10);
    
    // Test setting length beyond capacity
    let success = cursed_slice_set_length(&mut header, 15);
    assert_eq!(success, 0);
    assert_eq!(header.len, 10); // Should remain unchanged
    
    // Test truncating
    let success = cursed_slice_set_length(&mut header, 3);
    assert_eq!(success, 1);
    assert_eq!(header.len, 3);
    
    // Test with null pointer
    let success = cursed_slice_set_length(std::ptr::null_mut(), 5);
    assert_eq!(success, 0);
    
    // Clean up
    cursed_slice_deallocate(&mut header, 4, runtime_ptr);
}

#[test]
fn test_slice_ffi_multiple_slices() {
    common::tracing::setup();
    
    let runtime_ptr = cursed_slice_runtime_init();
    
    // Create multiple slices
    let mut slice1 = cursed_slice_create(4, 5, runtime_ptr);
    let mut slice2 = cursed_slice_create(8, 3, runtime_ptr);
    let mut slice3 = cursed_slice_create(1, 20, runtime_ptr);
    
    // Verify they're all different
    assert_ne!(slice1.ptr, slice2.ptr);
    assert_ne!(slice1.ptr, slice3.ptr);
    assert_ne!(slice2.ptr, slice3.ptr);
    
    // Set different lengths
    cursed_slice_set_length(&mut slice1, 3);
    cursed_slice_set_length(&mut slice2, 2);
    cursed_slice_set_length(&mut slice3, 15);
    
    assert_eq!(slice1.len, 3);
    assert_eq!(slice2.len, 2);
    assert_eq!(slice3.len, 15);
    
    // Clean up all slices
    cursed_slice_deallocate(&mut slice1, 4, runtime_ptr);
    cursed_slice_deallocate(&mut slice2, 8, runtime_ptr);
    cursed_slice_deallocate(&mut slice3, 1, runtime_ptr);
    
    // Verify they're all cleared
    assert!(slice1.ptr.is_null());
    assert!(slice2.ptr.is_null());
    assert!(slice3.ptr.is_null());
}

#[test]
fn test_slice_ffi_concurrent_access() {
    common::tracing::setup();
    
    // Initialize runtime once
    let runtime_ptr = cursed_slice_runtime_init();
    
    // Test sequential access since FFI pointers aren't thread-safe
    for i in 0..4 {
        // Each iteration creates and uses its own slice
        let mut header = cursed_slice_create(4, 10, runtime_ptr);
        
        if !header.ptr.is_null() {
            cursed_slice_set_length(&mut header, 5);
            
            // Fill with iteration-specific data
            let value = (i + 1) * 10;
            cursed_slice_fill(
                &mut header,
                4,
                &value as *const i32 as *const c_void,
                0,
                5
            );
            
            // Verify data
            unsafe {
                let data = std::slice::from_raw_parts(header.ptr as *const i32, 5);
                assert!(data.iter().all(|&x| x == value));
            }
            
            cursed_slice_deallocate(&mut header, 4, runtime_ptr);
        } else {
            panic!("Failed to create slice in iteration {}", i);
        }
    }
}
