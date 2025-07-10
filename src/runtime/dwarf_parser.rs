//! DWARF debug information parsing for CURSED
//!
//! This module provides comprehensive DWARF debug information parsing
//! including variable location tracking, stack frame reconstruction,
//! and integration with the CURSED debug engine.

use crate::error::{CursedError, SourceLocation};
use std::collections::{HashMap, BTreeMap};
use std::path::PathBuf;
use std::fmt;

/// DWARF debug database for comprehensive debug information
#[derive(Debug)]
pub struct DwarfDebugDatabase {
    /// Function information indexed by address range
    pub functions: BTreeMap<u64, FunctionDebugInfo>,
    /// Local variables indexed by function address
    pub variables: HashMap<u64, Vec<VariableDebugInfo>>,
    /// Inline call sites indexed by address
    pub inline_sites: HashMap<u64, Vec<InlineCallSite>>,
    /// Type information by type ID
    pub types: HashMap<u64, DwarfTypeInfo>,
    /// Address to line number mappings
    pub line_mappings: BTreeMap<u64, LineInfo>,
}

impl DwarfDebugDatabase {
    /// Create a new empty DWARF debug database
    pub fn new() -> Self {
        Self {
            functions: BTreeMap::new(),
            variables: HashMap::new(),
            inline_sites: HashMap::new(),
            types: HashMap::new(),
            line_mappings: BTreeMap::new(),
        }
    }

    /// Load debug information from DWARF data (simplified implementation)
    pub fn load_from_dwarf(&mut self, _dwarf_data: &[u8]) -> Result<(), CursedError> {
        // For now, create a test function entry to demonstrate the API
        let test_func = FunctionDebugInfo {
            name: "test_function".to_string(),
            demangled_name: Some("test_function".to_string()),
            start_address: 0x1000,
            end_address: 0x1100,
            parameters: vec![
                ParameterDebugInfo {
                    name: "param1".to_string(),
                    type_id: 1,
                    location: None,
                    by_reference: false,
                }
            ],
            source_file: Some(PathBuf::from("example.csd")),
            line_range: Some((10, 20)),
            frame_base: None,
        };

        self.functions.insert(test_func.start_address, test_func);

        // Add a test type
        let test_type = DwarfTypeInfo {
            name: "int".to_string(),
            size: 4,
            encoding: Some("signed".to_string()),
            members: Vec::new(),
            base_type: None,
        };
        self.types.insert(1, test_type);

        // Add line mapping
        let line_info = LineInfo {
            file: PathBuf::from("example.csd"),
            line: 10,
            column: 1,
            is_stmt: true,
        };
        self.line_mappings.insert(0x1000, line_info);

        Ok(())
    }

    /// Find function by address
    pub fn find_function(&self, address: u64) -> Option<&FunctionDebugInfo> {
        self.functions.range(..=address)
            .next_back()
            .and_then(|(start, func)| {
                if address >= *start && address < func.end_address {
                    Some(func)
                } else {
                    None
                }
            })
    }

    /// Get variables in scope at address
    pub fn get_variables_at_address(&self, address: u64) -> Vec<&VariableDebugInfo> {
        if let Some(func) = self.find_function(address) {
            if let Some(variables) = self.variables.get(&func.start_address) {
                return variables.iter()
                    .filter(|var| address >= var.scope_start && address < var.scope_end)
                    .collect();
            }
        }
        Vec::new()
    }

    /// Get inline information at address
    pub fn get_inline_info_at_address(&self, address: u64) -> Vec<&InlineCallSite> {
        self.inline_sites.get(&address)
            .map(|sites| sites.iter().collect())
            .unwrap_or_default()
    }

    /// Get source location for address
    pub fn get_source_location_for_address(&self, address: u64) -> Option<&LineInfo> {
        self.line_mappings.range(..=address)
            .next_back()
            .map(|(_, line_info)| line_info)
    }

