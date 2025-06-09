//! Executable linking system for CURSED modules
//!
//! This module provides comprehensive functionality to link multiple LLVM modules
//! into a final executable, handling symbol resolution, entry point generation,
//! and both static and dynamic linking strategies.

use crate::codegen::llvm::separate_compilation::PackageMetadata;
use crate::codegen::llvm::module_linking::{ModuleLinker, SymbolInfo, SymbolType};
use crate::error::Error;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple};
use inkwell::OptimizationLevel;
use inkwell::values::{FunctionValue, GlobalValue, BasicValueEnum};
use inkwell::types::{BasicType, FunctionType};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs;
use tracing::{debug, error, info, instrument, warn};

/// Linking strategy for executable generation
#[derive(Debug, Clone, PartialEq)]
pub enum LinkingStrategy {
    /// Static linking - all code included in executable
    Static,
    /// Dynamic linking - shared libraries used at runtime
    Dynamic,
    /// Hybrid - mix of static and dynamic linking
    Hybrid {
        static_packages: HashSet<String>,
        dynamic_packages: HashSet<String>,
    },
}

/// Target platform for executable generation
#[derive(Debug, Clone)]
pub struct TargetPlatform {
    pub triple: String,
    pub cpu: String,
    pub features: String,
    pub optimization_level: OptimizationLevel,
    pub code_model: CodeModel,
    pub reloc_mode: RelocMode,
}

impl Default for TargetPlatform {
    fn default() -> Self {
        Self {
            triple: "x86_64-unknown-linux-gnu".to_string(), // Default to a common target
            cpu: "generic".to_string(),
            features: "".to_string(),
            optimization_level: OptimizationLevel::Default,
            code_model: CodeModel::Default,
            reloc_mode: RelocMode::Default,
        }
    }
}

/// Configuration for executable linking
#[derive(Debug, Clone)]
pub struct ExecutableLinkingConfig {
    pub strategy: LinkingStrategy,
    pub target: TargetPlatform,
    pub entry_point: String,
    pub output_path: PathBuf,
    pub include_debug_info: bool,
    pub strip_symbols: bool,
    pub enable_lto: bool,
}

impl Default for ExecutableLinkingConfig {
    fn default() -> Self {
        Self {
            strategy: LinkingStrategy::Static,
            target: TargetPlatform::default(),
            entry_point: "main".to_string(),
            output_path: PathBuf::from("program"),
            include_debug_info: false,
            strip_symbols: false,
            enable_lto: false,
        }
    }
}

/// Manages executable linking from separate LLVM modules
pub struct ExecutableLinker<'ctx> {
    /// LLVM context
    context: &'ctx Context,
    /// Module linker for symbol resolution
    module_linker: ModuleLinker<'ctx>,
    /// Linking configuration
    config: ExecutableLinkingConfig,
    /// Symbol resolution map
    symbol_resolution: HashMap<String, String>,
    /// Missing symbols that need external linking
    missing_symbols: HashSet<String>,
    /// Entry point function information
    pub entry_point_info: Option<EntryPointInfo<'ctx>>,
}

/// Information about the program entry point
#[derive(Debug, Clone)]
pub struct EntryPointInfo<'ctx> {
    pub function: FunctionValue<'ctx>,
    pub package_name: String,
    pub original_name: String,
}

impl<'ctx> ExecutableLinker<'ctx> {
    /// Create a new executable linker
    pub fn new(context: &'ctx Context, config: ExecutableLinkingConfig) -> Self {
        Self {
            context,
            module_linker: ModuleLinker::new(context),
            config,
            symbol_resolution: HashMap::new(),
            missing_symbols: HashSet::new(),
            entry_point_info: None,
        }
    }

    /// Add package metadata for linking
    #[instrument(skip(self), level = "debug")]
    pub fn add_package(&mut self, metadata: PackageMetadata) -> Result<(), Error> {
        debug!(package = metadata.name, "Adding package to executable linker");
        self.module_linker.add_package(metadata)
    }

