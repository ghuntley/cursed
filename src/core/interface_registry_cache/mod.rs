//! Interface Registry Cache Module
//!
//! This module provides caching functionality for interface registry operations
//! to improve performance of interface type assertions and other registry queries.

pub mod test_common;

// Re-export key components for convenience
pub use test_common::{
    create_test_registry,
    create_test_registry_with_visualization,
    create_diamond_inheritance_registry,
    create_test_registry_cache,
    verify_test_registry,
    calculate_transitive_closure,
    populate_complex_hierarchy
};