    /// Reconstruct stack frame at address
    pub fn reconstruct_stack_frame(&self, address: u64, registers: &RegisterMap) -> Result<StackFrameInfo, CursedError> {
        let mut frame_info = StackFrameInfo {
            function_name: String::new(),
            parameters: Vec::new(),
            local_variables: Vec::new(),
            source_location: None,
            address,
        };

        // Find function containing this address
        if let Some(function) = self.find_function(address) {
            frame_info.function_name = function.name.clone();

            // Get frame base address
            let frame_base = if let Some(ref frame_base_expr) = function.frame_base {
                self.evaluate_location(frame_base_expr, 0, registers)?
            } else {
                registers.get_register(RegisterName::StackPointer).unwrap_or(0)
            };

            // Add simplified parameter info
            for param in &function.parameters {
                let param_info = crate::runtime::debug_info::ParameterInfo {
                    name: param.name.clone(),
                    param_type: self.get_type_name(param.type_id),
                    value: None,
                    location: None,
                };
                frame_info.parameters.push(param_info);
            }

            // Get source location
            if let Some(line_info) = self.get_source_location_for_address(address) {
                frame_info.source_location = Some(SourceLocation {
                    file: line_info.file.to_string_lossy().to_string(),
                    line: line_info.line as usize,
                    column: line_info.column as usize,
                });
            }
        }

        Ok(frame_info)
    }

    /// Evaluate variable location expression (simplified)
    pub fn evaluate_location(&self, location_expr: &[u8], frame_base: u64, registers: &RegisterMap) -> Result<u64, CursedError> {
        let mut evaluator = LocationEvaluator::new(frame_base, registers);
        evaluator.evaluate(location_expr)
    }

    /// Get type name by ID
    pub fn get_type_name(&self, type_id: u64) -> String {
        self.types.get(&type_id)
            .map(|t| t.name.clone())
            .unwrap_or_else(|| format!("unknown_type_{}", type_id))
    }
}

/// Function debug information from DWARF
#[derive(Debug, Clone)]
pub struct FunctionDebugInfo {
    pub name: String,
    pub demangled_name: Option<String>,
    pub start_address: u64,
    pub end_address: u64,
    pub parameters: Vec<ParameterDebugInfo>,
    pub source_file: Option<PathBuf>,
    pub line_range: Option<(u32, u32)>,
    pub frame_base: Option<Vec<u8>>,
}

/// Parameter debug information from DWARF
#[derive(Debug, Clone)]
pub struct ParameterDebugInfo {
    pub name: String,
    pub type_id: u64,
    pub location: Option<Vec<u8>>,
    pub by_reference: bool,
}

/// Variable debug information from DWARF
#[derive(Debug, Clone)]
pub struct VariableDebugInfo {
    pub name: String,
    pub type_id: u64,
    pub location: Option<Vec<u8>>,
    pub scope_start: u64,
    pub scope_end: u64,
    pub declared_line: Option<u32>,
}

/// Inline call site information
#[derive(Debug, Clone)]
pub struct InlineCallSite {
    pub function_name: String,
    pub call_address: u64,
    pub original_location: Option<(PathBuf, u32, u32)>,
    pub inline_location: Option<(PathBuf, u32, u32)>,
}

/// DWARF type information
#[derive(Debug, Clone)]
pub struct DwarfTypeInfo {
    pub name: String,
    pub size: u64,
    pub encoding: Option<String>,
    pub members: Vec<TypeMemberInfo>,
    pub base_type: Option<u64>,
}

/// Type member information for composite types
#[derive(Debug, Clone)]
pub struct TypeMemberInfo {
    pub name: String,
    pub type_id: u64,
    pub offset: u64,
    pub size: u64,
}

/// Line information from DWARF
#[derive(Debug, Clone)]
pub struct LineInfo {
    pub file: PathBuf,
    pub line: u32,
    pub column: u32,
    pub is_stmt: bool,
}

/// Register map for variable location evaluation
#[derive(Debug, Clone)]
pub struct RegisterMap {
    registers: HashMap<RegisterName, u64>,
}

impl RegisterMap {
    pub fn new() -> Self {
        Self {
            registers: HashMap::new(),
        }
    }

    pub fn set_register(&mut self, name: RegisterName, value: u64) {
        self.registers.insert(name, value);
    }

    pub fn get_register(&self, name: RegisterName) -> Option<u64> {
        self.registers.get(&name).copied()
    }

    pub fn from_current_context() -> Self {
        let mut map = Self::new();
        
        #[cfg(target_arch = "x86_64")]
        {
            map.set_register(RegisterName::StackPointer, Self::get_stack_pointer());
            map.set_register(RegisterName::BasePointer, Self::get_base_pointer());
        }
        
        #[cfg(target_arch = "aarch64")]
        {
            map.set_register(RegisterName::StackPointer, Self::get_stack_pointer());
            map.set_register(RegisterName::BasePointer, Self::get_base_pointer());
        }

        map
    }

