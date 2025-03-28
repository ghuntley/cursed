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
use std::alloc;
use std::ptr::NonNull;
use std::str::{self, FromStr, Chars, SplitWhitespace};
use std::collections::HashMap;
use std::fmt;
use std::cell::{RefCell, Ref, RefMut};
use crate::compiler::symbol_table::{Symbol, SymbolScope, SymbolTable};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::rc::Rc;
use std::cell::Cell;
use std::borrow::Borrow;
use std::string::ToString;

// Export the Vec extension trait
pub trait VecExt<T> {
    fn push(&mut self, item: T);
    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
    fn clear(&mut self);
    fn is_empty(&self) -> bool;
    fn as_slice(&self) -> &[T];
    fn as_mut_slice(&mut self) -> &mut [T];
}

// Export the String extension trait
pub trait StrExt {
    fn chars(&self) -> Chars<'_>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn as_bytes(&self) -> &[u8];
}

impl StrExt for str {
    fn chars(&self) -> Chars<'_> {
        <str>::chars(self)
    }
    
    fn len(&self) -> usize {
        <str>::len(self)
    }
    
    fn is_empty(&self) -> bool {
        <str>::is_empty(self)
    }
    
    fn as_bytes(&self) -> &[u8] {
        <str>::as_bytes(self)
    }
}

// Helper functions for string manipulation
pub fn str_chars(s: &str) -> Chars<'_> {
    s.chars()
}

// Reference extension traits
pub trait RefCellSymbolTableExt {
    fn borrow(&self) -> Ref<'_, SymbolTable>;
    fn borrow_mut(&self) -> RefMut<'_, SymbolTable>;
}

impl RefCellSymbolTableExt for RefCell<SymbolTable> {
    fn borrow(&self) -> Ref<'_, SymbolTable> {
        RefCell::borrow(self)
    }
    
    fn borrow_mut(&self) -> RefMut<'_, SymbolTable> {
        RefCell::borrow_mut(self)
    }
}

// Symbol scope extension
pub trait SymbolScopeExt {
    fn to_string(&self) -> String;
}

impl SymbolScopeExt for SymbolScope {
    fn to_string(&self) -> String {
        match self {
            SymbolScope::Global => "GLOBAL".to_string(),
            SymbolScope::Local => "LOCAL".to_string(),
            SymbolScope::Free => "FREE".to_string(),
            SymbolScope::Function => "FUNCTION".to_string(),
            SymbolScope::Builtin => "BUILTIN".to_string(),
        }
    }
}
