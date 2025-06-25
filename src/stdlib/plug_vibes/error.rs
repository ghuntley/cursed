use crate::error::CursedError;
/// CursedError handling for the PlugVibes plugin system
use std::fmt;

/// Errors that can occur during plugin operations
#[derive(Debug, Clone, PartialEq)]
pub enum PluginError {
    /// Plugin file not found
    PluginNotFound(String),
    /// Failed to load plugin from file
    LoadError(String),
    /// Symbol not found in plugin
    SymbolNotFound(String),
    /// Function not found in plugin
    FunctionNotFound(String),
    /// Plugin initialization failed
    InitializationFailed(String),
    /// Plugin cleanup failed
    CleanupFailed(String),
    /// Plugin version incompatible
    VersionIncompatible(String),
    /// Plugin signature verification failed
    SignatureVerificationFailed(String),
    /// Plugin already loaded/registered
    AlreadyLoaded(String),
    /// Plugin not loaded/registered
    NotLoaded(String),
    /// Plugin sandbox violation
    SandboxViolation(String),
    /// Plugin security violation
    SecurityViolation(String),
    /// Plugin dependency missing
    DependencyMissing(String),
    /// Plugin timeout during operation
    Timeout(String),
    /// Plugin registry error
    RegistryError(String),
    /// Plugin manager error
    ManagerError(String),
    /// Plugin hook error
    HookError(String),
    /// Plugin distribution error
    DistributionError(String),
    /// General plugin error
    General(String),
}

// impl fmt::Display for PluginError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             PluginError::PluginNotFound(path) => write!(f, "Plugin not found: {}", path),
//             PluginError::LoadError(msg) => write!(f, "Failed to load plugin: {}", msg),
//             PluginError::SymbolNotFound(symbol) => write!(f, "Symbol not found: {}", symbol),
//             PluginError::FunctionNotFound(func) => write!(f, "Function not found: {}", func),
//             PluginError::InitializationFailed(msg) => write!(f, "Plugin initialization failed: {}", msg),
//             PluginError::CleanupFailed(msg) => write!(f, "Plugin cleanup failed: {}", msg),
//             PluginError::VersionIncompatible(msg) => write!(f, "Plugin version incompatible: {}", msg),
//             PluginError::SignatureVerificationFailed(msg) => write!(f, "Signature verification failed: {}", msg),
//             PluginError::AlreadyLoaded(name) => write!(f, "Plugin already loaded: {}", name),
//             PluginError::NotLoaded(name) => write!(f, "Plugin not loaded: {}", name),
//             PluginError::SandboxViolation(msg) => write!(f, "Sandbox violation: {}", msg),
//             PluginError::SecurityViolation(msg) => write!(f, "Security violation: {}", msg),
//             PluginError::DependencyMissing(dep) => write!(f, "Missing dependency: {}", dep),
//             PluginError::Timeout(msg) => write!(f, "Plugin operation timeout: {}", msg),
//             PluginError::RegistryError(msg) => write!(f, "Plugin registry error: {}", msg),
//             PluginError::ManagerError(msg) => write!(f, "Plugin manager error: {}", msg),
//             PluginError::HookError(msg) => write!(f, "Plugin hook error: {}", msg),
//             PluginError::DistributionError(msg) => write!(f, "Plugin distribution error: {}", msg),
//             PluginError::General(msg) => write!(f, "Plugin error: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for PluginError {}
// 
// impl From<PluginError> for CursedError {
//     fn from(err: PluginError) -> Self {
//         CursedError::Runtime(format!("Plugin error: {}", err))
//     }
// }

/// Result type for plugin operations
pub type PluginResult<T> = std::result::Result<T, PluginError>;

/// Helper functions for creating specific plugin errors
impl PluginError {
    pub fn plugin_not_found(path: &str) -> Self {
        PluginError::PluginNotFound(path.to_string())
    }

    pub fn load_error(msg: &str) -> Self {
        PluginError::LoadError(msg.to_string())
    }

    pub fn symbol_not_found(symbol: &str) -> Self {
        PluginError::SymbolNotFound(symbol.to_string())
    }

    pub fn function_not_found(func: &str) -> Self {
        PluginError::FunctionNotFound(func.to_string())
    }

    pub fn initialization_failed(msg: &str) -> Self {
        PluginError::InitializationFailed(msg.to_string())
    }

    pub fn cleanup_failed(msg: &str) -> Self {
        PluginError::CleanupFailed(msg.to_string())
    }

    pub fn version_incompatible(msg: &str) -> Self {
        PluginError::VersionIncompatible(msg.to_string())
    }

    pub fn signature_verification_failed(msg: &str) -> Self {
        PluginError::SignatureVerificationFailed(msg.to_string())
    }

    pub fn already_loaded(name: &str) -> Self {
        PluginError::AlreadyLoaded(name.to_string())
    }

    pub fn not_loaded(name: &str) -> Self {
        PluginError::NotLoaded(name.to_string())
    }

    pub fn sandbox_violation(msg: &str) -> Self {
        PluginError::SandboxViolation(msg.to_string())
    }

    pub fn security_violation(msg: &str) -> Self {
        PluginError::SecurityViolation(msg.to_string())
    }

    pub fn dependency_missing(dep: &str) -> Self {
        PluginError::DependencyMissing(dep.to_string())
    }

    pub fn timeout(msg: &str) -> Self {
        PluginError::Timeout(msg.to_string())
    }

    pub fn registry_error(msg: &str) -> Self {
        PluginError::RegistryError(msg.to_string())
    }

    pub fn manager_error(msg: &str) -> Self {
        PluginError::ManagerError(msg.to_string())
    }

    pub fn hook_error(msg: &str) -> Self {
        PluginError::HookError(msg.to_string())
    }

    pub fn distribution_error(msg: &str) -> Self {
        PluginError::DistributionError(msg.to_string())
    }

    pub fn general(msg: &str) -> Self {
        PluginError::General(msg.to_string())
    }
}
