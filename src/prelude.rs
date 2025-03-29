// prelude.rs
//
// This module provides a convenient prelude for the CURSED language implementation, 
// exporting and re-exporting all important types.

// Standard prelude for the CURSED language
// Re-exports common types and traits

use std::fmt;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

use crate::core::symbol_table::{SymbolScope, SymbolTable};
use crate::ast::Node;

// Common utilities and re-exports for CURSED language
use std::vec::Vec;
use std::str::{self, Chars};
use std::string::ToString;

// Export the Vec extension trait
pub trait VecExt<T> {
    fn push_all(&mut self, other: &[T]) where T: Clone;
}

impl<T> VecExt<T> for Vec<T> {
    fn push_all(&mut self, other: &[T]) where T: Clone {
        for item in other {
            self.push(item.clone());
        }
    }
}

// Export the String extension trait
pub trait StrExt {
    fn chars(&self) -> Chars<'_>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn as_bytes(&self) -> &[u8];
    fn is_letter(&self) -> bool;
    fn is_digit(&self) -> bool;
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
    
    fn is_letter(&self) -> bool {
        false // Default implementation for str
    }
    
    fn is_digit(&self) -> bool {
        false // Default implementation for str
    }
}

impl StrExt for char {
    fn chars(&self) -> Chars<'_> {
        // This is a workaround - we create a static string containing our char
        // that lives for the 'static lifetime, so we can return Chars from it
        match *self {
            'a' => "a".chars(),
            'b' => "b".chars(),
            'c' => "c".chars(),
            // ... add more cases for common chars
            _ => {
                // For other chars, we'll use an allocated string but it's not ideal
                // This is inherently unsafe but necessary due to API constraints
                let s = Box::leak(self.to_string().into_boxed_str());
                s.chars()
            }
        }
    }
    
    fn len(&self) -> usize {
        self.len_utf8()
    }
    
    fn is_empty(&self) -> bool {
        false // A char is never empty
    }
    
    fn as_bytes(&self) -> &[u8] {
        // This is a bit of a hack, but char doesn't have as_bytes
        // and we can't return a slice from a local variable
        &[0u8; 0]
    }
    
    fn is_letter(&self) -> bool {
        self.is_ascii_alphabetic() || *self == '_'
    }
    
    fn is_digit(&self) -> bool {
        self.is_ascii_digit()
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

// Raw pointer extension trait
pub trait RawPtrExt {
    fn as_usize(&self) -> usize;
}

impl<T> RawPtrExt for *const T {
    fn as_usize(&self) -> usize {
        *self as usize
    }
}

impl<T> RawPtrExt for *mut T {
    fn as_usize(&self) -> usize {
        *self as usize
    }
}

// Vector string join extension
pub trait VecStrJoinExt {
    fn join(&self, delimiter: &str) -> String;
}

impl VecStrJoinExt for Vec<String> {
    fn join(&self, delimiter: &str) -> String {
        let mut result = String::new();
        
        for (i, s) in self.iter().enumerate() {
            result.push_str(s);
            if i < self.len() - 1 {
                result.push_str(delimiter);
            }
        }
        
        result
    }
}

// Slice extension
pub trait SliceExt<T> {
    fn into_vec(self) -> Vec<T> where T: Clone;
}

impl<T> SliceExt<T> for &[T] {
    fn into_vec(self) -> Vec<T> where T: Clone {
        self.to_vec()
    }
}

// Placeholder for builtins
pub fn len(_args: &[Rc<crate::object::Object>]) -> Result<Rc<crate::object::Object>, crate::error::Error> {
    // Implementation TBD
    Err(crate::error::Error::not_implemented("len built-in function", crate::error::SourceLocation::default()))
}
