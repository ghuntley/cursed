// Tagged Pointers for CURSED Memory Management
//
// A tagged pointer is a pointer that stores additional information in unused bits.
// This is useful for storing type information or flags in the pointer itself.

use std::marker::PhantomData;
use std::ptr::{NonNull, null_mut};
use num_traits::FromPrimitive;
use std::fmt;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use super::MIN_ALIGNMENT;
use std::rc::Rc;
use crate::prelude::RawPtrExt;
use crate::memory::gc::Traceable;
use std::ptr;

/// Number of bits used for the tag
pub const TAG_BITS: usize = 3;
/// Mask for extracting the tag
pub const TAG_MASK: usize = (1 << TAG_BITS) - 1;
/// Shift amount for the tag
pub const TAG_SHIFT: usize = 0;
/// Mask for extracting the pointer
pub const PTR_MASK: usize = !TAG_MASK;

/// Tags for different value types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Tag {
    /// Null pointer
    Null = 0,
    /// Integer value
    Int = 1,
    /// Float value
    Float = 2,
    /// Boolean value
    Bool = 3,
    /// String value
    String = 4,
    /// Array value
    Array = 5,
    /// Hash map value
    HashMap = 6,
    /// Function value
    Function = 7,
    /// Object value
    Object = 8,
    /// Closure value
    Closure = 9,
    /// Error value
    Error = 10,
    /// Return value
    Return = 11,
    /// Break value
    Break = 12,
    /// Continue value
    Continue = 13,
    /// Undefined value
    Undefined = 14,
    /// Immediate value
    Immediate = 15,
}

impl FromPrimitive for Tag {
    fn from_i64(n: i64) -> Option<Self> {
        Self::from_u8(n as u8)
    }

    fn from_u64(n: u64) -> Option<Self> {
        Self::from_u8(n as u8)
    }
}

impl Tag {
    /// Create a tag from a u8
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Tag::Null),
            1 => Some(Tag::Int),
            2 => Some(Tag::Float),
            3 => Some(Tag::Bool),
            4 => Some(Tag::String),
            5 => Some(Tag::Array),
            6 => Some(Tag::HashMap),
            7 => Some(Tag::Function),
            8 => Some(Tag::Object),
            9 => Some(Tag::Closure),
            10 => Some(Tag::Error),
            11 => Some(Tag::Return),
            12 => Some(Tag::Break),
            13 => Some(Tag::Continue),
            14 => Some(Tag::Undefined),
            15 => Some(Tag::Immediate),
            _ => None,
        }
    }
}

/// A pointer with a tag in the low bits
pub struct TaggedPtr<T: ?Sized> {
    /// Combined pointer and tag value
    ptr: NonNull<T>,
    /// Tag value
    tag: Tag,
}

impl<T: ?Sized> TaggedPtr<T> {
    /// Create a new tagged pointer
    pub fn new(ptr: *mut T, tag: Tag) -> Self {
        Self {
            ptr: unsafe { 
                NonNull::new_unchecked(ptr) 
            },
            tag,
        }
    }

    /// Create a null tagged pointer
    pub fn null() -> Self {
        Self {
            ptr: unsafe { NonNull::new_unchecked(null_mut()) },
            tag: Tag::Null,
        }
    }

    /// Get the raw pointer value
    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    /// Get the tag value
    pub fn tag(&self) -> Tag {
        self.tag
    }

    /// Check if the pointer is null
    pub fn is_null(&self) -> bool {
        self.ptr.as_ptr().is_null()
    }

    /// Convert to a non-null pointer if possible
    pub fn as_non_null(&self) -> Option<NonNull<T>> {
        NonNull::new(self.ptr.as_ptr())
    }

    /// Convert to a raw pointer
    pub fn as_raw_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    /// Convert to usize
    pub fn as_usize(&self) -> usize {
        let raw_ptr = self.ptr.as_ptr();
        if std::mem::size_of::<*mut T>() > std::mem::size_of::<usize>() {
            // Handle fat pointers by first casting to a thin pointer
            let thin_ptr = raw_ptr as *mut () as usize;
            thin_ptr
        } else {
            // For thin pointers, direct casting is fine
            raw_ptr as *mut () as usize
        }
    }

    /// Check if this is an immediate value
    pub fn is_immediate(&self) -> bool {
        self.as_usize() == 0
    }

