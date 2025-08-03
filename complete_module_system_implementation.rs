// Complete CURSED Module System Implementation
// This implements package declaration, module imports, dependency resolution,
// circular dependency detection, and module caching with versioning

use crate::error::{CursedError, Result};
use crate::ast::{Program, ImportStatement, PackageDeclaration};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{SystemTime, Duration};
use semver::{Version, VersionReq};
use std::fs;

/// Complete package and module system for CURSED
#[derive(Debug)]
pub struct CursedModuleSystem {
    /// Package registry for tracking installed packages
    package_registry: Arc<RwLock<PackageRegistry>>,
    /// Module loader for loading and caching modules
    module_loader: Arc<Mutex<ModuleLoader>>,
    /// Dependency resolver for handling complex dependency graphs
    dependency_resolver: DependencyResolver,
    /// Configuration for the module system
    config: ModuleSystemConfig,
}

/// Configuration for the module system
#[derive(Debug, Clone)]
pub struct ModuleSystemConfig {
    /// Search paths for modules
    pub search_paths: Vec<PathBuf>,
    /// Standard library path
    pub stdlib_path: PathBuf,
    /// Package cache directory
    pub package_cache_dir: PathBuf,
    /// Maximum circular dependency detection depth
    pub max_dependency_depth: usize,
    /// Enable module caching
    pub enable_caching: bool,
    /// Enable package versioning
    pub enable_versioning: bool,
    /// Module load timeout in seconds
    pub module_load_timeout: u64,
}

impl Default for ModuleSystemConfig {
    fn default() -> Self {
        Self {
            search_paths: vec![PathBuf::from(".")],
            stdlib_path: PathBuf::from("stdlib"),
            package_cache_dir: PathBuf::from(".cursed/packages"),
            max_dependency_depth: 100,
            enable_caching: true,
            enable_versioning: true,
            module_load_timeout: 30,
        }
    }
}

/// Package registry for managing installed packages
#[derive(Debug, Default)]
pub struct PackageRegistry {
    /// Map of package name to package information
    packages: HashMap<String, PackageInfo>,
    /// Package dependencies graph
    dependency_graph: HashMap<String, Vec<PackageDependency>>,
    /// Namespace mappings
    namespaces: HashMap<String, String>,
}

/// Information about an installed package
#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: Version,
    pub path: PathBuf,
    pub main_module: Option<PathBuf>,
    pub dependencies: Vec<PackageDependency>,
    pub namespace: Option<String>,
    pub install_time: SystemTime,
}

/// Package dependency information
#[derive(Debug, Clone)]
pub struct PackageDependency {
    pub name: String,
    pub version_requirement: VersionReq,
    pub resolved_version: Option<Version>,
    pub optional: bool,
}

/// Module loader with caching and compilation
#[derive(Debug)]
pub struct ModuleLoader {
    /// Cache of loaded modules
    module_cache: HashMap<PathBuf, Arc<LoadedModule>>,
    /// Module compilation status
    compilation_status: HashMap<PathBuf, ModuleStatus>,
    /// Load statistics
    stats: LoaderStats,
}

/// A loaded and cached module
#[derive(Debug)]
pub struct LoadedModule {
    pub name: String,
    pub path: PathBuf,
    pub package: Option<PackageDeclaration>,
    pub imports: Vec<ImportStatement>,
    pub program: Arc<Program>,
    pub exported_symbols: Vec<String>,
    pub dependencies: Vec<String>,
    pub load_time: SystemTime,
    pub version: Option<Version>,
    pub checksum: String,
}

/// Module loading status
#[derive(Debug, Clone, PartialEq)]
pub enum ModuleStatus {
    NotLoaded,
    Loading,
    Loaded,
    Failed(String),
}

/// Module loader statistics
#[derive(Debug, Default)]
pub struct LoaderStats {
    pub modules_loaded: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub compilation_errors: usize,
    pub circular_dependencies_detected: usize,
}

/// Dependency resolver for handling complex dependency graphs
#[derive(Debug)]
pub struct DependencyResolver {
    /// Dependency resolution cache
    resolution_cache: HashMap<String, ResolvedDependencies>,
    /// Circular dependency detection state
    resolution_stack: Vec<String>,
    /// Configuration
    config: DependencyResolverConfig,
}

