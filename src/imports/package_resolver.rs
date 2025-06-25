use crate::error::Error;
// Package import resolution

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

// use crate::package_manager::{PackageManager, PackageMetadata};
use super::{ImportError, ResolvedImport, ImportSource};

/// Package-specific import resolution
#[derive(Debug)]
pub struct PackageImportResolver {
    package_manager: Arc<Mutex<PackageManager>>,
    package_exports: HashMap<String, PackageExportInfo>,
}

/// Information about what a package exports
#[derive(Debug, Clone)]
pub struct PackageExportInfo {
    pub package_name: String,
    pub version: String,
    pub modules: HashMap<String, ModuleExportInfo>,
}

/// Information about what a module within a package exports
#[derive(Debug, Clone)]
pub struct ModuleExportInfo {
    pub module_path: String,
    pub functions: Vec<String>,
    pub types: Vec<String>,
    pub constants: Vec<String>,
}

/// Result of package resolution
#[derive(Debug, Clone)]
pub struct PackageResolution {
    pub package_metadata: PackageMetadata,
    pub module_path: String,
    pub resolved_import: ResolvedImport,
}

impl PackageImportResolver {
    /// Create new package import resolver
    pub fn new(package_manager: Arc<Mutex<PackageManager>>) -> Self {
        let mut resolver = Self {
            package_manager,
            package_exports: HashMap::new(),
        };
        
        resolver.init_known_package_exports();
        resolver
    }
    
    /// Initialize known package export information
    fn init_known_package_exports(&mut self) {
        // cursed-http package exports
        let mut http_modules = HashMap::new();
        http_modules.insert("client".to_string(), ModuleExportInfo {
            module_path: "client".to_string(),
            functions: vec![
                "get".to_string(),
                "post".to_string(),
                "put".to_string(),
                "delete".to_string(),
                "request".to_string(),
            ],
            types: vec![
                "HttpClient".to_string(),
                "Response".to_string(),
                "Request".to_string(),
            ],
            constants: vec![],
        });
        
        http_modules.insert("server".to_string(), ModuleExportInfo {
            module_path: "server".to_string(),
            functions: vec![
                "serve".to_string(),
                "route".to_string(),
                "middleware".to_string(),
            ],
            types: vec![
                "HttpServer".to_string(),
                "Handler".to_string(),
                "Middleware".to_string(),
            ],
            constants: vec![],
        });
        
        self.package_exports.insert("cursed-http".to_string(), PackageExportInfo {
            package_name: "cursed-http".to_string(),
            version: "1.0.0".to_string(),
            modules: http_modules,
        });
        
        // cursed-json package exports
        let mut json_modules = HashMap::new();
        json_modules.insert("parse".to_string(), ModuleExportInfo {
            module_path: "parse".to_string(),
            functions: vec![
                "parse".to_string(),
                "stringify".to_string(),
                "validate".to_string(),
            ],
            types: vec![
                "JsonValue".to_string(),
                "JsonObject".to_string(),
                "JsonArray".to_string(),
            ],
            constants: vec![],
        });
        
        self.package_exports.insert("cursed-json".to_string(), PackageExportInfo {
            package_name: "cursed-json".to_string(),
            version: "2.1.0".to_string(),
            modules: json_modules,
        });
        
        // cursed-db package exports
        let mut db_modules = HashMap::new();
        db_modules.insert("sql".to_string(), ModuleExportInfo {
            module_path: "sql".to_string(),
            functions: vec![
                "connect".to_string(),
                "query".to_string(),
                "execute".to_string(),
                "transaction".to_string(),
            ],
            types: vec![
                "Connection".to_string(),
                "QueryResult".to_string(),
                "Transaction".to_string(),
            ],
            constants: vec![],
        });
        
        self.package_exports.insert("cursed-db".to_string(), PackageExportInfo {
            package_name: "cursed-db".to_string(),
            version: "1.2.0".to_string(),
            modules: db_modules,
        });
    }
    
