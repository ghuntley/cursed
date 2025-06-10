/// DWARF debug information generation for LLVM
use crate::debug::{debug_symbols::{DebugSymbol, DebugSymbolType}, SourceLocation};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{debug, instrument, warn};

/// DWARF debug information generator
#[derive(Debug)]
pub struct DwarfGenerator {
    compile_unit: Option<DwarfCompileUnit>,
    subprograms: Vec<DwarfSubprogram>,
    variables: Vec<DwarfVariable>,
    types: HashMap<String, DwarfType>,
    current_file: Option<PathBuf>,
}

/// DWARF compile unit information
#[derive(Debug, Clone)]
pub struct DwarfCompileUnit {
    pub file: PathBuf,
    pub directory: PathBuf,
    pub producer: String,
    pub language: DwarfLanguage,
    pub version: u32,
}

/// DWARF language constants
#[derive(Debug, Clone, Copy)]
pub enum DwarfLanguage {
    C = 0x0001,
    CPlusPlus = 0x0004,
    Rust = 0x001C,
    Custom = 0x8000, // Custom language for CURSED
}

/// DWARF subprogram (function) information
#[derive(Debug, Clone)]
pub struct DwarfSubprogram {
    pub name: String,
    pub linkage_name: Option<String>,
    pub file: PathBuf,
    pub line: u32,
    pub column: u32,
    pub scope_line: u32,
    pub return_type: Option<String>,
    pub parameters: Vec<DwarfParameter>,
    pub local_variables: Vec<DwarfVariable>,
    pub is_external: bool,
    pub is_definition: bool,
    pub low_pc: Option<u64>,
    pub high_pc: Option<u64>,
}

/// DWARF parameter information
#[derive(Debug, Clone)]
pub struct DwarfParameter {
    pub name: String,
    pub type_name: String,
    pub file: PathBuf,
    pub line: u32,
    pub column: u32,
    pub location: Option<DwarfLocation>,
}

/// DWARF variable information
#[derive(Debug, Clone)]
pub struct DwarfVariable {
    pub name: String,
    pub type_name: String,
    pub file: PathBuf,
    pub line: u32,
    pub column: u32,
    pub scope_start: Option<u32>,
    pub scope_end: Option<u32>,
    pub location: Option<DwarfLocation>,
    pub is_artificial: bool,
}

/// DWARF location information
#[derive(Debug, Clone)]
pub enum DwarfLocation {
    Register(u32),
    StackOffset(i32),
    Address(u64),
    Expression(Vec<u8>), // DWARF expression bytecode
}

/// DWARF type information
#[derive(Debug, Clone)]
pub struct DwarfType {
    pub name: String,
    pub size: Option<u64>,
    pub alignment: Option<u32>,
    pub encoding: Option<DwarfEncoding>,
    pub base_type: Option<String>,
    pub members: Vec<DwarfMember>,
}

/// DWARF type encoding
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DwarfEncoding {
    Address = 0x01,
    Boolean = 0x02,
    Float = 0x04,
    Signed = 0x05,
    SignedChar = 0x06,
    Unsigned = 0x07,
    UnsignedChar = 0x08,
    UTF = 0x10,
}

/// DWARF struct/interface member
#[derive(Debug, Clone)]
pub struct DwarfMember {
    pub name: String,
    pub type_name: String,
    pub offset: u64,
    pub bit_offset: Option<u32>,
    pub bit_size: Option<u32>,
}

impl DwarfGenerator {
    /// Create a new DWARF generator
    pub fn new() -> Self {
        Self {
            compile_unit: None,
            subprograms: Vec::new(),
            variables: Vec::new(),
            types: HashMap::new(),
            current_file: None,
        }
    }

    /// Set the compile unit information
    #[instrument(skip(self))]
    pub fn set_compile_unit(&mut self, file: PathBuf, producer: String) {
        let directory = file.parent().unwrap_or_else(|| Path::new(".")).to_path_buf();
        
        self.compile_unit = Some(DwarfCompileUnit {
            file: file.clone(),
            directory,
            producer,
            language: DwarfLanguage::Custom,
            version: 4, // DWARF version 4
        });
        
        self.current_file = Some(file);
        debug!(compile_unit = ?self.compile_unit, "Set compile unit");
    }

