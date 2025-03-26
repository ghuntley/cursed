// prelude.rs
//
// This module provides a convenient prelude for the CURSED language implementation, 
// exporting and re-exporting all important types.

// Common utilities and re-exports for CURSED language

use std::ops::RangeBounds;
use std::vec::{Vec, Drain};
use std::slice::{Iter, SliceIndex};
use crate::memory::gc::Traceable;
use std::borrow::Cow;
use num_traits::{Saturating, WrappingAdd, WrappingMul, WrappingSub, WrappingNeg};
use std::alloc;
use std::ptr::NonNull;
// use crate::object::Hashable; // This doesn't exist, so remove it
// For len() and other slice methods
use std::ops::Deref;
use std::str::{self, FromStr};
use std::collections::HashMap;
use std::fmt;
use std::cell::{RefCell, Ref, RefMut};
use crate::compiler::symbol_table::{SymbolTable, Symbol};
use crate::memory::tagged::{TaggedPtr, Tag, TypedPtr, TaggedDynPtr, TaggedDynPtrExt};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::rc::Rc;
use std::cell::Cell;

/// Extension trait for Vec to make methods accessible across crate
pub trait VecExt<T> {
    fn push(&mut self, item: T);
    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
    fn clear(&mut self);
    fn is_empty(&self) -> bool;
    fn swap_remove(&mut self, index: usize) -> T;
    fn iter(&self) -> Iter<'_, T>;
    fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> Drain<'_, T>;
    fn reserve(&mut self, additional: usize);
    fn reverse(&mut self);
    /// Swap the elements at positions i and j in the vector
    unsafe fn swap_unchecked(&mut self, i: usize, j: usize);
    
    /// Returns a reference to an element at the given index, or None if out of bounds
    fn get(&self, index: usize) -> Option<&T>;
    
    /// Returns a mutable reference to an element at the given index, or None if out of bounds
    fn get_mut(&mut self, index: usize) -> Option<&mut T>;
    
    /// Returns a reference to the first element, or None if empty
    fn first(&self) -> Option<&T>;
    
    /// Returns a mutable reference to the first element, or None if empty
    fn first_mut(&mut self) -> Option<&mut T>;
    
    /// Returns a reference to the last element, or None if empty
    fn last(&self) -> Option<&T>;
    
    /// Returns a mutable reference to the last element, or None if empty
    fn last_mut(&mut self) -> Option<&mut T>;
    
    /// Removes the last element and returns it, or None if empty
    fn pop(&mut self) -> Option<T>;
    
    /// Removes the element at the given index and returns it
    fn remove(&mut self, index: usize) -> T;
    
    /// Truncates the vector to the given length
    fn truncate(&mut self, len: usize);

    fn as_slice(&self) -> &[T];
    fn as_mut_slice(&mut self) -> &mut [T];
}

impl<T> VecExt<T> for Vec<T> {
    fn push(&mut self, item: T) {
        std::vec::Vec::push(self, item)
    }
    
    fn len(&self) -> usize {
        std::vec::Vec::len(self)
    }
    
    fn capacity(&self) -> usize {
        std::vec::Vec::capacity(self)
    }
    
    fn clear(&mut self) {
        std::vec::Vec::clear(self)
    }
    
    fn is_empty(&self) -> bool {
        std::vec::Vec::is_empty(self)
    }
    
    fn swap_remove(&mut self, index: usize) -> T {
        std::vec::Vec::swap_remove(self, index)
    }
    
    fn iter(&self) -> Iter<'_, T> {
        self.as_slice().iter()
    }
    
    fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> Drain<'_, T> {
        std::vec::Vec::drain(self, range)
    }
    
    fn reserve(&mut self, additional: usize) {
        std::vec::Vec::reserve(self, additional)
    }
    
    fn reverse(&mut self) {
        std::vec::Vec::reverse(self)
    }
    
    /// Swap the elements at positions i and j in the vector
    unsafe fn swap_unchecked(&mut self, i: usize, j: usize) {
        std::ptr::swap_nonoverlapping(&mut self[i] as *mut T, &mut self[j] as *mut T, 1)
    }
    
    fn get(&self, index: usize) -> Option<&T> {
        self.as_slice().get(index)
    }
    
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.as_mut_slice().get_mut(index)
    }
    
    fn first(&self) -> Option<&T> {
        self.as_slice().first()
    }
    
    fn first_mut(&mut self) -> Option<&mut T> {
        self.as_mut_slice().first_mut()
    }
    
    fn last(&self) -> Option<&T> {
        self.as_slice().last()
    }
    
    fn last_mut(&mut self) -> Option<&mut T> {
        self.as_mut_slice().last_mut()
    }
    
    fn pop(&mut self) -> Option<T> {
        std::vec::Vec::pop(self)
    }
    
    fn remove(&mut self, index: usize) -> T {
        std::vec::Vec::remove(self, index)
    }
    
    fn truncate(&mut self, len: usize) {
        std::vec::Vec::truncate(self, len)
    }
    
    fn as_slice(&self) -> &[T] {
        std::vec::Vec::as_slice(self)
    }
    
    fn as_mut_slice(&mut self) -> &mut [T] {
        std::vec::Vec::as_mut_slice(self)
    }
}