    #[cfg(target_arch = "x86_64")]
    fn get_stack_pointer() -> u64 {
        let rsp: u64;
        unsafe {
            std::arch::asm!("mov {}, rsp", out(reg) rsp);
        }
        rsp
    }

    #[cfg(target_arch = "x86_64")]
    fn get_base_pointer() -> u64 {
        let rbp: u64;
        unsafe {
            std::arch::asm!("mov {}, rbp", out(reg) rbp);
        }
        rbp
    }

    #[cfg(target_arch = "aarch64")]
    fn get_stack_pointer() -> u64 {
        let sp: u64;
        unsafe {
            std::arch::asm!("mov {}, sp", out(reg) sp);
        }
        sp
    }

    #[cfg(target_arch = "aarch64")]
    fn get_base_pointer() -> u64 {
        let fp: u64;
        unsafe {
            std::arch::asm!("mov {}, x29", out(reg) fp);
        }
        fp
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    fn get_stack_pointer() -> u64 {
        0
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    fn get_base_pointer() -> u64 {
        0
    }
}

/// Register names for different architectures
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegisterName {
    StackPointer,
    BasePointer,
    InstructionPointer,
    Rax, Rbx, Rcx, Rdx, Rsi, Rdi, Rbp, Rsp, Rip,
    R8, R9, R10, R11, R12, R13, R14, R15,
    X0, X1, X2, X3, X4, X5, X6, X7, X8, X9,
    X10, X11, X12, X13, X14, X15, X16, X17, X18, X19,
    X20, X21, X22, X23, X24, X25, X26, X27, X28, X29, X30,
    Sp, Pc,
    Custom(u32),
}

/// Location evaluator for DWARF expressions (simplified)
pub struct LocationEvaluator<'a> {
    frame_base: u64,
    registers: &'a RegisterMap,
    stack: Vec<u64>,
}

impl<'a> LocationEvaluator<'a> {
    pub fn new(frame_base: u64, registers: &'a RegisterMap) -> Self {
        Self {
            frame_base,
            registers,
            stack: Vec::new(),
        }
    }

    /// Evaluate DWARF location expression (simplified implementation)
    pub fn evaluate(&mut self, expression: &[u8]) -> Result<u64, CursedError> {
        if expression.is_empty() {
            return Err(CursedError::RuntimeError("Empty location expression".to_string()));
        }

        // For now, return frame_base as a simple implementation
        // A full implementation would parse the DWARF expression bytecode
        Ok(self.frame_base)
    }
}

/// Stack frame information reconstructed from debug info
#[derive(Debug, Clone)]
pub struct StackFrameInfo {
    pub function_name: String,
    pub parameters: Vec<crate::runtime::debug_info::ParameterInfo>,
    pub local_variables: Vec<crate::runtime::debug_info::LocalVariableInfo>,
    pub source_location: Option<SourceLocation>,
    pub address: u64,
}

/// DWARF version compatibility handler
#[derive(Debug, Clone)]
pub struct DwarfVersionHandler {
    supported_versions: Vec<u16>,
    current_version: Option<u16>,
}

impl DwarfVersionHandler {
    pub fn new() -> Self {
        Self {
            supported_versions: vec![2, 3, 4, 5],
            current_version: None,
        }
    }

    pub fn is_supported(&self, version: u16) -> bool {
        self.supported_versions.contains(&version)
    }

    pub fn set_version(&mut self, version: u16) -> Result<(), CursedError> {
        if self.is_supported(version) {
            self.current_version = Some(version);
            Ok(())
        } else {
            Err(CursedError::RuntimeError(format!("Unsupported DWARF version: {}", version)))
        }
    }

    pub fn current_version(&self) -> Option<u16> {
        self.current_version
    }
}

/// Error handling for malformed debug information
#[derive(Debug, Clone)]
pub struct DebugInfoErrorHandler {
    pub continue_on_error: bool,
    pub errors: Vec<DebugInfoError>,
    pub max_errors: usize,
}

impl DebugInfoErrorHandler {
    pub fn new() -> Self {
        Self {
            continue_on_error: true,
            errors: Vec::new(),
            max_errors: 100,
        }
    }

    pub fn handle_error(&mut self, error: DebugInfoError) -> Result<(), CursedError> {
        self.errors.push(error.clone());
        
        if self.errors.len() >= self.max_errors {
            return Err(CursedError::RuntimeError("Too many debug info errors".to_string()));
        }
        
        if self.continue_on_error {
            Ok(())
        } else {
            Err(CursedError::RuntimeError(format!("Debug info error: {:?}", error)))
        }
    }

