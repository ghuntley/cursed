//! WebAssembly-specific functionality
//! 
//! This module provides WASM-specific implementations for features that
//! are not available or work differently in WebAssembly environments.

#[cfg(target_arch = "wasm32")]
pub mod panic_handler;

#[cfg(target_arch = "wasm32")]
pub mod networking;

#[cfg(target_arch = "wasm32")]
pub mod filesystem;

#[cfg(target_arch = "wasm32")]
pub mod crypto;

#[cfg(target_arch = "wasm32")]
pub mod ffi_stub;

#[cfg(target_arch = "wasm32")]
pub use panic_handler::*;

// Re-export WASM-specific implementations
#[cfg(target_arch = "wasm32")]
pub use networking::WasmNetworking;

#[cfg(target_arch = "wasm32")]
pub use filesystem::WasmFileSystem;

#[cfg(target_arch = "wasm32")]
pub use crypto::WasmCrypto;

#[cfg(target_arch = "wasm32")]
pub use ffi_stub::WasmFFIStub;