/// Extension traits for cloneable vectors
pub trait CloneableVecExt<T: Clone> {
    fn clone(&self) -> Vec<T>;
    fn reverse(&mut self);
}

impl<T: Clone> CloneableVecExt<T> for Vec<T> {
    fn clone(&self) -> Vec<T> {
        <Vec<T> as Clone>::clone(self)
    }
    
    fn reverse(&mut self) {
        std::vec::Vec::reverse(self)
    }
}

/// Extension trait for String and &str
pub trait StringExt {
    /// Split the string on whitespace
    fn split_on_whitespace(&self) -> Vec<String>;
    
    /// Join the string with new lines
    fn join_with_newlines(&self, strings: &[String]) -> String;
    
    /// Convert to a string
    fn to_string_ext(&self) -> String;
    
    /// Get the character at the given index
    fn char_at(&self, index: usize) -> Option<char>;
    
    /// Get the characters as a vector
    fn chars_vec(&self) -> Vec<char>;
    
    /// Convert to cursed object
    fn to_cursed_object(&self) -> TaggedDynPtr;
    
    /// Get the characters from the string
    fn chars(&self) -> std::str::Chars<'_>;
}

impl StringExt for String {
    fn split_on_whitespace(&self) -> Vec<String> {
        self.split_whitespace().map(|s| s.to_string()).collect()
    }
    
    fn join_with_newlines(&self, strings: &[String]) -> String {
        strings.join("\n")
    }
    
    fn to_string_ext(&self) -> String {
        self.clone()
    }
    
    fn char_at(&self, index: usize) -> Option<char> {
        self.chars().nth(index)
    }
    
    fn chars_vec(&self) -> Vec<char> {
        self.chars().collect()
    }
    
    fn to_cursed_object(&self) -> TaggedDynPtr {
        TaggedDynPtr::new(self.as_ptr() as *mut dyn Traceable, Tag::String)
    }
    
    fn chars(&self) -> std::str::Chars<'_> {
        self.chars()
    }
}

impl StringExt for str {
    fn split_on_whitespace(&self) -> Vec<String> {
        self.split_whitespace().map(|s| s.to_string()).collect()
    }
    
    fn join_with_newlines(&self, strings: &[String]) -> String {
        strings.join("\n")
    }
    
    fn to_string_ext(&self) -> String {
        self.to_string()
    }
    
    fn char_at(&self, index: usize) -> Option<char> {
        self.chars().nth(index)
    }
    
    fn chars_vec(&self) -> Vec<char> {
        self.chars().collect()
    }
    
    fn to_cursed_object(&self) -> TaggedDynPtr {
        TaggedDynPtr::new(self.as_ptr() as *mut dyn Traceable, Tag::String)
    }
    
    fn chars(&self) -> std::str::Chars<'_> {
        self.chars()
    }
}

/// Extension trait for character operations on strings
pub trait StrCharsExt {
    /// Get an iterator over the chars in the string
    fn chars(&self) -> std::str::Chars<'_>;
}

impl StrCharsExt for str {
    fn chars(&self) -> std::str::Chars<'_> {
        self.chars()
    }
}

// Error handling
pub use crate::error::{Error, ErrorReporter, SourceLocation};

// Lexer
pub use crate::lexer::Lexer;
pub use crate::lexer::Token;

// Parser
pub use crate::parser_impl::Parser;
pub use crate::ast::{
    Node, Program, Statement, Expression, 
    IntegerLiteral, FloatLiteral, StringLiteral, BooleanLiteral,
    Identifier, ImportStatement, PackageStatement
};

// Compiler
pub use crate::compiler::{
    Compiler, 
    Bytecode, 
    CompiledFunction, 
    Opcode, 
    symbol_table::SymbolScope,
    bytecode::Instructions
};

// Object is already imported at the top of the file
pub use crate::object::Object;

// VM
pub use crate::vm::{VM, Frame};