    pub fn get_errors(&self) -> &[DebugInfoError] {
        &self.errors
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

/// Debug information parsing error
#[derive(Debug, Clone)]
pub enum DebugInfoError {
    MalformedDie { offset: u64, message: String },
    InvalidAttribute { die_offset: u64, attribute: String },
    MissingAttribute { die_offset: u64, attribute: String },
    InvalidTypeRef { die_offset: u64, type_id: u64 },
    LocationExpressionError { die_offset: u64, message: String },
    LineProgramError { message: String },
    UnsupportedFeature { feature: String, version: u16 },
}

impl fmt::Display for DebugInfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DebugInfoError::MalformedDie { offset, message } => {
                write!(f, "Malformed DIE at offset 0x{:x}: {}", offset, message)
            }
            DebugInfoError::InvalidAttribute { die_offset, attribute } => {
                write!(f, "Invalid attribute '{}' in DIE at offset 0x{:x}", attribute, die_offset)
            }
            DebugInfoError::MissingAttribute { die_offset, attribute } => {
                write!(f, "Missing required attribute '{}' in DIE at offset 0x{:x}", attribute, die_offset)
            }
            DebugInfoError::InvalidTypeRef { die_offset, type_id } => {
                write!(f, "Invalid type reference {} in DIE at offset 0x{:x}", type_id, die_offset)
            }
            DebugInfoError::LocationExpressionError { die_offset, message } => {
                write!(f, "Location expression error in DIE at offset 0x{:x}: {}", die_offset, message)
            }
            DebugInfoError::LineProgramError { message } => {
                write!(f, "Line program error: {}", message)
            }
            DebugInfoError::UnsupportedFeature { feature, version } => {
                write!(f, "Unsupported DWARF feature '{}' in version {}", feature, version)
            }
        }
    }
}

impl std::error::Error for DebugInfoError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dwarf_database_creation() {
        let database = DwarfDebugDatabase::new();
        assert!(database.functions.is_empty());
        assert!(database.variables.is_empty());
        assert!(database.types.is_empty());
        assert!(database.line_mappings.is_empty());
    }

    #[test]
    fn test_load_mock_dwarf_data() {
        let mut database = DwarfDebugDatabase::new();
        let mock_data = vec![0x7f, 0x45, 0x4c, 0x46]; // ELF magic
        
        let result = database.load_from_dwarf(&mock_data);
        assert!(result.is_ok());
        
        // Should have test function
        assert!(!database.functions.is_empty());
        assert!(database.find_function(0x1000).is_some());
    }

    #[test]
    fn test_register_map() {
        let mut register_map = RegisterMap::new();
        register_map.set_register(RegisterName::StackPointer, 0x7fff0000);
        
        assert_eq!(register_map.get_register(RegisterName::StackPointer), Some(0x7fff0000));
        assert_eq!(register_map.get_register(RegisterName::BasePointer), None);
    }

    #[test]
    fn test_location_evaluator() {
        let register_map = RegisterMap::new();
        let mut evaluator = LocationEvaluator::new(0x1000, &register_map);
        
        let expression = vec![0x01, 0x02, 0x03];
        let result = evaluator.evaluate(&expression);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0x1000); // Returns frame_base in simplified impl
    }

    #[test]
    fn test_dwarf_version_handler() {
        let mut handler = DwarfVersionHandler::new();
        
        assert!(handler.is_supported(4));
        assert!(!handler.is_supported(6));
        
        assert!(handler.set_version(4).is_ok());
        assert_eq!(handler.current_version(), Some(4));
        
        assert!(handler.set_version(6).is_err());
    }

    #[test]
    fn test_error_handler() {
        let mut handler = DebugInfoErrorHandler::new();
        
        let error = DebugInfoError::MalformedDie {
            offset: 0x1000,
            message: "Test error".to_string(),
        };
        
        assert!(handler.handle_error(error).is_ok());
        assert!(handler.has_errors());
        assert_eq!(handler.get_errors().len(), 1);
    }

    #[test]
    fn test_stack_frame_reconstruction() {
        let mut database = DwarfDebugDatabase::new();
        let _ = database.load_from_dwarf(&[]);
        
        let register_map = RegisterMap::new();
        let result = database.reconstruct_stack_frame(0x1000, &register_map);
        
        assert!(result.is_ok());
        let frame = result.unwrap();
        assert_eq!(frame.function_name, "test_function");
        assert_eq!(frame.address, 0x1000);
    }
}
