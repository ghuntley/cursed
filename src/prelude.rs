// prelude.rs
//
// This module provides a convenient prelude for the CURSED language implementation, 
// exporting and re-exporting all important types.

// Common utilities and re-exports for CURSED language

use std::ops::RangeBounds;
use std::vec::{Vec, Drain};
use std::slice::Iter;

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
}

impl<T> VecExt<T> for Vec<T> {
    fn push(&mut self, item: T) {
        Vec::push(self, item);
    }
    
    fn len(&self) -> usize {
        Vec::len(self)
    }
    
    fn capacity(&self) -> usize {
        Vec::capacity(self)
    }
    
    fn clear(&mut self) {
        Vec::clear(self);
    }
    
    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }
    
    fn swap_remove(&mut self, index: usize) -> T {
        Vec::swap_remove(self, index)
    }
    
    fn iter(&self) -> Iter<'_, T> {
        // Use a direct approach to get the iterator
        self.as_slice().iter() // This returns a slice iterator
    }
    
    fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> Drain<'_, T> {
        Vec::drain(self, range)
    }
    
    fn reserve(&mut self, additional: usize) {
        Vec::reserve(self, additional);
    }
    
    fn reverse(&mut self) {
        // Directly access the Vec implementation
        let len = self.len();
        if len <= 1 {
            return;
        }
        
        // Manual swap approach with unsafe operations
        unsafe {
            let base_ptr = self.as_mut_ptr();
            for i in 0..len / 2 {
                let j = len - 1 - i;
                let ptr_i = base_ptr.add(i);
                let ptr_j = base_ptr.add(j);
                std::ptr::swap(ptr_i, ptr_j);
            }
        }
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
        // Manual implementation to avoid conflicts
        let len = self.len();
        if len <= 1 {
            return;
        }
        
        // Use indexes to swap elements without causing double mutable borrow
        for i in 0..len / 2 {
            let j = len - 1 - i;
            // Use a temporary index approach to avoid double borrow
            let temp = self[i].clone();
            self[i] = self[j].clone();
            self[j] = temp;
        }
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
    Compiler, Bytecode, Instructions, Object, 
    CompiledFunction, Opcode, SymbolTable
};

// VM
pub use crate::vm::{VM, Frame};

// This allows importing everything with a single use statement:
// `use crate::prelude::*;` 