// This allows importing everything with a single use statement:
// `use crate::prelude::*;` 

// Utility Functions for Memory Operations

// Fallback functions for pointer operations (to support older Rust versions)

/// Checks if a raw pointer is null
pub unsafe fn ptr_is_null<T>(ptr: *mut T) -> bool {
    ptr as usize == 0
}

/// Extension trait for raw pointers to provide some common methods
pub trait RawPtrExt<T: ?Sized> {
    /// Checks if the pointer is null
    fn is_null(self) -> bool;
    
    /// Gets the raw pointer value
    fn as_ptr(self) -> *mut T;
    
    /// Gets the raw pointer value as a usize
    fn as_usize(self) -> usize;
    
    /// Gets the raw pointer value as a NonNull
    fn as_non_null(self) -> Option<NonNull<T>>;
    
    /// Gets the raw pointer value as a reference
    fn as_ref(&self) -> Option<&T>;
    
    /// Gets the raw pointer value as a mutable reference
    fn as_mut(&mut self) -> Option<&mut T>;
    
    /// Gets the raw pointer value as a slice
    fn as_slice(&self, len: usize) -> Option<&[T]> where T: Sized;
    
    /// Gets the raw pointer value as a mutable slice
    fn as_mut_slice(&mut self, len: usize) -> Option<&mut [T]> where T: Sized;
    
    /// Gets the raw pointer value as a string slice
    fn as_str(&self) -> Option<&str> where T: Sized;
    
    /// Gets the raw pointer value as a mutable string slice
    fn as_mut_str(&mut self) -> Option<&mut str> where T: Sized;
    
    /// Calculates a pointer offset with wrapping behavior
    unsafe fn wrapping_offset(self, offset: isize) -> *mut T where T: Sized;
    
    /// Adds to a pointer, with wrapping behavior
    unsafe fn add(self, count: usize) -> *mut T where T: Sized;
}

/// Implementation of RawPtrExt for mutable raw pointers
impl<T: ?Sized> RawPtrExt<T> for *mut T {
    /// Checks if the pointer is null
    fn is_null(self) -> bool {
        self.is_null()
    }

    /// Gets the raw pointer value as a reference
    fn as_ref(&self) -> Option<&T> {
        if self.is_null() {
            None
        } else {
            unsafe { Some(&**self) }
        }
    }

    /// Gets the raw pointer value as a mutable reference
    fn as_mut(&mut self) -> Option<&mut T> {
        if self.is_null() {
            None
        } else {
            unsafe { Some(&mut **self) }
        }
    }

    /// Gets the raw pointer value
    fn as_ptr(self) -> *mut T {
        self
    }
    
    fn as_usize(self) -> usize {
        self as usize
    }
    
    fn as_non_null(self) -> Option<NonNull<T>> {
        NonNull::new(self)
    }
    
    fn as_slice(&self, len: usize) -> Option<&[T]> where T: Sized {
        if self.is_null() {
            None
        } else {
            unsafe { Some(std::slice::from_raw_parts(*self, len)) }
        }
    }
    
    fn as_mut_slice(&mut self, len: usize) -> Option<&mut [T]> where T: Sized {
        if self.is_null() {
            None
        } else {
            unsafe { Some(std::slice::from_raw_parts_mut(*self, len)) }
        }
    }
    
    fn as_str(&self) -> Option<&str> {
        if self.is_null() {
            None
        } else {
            unsafe { std::str::from_utf8(self.as_slice(self.len())?).ok() }
        }
    }
    
    fn as_mut_str(&mut self) -> Option<&mut str> {
        if self.is_null() {
            None
        } else {
            unsafe { std::str::from_utf8_mut(self.as_mut_slice(self.len())?).ok() }
        }
    }
    
    /// Calculates a pointer offset with wrapping behavior
    unsafe fn wrapping_offset(self, offset: isize) -> *mut T {
        self.wrapping_offset(offset)
    }
    
    /// Adds to a pointer, with wrapping behavior
    unsafe fn add(self, count: usize) -> *mut T {
        (self as *mut T).wrapping_add(count)
    }
}

/// Implementation of RawPtrExt for const raw pointers
impl<T: ?Sized> RawPtrExt<T> for *const T {
    /// Checks if the pointer is null
    fn is_null(self) -> bool {
        self.is_null()
    }
    
    /// Gets the raw pointer value
    fn as_ptr(self) -> *mut T {
        self as *mut T
    }
    
    /// Gets the raw pointer value as a usize
    fn as_usize(self) -> usize {
        self as usize
    }
    