    /// Get the size of the pointed-to value
    pub fn size(&self) -> usize {
        if let Some(value) = self.as_ref() {
            std::mem::size_of_val(value)
        } else {
            0
        }
    }

    /// Create a new tagged pointer with a different tag
    pub fn with_tag(&self, tag: Tag) -> TaggedPtr<T> {
        TaggedPtr { ptr: self.ptr, tag }
    }
    
    /// Get a reference to the value pointed to by this tagged pointer
    pub fn as_ref(&self) -> Option<&T> {
        if self.is_null() {
            None
        } else {
            unsafe { Some(&*self.ptr.as_ptr()) }
        }
    }
    
    /// Get a mutable reference to the value pointed to by this tagged pointer
    pub fn as_mut(&mut self) -> Option<&mut T> {
        if self.is_null() {
            None
        } else {
            unsafe { Some(&mut *self.ptr.as_ptr()) }
        }
    }

    /// Set the tag of this tagged pointer
    pub fn set_tag(&mut self, tag: Tag) {
        self.tag = tag;
    }
}

/// Debug implementation for TaggedPtr
impl<T: ?Sized> fmt::Debug for TaggedPtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TaggedPtr({:x}, {:?})", 
               self.ptr.as_ptr() as usize & PTR_MASK, 
               self.tag)
    }
}

/// Extension trait for NonNull
pub trait NonNullExt<T: Sized> {
    /// Create a tagged pointer from this non-null pointer
    fn with_tag(self, tag: Tag) -> TaggedPtr<T>;
}

impl<T: Sized> NonNullExt<T> for NonNull<T> {
    fn with_tag(self, tag: Tag) -> TaggedPtr<T> {
        TaggedPtr::new(self.as_ptr(), tag)
    }
}

/// Clone implementation for TaggedPtr
impl<T: ?Sized> Clone for TaggedPtr<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            tag: self.tag,
        }
    }
}

/// Copy implementation for TaggedPtr
impl<T: ?Sized> Copy for TaggedPtr<T> {}

/// Equality implementation for TaggedPtr
impl<T: ?Sized> PartialEq for TaggedPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr && self.tag == other.tag
    }
}

/// Equality implementation for TaggedPtr
impl<T: ?Sized> Eq for TaggedPtr<T> {}

impl<T: ?Sized + Thin> TaggedPtr<T> {
    /// Create a null tagged pointer
    pub fn null() -> Self {
        Self {
            ptr: unsafe { NonNull::new_unchecked(null_mut()) },
            tag: Tag::Null,
        }
    }
}

impl<T> Default for TaggedPtr<T> {
    fn default() -> Self {
        Self {
            ptr: NonNull::dangling(),
            tag: Tag::Null,
        }
    }
}

impl<T: ?Sized> From<Option<NonNull<T>>> for TaggedPtr<T> {
    fn from(ptr: Option<NonNull<T>>) -> Self {
        match ptr {
            Some(ptr) => Self::new(ptr.as_ptr(), Tag::Null),
            None => Self {
                ptr: NonNull::dangling(),
                tag: Tag::Null,
            },
        }
    }
}

impl<T: ?Sized> From<NonNull<T>> for TaggedPtr<T> {
    fn from(ptr: NonNull<T>) -> Self {
        Self::new(ptr.as_ptr(), Tag::Null)
    }
}

impl<T: ?Sized> From<*mut T> for TaggedPtr<T> {
    fn from(ptr: *mut T) -> Self {
        if ptr.is_null() {
            Self::new(null_mut(), Tag::Null)
        } else {
            unsafe {
                Self::new(ptr, Tag::Null)
            }
        }
    }
}

impl<T: ?Sized> From<&T> for TaggedPtr<T> {
    fn from(reference: &T) -> Self {
        unsafe {
            Self::new(reference as *const T as *mut T, Tag::Null)
        }
    }
}

impl<T: ?Sized> From<&mut T> for TaggedPtr<T> {
    fn from(reference: &mut T) -> Self {
        unsafe {
            Self::new(reference as *mut T, Tag::Null)
        }
    }
}

impl<T: ?Sized> Deref for TaggedPtr<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        if let Some(ptr) = self.as_non_null() {
            unsafe { &*ptr.as_ptr() }
        } else {
            panic!("Attempted to dereference a null TaggedPtr")
        }
    }
}

