/// LLVM debug information integration
use crate::error::SourceLocation;
// use crate::debug::DebugConfig;
use crate::error::CursedError;
use std::path::PathBuf;
use tracing::{debug, instrument};

/// LLVM code generator with debug information support
pub struct LlvmDebugCodeGenerator {
impl LlvmDebugCodeGenerator {
    /// Create a new debug-enabled LLVM code generator
    pub fn new(debug_config: DebugConfig) -> Self {
        Self {
        }
    }

    /// Initialize debug information for a compilation unit
    #[instrument(skip(self))]
    pub fn initialize_debug_info(&mut self, file: PathBuf, producer: String) -> crate::error::Result<()> {
        debug!(file = ?file, producer = %producer, "Initializing debug info for LLVM");
        self.debug_manager.initialize_compilation_unit(file, producer)
    /// Begin generating code for a function with debug information
    #[instrument(skip(self))]
    pub fn begin_function_with_debug(
    ) -> crate::error::Result<()> {
        debug!(function = %name, location = ?location, "Beginning function with debug info");
        
        self.debug_manager.begin_function(name.clone(), location.clone().into())?;
        self.current_function = Some(name.clone());
        
        // Generate LLVM IR with debug metadata
        let mut ir = String::new();
        
        // Function declaration with debug attributes
        ir.push_str(&format!(
            self.get_next_metadata_id()
        ));
        
        // Add debug location for function entry
        ir.push_str(&format!(
            self.debug_manager.generate_debug_location(&location.clone().into())
        ));
        ir.push('\n');
        
        Ok(ir)
    /// End function generation with debug information
    #[instrument(skip(self))]
    pub fn end_function_with_debug(&mut self) -> crate::error::Result<()> {
        debug!("Ending function with debug info");
        
        self.debug_manager.end_function()?;
        self.current_function = None;
        
        // Close function
        Ok("}\n".to_string())
    /// Generate variable declaration with debug information
    #[instrument(skip(self))]
    pub fn generate_variable_with_debug(
    ) -> crate::error::Result<()> {
        debug!(name = %name, type_name = %type_name, location = ?location, "Generating variable with debug");
        
        self.debug_manager.add_variable(name.clone(), type_name.clone(), location.clone().into())?;
        
        // Generate LLVM IR for variable with debug metadata
        let mut ir = String::new();
        
        // Allocate the variable
        ir.push_str(&format!("  %{} = alloca i32", name));
        ir.push_str(&self.debug_manager.generate_debug_location(&location.clone().into()));
        ir.push('\n');
        
        // Add debug declare intrinsic
        ir.push_str(&format!(
            self.debug_manager.generate_debug_location(&location.clone().into())
        ));
        ir.push('\n');
        
        Ok(ir)
    /// Generate assignment with debug information
    #[instrument(skip(self))]
    pub fn generate_assignment_with_debug(
    ) -> crate::error::Result<()> {
        debug!(variable = %variable, value = %value, location = ?location, "Generating assignment with debug");
        
        // Generate store instruction with debug location
        let mut ir = String::new();
        ir.push_str(&format!("  store i32 {}, i32* %{}", value, variable));
        ir.push_str(&self.debug_manager.generate_debug_location(&location.clone().into()));
        ir.push('\n');
        
        Ok(ir)
    /// Generate function call with debug information
    #[instrument(skip(self))]
    pub fn generate_call_with_debug(
    ) -> crate::error::Result<()> {
        debug!(function = %function, args = ?args, location = ?location, "Generating call with debug");
        
        // Generate call instruction with debug location
        let mut ir = String::new();
        let args_str = args.join(", ");
        ir.push_str(&format!("  call i32 @{}({})", function, args_str));
        ir.push_str(&self.debug_manager.generate_debug_location(&location.clone().into()));
        ir.push('\n');
        
        Ok(ir)
    /// Generate return statement with debug information
    #[instrument(skip(self))]
    pub fn generate_return_with_debug(
    ) -> crate::error::Result<()> {
        debug!(value = ?value, location = ?location, "Generating return with debug");
        
        let mut ir = String::new();
        if let Some(val) = value {
            ir.push_str(&format!("  ret i32 {}", val));
        } else {
            ir.push_str("  ret void");
        }
        ir.push_str(&self.debug_manager.generate_debug_location(&location.clone().into()));
        ir.push('\n');
        
        Ok(ir)
    /// Generate debug metadata for the entire module
    #[instrument(skip(self))]
    pub fn generate_debug_metadata(&mut self) -> crate::error::Result<()> {
        debug!("Generating module debug metadata");
        
        let mut metadata = self.debug_manager.generate_llvm_debug_metadata()?;
        
        // Add required LLVM debug intrinsics
        metadata.push_str("\n; Debug intrinsics\n");
        metadata.push_str("declare void @llvm.dbg.declare(metadata, metadata, metadata) #0\n");
        metadata.push_str("declare void @llvm.dbg.value(metadata, metadata, metadata) #0\n");
        metadata.push_str("declare void @llvm.dbg.addr(metadata, metadata, metadata) #0\n");
        
        // Add debug info version
        metadata.push_str("\n; Debug info version\n");
        metadata.push_str("!llvm.dbg.cu = !{!0}\n");
        metadata.push_str("!llvm.module.flags = !{!2, !3}\n");
        metadata.push_str("!llvm.ident = !{!4}\n");
        metadata.push_str("!2 = !{i32 7, !\"Dwarf Version\", i32 4}\n");
        metadata.push_str("!3 = !{i32 2, !\"Debug Info Version\", i32 3}\n");
        metadata.push_str("!4 = !{!\"CURSED Compiler with Debug Support\"}\n");
        
        Ok(metadata)
    /// Set current source location for subsequent operations
    pub fn set_current_location(&mut self, location: SourceLocation) {
        self.debug_manager.set_current_location(location.into());
    /// Get current source location
    pub fn current_location(&self) -> Option<&SourceLocation> {
        // Return None since our debug manager returns owned values not references
        None
    /// Generate line table for the module
    pub fn generate_line_table(&self) -> Vec<(u32, String)> {
        self.debug_manager.generate_line_table()
    /// Check if debug information is enabled
    pub fn debug_enabled(&self) -> bool {
        self.debug_manager.is_enabled()
    /// Get debug statistics
    pub fn debug_statistics(&self) -> String {
        format!("{}", self.debug_manager.statistics())
    /// Generate a complete LLVM module with debug information
    #[instrument(skip(self))]
    pub fn generate_module_with_debug(
    ) -> crate::error::Result<()> {
        debug!(module = %module_name, function_count = functions.len(), "Generating module with debug");
        
        self.current_module = Some(module_name.clone());
        
        let mut module_ir = String::new();
        
        // Module header
        module_ir.push_str(&format!("; ModuleID = '{}'\n", module_name));
        module_ir.push_str("target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"\n");
        module_ir.push_str("target triple = \"x86_64-pc-linux-gnu\"\n\n");
        
        // Generate functions
        for (func_name, location) in functions {
            module_ir.push_str(&self.begin_function_with_debug(func_name, location)?);
            // Function body would be generated here in a real implementation
            module_ir.push_str("  ret i32 0\n");
            module_ir.push_str(&self.end_function_with_debug()?);
            module_ir.push('\n');
        // Generate debug metadata
        module_ir.push_str(&self.generate_debug_metadata()?);
        
        Ok(module_ir)
    /// Generate debugging utilities
    pub fn generate_debug_utilities(&self) -> String {
        let mut utilities = String::new();
        
        // Add helper functions for debugging
        utilities.push_str("; Debug utility functions\n");
        utilities.push_str("define void @cursed_debug_print_int(i32 %value) {\n");
        utilities.push_str("  ; Print integer value for debugging\n");
        utilities.push_str("  ret void\n");
        utilities.push_str("}\n\n");
        
        utilities.push_str("define void @cursed_debug_print_string(i8* %str) {\n");
        utilities.push_str("  ; Print string value for debugging\n");
        utilities.push_str("  ret void\n");
        utilities.push_str("}\n\n");
        
        utilities.push_str("define void @cursed_debug_breakpoint() {\n");
        utilities.push_str("  ; Breakpoint for debugging\n");
        utilities.push_str("  ret void\n");
        utilities.push_str("}\n\n");
        
        utilities
    /// Validate debug information
    pub fn validate_debug_info(&self) -> Result<(), Vec<String>> {
        self.debug_manager.validate()
    /// Clear all debug information
    pub fn clear_debug_info(&mut self) {
        self.debug_manager.clear();
        self.current_module = None;
        self.current_function = None;
    /// Get a simplified metadata ID (in a real implementation, this would be more sophisticated)
    fn get_next_metadata_id(&self) -> usize {
        // This is a simplified implementation
        // In a real compiler, metadata IDs would be managed more carefully
        10 + self.debug_manager.statistics().symbol_count
    /// Update debug configuration
    pub fn update_debug_config(&mut self, config: DebugConfig) {
        self.debug_manager.update_config(config);
    /// Get debug configuration
    pub fn debug_config(&self) -> DebugConfig {
        self.debug_manager.config()
    /// Generate debug-aware optimization hints
    pub fn generate_optimization_hints(&self) -> String {
        let mut hints = String::new();
        
        if self.debug_manager.config().optimized_debug {
            hints.push_str("; Optimized debug information enabled\n");
            hints.push_str("attributes #0 = { nounwind readnone speculatable willreturn }\n");
        } else {
            hints.push_str("; Full debug information enabled\n");
            hints.push_str("attributes #0 = { nounwind }\n");
        hints
    }
}

impl Default for LlvmDebugCodeGenerator {
    fn default() -> Self {
        Self::new(DebugConfig::default())
    }
}