    /// Add debug symbols from the symbol table
    #[instrument(skip(self, symbols))]
    pub fn add_symbols(&mut self, symbols: Vec<DebugSymbol>) {
        debug!(symbol_count = symbols.len(), "Adding debug symbols");
        
        for symbol in symbols {
            match symbol.symbol_type {
                DebugSymbolType::Function => {
                    self.add_function_symbol(symbol);
                }
                DebugSymbolType::Variable => {
                    self.add_variable_symbol(symbol);
                }
                DebugSymbolType::Parameter => {
                    self.add_parameter_symbol(symbol);
                }
                DebugSymbolType::Type => {
                    self.add_type_symbol(symbol);
                }
                _ => {
                    debug!(symbol = ?symbol, "Skipping unsupported symbol type");
                }
            }
        }
    }

    /// Add a function symbol
    fn add_function_symbol(&mut self, symbol: DebugSymbol) {
        let subprogram = DwarfSubprogram {
            name: symbol.name,
            linkage_name: None,
            file: symbol.location.file,
            line: symbol.location.line,
            column: symbol.location.column,
            scope_line: symbol.location.line,
            return_type: None, // Will be inferred from type system
            parameters: Vec::new(),
            local_variables: Vec::new(),
            is_external: false,
            is_definition: true,
            low_pc: symbol.address,
            high_pc: symbol.address.map(|addr| addr + symbol.size.unwrap_or(0)),
        };
        
        self.subprograms.push(subprogram);
    }

    /// Add a variable symbol
    fn add_variable_symbol(&mut self, symbol: DebugSymbol) {
        let variable = DwarfVariable {
            name: symbol.name,
            type_name: symbol.type_name,
            file: symbol.location.file,
            line: symbol.location.line,
            column: symbol.location.column,
            scope_start: symbol.scope_start.map(|loc| loc.line),
            scope_end: symbol.scope_end.map(|loc| loc.line),
            location: symbol.address.map(DwarfLocation::Address),
            is_artificial: symbol.is_artificial,
        };
        
        self.variables.push(variable);
    }

    /// Add a parameter symbol
    fn add_parameter_symbol(&mut self, symbol: DebugSymbol) {
        // Find the most recent function to add this parameter to
        if let Some(subprogram) = self.subprograms.last_mut() {
            let parameter = DwarfParameter {
                name: symbol.name,
                type_name: symbol.type_name,
                file: symbol.location.file,
                line: symbol.location.line,
                column: symbol.location.column,
                location: symbol.address.map(DwarfLocation::Address),
            };
            
            subprogram.parameters.push(parameter);
        } else {
            warn!(symbol = ?symbol, "Parameter symbol found but no function context");
        }
    }

    /// Add a type symbol
    fn add_type_symbol(&mut self, symbol: DebugSymbol) {
        let dwarf_type = DwarfType {
            name: symbol.name.clone(),
            size: symbol.size,
            alignment: None,
            encoding: self.infer_encoding(&symbol.type_name),
            base_type: None,
            members: Vec::new(),
        };
        
        self.types.insert(symbol.name, dwarf_type);
    }

    /// Infer DWARF encoding from type name
    fn infer_encoding(&self, type_name: &str) -> Option<DwarfEncoding> {
        match type_name {
            "bool" => Some(DwarfEncoding::Boolean),
            "i8" | "i16" | "i32" | "i64" | "isize" => Some(DwarfEncoding::Signed),
            "u8" | "u16" | "u32" | "u64" | "usize" => Some(DwarfEncoding::Unsigned),
            "f32" | "f64" => Some(DwarfEncoding::Float),
            "char" => Some(DwarfEncoding::UTF),
            "string" | "str" => Some(DwarfEncoding::UTF),
            _ => None,
        }
    }

    /// Generate LLVM debug metadata calls
    pub fn generate_llvm_metadata(&self) -> String {
        let mut output = String::new();
        
        // Generate compile unit metadata
        if let Some(cu) = &self.compile_unit {
            output.push_str(&self.generate_compile_unit_metadata(cu));
        }
        
        // Generate file metadata
        output.push_str(&self.generate_file_metadata());
        
        // Generate subprogram metadata
        for subprogram in &self.subprograms {
            output.push_str(&self.generate_subprogram_metadata(subprogram));
        }
        
        // Generate type metadata
        for dwarf_type in self.types.values() {
            output.push_str(&self.generate_type_metadata(dwarf_type));
        }
        
        output
    }