    /// Gets the raw pointer value as a NonNull
    fn as_non_null(self) -> Option<NonNull<T>> {
        NonNull::new(self as *mut T)
    }
    
    /// Gets the raw pointer value as a reference
    fn as_ref(&self) -> Option<&T> {
        if self.is_null() {
            None
        } else {
            unsafe { Some(&**self) }
        }
    }
    
    /// Gets the raw pointer value as a mutable reference
    fn as_mut(&mut self) -> Option<&mut T> {
        if self.is_null() {
            None
        } else {
            unsafe { Some(&mut *(*self as *mut T)) }
        }
    }
    
    /// Gets the raw pointer value as a slice
    fn as_slice(&self, len: usize) -> Option<&[T]> where T: Sized {
        if self.is_null() {
            None
        } else {
            unsafe { Some(std::slice::from_raw_parts(*self, len)) }
        }
    }
    
    /// Gets the raw pointer value as a mutable slice
    fn as_mut_slice(&mut self, len: usize) -> Option<&mut [T]> where T: Sized {
        if self.is_null() {
            None
        } else {
            unsafe { Some(std::slice::from_raw_parts_mut(*self as *mut T, len)) }
        }
    }
    
    /// Gets the raw pointer value as a string slice
    fn as_str(&self) -> Option<&str> {
        if self.is_null() {
            None
        } else {
            unsafe {
                let bytes = std::slice::from_raw_parts(*self as *const u8, 
                    (*(*self as *const std::ffi::c_char)).to_string().len());
                std::str::from_utf8(bytes).ok()
            }
        }
    }
    
    /// Gets the raw pointer value as a mutable string slice
    fn as_mut_str(&mut self) -> Option<&mut str> {
        if self.is_null() {
            None
        } else {
            unsafe {
                let bytes = std::slice::from_raw_parts_mut(*self as *mut u8,
                    (*(*self as *const std::ffi::c_char)).to_string().len());
                std::str::from_utf8_mut(bytes).ok()
            }
        }
    }
    
    /// Calculates a pointer offset with wrapping behavior
    unsafe fn wrapping_offset(self, offset: isize) -> *mut T {
        self.wrapping_offset(offset) as *mut T
    }
    
    /// Adds to a pointer, with wrapping behavior
    unsafe fn add(self, count: usize) -> *mut T {
        (self as *mut T).wrapping_add(count)
    }
}

/// Calculates the offset from a pointer safely handling overflow/underflow
pub unsafe fn ptr_wrapping_offset<T>(ptr: *mut T, offset: isize) -> *mut T {
    ptr.wrapping_offset(offset)
}

/// Extension trait for arrays
pub trait ArrayExt<T, const N: usize> {
    /// Convert an array to a Vec
    fn to_vec(&self) -> Vec<T> where T: Clone;
    
    /// Get the length of the array
    fn len(&self) -> usize;
    
    /// Check if the array is empty
    fn is_empty(&self) -> bool;
}

impl<T: Clone, const N: usize> ArrayExt<T, N> for [T; N] {
    fn to_vec(&self) -> Vec<T> {
        Vec::from(self.clone())
    }
    
    fn len(&self) -> usize {
        N
    }
    
    fn is_empty(&self) -> bool {
        N == 0
    }
}

/// Extension trait for RefCell<SymbolTable>
pub trait RefCellSymbolTableExt {
    /// Define a symbol in the symbol table
    fn define(&self, name: String) -> usize;
    
    /// Resolve a symbol in the symbol table
    fn resolve(&self, name: &str) -> Option<usize>;
    
    /// Define a free symbol in the symbol table
    fn define_free(&self, original_symbol: usize) -> usize;
    
    /// Get all free symbols from the symbol table
    fn free_symbols(&self) -> Vec<usize>;
    
    /// Take the outer symbol table from this symbol table
    fn take_outer(&self) -> Option<Rc<RefCell<SymbolTable>>>;
    
    /// Create a new enclosed symbol table
    fn new_enclosed(&self) -> Rc<RefCell<SymbolTable>>;
}

/// Implementation of RefCellSymbolTableExt for RefCell<SymbolTable>
impl RefCellSymbolTableExt for RefCell<SymbolTable> {
    fn define(&self, name: String) -> usize {
        self.borrow_mut().define(name)
    }
    
    fn resolve(&self, name: &str) -> Option<usize> {
        self.borrow().resolve(name)
    }
    
    fn define_free(&self, original_symbol: usize) -> usize {
        self.borrow_mut().define_free(original_symbol)
    }
    
