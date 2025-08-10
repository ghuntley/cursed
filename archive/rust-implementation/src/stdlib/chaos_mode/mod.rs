use crate::error::CursedError;
/// ChaosMode - CURSED Runtime System Package
/// 
/// Provides comprehensive access to the CURSED runtime system for goroutine management,
/// debugging, memory control, and system information with a chaotic twist focused on
/// performance and observability.

pub mod core;
pub mod memory;
pub mod goroutines;
pub mod profiling;
pub mod runtime_info;
pub mod enhanced;
pub mod error;

pub use error::{ChaosError, ChaosResult};

// Re-export all core functionality
pub use core::{
    // Basic runtime functions
// };

pub use memory::{
    // Memory management
    
    // Enhanced memory debugging
// };

pub use goroutines::{
    // Stack and goroutine management
    
    // Enhanced goroutine management
// };

pub use profiling::{
    // Profiling and tracing
// };

pub use runtime_info::{
    // Runtime information
// };

pub use enhanced::{
    // Enhanced garbage collection
    
    // Performance tuning
// };

// Module initialization
use std::sync::Once;
static INIT: Once = Once::new();

/// Initialize the ChaosMode runtime system
pub fn initialize() -> ChaosResult<()> {
    INIT.call_once(|| {
        // Initialize subsystems
        if let Err(e) = core::initialize() {
            eprintln!("Failed to initialize ChaosMode core: {:?}", e);
        }
        if let Err(e) = memory::initialize() {
            eprintln!("Failed to initialize ChaosMode memory: {:?}", e);
        }
        if let Err(e) = goroutines::initialize() {
            eprintln!("Failed to initialize ChaosMode goroutines: {:?}", e);
        }
        if let Err(e) = profiling::initialize() {
            eprintln!("Failed to initialize ChaosMode profiling: {:?}", e);
        }
        if let Err(e) = enhanced::initialize() {
            eprintln!("Failed to initialize ChaosMode enhanced: {:?}", e);
        }
    });
    Ok(())
/// Cleanup the ChaosMode runtime system
pub fn cleanup() -> ChaosResult<()> {
    // Cleanup all subsystems
    enhanced::cleanup()?;
    profiling::cleanup()?;
    goroutines::cleanup()?;
    memory::cleanup()?;
    core::cleanup()?;
    Ok(())
/// Get comprehensive runtime statistics
pub fn chaos_stats() -> ChaosResult<serde_json::Value> {
    use serde_json::json;
    
    Ok(json!({
        "goroutines": {
        "memory": {
//             "gc_percent": crate::stdlib::vibecheck::get_gc_percent(),
        "system": {
        "enhanced": {
        }
    }))
}
