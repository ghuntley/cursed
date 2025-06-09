//! Utility functions for slice operations in the CURSED language
//!
//! This module provides helper functions for common slice operations including
//! copying, comparison, bounds checking, and element access. These utilities
//! are designed to be called from LLVM-generated code.

use std::ffi::c_void;
use std::ptr;
use std::cmp::Ordering;
use tracing::{debug, instrument};
use crate::runtime::slice_runtime::SliceHeader;

/// Copy elements from one slice to another
///
/// # Safety
///
/// The caller must ensure that:
/// - Both source and destination pointers are valid
/// - The element size is correct
/// - The copy operation does not exceed slice bounds
///
/// # Arguments
///
/// * `src` - Source slice header
/// * `dst` - Destination slice header
/// * `element_size` - Size of each element in bytes
/// * `src_start` - Starting index in source slice
/// * `dst_start` - Starting index in destination slice
/// * `count` - Number of elements to copy
///
/// # Returns
///
/// Number of elements actually copied
#[instrument]
pub unsafe fn slice_copy(
    src: &SliceHeader,
    dst: &mut SliceHeader,
    element_size: usize,
    src_start: usize,
    dst_start: usize,
    count: usize,
) -> usize {
    // Check for null pointers
    if src.ptr.is_null() || dst.ptr.is_null() {
        debug!("Null pointer in slice_copy");
        return 0;
    }

    // Calculate actual copy count based on bounds
    let src_available = src.len.saturating_sub(src_start);
    let dst_available = dst.capacity.saturating_sub(dst_start);
    let actual_count = count.min(src_available).min(dst_available);

    if actual_count == 0 {
        debug!("No elements to copy");
        return 0;
    }

    // Calculate byte offsets
    let src_offset = src_start * element_size;
    let dst_offset = dst_start * element_size;
    let copy_size = actual_count * element_size;

    // Perform the copy
    let src_ptr = (src.ptr as *const u8).add(src_offset);
    let dst_ptr = (dst.ptr as *mut u8).add(dst_offset);

    ptr::copy_nonoverlapping(src_ptr, dst_ptr, copy_size);

    // Update destination length if we're appending
    if dst_start + actual_count > dst.len {
        dst.len = dst_start + actual_count;
    }

    debug!(
        copied = actual_count,
        src_start = src_start,
        dst_start = dst_start,
        element_size = element_size,
        "Slice copy completed"
    );

    actual_count
}

/// Copy elements within the same slice (handles overlapping regions)
///
/// # Safety
///
/// The caller must ensure that:
/// - The slice pointer is valid
/// - The element size is correct
/// - The operation does not exceed slice bounds
///
/// # Arguments
///
/// * `slice` - Slice header
/// * `element_size` - Size of each element in bytes
/// * `src_start` - Starting index for source
/// * `dst_start` - Starting index for destination
/// * `count` - Number of elements to move
///
/// # Returns
///
/// Number of elements actually moved
#[instrument]
pub unsafe fn slice_move_within(
    slice: &mut SliceHeader,
    element_size: usize,
    src_start: usize,
    dst_start: usize,
    count: usize,
) -> usize {
    if slice.ptr.is_null() {
        debug!("Null pointer in slice_move_within");
        return 0;
    }

    // Calculate actual move count
    let available = slice.len.saturating_sub(src_start);
    let actual_count = count.min(available);

    if actual_count == 0 {
        debug!("No elements to move");
        return 0;
    }

    // Check destination bounds
    if dst_start + actual_count > slice.capacity {
        debug!("Move would exceed slice capacity");
        return 0;
    }

    // Calculate byte offsets
    let src_offset = src_start * element_size;
    let dst_offset = dst_start * element_size;
    let move_size = actual_count * element_size;

    // Perform the move (handles overlapping regions)
    let src_ptr = (slice.ptr as *mut u8).add(src_offset);
    let dst_ptr = (slice.ptr as *mut u8).add(dst_offset);

    ptr::copy(src_ptr, dst_ptr, move_size);

    // Update length if necessary
    if dst_start + actual_count > slice.len {
        slice.len = dst_start + actual_count;
    }

    debug!(
        moved = actual_count,
        src_start = src_start,
        dst_start = dst_start,
        "Slice move completed"
    );

    actual_count
}

/// Fill a slice with a specific value
///
/// # Safety
///
/// The caller must ensure that:
/// - The slice pointer is valid
/// - The element size is correct
/// - The value pointer points to valid data
///
/// # Arguments
///
/// * `slice` - Slice header
/// * `element_size` - Size of each element in bytes
/// * `value_ptr` - Pointer to the value to fill with
/// * `start` - Starting index
/// * `count` - Number of elements to fill
///
/// # Returns
///
/// Number of elements actually filled
#[instrument]
pub unsafe fn slice_fill(
    slice: &mut SliceHeader,
    element_size: usize,
    value_ptr: *const c_void,
    start: usize,
    count: usize,
) -> usize {
    if slice.ptr.is_null() || value_ptr.is_null() {
        debug!("Null pointer in slice_fill");
        return 0;
    }

    // Calculate actual fill count
    let available = slice.capacity.saturating_sub(start);
    let actual_count = count.min(available);

    if actual_count == 0 {
        debug!("No elements to fill");
        return 0;
    }

    // Fill elements one by one
    let base_ptr = slice.ptr as *mut u8;
    for i in 0..actual_count {
        let element_offset = (start + i) * element_size;
        let dst_ptr = base_ptr.add(element_offset);
        ptr::copy_nonoverlapping(value_ptr as *const u8, dst_ptr, element_size);
    }

    // Update length if we're extending the slice
    if start + actual_count > slice.len {
        slice.len = start + actual_count;
    }

    debug!(
        filled = actual_count,
        start = start,
        element_size = element_size,
        "Slice fill completed"
    );

    actual_count
}