    fn free_symbols(&self) -> Vec<usize> {
        let borrowed = self.borrow();
        if let Some(ref free_symbols) = borrowed.free_symbols {
            free_symbols.clone()
        } else {
            Vec::new()
        }
    }
    
    fn take_outer(&self) -> Option<Rc<RefCell<SymbolTable>>> {
        self.borrow_mut().outer.take()
    }
    
    fn new_enclosed(&self) -> Rc<RefCell<SymbolTable>> {
        SymbolTable::new_enclosed(self.clone())
    }
}

/// Extension trait for SymbolScope to add helper methods
pub trait SymbolScopeExt {
    fn is_global(&self) -> bool;
}

impl SymbolScopeExt for crate::compiler::symbol_table::SymbolScope {
    fn is_global(&self) -> bool {
        use crate::compiler::symbol_table::SymbolScope;
        match self {
            SymbolScope::Global => true,
            _ => false,
        }
    }
}

/// Extension trait for joining strings in a vector
pub trait VecStrJoinExt {
    /// Join the elements of a string vector with a delimiter
    fn join(&self, delimiter: &str) -> String;
}

impl VecStrJoinExt for Vec<&str> {
    fn join(&self, delimiter: &str) -> String {
        if self.is_empty() {
            return String::new();
        }
        
        let mut result = String::new();
        for (i, item) in self.iter().enumerate() {
            if i > 0 {
                result.push_str(delimiter);
            }
            result.push_str(item);
        }
        result
    }
}

pub trait TaggedPtrExt<T: ?Sized> {
    /// Get the tag value
    fn tag(&self) -> Tag;
    
    /// Check if the pointer is null
    fn is_null(&self) -> bool;
    
    /// Get the raw pointer value
    fn as_ptr(&self) -> *mut T;
    
    /// Get a reference to the value pointed to by this tagged pointer
    fn as_ref(&self) -> Option<&T>;
    
    /// Get a mutable reference to the value pointed to by this tagged pointer
    fn as_mut(&mut self) -> Option<&mut T>;
    
    /// Set the tag of this tagged pointer
    fn set_tag(&mut self, tag: Tag);
    
    /// Create a new tagged pointer with a different tag
    fn with_tag(&self, tag: Tag) -> TaggedPtr<T>;
    
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
}

// The implementation of TaggedPtrExt for TaggedPtr<T> is in src/memory/tagged.rs

/// Extension trait for str
pub trait StrExt {
    /// Check if the string is empty
    fn is_empty(&self) -> bool;
    
    /// Get the length of the string
    fn len(&self) -> usize;
    
    /// Trim whitespace from the start and end of the string
    fn trim(&self) -> &str;
    
    /// Trim whitespace from the start of the string
    fn trim_start(&self) -> &str;
    
    /// Trim whitespace from the end of the string
    fn trim_end(&self) -> &str;
    
    /// Repeat the string n times
    fn repeat(&self, n: usize) -> String;
    
    /// Convert the string to lowercase
    fn to_lowercase(&self) -> String;
    
    /// Check if the string contains a pattern
    fn contains(&self, pat: &str) -> bool;
    
    /// Split the string into an iterator of substrings
    fn split_whitespace(&self) -> std::str::SplitWhitespace;
    
    /// Split the string at the first occurrence of the pattern
    fn split_once(&self, pat: &str) -> Option<(&str, &str)>;
    
    /// Convert the first character to uppercase, leave the rest unchanged
    fn to_capitalized(&self) -> String;
    
    /// Get characters from the string
    fn chars(&self) -> std::str::Chars<'_>;
}

impl StrExt for str {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
    
    fn len(&self) -> usize {
        self.len()
    }
    
    fn trim(&self) -> &str {
        self.trim()
    }
    
    fn trim_start(&self) -> &str {
        self.trim_start()
    }
    
    fn trim_end(&self) -> &str {
        self.trim_end()
    }
    
    fn repeat(&self, n: usize) -> String {
        self.repeat(n)
    }
    
    fn to_lowercase(&self) -> String {
        self.to_lowercase()
    }
    
    fn contains(&self, pat: &str) -> bool {
        self.contains(pat)
    }
    
    fn split_whitespace(&self) -> std::str::SplitWhitespace {
        self.split_whitespace()
    }
    
    fn split_once(&self, pat: &str) -> Option<(&str, &str)> {
        self.split_once(pat)
    }
    
    fn to_capitalized(&self) -> String {
        let mut chars = self.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().chain(chars).collect(),
        }
    }
    
    fn chars(&self) -> std::str::Chars<'_> {
        self.chars()
    }
}