    /// Generate compile unit metadata
    fn generate_compile_unit_metadata(&self, cu: &DwarfCompileUnit) -> String {
        format!(
            r#"!0 = distinct !DICompileUnit(language: DW_LANG_lo_user, file: !1, producer: "{}", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug)
!1 = !DIFile(filename: "{}", directory: "{}")
"#,
            cu.producer,
            cu.file.file_name().unwrap_or_default().to_string_lossy(),
            cu.directory.to_string_lossy()
        )
    }

    /// Generate file metadata
    fn generate_file_metadata(&self) -> String {
        // This is typically generated as part of compile unit
        String::new()
    }

    /// Generate subprogram metadata
    fn generate_subprogram_metadata(&self, subprogram: &DwarfSubprogram) -> String {
        let mut metadata = format!(
            r#"!{} = distinct !DISubprogram(name: "{}", linkageName: "{}", scope: !1, file: !1, line: {}, type: !{}, scopeLine: {}, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0)
"#,
            self.next_metadata_id(),
            subprogram.name,
            subprogram.linkage_name.as_deref().unwrap_or(&subprogram.name),
            subprogram.line,
            self.next_metadata_id() + 1,
            subprogram.scope_line
        );

        // Add parameter metadata
        for (i, param) in subprogram.parameters.iter().enumerate() {
            metadata.push_str(&format!(
                r#"!{} = !DILocalVariable(name: "{}", arg: {}, scope: !{}, file: !1, line: {}, type: !{})
"#,
                self.next_metadata_id() + 2 + i,
                param.name,
                i + 1,
                self.next_metadata_id(),
                param.line,
                self.next_metadata_id() + 100 // Type metadata ID
            ));
        }

        metadata
    }

    /// Generate type metadata
    fn generate_type_metadata(&self, dwarf_type: &DwarfType) -> String {
        let encoding = dwarf_type.encoding.map(|e| e as u32).unwrap_or(0);
        format!(
            r#"!{} = !DIBasicType(name: "{}", size: {}, encoding: DW_ATE_{})
"#,
            self.next_metadata_id() + 100,
            dwarf_type.name,
            dwarf_type.size.unwrap_or(0) * 8, // Size in bits
            self.encoding_name(dwarf_type.encoding)
        )
    }

    /// Get encoding name for DWARF
    fn encoding_name(&self, encoding: Option<DwarfEncoding>) -> &'static str {
        match encoding {
            Some(DwarfEncoding::Boolean) => "boolean",
            Some(DwarfEncoding::Signed) => "signed",
            Some(DwarfEncoding::Unsigned) => "unsigned",
            Some(DwarfEncoding::Float) => "float",
            Some(DwarfEncoding::UTF) => "UTF",
            _ => "address",
        }
    }

    /// Get next metadata ID (simplified implementation)
    fn next_metadata_id(&self) -> usize {
        // This should be managed more carefully in a real implementation
        10 + self.subprograms.len() + self.types.len()
    }

    /// Generate debug location calls for LLVM IR
    pub fn generate_debug_location(&self, location: &SourceLocation) -> String {
        format!(
            ", !dbg !DILocation(line: {}, column: {}, scope: !{})",
            location.line,
            location.column,
            self.next_metadata_id()
        )
    }

    /// Generate line number information
    pub fn generate_line_table(&self) -> Vec<(u32, String)> {
        let mut line_table = Vec::new();
        
        for subprogram in &self.subprograms {
            line_table.push((
                subprogram.line,
                format!("{}:{}", subprogram.file.to_string_lossy(), subprogram.line)
            ));
        }
        
        line_table.sort_by_key(|(line, _)| *line);
        line_table
    }

    /// Clear all debug information
    pub fn clear(&mut self) {
        self.compile_unit = None;
        self.subprograms.clear();
        self.variables.clear();
        self.types.clear();
        self.current_file = None;
    }

    /// Get statistics about generated debug information
    pub fn statistics(&self) -> DwarfStatistics {
        DwarfStatistics {
            compile_units: if self.compile_unit.is_some() { 1 } else { 0 },
            subprograms: self.subprograms.len(),
            variables: self.variables.len(),
            types: self.types.len(),
            total_parameters: self.subprograms.iter().map(|s| s.parameters.len()).sum(),
        }
    }
}