/// Zero out a range of elements in a slice
///
/// # Safety
///
/// The caller must ensure that:
/// - The slice pointer is valid
/// - The operation does not exceed slice bounds
///
/// # Arguments
///
/// * `slice` - Slice header
/// * `element_size` - Size of each element in bytes
/// * `start` - Starting index
/// * `count` - Number of elements to zero
///
/// # Returns
///
/// Number of elements actually zeroed
#[instrument]
pub unsafe fn slice_zero(
    slice: &mut SliceHeader,
    element_size: usize,
    start: usize,
    count: usize,
) -> usize {
    if slice.ptr.is_null() {
        debug!("Null pointer in slice_zero");
        return 0;
    }

    // Calculate actual zero count
    let available = slice.len.saturating_sub(start);
    let actual_count = count.min(available);

    if actual_count == 0 {
        debug!("No elements to zero");
        return 0;
    }

    // Calculate byte range
    let start_offset = start * element_size;
    let zero_size = actual_count * element_size;

    // Zero the memory
    let start_ptr = (slice.ptr as *mut u8).add(start_offset);
    ptr::write_bytes(start_ptr, 0, zero_size);

    debug!(
        zeroed = actual_count,
        start = start,
        "Slice zero completed"
    );

    actual_count
}

/// Compare two slices element by element
///
/// # Safety
///
/// The caller must ensure that:
/// - Both slice pointers are valid
/// - The element size is correct
/// - The comparison function is appropriate for the element type
///
/// # Arguments
///
/// * `slice1` - First slice header
/// * `slice2` - Second slice header
/// * `element_size` - Size of each element in bytes
///
/// # Returns
///
/// Ordering result of the comparison
#[instrument]
pub unsafe fn slice_compare(
    slice1: &SliceHeader,
    slice2: &SliceHeader,
    element_size: usize,
) -> Ordering {
    // Handle null pointers
    match (slice1.ptr.is_null(), slice2.ptr.is_null()) {
        (true, true) => return slice1.len.cmp(&slice2.len),
        (true, false) => return Ordering::Less,
        (false, true) => return Ordering::Greater,
        (false, false) => {}
    }

    // Compare element by element
    let min_len = slice1.len.min(slice2.len);
    
    for i in 0..min_len {
        let offset = i * element_size;
        let ptr1 = (slice1.ptr as *const u8).add(offset);
        let ptr2 = (slice2.ptr as *const u8).add(offset);

        // Compare bytes (for generic element comparison)
        for byte_offset in 0..element_size {
            let byte1 = *ptr1.add(byte_offset);
            let byte2 = *ptr2.add(byte_offset);
            
            match byte1.cmp(&byte2) {
                Ordering::Equal => continue,
                other => return other,
            }
        }
    }

    // If all compared elements are equal, compare lengths
    slice1.len.cmp(&slice2.len)
}

/// Check if two slices are equal
///
/// # Safety
///
/// The caller must ensure that slice pointers are valid and element size is correct.
#[instrument]
pub unsafe fn slice_equals(
    slice1: &SliceHeader,
    slice2: &SliceHeader,
    element_size: usize,
) -> bool {
    // Quick length check
    if slice1.len != slice2.len {
        return false;
    }

    // Handle empty slices
    if slice1.len == 0 {
        return true;
    }

    // Handle null pointers
    match (slice1.ptr.is_null(), slice2.ptr.is_null()) {
        (true, true) => return true,
        (true, false) | (false, true) => return false,
        (false, false) => {}
    }

    // Compare memory
    let size = slice1.len * element_size;
    let result = std::ptr::eq(slice1.ptr, slice2.ptr) ||
        std::slice::from_raw_parts(slice1.ptr as *const u8, size) ==
        std::slice::from_raw_parts(slice2.ptr as *const u8, size);

    debug!(
        equal = result,
        len1 = slice1.len,
        len2 = slice2.len,
        element_size = element_size,
        "Slice equality check completed"
    );

    result
}

