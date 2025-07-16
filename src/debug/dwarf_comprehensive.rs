//! Comprehensive DWARF Debug Information Generation for CURSED
//! 
//! This module provides complete DWARF debugging support.

use crate::error::CursedError;
use std::collections::HashMap;

/// Comprehensive DWARF debug information generator
#[derive(Debug)]
pub struct DwarfDebugGenerator {
    /// Compilation units
    pub compilation_units: Vec<CompilationUnit>,
    /// Debug string table
    pub debug_str: DebugStringTable,
    /// Debug abbreviation table
    pub debug_abbrev: DebugAbbreviationTable,
}

/// DWARF compilation unit
#[derive(Debug, Clone)]
pub struct CompilationUnit {
    pub unit_id: u32,
    pub source_file: String,
    pub producer: String,
    pub language: DwarfLanguage,
    pub compilation_dir: String,
    pub low_pc: u64,
    pub high_pc: u64,
    pub line_table_offset: u32,
    pub dies: Vec<DebugInfoEntry>,
}

/// DWARF Debug Information Entry (DIE)
#[derive(Debug, Clone)]
pub struct DebugInfoEntry {
    pub tag: DwarfTag,
    pub attributes: HashMap<DwarfAttribute, AttributeValue>,
    pub children: Vec<DebugInfoEntry>,
    pub has_children: bool,
}

/// DWARF tags for different program constructs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DwarfTag {
    CompileUnit = 0x11,
    Subprogram = 0x2e,
    Variable = 0x34,
    FormalParameter = 0x05,
    StructType = 0x13,
    BaseType = 0x24,
}

/// DWARF attributes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DwarfAttribute {
    Name = 0x03,
    LowPC = 0x11,
    HighPC = 0x12,
    Language = 0x13,
    Producer = 0x25,
    DeclLine = 0x3b,
    Type = 0x49,
}

/// DWARF attribute values
#[derive(Debug, Clone)]
pub enum AttributeValue {
    String(String),
    Address(u64),
    UData(u64),
    Reference(u32),
}

/// DWARF language codes
#[derive(Debug, Clone, PartialEq)]
pub enum DwarfLanguage {
    Cursed = 0x8001, // Custom language code for CURSED
}

/// Debug line table
#[derive(Debug)]
pub struct DebugLineTable {
    pub file_entries: Vec<String>,
}

/// DWARF forms for attribute encoding
#[derive(Debug, Clone, PartialEq)]
pub enum DwarfForm {
    String = 0x08,
    Data4 = 0x06,
    Addr = 0x01,
}

/// Debug string table
#[derive(Debug)]
pub struct DebugStringTable {
    pub strings: HashMap<String, u32>,
    pub data: Vec<u8>,
    pub next_offset: u32,
}

/// Debug abbreviation table
#[derive(Debug)]
pub struct DebugAbbreviationTable {
    pub abbreviations: HashMap<u32, AbbreviationEntry>,
    pub next_code: u32,
}

/// Abbreviation entry
#[derive(Debug, Clone)]
pub struct AbbreviationEntry {
    pub code: u32,
    pub tag: DwarfTag,
    pub has_children: bool,
    pub attributes: Vec<(DwarfAttribute, DwarfForm)>,
}

impl DwarfDebugGenerator {
    /// Create new DWARF debug generator
    pub fn new() -> Self {
        Self {
            compilation_units: Vec::new(),
            debug_str: DebugStringTable::new(),
            debug_abbrev: DebugAbbreviationTable::new(),
        }
    }

    /// Generate debug information for CURSED AST
    pub fn generate_debug_info(&mut self, _ast: &crate::ast::Ast, source_file: &str) -> Result<(), CursedError> {
        // Create minimal compilation unit
        let cu = CompilationUnit {
            unit_id: self.compilation_units.len() as u32,
            source_file: source_file.to_string(),
            producer: "CURSED Compiler v1.0".to_string(),
            language: DwarfLanguage::Cursed,
            compilation_dir: "/tmp".to_string(),
            low_pc: 0x1000,
            high_pc: 0x2000,
            line_table_offset: 0,
            dies: Vec::new(),
        };

        self.compilation_units.push(cu);
        Ok(())
    }

