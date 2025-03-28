// Tagged pointer implementation for memory management
use std::ptr::NonNull;
use std::fmt;

/// Tag for identifying object types in memory
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
    Int,
    Float,
    String,
    Boolean,
    Array,
    Map,
    Function,
    Null,
    Object,
}

/// A pointer with a type tag
pub struct TaggedPtr<T> {
    ptr: NonNull<T>,
    tag: Tag,
}

/// A type-erased tagged pointer
pub struct TaggedDynPtr {
    ptr: NonNull<u8>,
    tag: Tag,
}

/// Extension trait for NonNull pointers
pub trait NonNullExt<T> {
    fn with_tag(self, tag: Tag) -> TaggedPtr<T>;
}

impl<T> NonNullExt<T> for NonNull<T> {
    fn with_tag(self, tag: Tag) -> TaggedPtr<T> {
        TaggedPtr { ptr: self, tag }
    }
}

impl<T> TaggedPtr<T> {
    /// Create a new tagged pointer
    pub fn new(ptr: NonNull<T>, tag: Tag) -> Self {
        TaggedPtr { ptr, tag }
    }
    
    /// Get the tag
    pub fn tag(&self) -> Tag {
        self.tag
    }
    
    /// Get the raw pointer
    pub fn ptr(&self) -> NonNull<T> {
        self.ptr
    }
    
    /// Cast to a different type
    pub unsafe fn cast<U>(self) -> TaggedPtr<U> {
        TaggedPtr {
            ptr: self.ptr.cast(),
            tag: self.tag,
        }
    }
    
    /// Cast to a type-erased pointer
    pub fn to_dyn(self) -> TaggedDynPtr {
        TaggedDynPtr {
            ptr: unsafe { self.ptr.cast() },
            tag: self.tag,
        }
    }
}

/// Extension trait for TaggedPtr
pub trait TaggedPtrExt<T> {
    fn as_ref(&self) -> &T;
    fn as_mut(&mut self) -> &mut T;
}

impl<T> TaggedPtrExt<T> for TaggedPtr<T> {
    fn as_ref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
    
    fn as_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T> Clone for TaggedPtr<T> {
    fn clone(&self) -> Self {
        TaggedPtr {
            ptr: self.ptr,
            tag: self.tag,
        }
    }
}

impl<T> Copy for TaggedPtr<T> {}

/// A typed tagged pointer, used for static type checking
pub struct TypedPtr<T> {
    ptr: NonNull<T>,
}

impl<T> TypedPtr<T> {
    /// Create a new typed pointer
    pub fn new(ptr: NonNull<T>) -> Self {
        TypedPtr { ptr }
    }
    
    /// Get the raw pointer
    pub fn ptr(&self) -> NonNull<T> {
        self.ptr
    }
    
    /// Convert to a tagged pointer
    pub fn to_tagged(self, tag: Tag) -> TaggedPtr<T> {
        TaggedPtr { ptr: self.ptr, tag }
    }
}

impl<T> Clone for TypedPtr<T> {
    fn clone(&self) -> Self {
        TypedPtr { ptr: self.ptr }
    }
}

impl<T> Copy for TypedPtr<T> {} 