/// Get a pointer to an element at a specific index
///
/// # Safety
///
/// The caller must ensure that:
/// - The slice pointer is valid
/// - The index is within bounds
/// - The element size is correct
///
/// # Arguments
///
/// * `slice` - Slice header
/// * `element_size` - Size of each element in bytes
/// * `index` - Index of the element
///
/// # Returns
///
/// Pointer to the element, or null if index is out of bounds
#[instrument]
pub unsafe fn slice_get_element_ptr(
    slice: &SliceHeader,
    element_size: usize,
    index: usize,
) -> *mut c_void {
    if slice.ptr.is_null() || index >= slice.len {
        debug!(
            index = index,
            len = slice.len,
            "Index out of bounds in slice_get_element_ptr"
        );
        return std::ptr::null_mut();
    }

    let offset = index * element_size;
    let element_ptr = (slice.ptr as *mut u8).add(offset);

    debug!(
        index = index,
        offset = offset,
        "Element pointer retrieved"
    );

    element_ptr as *mut c_void
}

/// Set the length of a slice (for truncation or extension)
///
/// # Safety
///
/// The caller must ensure that:
/// - The new length does not exceed capacity
/// - If extending, the new elements are properly initialized
///
/// # Arguments
///
/// * `slice` - Slice header
/// * `new_len` - New length for the slice
///
/// # Returns
///
/// true if the length was set successfully, false otherwise
#[instrument]
pub fn slice_set_length(slice: &mut SliceHeader, new_len: usize) -> bool {
    if new_len > slice.capacity {
        debug!(
            new_len = new_len,
            capacity = slice.capacity,
            "Cannot set length beyond capacity"
        );
        return false;
    }

    let old_len = slice.len;
    slice.len = new_len;

    debug!(
        old_len = old_len,
        new_len = new_len,
        "Slice length updated"
    );

    true
}

/// Find the first occurrence of a value in a slice
///
/// # Safety
///
/// The caller must ensure that:
/// - The slice pointer is valid
/// - The value pointer points to valid data
/// - The element size is correct
///
/// # Arguments
///
/// * `slice` - Slice header
/// * `element_size` - Size of each element in bytes
/// * `value_ptr` - Pointer to the value to search for
///
/// # Returns
///
/// Index of the first occurrence, or usize::MAX if not found
#[instrument]
pub unsafe fn slice_find(
    slice: &SliceHeader,
    element_size: usize,
    value_ptr: *const c_void,
) -> usize {
    if slice.ptr.is_null() || value_ptr.is_null() {
        debug!("Null pointer in slice_find");
        return usize::MAX;
    }

    let value_bytes = std::slice::from_raw_parts(value_ptr as *const u8, element_size);
    
    for i in 0..slice.len {
        let element_offset = i * element_size;
        let element_ptr = (slice.ptr as *const u8).add(element_offset);
        let element_bytes = std::slice::from_raw_parts(element_ptr, element_size);
        
        if element_bytes == value_bytes {
            debug!(index = i, "Value found in slice");
            return i;
        }
    }

    debug!("Value not found in slice");
    usize::MAX
}

/// Check if a slice contains a specific value
///
/// # Safety
///
/// The caller must ensure that slice and value pointers are valid.
#[instrument]
pub unsafe fn slice_contains(
    slice: &SliceHeader,
    element_size: usize,
    value_ptr: *const c_void,
) -> bool {
    slice_find(slice, element_size, value_ptr) != usize::MAX
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::slice_runtime::SliceRuntime;

    #[test]
    fn test_slice_copy() {
        let runtime = SliceRuntime::new();
        
        // Create source slice with some data
        let mut src = runtime.create_slice(4, Some(5)).unwrap();
        let mut dst = runtime.create_slice(4, Some(5)).unwrap();
        
        // Fill source with test data
        unsafe {
            let src_data = [1i32, 2, 3, 4, 5];
            std::ptr::copy_nonoverlapping(
                src_data.as_ptr() as *const u8,
                src.ptr as *mut u8,
                20
            );
            src.len = 5;
            
            // Copy data
            let copied = slice_copy(&src, &mut dst, 4, 1, 0, 3);
            assert_eq!(copied, 3);
            assert_eq!(dst.len, 3);
            
            // Verify copied data
            let dst_data = std::slice::from_raw_parts(dst.ptr as *const i32, 3);
            assert_eq!(dst_data, &[2, 3, 4]);
        }
    }

    #[test]
    fn test_slice_fill() {
        let runtime = SliceRuntime::new();
        let mut slice = runtime.create_slice(4, Some(5)).unwrap();
        
        unsafe {
            let value = 42i32;
            let filled = slice_fill(&mut slice, 4, &value as *const i32 as *const c_void, 0, 3);
            
            assert_eq!(filled, 3);
            assert_eq!(slice.len, 3);
            
            let data = std::slice::from_raw_parts(slice.ptr as *const i32, 3);
            assert_eq!(data, &[42, 42, 42]);
        }
    }

    #[test]
    fn test_slice_find() {
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
            
            let search_value = 20i32;
            let index = slice_find(&slice, 4, &search_value as *const i32 as *const c_void);
            assert_eq!(index, 1); // First occurrence at index 1
            
            let not_found = 99i32;
            let index = slice_find(&slice, 4, &not_found as *const i32 as *const c_void);
            assert_eq!(index, usize::MAX);
        }
    }
}