/// Configuration for dependency resolver
#[derive(Debug, Clone)]
pub struct DependencyResolverConfig {
    pub max_resolution_depth: usize,
    pub allow_circular_dependencies: bool,
    pub strict_version_matching: bool,
    pub enable_resolution_caching: bool,
}

impl Default for DependencyResolverConfig {
    fn default() -> Self {
        Self {
            max_resolution_depth: 100,
            allow_circular_dependencies: false,
            strict_version_matching: false,
            enable_resolution_caching: true,
        }
    }
}

/// Resolved dependencies for a module
#[derive(Debug, Clone)]
pub struct ResolvedDependencies {
    pub direct_dependencies: Vec<ResolvedModule>,
    pub transitive_dependencies: Vec<ResolvedModule>,
    pub resolution_order: Vec<String>,
    pub circular_dependencies: Vec<Vec<String>>,
}

/// A resolved module with full path and version information
#[derive(Debug, Clone)]
pub struct ResolvedModule {
    pub name: String,
    pub path: PathBuf,
    pub version: Option<Version>,
    pub package: Option<String>,
    pub namespace: Option<String>,
}

impl CursedModuleSystem {
    /// Create a new module system with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(ModuleSystemConfig::default())
    }

    /// Create a new module system with custom configuration
    pub fn with_config(config: ModuleSystemConfig) -> Result<Self> {
        let package_registry = Arc::new(RwLock::new(PackageRegistry::default()));
        let module_loader = Arc::new(Mutex::new(ModuleLoader::new()));
        
        let dependency_resolver = DependencyResolver::new(DependencyResolverConfig::default())?;

        Ok(Self {
            package_registry,
            module_loader,
            dependency_resolver,
            config,
        })
    }

    /// Declare a package for the current module
    pub fn declare_package(&mut self, package_name: &str, version: Option<&str>) -> Result<()> {
        let mut registry = self.package_registry.write()
            .map_err(|_| CursedError::ImportError("Failed to acquire package registry lock".to_string()))?;

        let version = if let Some(v) = version {
            Version::parse(v)
                .map_err(|e| CursedError::ImportError(format!("Invalid version: {}", e)))?
        } else {
            Version::new(0, 1, 0)
        };

        let package_info = PackageInfo {
            name: package_name.to_string(),
            version,
            path: self.config.search_paths[0].clone(),
            main_module: None,
            dependencies: Vec::new(),
            namespace: Some(package_name.to_string()),
            install_time: SystemTime::now(),
        };

        registry.packages.insert(package_name.to_string(), package_info);
        registry.namespaces.insert(package_name.to_string(), package_name.to_string());

        Ok(())
    }

    /// Load and resolve a module with all its dependencies
    pub async fn load_module(&mut self, module_path: &str) -> Result<Arc<LoadedModule>> {
        // Start dependency resolution
        let resolved_deps = self.dependency_resolver
            .resolve_dependencies(module_path, &self.config)
            .await?;

        // Load modules in dependency order
        let mut loaded_modules = HashMap::new();
        
        for module_name in &resolved_deps.resolution_order {
            if let Some(resolved_module) = resolved_deps.direct_dependencies
                .iter()
                .chain(resolved_deps.transitive_dependencies.iter())
                .find(|m| &m.name == module_name) {
                
                let loaded_module = self.load_single_module(&resolved_module.path).await?;
                loaded_modules.insert(module_name.clone(), loaded_module);
            }
        }

        // Return the main module
        if let Some(main_module) = loaded_modules.get(module_path) {
            Ok(Arc::clone(main_module))
        } else {
            Err(CursedError::ImportError(format!("Failed to load main module: {}", module_path)))
        }
    }

    /// Load a single module file
    async fn load_single_module(&mut self, module_path: &Path) -> Result<Arc<LoadedModule>> {
        let mut loader = self.module_loader.lock()
            .map_err(|_| CursedError::ImportError("Failed to acquire module loader lock".to_string()))?;

        // Check cache first
        if self.config.enable_caching {
            if let Some(cached_module) = loader.module_cache.get(module_path) {
                loader.stats.cache_hits += 1;
                return Ok(Arc::clone(cached_module));
            }
        }

        loader.stats.cache_misses += 1;

        // Check if already loading (circular dependency)
        if loader.compilation_status.get(module_path) == Some(&ModuleStatus::Loading) {
            return Err(CursedError::ImportError(format!(
                "Circular dependency detected while loading: {}", 
                module_path.display()
            )));
        }

        // Mark as loading
        loader.compilation_status.insert(module_path.to_path_buf(), ModuleStatus::Loading);

        // Load and compile the module
        let result = self.compile_module(module_path).await;

        match result {
            Ok(module) => {
                let arc_module = Arc::new(module);
                
                // Cache the loaded module
                if self.config.enable_caching {
                    loader.module_cache.insert(module_path.to_path_buf(), Arc::clone(&arc_module));
                }
                
                loader.compilation_status.insert(module_path.to_path_buf(), ModuleStatus::Loaded);
                loader.stats.modules_loaded += 1;
                
                Ok(arc_module)
            }
            Err(e) => {
                loader.compilation_status.insert(
                    module_path.to_path_buf(), 
                    ModuleStatus::Failed(e.to_string())
                );
                loader.stats.compilation_errors += 1;
                Err(e)
            }
        }
    }

    /// Compile a module from source
    async fn compile_module(&self, module_path: &Path) -> Result<LoadedModule> {
        // Read source file with timeout
        let source = self.read_file_with_timeout(module_path, 
            Duration::from_secs(self.config.module_load_timeout)).await?;

        // Calculate checksum
        let checksum = self.calculate_checksum(&source);

        // Parse the module
        let mut lexer = crate::lexer::Lexer::new(source.clone());
        let tokens = lexer.tokenize()
            .map_err(|e| CursedError::ImportError(format!("Lexer error in {}: {}", module_path.display(), e)))?;

        let mut parser = crate::parser::Parser::from_tokens(tokens);
        let program = parser.parse_program()
            .map_err(|e| CursedError::ImportError(format!("Parse error in {}: {}", module_path.display(), e)))?;

        // Extract module information
        let module_name = module_path.file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let package = program.package.clone();
        let imports = program.imports.clone();
        let exported_symbols = self.extract_exported_symbols(&program);
        let dependencies = imports.iter().map(|imp| imp.path.clone()).collect();

        Ok(LoadedModule {
            name: module_name,
            path: module_path.to_path_buf(),
            package,
            imports,
            program: Arc::new(program),
            exported_symbols,
            dependencies,
            load_time: SystemTime::now(),
            version: None, // TODO: Extract from package declaration
            checksum,
        })
    }

    /// Read a file with timeout
    async fn read_file_with_timeout(&self, path: &Path, timeout: Duration) -> Result<String> {
        let path = path.to_path_buf();
        
        tokio::time::timeout(timeout, async move {
            tokio::fs::read_to_string(&path).await
                .map_err(|e| CursedError::ImportError(format!("Failed to read {}: {}", path.display(), e)))
        }).await
        .map_err(|_| CursedError::ImportError(format!("Timeout reading file: {}", path.display())))?
    }

    /// Calculate file checksum for change detection
    fn calculate_checksum(&self, content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Extract exported symbols from a program
    fn extract_exported_symbols(&self, program: &Program) -> Vec<String> {
        use crate::ast::{Statement, Visibility};
        
        let mut symbols = Vec::new();
        
        for statement in &program.statements {
            match statement {
                Statement::Function(func) => {
                    if func.visibility == Visibility::Public {
                        symbols.push(func.name.clone());
                    }
                }
                Statement::Let(let_stmt) => {
                    if let_stmt.visibility == Visibility::Public {
                        symbols.push(let_stmt.target.primary_name());
                    }
                }
                _ => {}
            }
        }
        
        symbols.sort();
        symbols.dedup();
        symbols
    }

    /// Install a package from a registry or local path
    pub async fn install_package(&mut self, package_name: &str, version_req: Option<&str>) -> Result<()> {
        let mut registry = self.package_registry.write()
            .map_err(|_| CursedError::ImportError("Failed to acquire package registry lock".to_string()))?;

        // Check if package is already installed
        if let Some(existing_package) = registry.packages.get(package_name) {
            if let Some(req_str) = version_req {
                let req = VersionReq::parse(req_str)
                    .map_err(|e| CursedError::ImportError(format!("Invalid version requirement: {}", e)))?;
                
                if req.matches(&existing_package.version) {
                    return Ok(()); // Already satisfied
                }
            } else {
                return Ok(()); // Any version is fine
            }
        }

        // For now, simulate package installation
        // In a real implementation, this would download from a registry
        let version = Version::new(1, 0, 0);
        let package_path = self.config.package_cache_dir.join(package_name);

        let package_info = PackageInfo {
            name: package_name.to_string(),
            version,
            path: package_path,
            main_module: None,
            dependencies: Vec::new(),
            namespace: Some(package_name.to_string()),
            install_time: SystemTime::now(),
        };

        registry.packages.insert(package_name.to_string(), package_info);
        
        Ok(())
    }

    /// Get module loading statistics
    pub fn get_stats(&self) -> Result<LoaderStats> {
        let loader = self.module_loader.lock()
            .map_err(|_| CursedError::ImportError("Failed to acquire module loader lock".to_string()))?;
        Ok(loader.stats.clone())
    }

    /// Clear module cache
    pub fn clear_cache(&mut self) -> Result<()> {
        let mut loader = self.module_loader.lock()
            .map_err(|_| CursedError::ImportError("Failed to acquire module loader lock".to_string()))?;
        
        loader.module_cache.clear();
        loader.compilation_status.clear();
        
        Ok(())
    }

    /// Validate package dependencies
    pub async fn validate_dependencies(&self, package_name: &str) -> Result<Vec<String>> {
        let registry = self.package_registry.read()
            .map_err(|_| CursedError::ImportError("Failed to acquire package registry lock".to_string()))?;

        let mut missing_deps = Vec::new();

        if let Some(package) = registry.packages.get(package_name) {
            for dep in &package.dependencies {
                if !registry.packages.contains_key(&dep.name) {
                    missing_deps.push(dep.name.clone());
                }
            }
        }

        Ok(missing_deps)
    }
}

