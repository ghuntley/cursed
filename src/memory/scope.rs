//! Utilities for creating and managing GC root scopes
//!
//! This module provides convenient helpers for creating scoped garbage collection
//! roots that automatically clean up when they go out of scope.

use std::sync::Arc;

use crate::memory::gc::GarbageCollector;
use crate::memory::{RootScope, RootScopeGuard, ROOT_MANAGER};

/// Create a new root scope with the given garbage collector
///
/// This function creates a new root scope and returns a guard that will
/// automatically clean up the scope when it is dropped. Any objects allocated
/// during the lifetime of the guard will be automatically tracked as roots.
///
/// # Arguments
///
/// * `gc` - The garbage collector to use for this scope
///
/// # Returns
///
/// A guard that will clean up the scope when dropped
///
/// # Example
///
/// ```
/// use cursed::memory::gc::GarbageCollector;
/// use cursed::memory::scope::with_gc_scope;
///
/// let gc = std::sync::Arc::new(GarbageCollector::new());
///
/// // Create a scope
/// {
///     let _guard = with_gc_scope(gc.clone());
///
///     // Objects allocated here will be tracked as roots
///     // and automatically cleaned up when the guard is dropped
/// }
/// // Scope is cleaned up here
/// ```
pub fn with_gc_scope(gc: Arc<GarbageCollector>) -> RootScopeGuard {
    println!("Creating new GC root scope");
    RootScopeGuard::new(gc)
}

/// Create a new root scope with a new garbage collector
///
/// This function creates a new garbage collector and a new root scope,
/// returning both the GC and a guard that will clean up the scope when
/// it is dropped.
///
/// # Returns
///
/// A tuple containing the garbage collector and a guard that will clean up
/// the scope when dropped
///
/// # Example
///
/// ```
/// use cursed::memory::scope::with_new_gc;
///
/// // Create a scope with a new GC
/// {
///     let (gc, _guard) = with_new_gc();
///
///     // Objects allocated here will be tracked as roots
///     // and automatically cleaned up when the guard is dropped
/// }
/// // Scope is cleaned up here
/// ```
pub fn with_new_gc() -> (Arc<GarbageCollector>, RootScopeGuard) {
    println!("Creating new GC and root scope");
    let gc = Arc::new(GarbageCollector::new());
    let guard = with_gc_scope(gc.clone());
    (gc, guard)
}

/// Run a function with a new GC root scope
///
/// This function creates a new root scope and runs the given function
/// within that scope. Any objects allocated during the function will be
/// automatically tracked as roots and cleaned up when the function returns.
///
/// # Arguments
///
/// * `gc` - The garbage collector to use for this scope
/// * `f` - The function to run within the scope
///
/// # Returns
///
/// The result of the function
///
/// # Example
///
/// ```
/// use cursed::memory::gc::GarbageCollector;
/// use cursed::memory::scope::with_gc_scope_fn;
///
/// let gc = std::sync::Arc::new(GarbageCollector::new());
///
/// // Run a function with a scope
/// with_gc_scope_fn(gc.clone(), |gc| {
///     // Objects allocated here will be tracked as roots
///     // and automatically cleaned up when the function returns
/// });
/// // Scope is cleaned up here
/// ```
pub fn with_gc_scope_fn<F, R>(gc: Arc<GarbageCollector>, f: F) -> R
where
    F: FnOnce(Arc<GarbageCollector>) -> R,
{
    println!("Running function with new GC root scope");
    let _guard = with_gc_scope(gc.clone());
    f(gc)
}

/// Run a function with a new garbage collector and root scope
///
/// This function creates a new garbage collector and a new root scope,
/// then runs the given function within that scope. Any objects allocated
/// during the function will be automatically tracked as roots and cleaned
/// up when the function returns.
///
/// # Arguments
///
/// * `f` - The function to run within the scope
///
/// # Returns
///
/// The result of the function
///
/// # Example
///
/// ```
/// use cursed::memory::scope::with_new_gc_fn;
///
/// // Run a function with a new GC and scope
/// with_new_gc_fn(|gc| {
///     // Objects allocated here will be tracked as roots
///     // and automatically cleaned up when the function returns
/// });
/// // Scope is cleaned up here
/// ```
pub fn with_new_gc_fn<F, R>(f: F) -> R
where
    F: FnOnce(Arc<GarbageCollector>) -> R,
{
    println!("Running function with new GC and root scope");
    let (gc, _guard) = with_new_gc();
    f(gc)
}