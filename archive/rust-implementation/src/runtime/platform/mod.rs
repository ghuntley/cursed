//! Runtime Platform Abstraction and Detection
//! 
//! This module provides complete runtime platform detection and abstraction
//! without relying on compile-time cfg! macros. It enables a single binary
//! to adapt to different platforms at runtime.

pub mod runtime_detector;
pub mod cross_platform_factory;
pub mod dynamic_codegen;
pub mod runtime_library_resolver;

pub use runtime_detector::*;
pub use cross_platform_factory::*;
pub use dynamic_codegen::*;
pub use runtime_library_resolver::*;

use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    /// Global runtime platform detector instance
    static ref RUNTIME_DETECTOR: RuntimePlatformDetector = RuntimePlatformDetector::new();
    
    /// Global platform info cache
    static ref PLATFORM_INFO_CACHE: RwLock<Option<RuntimePlatformInfo>> = RwLock::new(None);
    
    /// Global cross-platform factory
    static ref CROSS_PLATFORM_FACTORY: CrossPlatformFactory = CrossPlatformFactory::new();
}

/// Get the current platform information (cached after first detection)
pub fn get_runtime_platform_info() -> RuntimePlatformInfo {
    // Check cache first
    if let Some(cached) = PLATFORM_INFO_CACHE.read().unwrap().as_ref() {
        return cached.clone();
    }
    
    // Detect platform and cache it
    let info = RUNTIME_DETECTOR.detect();
    *PLATFORM_INFO_CACHE.write().unwrap() = Some(info.clone());
    info
}

/// Get the appropriate cross-platform factory for the current platform
pub fn get_cross_platform_factory() -> &'static CrossPlatformFactory {
    &CROSS_PLATFORM_FACTORY
}

/// Force re-detection of platform (useful for testing or when platform changes)
pub fn force_platform_redetection() -> RuntimePlatformInfo {
    *PLATFORM_INFO_CACHE.write().unwrap() = None;
    get_runtime_platform_info()
}

/// Runtime platform-aware initialization
pub fn initialize_runtime_platform() -> Result<(), PlatformError> {
    let info = get_runtime_platform_info();
    
    // Initialize platform-specific components
    let factory = get_cross_platform_factory();
    factory.initialize_for_platform(&info)?;
    
    // Set up runtime library resolver
    let resolver = RuntimeLibraryResolver::new();
    resolver.resolve_libraries_for_platform(&info)?;
    
    println!("Runtime platform initialized: {} on {}", 
             format!("{:?}", info.architecture), 
             format!("{:?}", info.operating_system));
    
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum PlatformError {
    #[error("Platform detection failed: {0}")]
    DetectionFailed(String),
    
    #[error("Unsupported platform combination: {arch:?} on {os:?}")]
    UnsupportedPlatform { arch: RuntimeArchitecture, os: RuntimeOperatingSystem },
    
    #[error("Platform initialization failed: {0}")]
    InitializationFailed(String),
    
    #[error("Library resolution failed: {0}")]
    LibraryResolutionFailed(String),
    
    #[error("Dynamic codegen failed: {0}")]
    DynamicCodegenFailed(String),
}