impl ModuleLoader {
    fn new() -> Self {
        Self {
            module_cache: HashMap::new(),
            compilation_status: HashMap::new(),
            stats: LoaderStats::default(),
        }
    }
}

impl DependencyResolver {
    fn new(config: DependencyResolverConfig) -> Result<Self> {
        Ok(Self {
            resolution_cache: HashMap::new(),
            resolution_stack: Vec::new(),
            config,
        })
    }

    /// Resolve all dependencies for a module
    async fn resolve_dependencies(
        &mut self, 
        module_path: &str, 
        module_config: &ModuleSystemConfig
    ) -> Result<ResolvedDependencies> {
        // Check cache first
        if self.config.enable_resolution_caching {
            if let Some(cached) = self.resolution_cache.get(module_path) {
                return Ok(cached.clone());
            }
        }

        // Check for circular dependencies
        if self.resolution_stack.contains(&module_path.to_string()) {
            let cycle_start = self.resolution_stack.iter()
                .position(|p| p == module_path)
                .unwrap_or(0);
            let cycle = self.resolution_stack[cycle_start..].to_vec();
            
            if !self.config.allow_circular_dependencies {
                return Err(CursedError::ImportError(format!(
                    "Circular dependency detected: {}", 
                    cycle.join(" -> ")
                )));
            }
        }

        // Check resolution depth
        if self.resolution_stack.len() > self.config.max_resolution_depth {
            return Err(CursedError::ImportError(format!(
                "Maximum dependency resolution depth exceeded: {}", 
                self.config.max_resolution_depth
            )));
        }

        self.resolution_stack.push(module_path.to_string());

        // Resolve dependencies using breadth-first search
        let result = self.resolve_dependencies_bfs(module_path, module_config).await;

        self.resolution_stack.pop();

        let resolved = result?;

        // Cache the result
        if self.config.enable_resolution_caching {
            self.resolution_cache.insert(module_path.to_string(), resolved.clone());
        }

        Ok(resolved)
    }