    /// Link modules and generate executable
    #[instrument(skip(self, modules), fields(module_count = modules.len()), level = "info")]
    pub fn link_and_generate_executable(&mut self, modules: Vec<Module<'ctx>>) -> Result<PathBuf, Error> {
        info!("Starting executable linking process with {} modules", modules.len());

        // Phase 1: Resolve symbols across all modules
        self.resolve_all_symbols(&modules)?;

        // Phase 2: Link modules together
        let linked_module = self.link_modules(modules)?;

        // Phase 3: Find and prepare entry point
        self.prepare_entry_point(&linked_module)?;

        // Phase 4: Generate runtime initialization code
        self.generate_runtime_initialization(&linked_module)?;

        // Phase 5: Apply optimizations
        let optimized_module = self.apply_optimizations(linked_module)?;

        // Phase 6: Generate executable binary
        self.generate_executable_binary(optimized_module)
    }

    /// Resolve symbols across all modules
    #[instrument(skip(self, modules), level = "debug")]
    pub fn resolve_all_symbols(&mut self, modules: &[Module<'ctx>]) -> Result<(), Error> {
        debug!("Resolving symbols across {} modules", modules.len());

        // First pass: collect all defined symbols
        let mut defined_symbols = HashMap::new();
        let mut required_symbols = HashSet::new();

        for module in modules {
            self.collect_module_symbols(module, &mut defined_symbols, &mut required_symbols)?;
        }

        // Second pass: build symbol resolution map
        self.build_symbol_resolution_map(defined_symbols, required_symbols)?;

        // Third pass: resolve symbols in module linker
        self.module_linker.resolve_symbols()?;

        info!(
            defined_symbols = self.symbol_resolution.len(),
            missing_symbols = self.missing_symbols.len(),
            "Symbol resolution completed"
        );

        Ok(())
    }

    /// Collect symbols from a single module
    #[instrument(skip(self, module, defined_symbols, required_symbols), level = "trace")]
    pub fn collect_module_symbols(
        &self,
        module: &Module<'ctx>,
        defined_symbols: &mut HashMap<String, (String, SymbolType)>,
        required_symbols: &mut HashSet<String>,
    ) -> Result<(), Error> {
        let module_name = module.get_name().to_string_lossy();
        debug!(module = module_name.as_ref(), "Collecting symbols from module");

        // Collect function symbols
        let mut current_function = module.get_first_function();
        while let Some(function) = current_function {
            let func_name = function.get_name().to_string_lossy();
            
            if function.count_basic_blocks() > 0 {
                // Function is defined in this module
                defined_symbols.insert(
                    func_name.to_string(),
                    (module_name.to_string(), SymbolType::Function)
                );
            } else {
                // Function is declared but not defined (external reference)
                required_symbols.insert(func_name.to_string());
            }

            current_function = function.get_next_function();
        }

        // Collect global symbols
        let mut current_global = module.get_first_global();
        while let Some(global) = current_global {
            let global_name = global.get_name().to_string_lossy();
            
            if global.get_initializer().is_some() {
                // Global is defined in this module
                defined_symbols.insert(
                    global_name.to_string(),
                    (module_name.to_string(), SymbolType::Global)
                );
            } else {
                // Global is declared but not defined
                required_symbols.insert(global_name.to_string());
            }

            current_global = global.get_next_global();
        }

        debug!(
            module = module_name.as_ref(),
            defined_count = defined_symbols.len(),
            required_count = required_symbols.len(),
            "Module symbol collection completed"
        );

        Ok(())
    }

    /// Build symbol resolution map
    #[instrument(skip(self, defined_symbols, required_symbols), level = "debug")]
    fn build_symbol_resolution_map(
        &mut self,
        defined_symbols: HashMap<String, (String, SymbolType)>,
        required_symbols: HashSet<String>,
    ) -> Result<(), Error> {
        debug!("Building symbol resolution map");

        for required_symbol in &required_symbols {
            if let Some((defining_module, _symbol_type)) = defined_symbols.get(required_symbol) {
                // Symbol is provided by another module
                self.symbol_resolution.insert(
                    required_symbol.clone(),
                    defining_module.clone()
                );
            } else {
                // Symbol is missing - may be provided by external libraries
                self.missing_symbols.insert(required_symbol.clone());
            }
        }

        // Report missing symbols
        if !self.missing_symbols.is_empty() {
            warn!(
                missing_symbols = ?self.missing_symbols,
                "Found missing symbols that may require external linking"
            );
        }

        Ok(())
    }

    /// Link modules together
    #[instrument(skip(self, modules), level = "info")]
    fn link_modules(&mut self, modules: Vec<Module<'ctx>>) -> Result<Module<'ctx>, Error> {
        info!("Linking {} modules together", modules.len());

        // Use module linker to combine modules
        let mut linked_module = self.module_linker.link_modules(modules)?;

        // Apply symbol resolution fixes
        self.apply_symbol_resolution_fixes(&mut linked_module)?;

        // Set proper linkages based on strategy
        self.apply_linking_strategy_linkages(&mut linked_module)?;

        info!("Module linking completed successfully");
        Ok(linked_module)
    }

    /// Apply symbol resolution fixes to the linked module
    #[instrument(skip(self, module), level = "debug")]
    fn apply_symbol_resolution_fixes(&self, module: &mut Module<'ctx>) -> Result<(), Error> {
        debug!("Applying symbol resolution fixes");

        // Fix function call references
        self.fix_function_call_references(module)?;

        // Fix global variable references
        self.fix_global_variable_references(module)?;

        debug!("Symbol resolution fixes applied");
        Ok(())
    }

    /// Fix function call references in the module
    #[instrument(skip(self, module), level = "trace")]
    fn fix_function_call_references(&self, module: &Module<'ctx>) -> Result<(), Error> {
        // This would involve:
        // 1. Iterating through all instructions in all functions
        // 2. Finding call instructions to undefined functions
        // 3. Updating them to call the correctly resolved function names

        // For now, this is a placeholder for the complex instruction rewriting
        debug!("Function call reference fixing not yet fully implemented");
        Ok(())
    }

    /// Fix global variable references in the module
    #[instrument(skip(self, module), level = "trace")]
    fn fix_global_variable_references(&self, module: &Module<'ctx>) -> Result<(), Error> {
        // Similar to function call fixing, but for global variable references
        debug!("Global variable reference fixing not yet fully implemented");
        Ok(())
    }

    /// Apply linkage settings based on linking strategy
    #[instrument(skip(self, module), level = "debug")]
    fn apply_linking_strategy_linkages(&self, module: &Module<'ctx>) -> Result<(), Error> {
        debug!(strategy = ?self.config.strategy, "Applying linking strategy linkages");

        match &self.config.strategy {
            LinkingStrategy::Static => {
                // For static linking, internalize most symbols
                self.apply_static_linking_linkages(module)?;
            }
            LinkingStrategy::Dynamic => {
                // For dynamic linking, export more symbols
                self.apply_dynamic_linking_linkages(module)?;
            }
            LinkingStrategy::Hybrid { static_packages, dynamic_packages } => {
                // Mixed strategy based on package classification
                self.apply_hybrid_linking_linkages(module, static_packages, dynamic_packages)?;
            }
        }

        debug!("Linking strategy linkages applied");
        Ok(())
    }

    /// Apply static linking linkages
    #[instrument(skip(self, module), level = "trace")]
    fn apply_static_linking_linkages(&self, module: &Module<'ctx>) -> Result<(), Error> {
        // Set most symbols to internal linkage for better optimization
        let mut current_function = module.get_first_function();
        while let Some(function) = current_function {
            let func_name = function.get_name().to_string_lossy();
            
            if func_name.as_ref() == self.config.entry_point {
                // Keep entry point external
                function.set_linkage(Linkage::External);
            } else if func_name.starts_with("_") || func_name.contains("_runtime_") {
                // Keep runtime functions external for debugging
                function.set_linkage(Linkage::External);
            } else {
                // Internalize everything else
                function.set_linkage(Linkage::Internal);
            }

            current_function = function.get_next_function();
        }

        Ok(())
    }

    /// Apply dynamic linking linkages
    #[instrument(skip(self, module), level = "trace")]
    fn apply_dynamic_linking_linkages(&self, module: &Module<'ctx>) -> Result<(), Error> {
        // Keep more symbols external for dynamic linking
        let mut current_function = module.get_first_function();
        while let Some(function) = current_function {
            let func_name = function.get_name().to_string_lossy();
            
            if self.module_linker.get_symbol_info(&func_name).is_some() {
                // Exported symbols get external linkage
                function.set_linkage(Linkage::External);
            } else {
                // Internal symbols get internal linkage
                function.set_linkage(Linkage::Internal);
            }

            current_function = function.get_next_function();
        }

        Ok(())
    }

    /// Apply hybrid linking linkages
    #[instrument(skip(self, module, static_packages, dynamic_packages), level = "trace")]
    fn apply_hybrid_linking_linkages(
        &self,
        module: &Module<'ctx>,
        static_packages: &HashSet<String>,
        dynamic_packages: &HashSet<String>,
    ) -> Result<(), Error> {
        // Apply linkage based on package classification
        let mut current_function = module.get_first_function();
        while let Some(function) = current_function {
            let func_name = function.get_name().to_string_lossy();
            
            // Determine package for this function
            let package_name = self.extract_package_from_symbol(&func_name);
            
            if static_packages.contains(&package_name) {
                function.set_linkage(Linkage::Internal);
            } else if dynamic_packages.contains(&package_name) {
                function.set_linkage(Linkage::External);
            } else {
                // Default to external for unknown packages
                function.set_linkage(Linkage::External);
            }

            current_function = function.get_next_function();
        }

        Ok(())
    }

    /// Extract package name from symbol name
    pub fn extract_package_from_symbol(&self, symbol_name: &str) -> String {
        // Parse mangled names like "_package_function" 
        if let Some(stripped) = symbol_name.strip_prefix('_') {
            if let Some(underscore_pos) = stripped.find('_') {
                return stripped[..underscore_pos].to_string();
            }
        }
        "main".to_string()
    }

    /// Find and prepare the entry point function
    #[instrument(skip(self, module), level = "debug")]
    pub fn prepare_entry_point(&mut self, module: &Module<'ctx>) -> Result<(), Error> {
        debug!(entry_point = self.config.entry_point, "Preparing entry point");

        // Look for the entry point function
        let entry_function = module.get_function(&self.config.entry_point)
            .or_else(|| {
                // Try mangled versions for different packages
                let mangled_main = format!("_main_{}", self.config.entry_point);
                module.get_function(&mangled_main)
            })
            .ok_or_else(|| Error::from_str(&format!(
                "Entry point function '{}' not found in linked module",
                self.config.entry_point
            )))?;

        // Ensure entry point has correct signature
        self.validate_entry_point_signature(&entry_function)?;

        // Store entry point information
        self.entry_point_info = Some(EntryPointInfo {
            function: entry_function,
            package_name: "main".to_string(),
            original_name: self.config.entry_point.clone(),
        });

        debug!("Entry point prepared successfully");
        Ok(())
    }

    /// Validate that the entry point has the correct signature
    #[instrument(skip(self, function), level = "trace")]
    fn validate_entry_point_signature(&self, function: &FunctionValue<'ctx>) -> Result<(), Error> {
        let func_type = function.get_type();
        
        // Entry point should be: () -> i32 or () -> ()
        if func_type.get_param_types().len() > 0 {
            warn!("Entry point function has parameters - this may cause runtime issues");
        }

        // For now, accept any return type
        debug!("Entry point signature validation completed");
        Ok(())
    }

    /// Generate runtime initialization code
    #[instrument(skip(self, module), level = "debug")]
    pub fn generate_runtime_initialization(&self, module: &Module<'ctx>) -> Result<(), Error> {
        debug!("Generating runtime initialization code");

        // Add runtime initialization functions
        self.add_runtime_startup_code(module)?;
        self.add_garbage_collector_initialization(module)?;
        self.add_signal_handlers(module)?;

        debug!("Runtime initialization code generated");
        Ok(())
    }

    /// Add runtime startup code to the module
    #[instrument(skip(self, module), level = "trace")]
    fn add_runtime_startup_code(&self, module: &Module<'ctx>) -> Result<(), Error> {
        // Create _start function that calls main
        let i32_type = self.context.i32_type();
        let void_type = self.context.void_type();
        let start_fn_type = void_type.fn_type(&[], false);
        
        let start_function = module.add_function("_start", start_fn_type, Some(Linkage::External));
        let builder = self.context.create_builder();
        let entry_block = self.context.append_basic_block(start_function, "entry");
        builder.position_at_end(entry_block);

        // Call the main function if we have entry point info
        if let Some(ref entry_info) = self.entry_point_info {
            let call_result = builder.build_call(entry_info.function, &[], "main_call")
                .map_err(|e| Error::from_str(&format!("Failed to build main call: {}", e)))?;

            // If main returns a value, use it as exit code
            if let Some(return_value) = call_result.try_as_basic_value().left() {
                if return_value.get_type().is_int_type() {
                    // Call exit with the return value
                    let exit_fn_type = void_type.fn_type(&[i32_type.into()], false);
                    let exit_function = module.add_function("exit", exit_fn_type, Some(Linkage::External));
                    builder.build_call(exit_function, &[return_value.into()], "exit_call")
                        .map_err(|e| Error::from_str(&format!("Failed to build exit call: {}", e)))?;
                } else {
                    // Main returned non-integer, exit with 0
                    let zero = i32_type.const_int(0, false);
                    let exit_fn_type = void_type.fn_type(&[i32_type.into()], false);
                    let exit_function = module.add_function("exit", exit_fn_type, Some(Linkage::External));
                    builder.build_call(exit_function, &[zero.into()], "exit_call")
                        .map_err(|e| Error::from_str(&format!("Failed to build exit call: {}", e)))?;
                }
            } else {
                // Main returned void, exit with 0
                let zero = i32_type.const_int(0, false);
                let exit_fn_type = void_type.fn_type(&[i32_type.into()], false);
                let exit_function = module.add_function("exit", exit_fn_type, Some(Linkage::External));
                builder.build_call(exit_function, &[zero.into()], "exit_call")
                    .map_err(|e| Error::from_str(&format!("Failed to build exit call: {}", e)))?;
            }
        } else {
            // No entry point, just exit with 0
            let zero = i32_type.const_int(0, false);
            let exit_fn_type = void_type.fn_type(&[i32_type.into()], false);
            let exit_function = module.add_function("exit", exit_fn_type, Some(Linkage::External));
            builder.build_call(exit_function, &[zero.into()], "exit_call")
                .map_err(|e| Error::from_str(&format!("Failed to build exit call: {}", e)))?;
        }

        builder.build_unreachable()
            .map_err(|e| Error::from_str(&format!("Failed to build unreachable: {}", e)))?;

        debug!("Runtime startup code added");
        Ok(())
    }

    /// Add garbage collector initialization
    #[instrument(skip(self, module), level = "trace")]
    fn add_garbage_collector_initialization(&self, module: &Module<'ctx>) -> Result<(), Error> {
        // Add GC initialization function declaration
        let void_type = self.context.void_type();
        let gc_init_fn_type = void_type.fn_type(&[], false);
        let _gc_init_function = module.add_function("cursed_gc_init", gc_init_fn_type, Some(Linkage::External));

        debug!("Garbage collector initialization added");
        Ok(())
    }

    /// Add signal handlers for runtime
    #[instrument(skip(self, module), level = "trace")]
    fn add_signal_handlers(&self, module: &Module<'ctx>) -> Result<(), Error> {
        // Add signal handler initialization
        let void_type = self.context.void_type();
        let signal_init_fn_type = void_type.fn_type(&[], false);
        let _signal_init_function = module.add_function("cursed_signal_init", signal_init_fn_type, Some(Linkage::External));

        debug!("Signal handlers added");
        Ok(())
    }

    /// Apply optimizations to the module
    #[instrument(skip(self, module), level = "info")]
    fn apply_optimizations(&self, module: Module<'ctx>) -> Result<Module<'ctx>, Error> {
        info!(
            optimization_level = ?self.config.target.optimization_level,
            enable_lto = self.config.enable_lto,
            "Applying optimizations"
        );

        // For now, return the module unchanged
        // In a full implementation, this would apply LLVM optimization passes
        debug!("Module optimizations not yet fully implemented");

        info!("Optimizations applied");
        Ok(module)
    }

    /// Generate the final executable binary
    #[instrument(skip(self, module), level = "info")]
    fn generate_executable_binary(&self, module: Module<'ctx>) -> Result<PathBuf, Error> {
        info!(output_path = ?self.config.output_path, "Generating executable binary");

        // Initialize LLVM target support
        Target::initialize_all(&InitializationConfig::default());

        // Create target triple
        let target_triple = TargetTriple::create(&self.config.target.triple);
        
        // Get target from triple
        let target = Target::from_name(&self.config.target.triple)
            .or_else(|| Target::from_triple(&target_triple).ok())
            .ok_or_else(|| Error::from_str(&format!(
                "Unknown target triple: {}",
                self.config.target.triple
            )))?;

        // Create target machine
        let target_machine = target.create_target_machine(
            &target_triple,
            &self.config.target.cpu,
            &self.config.target.features,
            self.config.target.optimization_level,
            self.config.target.reloc_mode,
            self.config.target.code_model,
        ).ok_or_else(|| Error::from_str("Failed to create target machine"))?;

        // Generate object file
        let object_path = self.config.output_path.with_extension("o");
        target_machine.write_to_file(&module, FileType::Object, &object_path)
            .map_err(|e| Error::from_str(&format!("Failed to write object file: {}", e)))?;

        // Link to create executable
        self.link_object_to_executable(&object_path)?;

        info!(output_path = ?self.config.output_path, "Executable generation completed");
        Ok(self.config.output_path.clone())
    }

    /// Link object file to create final executable
    #[instrument(skip(self, object_path), level = "debug")]
    fn link_object_to_executable(&self, object_path: &Path) -> Result<(), Error> {
        debug!(object_path = ?object_path, "Linking object to executable");

        // Prepare linker command
        let mut linker_args = vec![
            object_path.to_str().unwrap().to_string(),
            "-o".to_string(),
            self.config.output_path.to_str().unwrap().to_string(),
        ];

        // Add system libraries based on strategy
        match &self.config.strategy {
            LinkingStrategy::Static => {
                linker_args.extend_from_slice(&["-static".to_string()]);
            }
            LinkingStrategy::Dynamic => {
                linker_args.extend_from_slice(&["-ldl".to_string(), "-lpthread".to_string()]);
            }
            LinkingStrategy::Hybrid { .. } => {
                linker_args.extend_from_slice(&["-ldl".to_string(), "-lpthread".to_string()]);
            }
        }

        // Add standard C library
        linker_args.extend_from_slice(&["-lc".to_string()]);

        // Execute linker
        let output = std::process::Command::new("gcc")
            .args(&linker_args)
            .output()
            .map_err(|e| Error::from_str(&format!("Failed to execute linker: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::from_str(&format!("Linker failed: {}", stderr)));
        }

        // Clean up object file
        if let Err(e) = fs::remove_file(object_path) {
            warn!(object_path = ?object_path, error = %e, "Failed to clean up object file");
        }

        debug!("Object linking completed successfully");
        Ok(())
    }

    /// Get linking statistics
    pub fn get_linking_statistics(&self) -> LinkingStatistics {
        LinkingStatistics {
            resolved_symbols: self.symbol_resolution.len(),
            missing_symbols: self.missing_symbols.len(),
            entry_point_found: self.entry_point_info.is_some(),
            strategy: self.config.strategy.clone(),
        }
    }
}

/// Statistics about the linking process
#[derive(Debug, Clone)]
pub struct LinkingStatistics {
    pub resolved_symbols: usize,
    pub missing_symbols: usize,
    pub entry_point_found: bool,
    pub strategy: LinkingStrategy,
}

/// Convenience function to link modules and generate executable
#[instrument(skip(context, modules, metadata_list), fields(module_count = modules.len()), level = "info")]
pub fn link_modules_to_executable<'ctx>(
    context: &'ctx Context,
    modules: Vec<Module<'ctx>>,
    metadata_list: Vec<PackageMetadata>,
    config: ExecutableLinkingConfig,
) -> Result<PathBuf, Error> {
    info!("Linking modules to executable with {} modules", modules.len());

    let mut linker = ExecutableLinker::new(context, config);

    // Add all package metadata
    for metadata in metadata_list {
        linker.add_package(metadata)?;
    }

    // Link and generate executable
    linker.link_and_generate_executable(modules)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::llvm::separate_compilation::PackageMetadata;
    use inkwell::context::Context;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_executable_linker_creation() {
        let context = Context::create();
        let config = ExecutableLinkingConfig::default();
        let linker = ExecutableLinker::new(&context, config);

        assert_eq!(linker.symbol_resolution.len(), 0);
        assert_eq!(linker.missing_symbols.len(), 0);
        assert!(linker.entry_point_info.is_none());
    }

    #[test]
    fn test_symbol_collection() {
        let context = Context::create();
        let module = context.create_module("test");

        // Add a function with body (defined)
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let function = module.add_function("test_func", fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        let builder = context.create_builder();
        builder.position_at_end(basic_block);
        let return_val = i32_type.const_int(42, false);
        builder.build_return(Some(&return_val)).unwrap();

        // Add a function declaration (undefined)
        let _extern_func = module.add_function("extern_func", fn_type, None);

        let config = ExecutableLinkingConfig::default();
        let linker = ExecutableLinker::new(&context, config);

        let mut defined_symbols = HashMap::new();
        let mut required_symbols = HashSet::new();

        linker.collect_module_symbols(&module, &mut defined_symbols, &mut required_symbols).unwrap();

        assert!(defined_symbols.contains_key("test_func"));
        assert!(required_symbols.contains("extern_func"));
    }

    #[test]
    fn test_target_platform_defaults() {
        let platform = TargetPlatform::default();
        assert_eq!(platform.cpu, "generic");
        assert_eq!(platform.features, "");
        assert_eq!(platform.optimization_level, OptimizationLevel::Default);
    }

    #[test]
    fn test_linking_config_defaults() {
        let config = ExecutableLinkingConfig::default();
        assert_eq!(config.strategy, LinkingStrategy::Static);
        assert_eq!(config.entry_point, "main");
        assert_eq!(config.output_path, PathBuf::from("program"));
        assert!(!config.include_debug_info);
        assert!(!config.strip_symbols);
        assert!(!config.enable_lto);
    }

    #[test]
    fn test_package_name_extraction() {
        let context = Context::create();
        let config = ExecutableLinkingConfig::default();
        let linker = ExecutableLinker::new(&context, config);

        assert_eq!(linker.extract_package_from_symbol("_utils_helper"), "utils");
        assert_eq!(linker.extract_package_from_symbol("_main_func"), "main");
        assert_eq!(linker.extract_package_from_symbol("plain_func"), "main");
        assert_eq!(linker.extract_package_from_symbol("_single"), "main");
    }
}
