#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_allocation_tracking() {
        // Test basic allocation and deallocation
        let ptr1 = rust_heap_allocate(1024, 1);
        assert!(!ptr1.is_null());
        
        // Verify pointer is tracked
        let ptr_addr = ptr1 as usize;
        let is_tracked = if let Ok(tracker) = ALLOCATION_TRACKER.lock() {
            tracker.contains(&ptr_addr)
        } else {
            false
        };
        assert!(is_tracked, "Pointer should be tracked after allocation");
        
        // Test deallocation
        rust_heap_deallocate(ptr1);
        
        // Verify pointer is no longer tracked
        let is_tracked_after = if let Ok(tracker) = ALLOCATION_TRACKER.lock() {
            tracker.contains(&ptr_addr)
        } else {
            true
        };
        assert!(!is_tracked_after, "Pointer should not be tracked after deallocation");
    }

    #[test]
    fn test_double_free_prevention() {
        // Test that double-free is prevented
        let ptr = rust_heap_allocate(512, 2);
        assert!(!ptr.is_null());
        
        // First free should work
        rust_heap_deallocate(ptr);
        
        // Second free should be safely ignored (no crash)
        rust_heap_deallocate(ptr);
        
        // This test passes if we don't crash
    }

    #[test]
    fn test_invalid_free_prevention() {
        // Test that freeing untracked pointers is prevented
        let fake_ptr = 0x1234 as *mut std::ffi::c_void;
        
        // This should not crash and should be safely ignored
        rust_heap_deallocate(fake_ptr);
        
        // Test passes if we don't crash
    }

    #[test]
    fn test_realloc_tracking() {
        // Test reallocation tracking
        let ptr1 = rust_heap_allocate(1024, 1);
        assert!(!ptr1.is_null());
        
        let ptr2 = rust_heap_reallocate(ptr1, 2048);
        assert!(!ptr2.is_null());
        
        // Original pointer should no longer be tracked
        let ptr1_addr = ptr1 as usize;
        let is_ptr1_tracked = if let Ok(tracker) = ALLOCATION_TRACKER.lock() {
            tracker.contains(&ptr1_addr)
        } else {
            true
        };
        assert!(!is_ptr1_tracked, "Original pointer should not be tracked after realloc");
        
        // New pointer should be tracked
        let ptr2_addr = ptr2 as usize;
        let is_ptr2_tracked = if let Ok(tracker) = ALLOCATION_TRACKER.lock() {
            tracker.contains(&ptr2_addr)
        } else {
            false
        };
        assert!(is_ptr2_tracked, "New pointer should be tracked after realloc");
        
        rust_heap_deallocate(ptr2);
    }

    #[test]
    fn test_null_pointer_safety() {
        // Test that null pointer operations are safe
        rust_heap_deallocate(ptr::null_mut());
        
        let result = rust_heap_reallocate(ptr::null_mut(), 1024);
        assert!(!result.is_null());
        
        rust_heap_deallocate(result);
    }
}