    /// Breadth-first search dependency resolution
    async fn resolve_dependencies_bfs(
        &self, 
        root_module: &str, 
        module_config: &ModuleSystemConfig
    ) -> Result<ResolvedDependencies> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut direct_dependencies = Vec::new();
        let mut transitive_dependencies = Vec::new();
        let mut resolution_order = Vec::new();
        let mut circular_dependencies = Vec::new();

        queue.push_back((root_module.to_string(), 0)); // (module, depth)

        while let Some((current_module, depth)) = queue.pop_front() {
            if visited.contains(&current_module) {
                continue;
            }

            visited.insert(current_module.clone());
            resolution_order.push(current_module.clone());

            // Find the module file
            let module_path = self.find_module_path(&current_module, module_config)?;
            let resolved_module = ResolvedModule {
                name: current_module.clone(),
                path: module_path.clone(),
                version: None,
                package: None,
                namespace: None,
            };

            if depth == 0 {
                // This is the root module, don't add to dependencies
            } else if depth == 1 {
                direct_dependencies.push(resolved_module);
            } else {
                transitive_dependencies.push(resolved_module);
            }

            // Get dependencies of this module
            if let Ok(deps) = self.get_module_dependencies(&module_path).await {
                for dep in deps {
                    if !visited.contains(&dep) {
                        queue.push_back((dep, depth + 1));
                    } else if depth > 0 {
                        // Potential circular dependency
                        circular_dependencies.push(vec![current_module.clone(), dep]);
                    }
                }
            }
        }