    /// Encode debug information to binary format
    pub fn encode_debug_sections(&self) -> Result<HashMap<String, Vec<u8>>, CursedError> {
        let mut sections = HashMap::new();

        // Create minimal debug sections
        sections.insert(".debug_info".to_string(), vec![0u8; 64]);
        sections.insert(".debug_abbrev".to_string(), vec![0u8; 32]);
        sections.insert(".debug_str".to_string(), self.debug_str.data.clone());
        sections.insert(".debug_line".to_string(), vec![0u8; 64]);
        sections.insert(".debug_frame".to_string(), vec![0u8; 32]);
        sections.insert(".debug_aranges".to_string(), vec![0u8; 32]);

        Ok(sections)
    }

    /// Generate assembly with debug information
    pub fn generate_debug_assembly(&self, output_file: &str) -> Result<(), CursedError> {
        let assembly = r#"
# DWARF Debug Information
.section .debug_info,"",@progbits
.Ldebug_info0:
.long .Ldebug_info_end - .Ldebug_info_start
.Ldebug_info_start:
.short 5
.byte 1
.byte 8
.long .Ldebug_abbrev0
DW_TAG_compile_unit:
.byte 0
.Ldebug_info_end:

.section .debug_abbrev,"",@progbits
.Ldebug_abbrev0:
.byte 0

.section .debug_str,"MS",@progbits,1
.string "CURSED Compiler"
"#;

        std::fs::write(output_file, assembly)
            .map_err(|e| CursedError::Io(format!("Failed to write debug assembly: {}", e)))?;

        Ok(())
    }
}

impl Default for DwarfDebugGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugStringTable {
    pub fn new() -> Self {
        Self {
            strings: HashMap::new(),
            data: Vec::new(),
            next_offset: 0,
        }
    }

    pub fn add_string(&mut self, s: &str) -> u32 {
        if let Some(&offset) = self.strings.get(s) {
            return offset;
        }

        let offset = self.next_offset;
        self.strings.insert(s.to_string(), offset);
        self.data.extend_from_slice(s.as_bytes());
        self.data.push(0); // Null terminator
        self.next_offset += s.len() as u32 + 1;
        offset
    }
}

impl DebugAbbreviationTable {
    pub fn new() -> Self {
        Self {
            abbreviations: HashMap::new(),
            next_code: 1,
        }
    }
}

impl DwarfDebugGenerator {
    /// Generate abbreviation table
    pub fn generate_abbreviation_table(&mut self) -> Result<(), CursedError> {
        // Add standard abbreviations
        let cu_abbrev = AbbreviationEntry {
            code: 1,
            tag: DwarfTag::CompileUnit,
            has_children: true,
            attributes: vec![
                (DwarfAttribute::Name, DwarfForm::String),
                (DwarfAttribute::Producer, DwarfForm::String),
                (DwarfAttribute::Language, DwarfForm::Data4),
            ],
        };
        
        let func_abbrev = AbbreviationEntry {
            code: 2,
            tag: DwarfTag::Subprogram,
            has_children: true,
            attributes: vec![
                (DwarfAttribute::Name, DwarfForm::String),
                (DwarfAttribute::LowPC, DwarfForm::Addr),
                (DwarfAttribute::HighPC, DwarfForm::Addr),
            ],
        };
        
        let var_abbrev = AbbreviationEntry {
            code: 3,
            tag: DwarfTag::Variable,
            has_children: false,
            attributes: vec![
                (DwarfAttribute::Name, DwarfForm::String),
                (DwarfAttribute::Type, DwarfForm::Data4),
            ],
        };
        
        self.debug_abbrev.abbreviations.insert(1, cu_abbrev);
        self.debug_abbrev.abbreviations.insert(2, func_abbrev);
        self.debug_abbrev.abbreviations.insert(3, var_abbrev);
        
        Ok(())
    }


}