    /// Resolve package import (e.g., "cursed-http::client")
    pub async fn resolve_package_import(&self, import_path: &str) -> Result<(), Error> {
        // Parse package import path
        let parts: Vec<&str> = import_path.split("::").collect();
        if parts.len() < 2 {
            return Err(ImportError::InvalidPath {
                path: import_path.to_string(),
                reason: "Package import must have at least package::module format".to_string(),
            });
        }
        
        let package_name = parts[0];
        let module_path = parts[1..].join("::");
        
        // Ensure package is installed
        let package_metadata = self.ensure_package_available(package_name).await?;
        
        // Get package export information
        let package_exports = self.package_exports.get(package_name)
            .ok_or_else(|| ImportError::NotFound {
                import_path: import_path.to_string(),
            })?;
        
        // Find the specific module
        let module_info = package_exports.modules.get(&module_path)
            .or_else(|| {
                // Try finding by the last part of the path
                let module_name = parts.last().unwrap();
                package_exports.modules.get(*module_name)
            })
            .ok_or_else(|| ImportError::NotFound {
                import_path: format!("{}::{}", package_name, module_path),
            })?;
        
        // Build resolved path to the package module
        let cache_dir = dirs::cache_dir().unwrap_or_default().join("cursed/packages");
        let package_dir = cache_dir.join(&package_name).join(&package_metadata.version);
        let module_file_path = package_dir.join("src").join(&module_info.module_path).with_extension("csd");
        
        // Combine exports
        let mut all_exports = module_info.functions.clone();
        all_exports.extend(module_info.constants.clone());
        
        Ok(ResolvedImport {
            original_path: import_path.to_string(),
            source: ImportSource::InstalledPackage {
                package_name: package_name.to_string(),
            },
            resolved_path: module_file_path,
            alias: None,
            exports: all_exports,
            types: module_info.types.clone(),
        })
    }
    
    /// Ensure package is available for import
    async fn ensure_package_available(&self, package_name: &str) -> Result<(), Error> {
        let mut package_manager = self.package_manager.lock().map_err(|_| {
            ImportError::ModuleLoadError {
                module: package_name.to_string(),
                error: "Failed to lock package manager".to_string(),
            }
        })?;
        
        // Check if package is already installed
        let installed = package_manager.list_installed()?;
        if let Some(metadata) = installed.iter().find(|p| p.name == package_name) {
            return Ok(metadata.clone());
        }
        
        // Package not installed, try to install it
        tracing::info!(package = package_name, "Package not found, attempting to install");
        
        let installed_packages = package_manager.install_package(package_name, None).await?;
        
        installed_packages.into_iter()
            .find(|p| p.name == package_name)
            .ok_or_else(|| ImportError::PackageNotInstalled {
                package: package_name.to_string(),
            })
    }
    
    /// Update package export information from installed packages
    pub async fn update_package_exports(&mut self) -> Result<(), Error> {
        let installed = {
            let package_manager = self.package_manager.lock().map_err(|_| {
                ImportError::ModuleLoadError {
                    module: "package_manager".to_string(),
                    error: "Failed to lock package manager".to_string(),
                }
            })?;
            package_manager.list_installed()?
        };
        
        for package in installed {
            if !self.package_exports.contains_key(&package.name) {
                // Try to discover exports for unknown packages
                self.discover_package_exports(&package)?;
            }
        }
        
        Ok(())
    }
    
    /// Discover exports for an unknown package
    fn discover_package_exports(&mut self, package: &PackageMetadata) -> Result<(), Error> {
        // In a real implementation, this would parse the package files
        // For now, create a basic export info
        let mut modules = HashMap::new();
        modules.insert("main".to_string(), ModuleExportInfo {
            module_path: "main".to_string(),
            functions: vec!["main".to_string()],
            types: vec![],
            constants: vec![],
        });
        
        let export_info = PackageExportInfo {
            package_name: package.name.clone(),
            version: package.version.clone(),
            modules,
        };
        
        self.package_exports.insert(package.name.clone(), export_info);
        
        tracing::info!(package = package.name, "Discovered basic exports for package");
        Ok(())
    }
    
    /// Get available packages for completion/suggestions
    pub fn get_available_packages(&self) -> Vec<String> {
        self.package_exports.keys().cloned().collect()
    }
    
    /// Get available modules for a package
    pub fn get_package_modules(&self, package_name: &str) -> Option<Vec<String>> {
        self.package_exports.get(package_name)
            .map(|exports| exports.modules.keys().cloned().collect())
    }
    
    /// Get exports for a specific package module
    pub fn get_module_exports(&self, package_name: &str, module_name: &str) -> Option<&ModuleExportInfo> {
        self.package_exports.get(package_name)
            .and_then(|exports| exports.modules.get(module_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package_manager::PackageManagerConfig;
    
    #[tokio::test]
    async fn test_package_import_resolution() {
        let config = PackageManagerConfig::default();
        let package_manager = Arc::new(Mutex::new(
            PackageManager::new(config).unwrap()
        ));
        
        let resolver = PackageImportResolver::new(package_manager);
        
        // This would require the package to be actually installed
        // For testing, we rely on the known exports
        assert!(resolver.package_exports.contains_key("cursed-http"));
        assert!(resolver.get_package_modules("cursed-http").unwrap().contains(&"client".to_string()));
    }
}
