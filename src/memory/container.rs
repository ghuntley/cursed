//! Memory layout optimization for specialized container types

use crate::memory::gc::GarbageCollector;
use crate::memory::{Tag, Traceable, Visitor};
use std::alloc::{alloc, dealloc, Layout};
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::sync::Arc;

/// A tag indicating the type of container for specialized memory layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerType {
    /// Vector-like container (dynamic array)
    Vector,
    /// Map-like container (hash table)
    Map,
    /// Set-like container
    Set,
}

/// Trait for types that can have specialized container implementations
pub trait Specializable: Clone + 'static {
    /// Get the size of the type for memory layout optimization
    fn specialized_size() -> usize;

    /// Get alignment requirements for the type
    fn specialized_alignment() -> usize;

    /// Create a specialized container layout for this type
    fn create_specialized_layout(container_type: ContainerType, capacity: usize) -> Layout;
}

// Implementation for i32 is provided below

/// A specialized vector container optimized for memory layout
pub struct SpecializedVector<T: Specializable> {
    /// Raw pointer to the data buffer
    data: NonNull<T>,
    /// Length of the vector (number of elements)
    length: usize,
    /// Capacity of the vector (maximum number of elements without reallocation)
    capacity: usize,
    /// Reference to the garbage collector for memory management
    collector: Arc<GarbageCollector>,
    /// Phantom data for type parameter
    _marker: PhantomData<T>,
}

impl<T: Specializable> SpecializedVector<T> {
    /// Create a new specialized vector with the given capacity
    pub fn new(capacity: usize, collector: Arc<GarbageCollector>) -> Self {
        let layout = T::create_specialized_layout(ContainerType::Vector, capacity);

        // Allocate memory for the vector
        let ptr = unsafe {
            let ptr = alloc(layout);
            NonNull::new(ptr as *mut T).unwrap_or_else(|| {
                std::alloc::handle_alloc_error(layout);
            })
        };

        SpecializedVector {
            data: ptr,
            length: 0,
            capacity,
            collector,
            _marker: PhantomData,
        }
    }

    /// Get the length of the vector
    pub fn len(&self) -> usize {
        self.length
    }

    /// Check if the vector is empty
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Get the capacity of the vector
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get a reference to an element at the given index
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            None
        } else {
            unsafe { Some(&*self.data.as_ptr().add(index)) }
        }
    }

    /// Get a mutable reference to an element at the given index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.length {
            None
        } else {
            unsafe { Some(&mut *self.data.as_ptr().add(index)) }
        }
    }

    /// Push an element to the end of the vector
    pub fn push(&mut self, value: T) {
        if self.length == self.capacity {
            self.grow();
        }

        unsafe {
            std::ptr::write(self.data.as_ptr().add(self.length), value);
        }

        self.length += 1;
    }

    /// Pop an element from the end of the vector
    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            None
        } else {
            self.length -= 1;
            unsafe {
                let value = std::ptr::read(self.data.as_ptr().add(self.length));
                Some(value)
            }
        }
    }

    /// Grow the capacity of the vector
    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            1
        } else {
            self.capacity * 2
        };
        let new_layout = T::create_specialized_layout(ContainerType::Vector, new_capacity);

        let new_ptr = unsafe {
            let new_ptr = alloc(new_layout);
            let new_ptr = NonNull::new(new_ptr as *mut T).unwrap_or_else(|| {
                std::alloc::handle_alloc_error(new_layout);
            });

            // Copy elements from old buffer to new buffer
            for i in 0..self.length {
                std::ptr::copy_nonoverlapping(
                    self.data.as_ptr().add(i),
                    new_ptr.as_ptr().add(i),
                    1,
                );
            }

            // Deallocate old buffer
            let old_layout = T::create_specialized_layout(ContainerType::Vector, self.capacity);
            dealloc(self.data.as_ptr() as *mut u8, old_layout);

            new_ptr
        };

        self.data = new_ptr;
        self.capacity = new_capacity;
    }
}

impl<T: Specializable> Drop for SpecializedVector<T> {
    fn drop(&mut self) {
        unsafe {
            // Call destructors for all elements
            for i in 0..self.length {
                std::ptr::drop_in_place(self.data.as_ptr().add(i));
            }

            // Deallocate the buffer
            let layout = T::create_specialized_layout(ContainerType::Vector, self.capacity);
            dealloc(self.data.as_ptr() as *mut u8, layout);
        }
    }
}

impl<T: Specializable + Traceable> Traceable for SpecializedVector<T> {
    fn trace(&self, visitor: &mut dyn Visitor) {
        // Trace all elements in the vector
        for i in 0..self.length {
            unsafe {
                let element = &*self.data.as_ptr().add(i);
                element.trace(visitor);
            }
        }
    }

    fn size(&self) -> usize {
        // Size of the structure plus size of all elements
        std::mem::size_of::<Self>() + T::specialized_size() * self.capacity
    }

    fn tag(&self) -> Tag {
        Tag::Array
    }
}

/// Special implementation for primitive types like i32 that aren't Traceable
impl Specializable for i32 {
    fn specialized_size() -> usize {
        std::mem::size_of::<i32>()
    }

    fn specialized_alignment() -> usize {
        std::mem::align_of::<i32>()
    }

    fn create_specialized_layout(container_type: ContainerType, capacity: usize) -> Layout {
        // For vectors, create a more efficient layout
        if container_type == ContainerType::Vector {
            // Create a layout with proper alignment that includes metadata
            // Metadata: length, capacity, element_size
            let metadata_size = std::mem::size_of::<usize>() * 3;
            let elements_size = Self::specialized_size() * capacity;

            // Ensure proper alignment
            unsafe {
                Layout::from_size_align_unchecked(
                    metadata_size + elements_size,
                    Self::specialized_alignment(),
                )
            }
        } else {
            // For other container types, use default layout for now
            unsafe {
                Layout::from_size_align_unchecked(
                    std::mem::size_of::<usize>() * 2 + Self::specialized_size() * capacity,
                    std::mem::align_of::<usize>(),
                )
            }
        }
    }
}