impl Default for DwarfGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about generated DWARF information
#[derive(Debug, Clone)]
pub struct DwarfStatistics {
    pub compile_units: usize,
    pub subprograms: usize,
    pub variables: usize,
    pub types: usize,
    pub total_parameters: usize,
}

impl std::fmt::Display for DwarfStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DWARF Statistics: {} CUs, {} functions, {} variables, {} types, {} parameters",
            self.compile_units,
            self.subprograms,
            self.variables,
            self.types,
            self.total_parameters
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::{DebugSymbol, DebugSymbolType};
    use std::path::PathBuf;

    #[test]
    fn test_dwarf_generator_creation() {
        let generator = DwarfGenerator::new();
        assert!(generator.compile_unit.is_none());
        assert!(generator.subprograms.is_empty());
    }

    #[test]
    fn test_compile_unit_creation() {
        let mut generator = DwarfGenerator::new();
        let file = PathBuf::from("test.csd");
        let producer = "CURSED Compiler v1.0".to_string();
        
        generator.set_compile_unit(file.clone(), producer.clone());
        
        assert!(generator.compile_unit.is_some());
        let cu = generator.compile_unit.as_ref().unwrap();
        assert_eq!(cu.file, file);
        assert_eq!(cu.producer, producer);
    }

    #[test]
    fn test_symbol_addition() {
        let mut generator = DwarfGenerator::new();
        let location = SourceLocation::new(PathBuf::from("test.csd"), 10, 5);
        
        let symbols = vec![
            DebugSymbol::function("test_func".to_string(), location.clone()),
            DebugSymbol::variable("x".to_string(), "int".to_string(), location),
        ];
        
        generator.add_symbols(symbols);
        
        assert_eq!(generator.subprograms.len(), 1);
        assert_eq!(generator.variables.len(), 1);
        assert_eq!(generator.subprograms[0].name, "test_func");
        assert_eq!(generator.variables[0].name, "x");
    }

    #[test]
    fn test_type_encoding_inference() {
        let generator = DwarfGenerator::new();
        
        assert_eq!(generator.infer_encoding("bool"), Some(DwarfEncoding::Boolean));
        assert_eq!(generator.infer_encoding("i32"), Some(DwarfEncoding::Signed));
        assert_eq!(generator.infer_encoding("u64"), Some(DwarfEncoding::Unsigned));
        assert_eq!(generator.infer_encoding("f64"), Some(DwarfEncoding::Float));
        assert_eq!(generator.infer_encoding("string"), Some(DwarfEncoding::UTF));
        assert_eq!(generator.infer_encoding("custom"), None);
    }

    #[test]
    fn test_llvm_metadata_generation() {
        let mut generator = DwarfGenerator::new();
        generator.set_compile_unit(
            PathBuf::from("test.csd"),
            "CURSED Compiler".to_string()
        );
        
        let metadata = generator.generate_llvm_metadata();
        assert!(metadata.contains("!DICompileUnit"));
        assert!(metadata.contains("!DIFile"));
        assert!(metadata.contains("CURSED Compiler"));
    }

    #[test]
    fn test_debug_location_generation() {
        let generator = DwarfGenerator::new();
        let location = SourceLocation::new(PathBuf::from("test.csd"), 42, 10);
        
        let debug_loc = generator.generate_debug_location(&location);
        assert!(debug_loc.contains("line: 42"));
        assert!(debug_loc.contains("column: 10"));
    }

    #[test]
    fn test_statistics() {
        let mut generator = DwarfGenerator::new();
        let location = SourceLocation::new(PathBuf::from("test.csd"), 10, 5);
        
        generator.set_compile_unit(PathBuf::from("test.csd"), "Test".to_string());
        generator.add_symbols(vec![
            DebugSymbol::function("func".to_string(), location.clone()),
            DebugSymbol::variable("var".to_string(), "int".to_string(), location),
        ]);
        
        let stats = generator.statistics();
        assert_eq!(stats.compile_units, 1);
        assert_eq!(stats.subprograms, 1);
        assert_eq!(stats.variables, 1);
    }
}