impl<T: ?Sized> DerefMut for TaggedPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if let Some(ptr) = self.as_non_null() {
            unsafe { &mut *ptr.as_ptr() }
        } else {
            panic!("Attempted to dereference a null TaggedPtr")
        }
    }
}

/// Extension trait for TaggedPtr
pub trait TaggedPtrExt<T: ?Sized> {
    /// Get the tag value
    fn tag(&self) -> Tag;
    
    /// Check if the pointer is null
    fn is_null(&self) -> bool;
    
    /// Get the raw pointer value
    fn as_ptr(&self) -> *mut T;
    
    /// Get a reference to the value pointed to by this tagged pointer
    fn as_ref(&self) -> Option<&T>;
    
    /// Get the size of the pointed-to value
    fn size(&self) -> usize;
    
    /// Convert to a non-null pointer if possible
    fn as_non_null(&self) -> Option<NonNull<T>>;
    
    /// Convert to a raw pointer
    fn as_raw_ptr(&self) -> *mut T;
    
    /// Convert to usize
    fn as_usize(&self) -> usize;
    
    /// Check if this is an immediate value
    fn is_immediate(&self) -> bool;
    
    /// Get a mutable reference to the value pointed to by this tagged pointer
    fn as_mut(&mut self) -> Option<&mut T>;
    
    /// Set the tag of this tagged pointer
    fn set_tag(&mut self, tag: Tag);
    
    /// Create a new tagged pointer with a different tag
    fn with_tag(&self, tag: Tag) -> TaggedPtr<T>;
}

/// Extension trait for TaggedPtr with mutable methods
pub trait TaggedPtrExtMut<T: ?Sized> {
    /// Get a mutable reference to the value pointed to by this tagged pointer
    fn as_mut(&mut self) -> Option<&mut T>;
    
    /// Set the tag of this tagged pointer
    fn set_tag(&mut self, tag: Tag);
    
    /// Create a new tagged pointer with a different tag
    fn with_tag(&self, tag: Tag) -> TaggedPtr<T>;
}

/// Extension trait for TaggedPtr type-checking methods
pub trait TypedPtr<T: ?Sized> {
    /// Check if this pointer has the expected tag type
    fn has_tag(&self, expected_tag: Tag) -> bool;
    
    /// Try to get a reference assuming a specific tag type
    fn as_type<U>(&self, expected_tag: Tag) -> Option<&U>;
    
    /// Try to get a mutable reference assuming a specific tag type
    fn as_type_mut<U>(&mut self, expected_tag: Tag) -> Option<&mut U>;
    
    /// Cast to a different pointer type if tags match
    fn cast<U>(&self, expected_tag: Tag) -> Option<TaggedPtr<U>>;
    
    /// Safely unwrap pointer with type checking
    fn unwrap_as<U>(&self, expected_tag: Tag) -> Result<&U, &'static str>;
    
    /// Safely unwrap mutable pointer with type checking
    fn unwrap_as_mut<U>(&mut self, expected_tag: Tag) -> Result<&mut U, &'static str>;
}

impl<T: ?Sized> TypedPtr<T> for TaggedPtr<T> {
    fn has_tag(&self, expected_tag: Tag) -> bool {
        self.tag == expected_tag
    }
    
    fn as_type<U>(&self, expected_tag: Tag) -> Option<&U> {
        if self.has_tag(expected_tag) {
            unsafe {
                let raw_ptr = self.ptr.as_ptr();
                // Cast through a void pointer to avoid type system issues
                let ptr = raw_ptr as *mut () as *const U;
                if ptr.is_null() {
                    None
                } else {
                    Some(&*ptr)
                }
            }
        } else {
            None
        }
    }
    
    fn as_type_mut<U>(&mut self, expected_tag: Tag) -> Option<&mut U> {
        if self.has_tag(expected_tag) {
            unsafe {
                let raw_ptr = self.ptr.as_ptr();
                // Cast through a void pointer to avoid type system issues  
                let ptr = raw_ptr as *mut () as *mut U;
                if ptr.is_null() {
                    None
                } else {
                    Some(&mut *ptr)
                }
            }
        } else {
            None
        }
    }
    