        Ok(ResolvedDependencies {
            direct_dependencies,
            transitive_dependencies,
            resolution_order,
            circular_dependencies,
        })
    }

    /// Find the path to a module
    fn find_module_path(&self, module_name: &str, config: &ModuleSystemConfig) -> Result<PathBuf> {
        // Try search paths
        for search_path in &config.search_paths {
            let candidates = vec![
                search_path.join(format!("{}.csd", module_name)),
                search_path.join(module_name).join("mod.csd"),
                search_path.join(module_name).join("lib.csd"),
            ];

            for candidate in candidates {
                if candidate.exists() {
                    return Ok(candidate);
                }
            }
        }

        // Try stdlib
        let stdlib_candidates = vec![
            config.stdlib_path.join(format!("{}.csd", module_name)),
            config.stdlib_path.join(module_name).join("mod.csd"),
        ];

        for candidate in stdlib_candidates {
            if candidate.exists() {
                return Ok(candidate);
            }
        }

        Err(CursedError::ImportError(format!("Module not found: {}", module_name)))
    }

    /// Get dependencies of a module by parsing its import statements
    async fn get_module_dependencies(&self, module_path: &Path) -> Result<Vec<String>> {
        let content = fs::read_to_string(module_path)
            .map_err(|e| CursedError::ImportError(format!("Failed to read {}: {}", module_path.display(), e)))?;

        let mut dependencies = Vec::new();

        // Simple regex-based dependency extraction
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("yeet ") {
                if let Some(start_quote) = line.find('"') {
                    if let Some(end_quote) = line.rfind('"') {
                        if start_quote < end_quote {
                            let import_path = &line[start_quote + 1..end_quote];
                            if !import_path.is_empty() {
                                dependencies.push(import_path.to_string());
                            }
                        }
                    }
                }
            }
        }

        Ok(dependencies)
    }
}

impl Default for LoaderStats {
    fn default() -> Self {
        Self {
            modules_loaded: 0,
            cache_hits: 0,
            cache_misses: 0,
            compilation_errors: 0,
            circular_dependencies_detected: 0,
        }
    }
}

impl Clone for LoaderStats {
    fn clone(&self) -> Self {
        Self {
            modules_loaded: self.modules_loaded,
            cache_hits: self.cache_hits,
            cache_misses: self.cache_misses,
            compilation_errors: self.compilation_errors,
            circular_dependencies_detected: self.circular_dependencies_detected,
        }
    }
}
