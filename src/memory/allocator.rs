// Allocator interface for memory management
use std::ptr::NonNull;

/// Base trait for allocators
pub trait AllocatorBase {
    /// Allocate a block of memory
    unsafe fn allocate(&mut self, size: usize, align: usize) -> Option<NonNull<u8>>;
    
    /// Deallocate a block of memory
    unsafe fn deallocate(&mut self, ptr: NonNull<u8>, size: usize, align: usize);
    
    /// Reallocate a block of memory
    unsafe fn reallocate(
        &mut self,
        ptr: NonNull<u8>,
        old_size: usize,
        new_size: usize,
        align: usize,
    ) -> Option<NonNull<u8>>;
}

/// Extended allocator trait with additional methods
pub trait Allocator: AllocatorBase {
    /// Allocate memory for a specific type
    fn allocate_type<T>(&mut self) -> Option<NonNull<T>> {
        let layout = std::alloc::Layout::new::<T>();
        unsafe {
            self.allocate(layout.size(), layout.align())
                .map(|ptr| ptr.cast::<T>())
        }
    }
    
    /// Deallocate memory for a specific type
    unsafe fn deallocate_type<T>(&mut self, ptr: NonNull<T>) {
        let layout = std::alloc::Layout::new::<T>();
        self.deallocate(ptr.cast::<u8>(), layout.size(), layout.align());
    }
    
    /// Allocate memory for a slice of a specific type
    fn allocate_slice<T>(&mut self, len: usize) -> Option<NonNull<T>> {
        let layout = std::alloc::Layout::array::<T>(len).ok()?;
        unsafe {
            self.allocate(layout.size(), layout.align())
                .map(|ptr| ptr.cast::<T>())
        }
    }
    
    /// Deallocate memory for a slice of a specific type
    unsafe fn deallocate_slice<T>(&mut self, ptr: NonNull<T>, len: usize) {
        let layout = std::alloc::Layout::array::<T>(len).unwrap_unchecked();
        self.deallocate(ptr.cast::<u8>(), layout.size(), layout.align());
    }
} 