    fn cast<U>(&self, expected_tag: Tag) -> Option<TaggedPtr<U>> {
        if self.has_tag(expected_tag) {
            let tag = self.tag;
            let ptr = self.ptr.as_ptr();
            unsafe {
                // Cast through a void pointer to avoid type issues
                let raw_ptr = ptr as *mut () as *mut U;
                if raw_ptr.is_null() {
                    None
                } else {
                    Some(TaggedPtr {
                        ptr: NonNull::new_unchecked(raw_ptr),
                        tag,
                    })
                }
            }
        } else {
            None
        }
    }
    
    fn unwrap_as<U>(&self, expected_tag: Tag) -> Result<&U, &'static str> {
        self.as_type(expected_tag).ok_or("Invalid type tag or null pointer")
    }
    
    fn unwrap_as_mut<U>(&mut self, expected_tag: Tag) -> Result<&mut U, &'static str> {
        self.as_type_mut(expected_tag).ok_or("Invalid type tag or null pointer")
    }
}

/// A tagged pointer for trait objects, i.e., a fat pointer
#[derive(Clone, Copy)]
pub struct TaggedDynPtr {
    /// Raw pointer to the traceable object
    pub ptr: *mut dyn Traceable,
    /// Tag for the object
    pub tag: Tag,
}

impl TaggedDynPtr {
    /// Create a new TaggedDynPtr
    pub fn new(ptr: *mut dyn Traceable, tag: Tag) -> Self {
        Self { ptr, tag }
    }

    /// Create a null TaggedDynPtr
    pub fn null() -> Self {
        Self {
            ptr: std::ptr::null_mut(), 
            tag: Tag::Null,
        }
    }

    /// Convert from a concrete Traceable type
    pub fn from_traceable<T: Traceable + 'static>(obj: &T) -> Self {
        let ptr = obj as *const T as *mut T as *mut dyn Traceable;
        Self { ptr, tag: Tag::Object }
    }

    /// Convert from a concrete Traceable type with a specific tag
    pub fn from_traceable_with_tag<T: Traceable + 'static>(obj: &T, tag: Tag) -> Self {
        let ptr = obj as *const T as *mut T as *mut dyn Traceable;
        Self { ptr, tag }
    }
    
    /// Get the tag
    pub fn tag(&self) -> Tag {
        self.tag
    }
    
    /// Convert to a usize
    pub fn as_usize(&self) -> usize {
        self.ptr as *mut () as usize
    }
    
    /// Check if the pointer is null
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
    
    /// Get the raw pointer
    pub fn as_raw_ptr(&self) -> *mut dyn Traceable {
        self.ptr
    }
    
    /// Get a non-null pointer if possible
    pub fn as_non_null(&self) -> Option<NonNull<dyn Traceable>> {
        NonNull::new(self.ptr)
    }
    
    /// Get the size of the object
    pub fn size(&self) -> usize {
        if self.is_null() {
            0
        } else {
            unsafe {
                let obj_ref = &*self.ptr;
                std::mem::size_of_val(obj_ref)
            }
        }
    }
}

/// Debug implementation for TaggedDynPtr
impl fmt::Debug for TaggedDynPtr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TaggedDynPtr({:x}, {:?})", 
               self.ptr as *mut () as usize & PTR_MASK, 
               self.tag)
    }
}

/// Trait for constructing tagged pointers
pub trait TaggedPtrConstructor<T: ?Sized> {
    /// Create a new tagged pointer
    fn new(ptr: *mut T, tag: Tag) -> TaggedPtr<T>;
}

impl<T: ?Sized> TaggedPtrConstructor<T> for TaggedPtr<T> {
    fn new(ptr: *mut T, tag: Tag) -> TaggedPtr<T> {
        if ptr.is_null() {
            Self::null()
        } else {
            unsafe {
                TaggedPtr {
                    ptr: NonNull::new_unchecked(ptr),
                    tag,
                }
            }
        }
    }
}

/// Implementation of TaggedPtrExt for TaggedDynPtr
impl TaggedPtrExt<dyn Traceable> for TaggedDynPtr {
    fn tag(&self) -> Tag {
        self.tag
    }
    
    fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
    
    fn as_ptr(&self) -> *mut dyn Traceable {
        self.ptr
    }
    
    fn as_ref(&self) -> Option<&dyn Traceable> {
        if self.is_null() {
            None
        } else {
            unsafe { Some(&*self.ptr) }
        }
    }
    
    fn as_mut(&mut self) -> Option<&mut dyn Traceable> {
        if self.is_null() {
            None
        } else {
            unsafe { Some(&mut *self.ptr) }
        }
    }
    
