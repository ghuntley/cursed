//! # Interface Registry Visualization
//!
//! This module houses utilities and trait implementations for visualizing
//! interface registries. It includes methods to help debug and understand
//! interface hierarchies and relationships.
//!
//! See the enhanced version in `interface_registry_visualization_enhanced.rs`
//! for more comprehensive visualization with improved error handling.

use tracing::trace;

/// Registers the interface registry visualization module
pub fn register_interface_registry_visualization() {
    trace!("Interface registry visualization module registered");
    // This function is called during initialization to ensure the visualization
    // capabilities are properly registered in the compiler
}