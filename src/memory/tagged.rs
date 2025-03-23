// Tagged Pointers for CURSED Memory Management
//
// A tagged pointer is a pointer that stores additional information in unused bits.
// This is useful for storing type information or flags in the pointer itself.

use std::marker::PhantomData;
use std::ptr::{NonNull, null_mut};
use num_traits::FromPrimitive;
use std::fmt;
use std::fmt::Debug;
use super::MIN_ALIGNMENT;

/// Number of bits used for the tag
pub const TAG_BITS: usize = 3;
/// Mask for extracting the tag
pub const TAG_MASK: usize = (1 << TAG_BITS) - 1;
/// Shift amount for the tag
pub const TAG_SHIFT: usize = 0;
/// Mask for extracting the pointer
pub const PTR_MASK: usize = !TAG_MASK;

/// Tags for different value types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Tag {
    /// Null value
    Null = 0,
    /// Boolean value
    Boolean = 1,
    /// Integer value
    Integer = 2,
    /// Float value
    Float = 3,
    /// String value
    String = 4,
    /// Array value
    Array = 5,
    /// Hash table value
    Map = 6,
    /// Function value
    Function = 7,
}

impl Tag {
    /// Create a tag from a u8
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Tag::Null),
            1 => Some(Tag::Boolean),
            2 => Some(Tag::Integer),
            3 => Some(Tag::Float),
            4 => Some(Tag::String),
            5 => Some(Tag::Array),
            6 => Some(Tag::Map),
            7 => Some(Tag::Function),
            _ => None,
        }
    }
}

impl num_traits::FromPrimitive for Tag {
    fn from_i64(n: i64) -> Option<Self> {
        Self::from_u8(n as u8)
    }

    fn from_u64(n: u64) -> Option<Self> {
        Self::from_u8(n as u8)
    }
}

/// A pointer with a tag in the low bits
pub struct TaggedPtr<T> {
    /// Combined pointer and tag value
    value: usize,
    /// Phantom data for type safety
    _phantom: PhantomData<T>,
}

impl<T> TaggedPtr<T> {
    /// Create a new tagged pointer
    pub fn new(ptr: Option<NonNull<T>>, tag: Tag) -> Self {
        // Verify alignment
        if let Some(p) = ptr {
            assert!(
                (p.as_ptr() as usize) & TAG_MASK == 0,
                "Pointer not aligned to {} bytes", 1 << TAG_BITS
            );
        }
        
        let ptr_part = ptr.map(|p| p.as_ptr() as usize).unwrap_or(0) & PTR_MASK;
        let tag_value = (tag as usize) << TAG_SHIFT;
        
        Self {
            value: ptr_part | tag_value,
            _phantom: PhantomData,
        }
    }
    
    /// Create a null tagged pointer
    pub fn null(tag: Tag) -> Self {
        Self::new(None, tag)
    }
    
    /// Get the pointer part
    pub fn ptr(&self) -> Option<NonNull<T>> {
        let ptr_part = self.value & PTR_MASK;
        if ptr_part == 0 {
            None
        } else {
            Some(unsafe { NonNull::new_unchecked(ptr_part as *mut T) })
        }
    }
    
    /// Get the tag part
    pub fn tag(&self) -> Tag {
        let tag_value = (self.value & TAG_MASK) >> TAG_SHIFT;
        Tag::from_u8(tag_value as u8).unwrap_or(Tag::Null)
    }
    
    /// Check if this is a null pointer
    pub fn is_null(&self) -> bool {
        (self.value & PTR_MASK) == 0
    }
    
    /// Get the raw value
    pub fn value(&self) -> usize {
        self.value
    }
}

/// Debug implementation for TaggedPtr
impl<T> fmt::Debug for TaggedPtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TaggedPtr({:x}, {:?})", 
               self.value & PTR_MASK, 
               Tag::from_u8((self.value & TAG_MASK) as u8).unwrap_or(Tag::Null))
    }
}

/// Extension trait for NonNull
pub trait NonNullExt<T> {
    /// Create a tagged pointer from this non-null pointer
    fn with_tag(self, tag: Tag) -> TaggedPtr<T>;
}

impl<T> NonNullExt<T> for NonNull<T> {
    fn with_tag(self, tag: Tag) -> TaggedPtr<T> {
        TaggedPtr::new(Some(self), tag)
    }
}

/// Clone implementation for TaggedPtr
impl<T> Clone for TaggedPtr<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            _phantom: PhantomData,
        }
    }
}

/// Copy implementation for TaggedPtr
impl<T> Copy for TaggedPtr<T> {}

/// Equality implementation for TaggedPtr
impl<T> PartialEq for TaggedPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

/// Equality implementation for TaggedPtr
impl<T> Eq for TaggedPtr<T> {} 