    fn set_tag(&mut self, tag: Tag) {
        self.tag = tag;
    }
    
    fn with_tag(&self, tag: Tag) -> TaggedPtr<dyn Traceable> {
        TaggedPtr::new(self.ptr, tag)
    }
    
    fn size(&self) -> usize {
        if self.is_null() {
            0
        } else {
            unsafe {
                let obj_ref = &*self.ptr;
                std::mem::size_of_val(obj_ref)
            }
        }
    }
    
    fn as_non_null(&self) -> Option<NonNull<dyn Traceable>> {
        NonNull::new(self.ptr)
    }
    
    fn as_raw_ptr(&self) -> *mut dyn Traceable {
        self.ptr
    }
    
    fn as_usize(&self) -> usize {
        self.ptr as *mut () as usize
    }
    
    fn is_immediate(&self) -> bool {
        self.as_usize() == 0
    }
}

/// Extension trait for NonNull
pub trait Thin {}

// Implement Thin for all types that can be cast to thin pointers
impl<T> Thin for T {} 

impl<T: ?Sized> TaggedPtrExt<T> for TaggedPtr<T> {
    fn tag(&self) -> Tag {
        self.tag
    }
    
    fn is_null(&self) -> bool {
        self.ptr.as_ptr().is_null()
    }
    
    fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
    
    fn as_ref(&self) -> Option<&T> {
        if self.is_null() {
            None
        } else {
            unsafe { Some(&*self.ptr.as_ptr()) }
        }
    }
    
    fn as_mut(&mut self) -> Option<&mut T> {
        if self.is_null() {
            None
        } else {
            unsafe { Some(&mut *self.ptr.as_ptr()) }
        }
    }
    
    fn set_tag(&mut self, tag: Tag) {
        self.tag = tag
    }
    
    fn with_tag(&self, tag: Tag) -> TaggedPtr<T> {
        TaggedPtr { ptr: self.ptr, tag }
    }
    
    fn size(&self) -> usize {
        if let Some(value) = self.as_ref() {
            std::mem::size_of_val(value)
        } else {
            0
        }
    }
    
    fn as_non_null(&self) -> Option<NonNull<T>> {
        NonNull::new(self.ptr.as_ptr())
    }
    
    fn as_raw_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
    
    fn as_usize(&self) -> usize {
        let raw_ptr = self.ptr.as_ptr();
        if std::mem::size_of::<*mut T>() > std::mem::size_of::<usize>() {
            // Handle fat pointers by first casting to a thin pointer
            let thin_ptr = raw_ptr as *mut () as usize;
            thin_ptr
        } else {
            // For thin pointers, direct casting is fine
            raw_ptr as *mut () as usize
        }
    }
    
    fn is_immediate(&self) -> bool {
        self.as_usize() == 0
    }
}

/// Implementation of TaggedPtrExt for TaggedDynPtr
impl TaggedPtrExt<dyn Traceable> for TaggedDynPtr {
    fn tag(&self) -> Tag {
        self.tag
    }
    
    fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
    
    fn as_ptr(&self) -> *mut dyn Traceable {
        self.ptr
    }
    
    fn as_ref(&self) -> Option<&dyn Traceable> {
        if self.is_null() {
            None
        } else {
            unsafe { Some(&*self.ptr) }
        }
    }
    
    fn as_mut(&mut self) -> Option<&mut dyn Traceable> {
        if self.is_null() {
            None
        } else {
            unsafe { Some(&mut *self.ptr) }
        }
    }
    
    fn set_tag(&mut self, tag: Tag) {
        self.tag = tag;
    }
    
    fn with_tag(&self, tag: Tag) -> TaggedPtr<dyn Traceable> {
        TaggedPtr::new(self.ptr, tag)
    }
    
    fn size(&self) -> usize {
        if self.is_null() {
            0
        } else {
            unsafe {
                let obj_ref = &*self.ptr;
                std::mem::size_of_val(obj_ref)
            }
        }
    }
    
    fn as_non_null(&self) -> Option<NonNull<dyn Traceable>> {
        NonNull::new(self.ptr)
    }
    
    fn as_raw_ptr(&self) -> *mut dyn Traceable {
        self.ptr
    }
    
    fn as_usize(&self) -> usize {
        self.ptr as *mut () as usize
    }
    
    fn is_immediate(&self) -> bool {
        self.as_usize() == 0
    }
}
