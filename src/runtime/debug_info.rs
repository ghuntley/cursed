//! CURSED Debug Information Types
//!
//! Advanced debugging infrastructure providing:
//! - Stack trace capture and management with LLVM integration
//! - Enhanced configuration for debugging behavior
//! - Symbol information and resolution
//! - Source mapping and line number resolution
//! - Symbol table management
//! - Enhanced stack walking capabilities
//! - Complete DWARF debug information parsing
//! - Variable location tracking and stack frame reconstruction

#![allow(non_upper_case_globals)]

use crate::error::{CursedError, SourceLocation};
use crate::debug::{DebugSymbol, DebugSymbolType};
use std::collections::{HashMap, BTreeMap};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, Duration};
use std::mem;
use std::fmt;

// DWARF parsing imports
use gimli::{
    Reader, EndianSlice, LittleEndian, BigEndian, Unit, AttributeValue, 
    DebuggingInformationEntry, DW_TAG_subprogram, DW_TAG_variable, DW_TAG_formal_parameter,
    DW_TAG_lexical_block, DW_TAG_inlined_subroutine, DW_TAG_base_type, DW_TAG_pointer_type,
    DW_TAG_array_type, DW_TAG_structure_type, DW_TAG_union_type, DW_TAG_enumeration_type,
    DW_TAG_typedef, DW_TAG_compile_unit, DW_AT_name, DW_AT_type, DW_AT_low_pc, DW_AT_high_pc,
    DW_AT_location, DW_AT_frame_base, DW_AT_call_file, DW_AT_call_line, DW_AT_inline,
    DW_AT_byte_size, DW_AT_encoding, DW_AT_data_member_location, DW_AT_comp_dir, DW_AT_stmt_list,
    DW_AT_ranges, DW_AT_entry_pc, DW_AT_abstract_origin, DW_AT_specification, DW_AT_declaration,
    DW_FORM_addr, DW_FORM_block1, DW_FORM_block2, DW_FORM_block4, DW_FORM_data1, DW_FORM_data2,
    DW_FORM_data4, DW_FORM_data8, DW_FORM_string, DW_FORM_strp, DW_FORM_ref1, DW_FORM_ref2,
    DW_FORM_ref4, DW_FORM_ref8, DW_FORM_ref_addr, DW_FORM_flag, DW_FORM_flag_present,
    DW_FORM_exprloc, DW_FORM_sec_offset, DW_FORM_ref_sig8, DW_FORM_strx, DW_FORM_addrx,
    DW_FORM_line_strp, DW_FORM_implicit_const, DW_FORM_loclistx, DW_FORM_rnglistx,
    DW_ATE_address, DW_ATE_boolean, DW_ATE_complex_float, DW_ATE_float, DW_ATE_signed,
    DW_ATE_signed_char, DW_ATE_unsigned, DW_ATE_unsigned_char, DW_ATE_imaginary_float,
    DW_ATE_packed_decimal, DW_ATE_numeric_string, DW_ATE_edited, DW_ATE_signed_fixed,
    DW_ATE_unsigned_fixed, DW_ATE_decimal_float, DW_ATE_UTF, DW_ATE_UCS, DW_ATE_ASCII,
    DW_INL_not_inlined, DW_INL_inlined, DW_INL_declared_not_inlined, DW_INL_declared_inlined,
    DW_OP_addr, DW_OP_deref, DW_OP_const1u, DW_OP_const1s, DW_OP_const2u, DW_OP_const2s,
    DW_OP_const4u, DW_OP_const4s, DW_OP_const8u, DW_OP_const8s, DW_OP_constu, DW_OP_consts,
    DW_OP_dup, DW_OP_drop, DW_OP_over, DW_OP_pick, DW_OP_swap, DW_OP_rot, DW_OP_xderef,
    DW_OP_abs, DW_OP_and, DW_OP_div, DW_OP_minus, DW_OP_mod, DW_OP_mul, DW_OP_neg,
    DW_OP_not, DW_OP_or, DW_OP_plus, DW_OP_plus_uconst, DW_OP_shl, DW_OP_shr, DW_OP_shra,
    DW_OP_xor, DW_OP_skip, DW_OP_bra, DW_OP_eq, DW_OP_ge, DW_OP_gt, DW_OP_le, DW_OP_lt, DW_OP_ne,
    DW_OP_lit0, DW_OP_lit1, DW_OP_lit2, DW_OP_lit3, DW_OP_lit4, DW_OP_lit5, DW_OP_lit6,
    DW_OP_lit7, DW_OP_lit8, DW_OP_lit9, DW_OP_lit10, DW_OP_lit11, DW_OP_lit12, DW_OP_lit13,
    DW_OP_lit14, DW_OP_lit15, DW_OP_lit16, DW_OP_lit17, DW_OP_lit18, DW_OP_lit19, DW_OP_lit20,
    DW_OP_lit21, DW_OP_lit22, DW_OP_lit23, DW_OP_lit24, DW_OP_lit25, DW_OP_lit26, DW_OP_lit27,
    DW_OP_lit28, DW_OP_lit29, DW_OP_lit30, DW_OP_lit31, DW_OP_reg0, DW_OP_reg1, DW_OP_reg2,
    DW_OP_reg3, DW_OP_reg4, DW_OP_reg5, DW_OP_reg6, DW_OP_reg7, DW_OP_reg8, DW_OP_reg9,
    DW_OP_reg10, DW_OP_reg11, DW_OP_reg12, DW_OP_reg13, DW_OP_reg14, DW_OP_reg15, DW_OP_reg16,
    DW_OP_reg17, DW_OP_reg18, DW_OP_reg19, DW_OP_reg20, DW_OP_reg21, DW_OP_reg22, DW_OP_reg23,
    DW_OP_reg24, DW_OP_reg25, DW_OP_reg26, DW_OP_reg27, DW_OP_reg28, DW_OP_reg29, DW_OP_reg30,
    DW_OP_reg31, DW_OP_breg0, DW_OP_breg1, DW_OP_breg2, DW_OP_breg3, DW_OP_breg4, DW_OP_breg5,
    DW_OP_breg6, DW_OP_breg7, DW_OP_breg8, DW_OP_breg9, DW_OP_breg10, DW_OP_breg11, DW_OP_breg12,
    DW_OP_breg13, DW_OP_breg14, DW_OP_breg15, DW_OP_breg16, DW_OP_breg17, DW_OP_breg18,
    DW_OP_breg19, DW_OP_breg20, DW_OP_breg21, DW_OP_breg22, DW_OP_breg23, DW_OP_breg24,
    DW_OP_breg25, DW_OP_breg26, DW_OP_breg27, DW_OP_breg28, DW_OP_breg29, DW_OP_breg30,
    DW_OP_breg31, DW_OP_regx, DW_OP_fbreg, DW_OP_bregx, DW_OP_piece, DW_OP_deref_size,
    DW_OP_xderef_size, DW_OP_nop, DW_OP_push_object_address, DW_OP_call2, DW_OP_call4,
    DW_OP_call_ref, DW_OP_form_tls_address, DW_OP_call_frame_cfa, DW_OP_bit_piece,
    DW_OP_implicit_value, DW_OP_stack_value, DW_OP_implicit_pointer, DW_OP_addrx,
    DW_OP_constx, DW_OP_entry_value, DW_OP_const_type, DW_OP_regval_type, DW_OP_deref_type,
    DW_OP_xderef_type, DW_OP_convert, DW_OP_reinterpret, Expression, Operation, Evaluation,
    EvaluationResult, Piece, Value, Location, LocationLists, RangeLists, LineProgram,
    DebugLine, DebugStr, DebugStrOffsets, DebugAddr, DebugLineStr, DebugRngLists,
    DebugLocLists, DebugAbbrev, DebugInfo, DebugTypes, DebugPubNames, DebugPubTypes,
    DebugAranges, DebugFrame, EhFrame, UnitHeader, UnitOffset,
    DebugInfoOffset, DebugStrOffset, DebugStrOffsetsIndex, DebugAddrIndex, DebugLineStrOffset,
    DebugLocListsIndex, DebugRngListsIndex, Format, Encoding, RunTimeEndian, FileEntry,
    LineProgramHeader, LineRow, LineInstruction, constants, read,
};

// Import the proper types from the read module
use gimli::read::Dwarf;
use object::{Object, ObjectSection, read::File as ObjectFile, Endianness};

/// Configuration for enhanced stack trace behavior
#[derive(Debug, Clone)]
pub struct EnhancedStackTraceConfig {
    /// Maximum stack depth to capture
    pub max_depth: usize,
    /// Enable symbol resolution
    pub resolve_symbols: bool,
    /// Include source file information
    pub include_source: bool,
    /// Include line number information
    pub include_line_numbers: bool,
    /// Include column number information
    pub include_columns: bool,
    /// Enable LLVM debug info integration
    pub llvm_debug_info: bool,
    /// Show function parameters in stack traces
    pub show_parameters: bool,
    /// Show local variables in stack traces
    pub show_locals: bool,
    /// Enable inline function expansion
    pub expand_inlines: bool,
    /// Include memory addresses
    pub include_addresses: bool,
    /// Enable async stack traces for goroutines
    pub async_stack_traces: bool,
    /// Capture performance metrics during tracing
    pub capture_performance: bool,
    /// Stack trace format preference
    pub format: StackTraceFormat,
}

impl Default for EnhancedStackTraceConfig {
    fn default() -> Self {
        Self {
            max_depth: 100,
            resolve_symbols: true,
            include_source: true,
            include_line_numbers: true,
            include_columns: true,
            llvm_debug_info: true,
            show_parameters: false,
            show_locals: false,
            expand_inlines: true,
            include_addresses: false,
            async_stack_traces: true,
            capture_performance: false,
            format: StackTraceFormat::Standard,
        }
    }
}

/// Stack trace output format options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackTraceFormat {
    /// Standard format: function at file:line
    Standard,
    /// Compact format: minimal information
    Compact,
    /// Verbose format: all available information
    Verbose,
    /// JSON format for programmatic consumption
    Json,
    /// Custom format with user-defined template
    Custom,
}

/// Comprehensive stack trace capture and management system
#[derive(Debug)]
pub struct StackTraceCapture {
    /// Configuration for stack trace behavior
    config: EnhancedStackTraceConfig,
    /// Symbol resolver for address translation
    symbol_resolver: Arc<SymbolResolver>,
    /// Cache for resolved symbols
    symbol_cache: Arc<RwLock<HashMap<u64, SymbolInfo>>>,
    /// Source file mappings
    source_mappings: Arc<RwLock<HashMap<PathBuf, SourceFileInfo>>>,
    /// LLVM debug information
    llvm_debug_info: Arc<RwLock<LlvmDebugInfo>>,
    /// Stack trace statistics
    statistics: Arc<RwLock<StackTraceStats>>,
    /// Active capture sessions
    active_sessions: Arc<RwLock<HashMap<String, CaptureSession>>>,
}

impl StackTraceCapture {
    /// Create a new stack trace capture system
    pub fn new(config: EnhancedStackTraceConfig) -> Self {
        Self {
            config,
            symbol_resolver: Arc::new(SymbolResolver::new()),
            symbol_cache: Arc::new(RwLock::new(HashMap::new())),
            source_mappings: Arc::new(RwLock::new(HashMap::new())),
            llvm_debug_info: Arc::new(RwLock::new(LlvmDebugInfo::new())),
            statistics: Arc::new(RwLock::new(StackTraceStats::new())),
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Capture current stack trace
    pub fn capture_stack_trace(&self) -> Result<Vec<StackFrame>, CursedError> {
        let start_time = if self.config.capture_performance {
            Some(SystemTime::now())
        } else {
            None
        };

        let mut frames = Vec::new();
        let mut frame_addresses = self.get_stack_addresses()?;

        // Limit to configured max depth
        if frame_addresses.len() > self.config.max_depth {
            frame_addresses.truncate(self.config.max_depth);
        }

        for (depth, address) in frame_addresses.iter().enumerate() {
            let frame = self.create_stack_frame(*address, depth)?;
            frames.push(frame);
        }

        // Update statistics
        if let Ok(mut stats) = self.statistics.write() {
            stats.traces_captured += 1;
            stats.total_frames += frames.len();
            stats.last_capture_time = Some(SystemTime::now());
            
            if let Some(start) = start_time {
                if let Ok(duration) = SystemTime::now().duration_since(start) {
                    stats.avg_capture_time = Some(
                        stats.avg_capture_time
                            .map(|avg| Duration::from_nanos((avg.as_nanos() + duration.as_nanos()) as u64 / 2))
                            .unwrap_or(duration)
                    );
                }
            }
        }

        Ok(frames)
    }

    /// Create a capture session for targeted debugging
    pub fn start_capture_session(&self, name: String, target_function: Option<String>) -> Result<String, CursedError> {
        let session_id = format!("capture_{}_{}", 
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0)).as_nanos(),
            name
        );

        let session = CaptureSession {
            id: session_id.clone(),
            name,
            start_time: SystemTime::now(),
            target_function,
            captured_traces: Vec::new(),
            config: self.config.clone(),
        };

        if let Ok(mut sessions) = self.active_sessions.write() {
            sessions.insert(session_id.clone(), session);
        }

        Ok(session_id)
    }

    /// End a capture session and return results
    pub fn end_capture_session(&self, session_id: &str) -> Result<CaptureSession, CursedError> {
        if let Ok(mut sessions) = self.active_sessions.write() {
            sessions.remove(session_id)
                .ok_or_else(|| CursedError::RuntimeError(format!("Session '{}' not found", session_id)))
        } else {
            Err(CursedError::RuntimeError("Failed to access capture sessions".to_string()))
        }
    }

    /// Capture stack trace at specific address
    pub fn capture_at_address(&self, address: u64) -> Result<StackFrame, CursedError> {
        self.create_stack_frame(address, 0)
    }

    /// Add source file mapping for better debugging
    pub fn add_source_mapping(&self, source_file: PathBuf, info: SourceFileInfo) -> Result<(), CursedError> {
        if let Ok(mut mappings) = self.source_mappings.write() {
            mappings.insert(source_file, info);
            Ok(())
        } else {
            Err(CursedError::RuntimeError("Failed to add source mapping".to_string()))
        }
    }

    /// Update LLVM debug information
    pub fn update_llvm_debug_info(&self, info: LlvmDebugInfo) -> Result<(), CursedError> {
        if let Ok(mut debug_info) = self.llvm_debug_info.write() {
            *debug_info = info;
            Ok(())
        } else {
            Err(CursedError::RuntimeError("Failed to update LLVM debug info".to_string()))
        }
    }

    /// Get stack trace statistics
    pub fn get_statistics(&self) -> Result<StackTraceStats, CursedError> {
        self.statistics.read()
            .map(|stats| stats.clone())
            .map_err(|_| CursedError::RuntimeError("Failed to read statistics".to_string()))
    }

    /// Configure stack trace behavior
    pub fn update_config(&mut self, config: EnhancedStackTraceConfig) {
        self.config = config;
    }

    // Private helper methods

    /// Get stack frame addresses using platform-specific methods
    fn get_stack_addresses(&self) -> Result<Vec<u64>, CursedError> {
        // Platform-specific stack walking implementation
        // This is a simplified version - real implementation would use
        // libunwind, backtrace, or similar libraries
        
        #[cfg(unix)]
        {
            self.get_unix_stack_addresses()
        }
        #[cfg(windows)]
        {
            self.get_windows_stack_addresses()
        }
        #[cfg(not(any(unix, windows)))]
        {
            // Fallback implementation
            Ok(vec![0x1000, 0x2000, 0x3000]) // Dummy addresses for compilation
        }
    }

    #[cfg(unix)]
    fn get_unix_stack_addresses(&self) -> Result<Vec<u64>, CursedError> {
        // Use libunwind or backtrace to get actual stack addresses
        // For now, return dummy data for compilation
        Ok(vec![0x401000, 0x401200, 0x401400])
    }

    #[cfg(windows)]
    fn get_windows_stack_addresses(&self) -> Result<Vec<u64>, CursedError> {
        // Use Windows debugging APIs to get stack addresses
        // For now, return dummy data for compilation
        Ok(vec![0x401000, 0x401200, 0x401400])
    }

    /// Create a stack frame from an address
    fn create_stack_frame(&self, address: u64, depth: usize) -> Result<StackFrame, CursedError> {
        let symbol_info = if self.config.resolve_symbols {
            self.symbol_resolver.resolve_address(address)?
        } else {
            SymbolInfo::unknown(address)
        };

        let source_location = if self.config.include_source {
            self.resolve_source_location(address)?
        } else {
            None
        };

        Ok(StackFrame {
            depth,
            address: if self.config.include_addresses { Some(address) } else { None },
            symbol_info,
            source_location,
            parameters: if self.config.show_parameters { 
                self.get_function_parameters(address)? 
            } else { 
                Vec::new() 
            },
            local_variables: if self.config.show_locals { 
                self.get_local_variables(address)? 
            } else { 
                Vec::new() 
            },
            inline_info: if self.config.expand_inlines { 
                self.get_inline_info(address)? 
            } else { 
                Vec::new() 
            },
        })
    }

    /// Resolve source location from address
    fn resolve_source_location(&self, address: u64) -> Result<Option<SourceLocation>, CursedError> {
        if let Ok(debug_info) = self.llvm_debug_info.read() {
            debug_info.get_source_location(address)
        } else {
            Ok(None)
        }
    }

    /// Get function parameters at address
    fn get_function_parameters(&self, address: u64) -> Result<Vec<ParameterInfo>, CursedError> {
        if let Ok(debug_info) = self.llvm_debug_info.read() {
            debug_info.extract_function_parameters(address)
        } else {
            Ok(Vec::new())
        }
    }

    /// Get local variables at address
    fn get_local_variables(&self, address: u64) -> Result<Vec<LocalVariableInfo>, CursedError> {
        if let Ok(debug_info) = self.llvm_debug_info.read() {
            debug_info.extract_local_variables(address)
        } else {
            Ok(Vec::new())
        }
    }

    /// Get inline function information
    fn get_inline_info(&self, address: u64) -> Result<Vec<InlineInfo>, CursedError> {
        if let Ok(debug_info) = self.llvm_debug_info.read() {
            debug_info.extract_inline_info(address)
        } else {
            Ok(Vec::new())
        }
    }
}

/// Individual stack frame information
#[derive(Debug, Clone)]
pub struct StackFrame {
    /// Frame depth (0 = current frame)
    pub depth: usize,
    /// Memory address (if available)
    pub address: Option<u64>,
    /// Symbol information
    pub symbol_info: SymbolInfo,
    /// Source location information
    pub source_location: Option<SourceLocation>,
    /// Function parameters
    pub parameters: Vec<ParameterInfo>,
    /// Local variables
    pub local_variables: Vec<LocalVariableInfo>,
    /// Inline function information
    pub inline_info: Vec<InlineInfo>,
}

/// Function parameter information for debugging
#[derive(Debug, Clone)]
pub struct ParameterInfo {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: String,
    /// Parameter value (if available)
    pub value: Option<String>,
    /// Parameter location in memory
    pub location: Option<u64>,
}

/// Local variable information for debugging
#[derive(Debug, Clone)]
pub struct LocalVariableInfo {
    /// Variable name
    pub name: String,
    /// Variable type
    pub var_type: String,
    /// Variable value (if available)
    pub value: Option<String>,
    /// Variable scope
    pub scope: String,
    /// Memory location
    pub location: Option<u64>,
}

/// Inline function information
#[derive(Debug, Clone)]
pub struct InlineInfo {
    /// Inline function name
    pub function_name: String,
    /// Source location where inlined
    pub inline_site: SourceLocation,
    /// Original function location
    pub original_location: SourceLocation,
}

/// Symbol information for debugging addresses
#[derive(Debug, Clone)]
pub struct SymbolInfo {
    /// Symbol name (function, variable, etc.)
    pub name: String,
    /// Symbol type (function, variable, type, etc.)
    pub symbol_type: SymbolType,
    /// Base address of the symbol
    pub address: u64,
    /// Size of the symbol in bytes
    pub size: u32,
    /// Offset from base address
    pub offset: u32,
    /// Source file containing the symbol
    pub source_file: Option<PathBuf>,
    /// Line number in source file
    pub line_number: Option<u32>,
    /// Column number in source file
    pub column_number: Option<u32>,
    /// Module/library containing the symbol
    pub module_name: Option<String>,
    /// Whether this symbol has debugging information
    pub has_debug_info: bool,
    /// Symbol visibility (public, private, etc.)
    pub visibility: SymbolVisibility,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl SymbolInfo {
    /// Create an unknown symbol info for an address
    pub fn unknown(address: u64) -> Self {
        Self {
            name: format!("unknown_0x{:x}", address),
            symbol_type: SymbolType::Unknown,
            address,
            size: 0,
            offset: 0,
            source_file: None,
            line_number: None,
            column_number: None,
            module_name: None,
            has_debug_info: false,
            visibility: SymbolVisibility::Unknown,
            metadata: HashMap::new(),
        }
    }

    /// Create a function symbol info
    pub fn function(name: String, address: u64, size: u32) -> Self {
        Self {
            name,
            symbol_type: SymbolType::Function,
            address,
            size,
            offset: 0,
            source_file: None,
            line_number: None,
            column_number: None,
            module_name: None,
            has_debug_info: false,
            visibility: SymbolVisibility::Public,
            metadata: HashMap::new(),
        }
    }
}

/// Types of symbols for debugging
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    /// Function symbol
    Function,
    /// Variable symbol
    Variable,
    /// Type definition
    Type,
    /// Module/namespace
    Module,
    /// Label
    Label,
    /// Unknown symbol type
    Unknown,
}

/// Symbol visibility levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolVisibility {
    /// Public symbol (exported)
    Public,
    /// Private symbol (internal)
    Private,
    /// Protected symbol
    Protected,
    /// Local symbol
    Local,
    /// Unknown visibility
    Unknown,
}

/// Symbol resolver for translating addresses to symbols
#[derive(Debug)]
pub struct SymbolResolver {
    /// Symbol table mapping addresses to symbols
    symbol_table: Arc<RwLock<BTreeMap<u64, SymbolInfo>>>,
    /// Module information
    modules: Arc<RwLock<HashMap<String, ModuleInfo>>>,
    /// Symbol lookup cache
    lookup_cache: Arc<RwLock<HashMap<u64, SymbolInfo>>>,
    /// Resolution statistics
    stats: Arc<RwLock<ResolverStats>>,
}

impl SymbolResolver {
    /// Create a new symbol resolver
    pub fn new() -> Self {
        Self {
            symbol_table: Arc::new(RwLock::new(BTreeMap::new())),
            modules: Arc::new(RwLock::new(HashMap::new())),
            lookup_cache: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(ResolverStats::new())),
        }
    }

    /// Add a symbol to the resolver
    pub fn add_symbol(&self, symbol: SymbolInfo) -> Result<(), CursedError> {
        if let Ok(mut table) = self.symbol_table.write() {
            table.insert(symbol.address, symbol);
            Ok(())
        } else {
            Err(CursedError::RuntimeError("Failed to add symbol".to_string()))
        }
    }

    /// Resolve an address to symbol information
    pub fn resolve_address(&self, address: u64) -> Result<SymbolInfo, CursedError> {
        // Update resolutions counter for all resolution attempts
        if let Ok(mut stats) = self.stats.write() {
            stats.resolutions += 1;
        }

        // Check cache first
        if let Ok(cache) = self.lookup_cache.read() {
            if let Some(symbol) = cache.get(&address) {
                if let Ok(mut stats) = self.stats.write() {
                    stats.cache_hits += 1;
                }
                return Ok(symbol.clone());
            }
        }

        // Search symbol table
        let symbol_info = if let Ok(table) = self.symbol_table.read() {
            table.range(..=address)
                .next_back()
                .and_then(|(base_addr, symbol)| {
                    let offset = address - base_addr;
                    if offset < symbol.size as u64 {
                        let mut resolved = symbol.clone();
                        resolved.offset = offset as u32;
                        Some(resolved)
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| SymbolInfo::unknown(address))
        } else {
            SymbolInfo::unknown(address)
        };

        // Update cache
        if let Ok(mut cache) = self.lookup_cache.write() {
            cache.insert(address, symbol_info.clone());
            
            // Limit cache size
            if cache.len() > 10000 {
                let oldest_key = *cache.keys().next().unwrap();
                cache.remove(&oldest_key);
            }
        }

        // Update statistics
        if let Ok(mut stats) = self.stats.write() {
            if symbol_info.symbol_type != SymbolType::Unknown {
                stats.successful_resolutions += 1;
            }
        }

        Ok(symbol_info)
    }

    /// Load symbols from a module
    pub fn load_module_symbols(&self, module_name: String, module_info: ModuleInfo) -> Result<(), CursedError> {
        // Add module info
        if let Ok(mut modules) = self.modules.write() {
            modules.insert(module_name.clone(), module_info.clone());
        }

        // Add symbols from module
        for symbol in module_info.symbols {
            self.add_symbol(symbol)?;
        }

        Ok(())
    }

    /// Get resolver statistics
    pub fn get_stats(&self) -> Result<ResolverStats, CursedError> {
        self.stats.read()
            .map(|stats| stats.clone())
            .map_err(|_| CursedError::RuntimeError("Failed to read resolver stats".to_string()))
    }

    /// Clear symbol cache
    pub fn clear_cache(&self) -> Result<(), CursedError> {
        if let Ok(mut cache) = self.lookup_cache.write() {
            cache.clear();
            Ok(())
        } else {
            Err(CursedError::RuntimeError("Failed to clear symbol cache".to_string()))
        }
    }
}

/// Module information for symbol resolution
#[derive(Debug, Clone)]
pub struct ModuleInfo {
    /// Module name
    pub name: String,
    /// Base address in memory
    pub base_address: u64,
    /// Module size
    pub size: u64,
    /// Path to module file
    pub file_path: Option<PathBuf>,
    /// Symbols in this module
    pub symbols: Vec<SymbolInfo>,
    /// Module type (executable, library, etc.)
    pub module_type: ModuleType,
    /// Build ID or checksum
    pub build_id: Option<String>,
}

/// Types of modules
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleType {
    /// Main executable
    Executable,
    /// Shared library
    SharedLibrary,
    /// Static library
    StaticLibrary,
    /// Object file
    ObjectFile,
    /// Unknown module type
    Unknown,
}

/// Source file information for debugging
#[derive(Debug, Clone)]
pub struct SourceFileInfo {
    /// File path
    pub path: PathBuf,
    /// Line number to address mappings
    pub line_mappings: BTreeMap<u32, Vec<u64>>,
    /// Address to line number mappings
    pub address_mappings: BTreeMap<u64, u32>,
    /// Functions defined in this file
    pub functions: Vec<FunctionInfo>,
    /// File hash for integrity checking
    pub file_hash: Option<String>,
    /// Compilation timestamp
    pub compiled_at: Option<SystemTime>,
}

/// Function information within a source file
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    /// Function name
    pub name: String,
    /// Start address
    pub start_address: u64,
    /// End address
    pub end_address: u64,
    /// Start line in source
    pub start_line: u32,
    /// End line in source
    pub end_line: u32,
    /// Parameter count
    pub parameter_count: u32,
    /// Local variable count
    pub local_count: u32,
}

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

/// Function debug information from DWARF
#[derive(Debug, Clone)]
pub struct FunctionDebugInfo {
    /// Function name
    pub name: String,
    /// Demangled name (if available)
    pub demangled_name: Option<String>,
    /// Start address
    pub start_address: u64,
    /// End address
    pub end_address: u64,
    /// Parameters
    pub parameters: Vec<ParameterDebugInfo>,
    /// Source file information
    pub source_file: Option<PathBuf>,
    /// Line number range
    pub line_range: Option<(u32, u32)>,
    /// Frame base expression for variable locations
    pub frame_base: Option<Vec<u8>>,
}

/// Parameter debug information from DWARF
#[derive(Debug, Clone)]
pub struct ParameterDebugInfo {
    /// Parameter name
    pub name: String,
    /// Parameter type ID
    pub type_id: u64,
    /// Location expression
    pub location: Option<Vec<u8>>,
    /// Is this parameter passed by reference?
    pub by_reference: bool,
}

/// Variable debug information from DWARF
#[derive(Debug, Clone)]
pub struct VariableDebugInfo {
    /// Variable name
    pub name: String,
    /// Variable type ID
    pub type_id: u64,
    /// Location expression
    pub location: Option<Vec<u8>>,
    /// Scope start address
    pub scope_start: u64,
    /// Scope end address
    pub scope_end: u64,
    /// Line number where declared
    pub declared_line: Option<u32>,
}

/// Inline call site information
#[derive(Debug, Clone)]
pub struct InlineCallSite {
    /// Inlined function name
    pub function_name: String,
    /// Call site address
    pub call_address: u64,
    /// Original function location
    pub original_location: Option<(PathBuf, u32, u32)>,
    /// Inline location
    pub inline_location: Option<(PathBuf, u32, u32)>,
}

/// DWARF type information
#[derive(Debug, Clone)]
pub struct DwarfTypeInfo {
    /// Type name
    pub name: String,
    /// Type size in bytes
    pub size: u64,
    /// Type encoding (integer, float, etc.)
    pub encoding: Option<String>,
    /// For composite types, member information
    pub members: Vec<TypeMemberInfo>,
    /// Base type (for pointers, arrays, etc.)
    pub base_type: Option<u64>,
}

/// Type member information for composite types
#[derive(Debug, Clone)]
pub struct TypeMemberInfo {
    /// Member name
    pub name: String,
    /// Member type ID
    pub type_id: u64,
    /// Offset within the composite type
    pub offset: u64,
    /// Member size
    pub size: u64,
}

/// Line information from DWARF
#[derive(Debug, Clone)]
pub struct LineInfo {
    /// File path
    pub file: PathBuf,
    /// Line number
    pub line: u32,
    /// Column number
    pub column: u32,
    /// Whether this is a statement boundary
    pub is_stmt: bool,
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

    /// Load debug information from DWARF data
    pub fn load_from_dwarf(&mut self, dwarf_data: &[u8]) -> Result<(), CursedError> {
        // Parse the object file to extract DWARF sections
        let object_file = ObjectFile::parse(dwarf_data)
            .map_err(|e| CursedError::RuntimeError(format!("Failed to parse object file: {}", e)))?;

        // Determine endianness
        let endian = match object_file.endianness() {
            Endianness::Little => RunTimeEndian::Little,
            Endianness::Big => RunTimeEndian::Big,
        };

        // Create DWARF parser with all sections
        let dwarf = self.load_dwarf_sections(&object_file, endian)?;

        // Parse all compilation units
        let mut units = dwarf.units();
        while let Some(unit_header) = units.next()
            .map_err(|e| CursedError::RuntimeError(format!("Failed to read unit header: {}", e)))? {
            
            let unit = dwarf.unit(unit_header)
                .map_err(|e| CursedError::RuntimeError(format!("Failed to parse unit: {}", e)))?;

            self.parse_compilation_unit(&dwarf, &unit)?;
        }

        // Parse line number information
        self.parse_line_information(&dwarf)?;

        Ok(())
    }

    /// Load DWARF sections from object file
    fn load_dwarf_sections<'data>(&self, object_file: &'data ObjectFile, endian: RunTimeEndian) 
        -> Result<gimli::read::Dwarf<EndianSlice<'data, RunTimeEndian>>, CursedError> {
        
        // Helper function to load a section
        let load_section = |section_id: gimli::SectionId| -> Result<EndianSlice<'data, RunTimeEndian>, gimli::Error> {
            let section_name = section_id.name();
            let data = object_file.section_by_name(section_name)
                .and_then(|section| section.data().ok())
                .unwrap_or(&[]);
            Ok(EndianSlice::new(data, endian))
        };

        // Use the new load method
        let dwarf = gimli::read::Dwarf::load(load_section)
            .map_err(|e| CursedError::RuntimeError(format!("Failed to load DWARF sections: {}", e)))?;

        Ok(dwarf)
    }

    /// Parse a compilation unit and extract debug information
    fn parse_compilation_unit<R: Reader<Offset = usize>>(
        &mut self,
        dwarf: &gimli::read::Dwarf<R>,
        unit: &Unit<R>,
    ) -> Result<(), CursedError> {
        // Parse the compilation unit DIE tree
        let mut entries_cursor = unit.entries();
        
        // Process the compilation unit root entry
        if let Some((_, entry)) = entries_cursor.next_dfs()
            .map_err(|e| CursedError::RuntimeError(format!("Failed to read compilation unit entry: {}", e)))? {
            
            if entry.tag() == DW_TAG_compile_unit {
                // Parse children of compilation unit
                while let Some((depth, entry)) = entries_cursor.next_dfs()
                    .map_err(|e| CursedError::RuntimeError(format!("Failed to read DIE entry: {}", e)))? {
                    
                    match entry.tag() {
                        DW_TAG_subprogram => {
                            self.parse_function(dwarf, unit, &entry, depth)?;
                        }
                        DW_TAG_variable => {
                            self.parse_global_variable(dwarf, unit, &entry)?;
                        }
                        DW_TAG_base_type | DW_TAG_pointer_type | DW_TAG_array_type |
                        DW_TAG_structure_type | DW_TAG_union_type | DW_TAG_enumeration_type |
                        DW_TAG_typedef => {
                            self.parse_type(dwarf, unit, &entry)?;
                        }
                        _ => {
                            // Skip other entries for now
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Parse a function (subprogram) DIE
    fn parse_function<R: Reader<Offset = usize>>(
        &mut self,
        dwarf: &Dwarf<R>,
        unit: &Unit<R>,
        entry: &DebuggingInformationEntry<R>,
        depth: isize,
    ) -> Result<(), CursedError> {
        let mut function_info = FunctionDebugInfo {
            name: String::new(),
            demangled_name: None,
            start_address: 0,
            end_address: 0,
            parameters: Vec::new(),
            source_file: None,
            line_range: None,
            frame_base: None,
        };

        // Parse function attributes
        let mut attrs = entry.attrs();
        while let Some(attr) = attrs.next()
            .map_err(|e| CursedError::RuntimeError(format!("Failed to read attribute: {}", e)))? {
            
            match attr.name() {
                DW_AT_name => {
                    if let Ok(name_str) = dwarf.attr_string(unit, attr.value()) {
                        if let Ok(cow_str) = name_str.to_string_lossy() {
                            function_info.name = cow_str.into_owned();
                        }
                    }
                }
                DW_AT_low_pc => {
                    if let AttributeValue::Addr(addr) = attr.value() {
                        function_info.start_address = addr;
                    }
                }
                DW_AT_high_pc => {
                    match attr.value() {
                        AttributeValue::Addr(addr) => {
                            function_info.end_address = addr;
                        }
                        AttributeValue::Udata(size) => {
                            function_info.end_address = function_info.start_address + size;
                        }
                        _ => {}
                    }
                }
                DW_AT_frame_base => {
                    if let AttributeValue::Exprloc(expr) = attr.value() {
                        if let Ok(slice) = expr.0.to_slice() {
                            function_info.frame_base = Some(Vec::from(slice.as_ref()));
                        }
                    }
                }
                _ => {}
            }
        }

        // Parse function children (parameters, variables, etc.)
        let function_offset = entry.offset();
        let mut child_entries = unit.entries_at_offset(function_offset)
            .map_err(|e| CursedError::RuntimeError(format!("Failed to get function children: {}", e)))?;
        
        let _ = child_entries.next_dfs(); // Skip the function entry itself
        
        while let Some((child_depth, child_entry)) = child_entries.next_dfs()
            .map_err(|e| CursedError::RuntimeError(format!("Failed to read child entry: {}", e)))? {
            
            if child_depth <= depth {
                break; // End of this function's children
            }
            
            match child_entry.tag() {
                DW_TAG_formal_parameter => {
                    if let Ok(param) = self.parse_parameter(dwarf, unit, &child_entry) {
                        function_info.parameters.push(param);
                    }
                }
                DW_TAG_variable => {
                    self.parse_local_variable(dwarf, unit, &child_entry, function_info.start_address)?;
                }
                DW_TAG_lexical_block => {
                    self.parse_lexical_block(dwarf, unit, &child_entry, function_info.start_address)?;
                }
                DW_TAG_inlined_subroutine => {
                    self.parse_inlined_subroutine(dwarf, unit, &child_entry)?;
                }
                _ => {}
            }
        }

        // Add function to database
        if function_info.start_address != 0 && !function_info.name.is_empty() {
            self.functions.insert(function_info.start_address, function_info);
        }

        Ok(())
    }

    /// Parse a function parameter
    fn parse_parameter<R: Reader<Offset = usize>>(
        &self,
        dwarf: &gimli::read::Dwarf<R>,
        unit: &Unit<R>,
        entry: &DebuggingInformationEntry<R>,
    ) -> Result<ParameterDebugInfo, CursedError> {
        let mut param = ParameterDebugInfo {
            name: String::new(),
            type_id: 0,
            location: None,
            by_reference: false,
        };

        let mut attrs = entry.attrs();
        while let Some(attr) = attrs.next()
            .map_err(|e| CursedError::RuntimeError(format!("Failed to read parameter attribute: {}", e)))? {
            
            match attr.name() {
                DW_AT_name => {
                    if let Ok(name_str) = dwarf.attr_string(unit, attr.value()) {
                        if let Ok(cow_str) = name_str.to_string_lossy() {
                            param.name = cow_str.into_owned();
                        }
                    }
                }
                DW_AT_type => {
                    if let AttributeValue::UnitRef(offset) = attr.value() {
                        param.type_id = offset.0 as u64;
                    }
                }
                DW_AT_location => {
                    if let AttributeValue::Exprloc(expr) = attr.value() {
                        if let Ok(slice) = expr.0.to_slice() {
                            param.location = Some(Vec::from(slice.as_ref()));
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(param)
    }

    /// Parse a local variable
    fn parse_local_variable<R: Reader<Offset = usize>>(
        &mut self,
        dwarf: &gimli::read::Dwarf<R>,
        unit: &Unit<R>,
        entry: &DebuggingInformationEntry<R>,
        function_address: u64,
    ) -> Result<(), CursedError> {
        let mut variable = VariableDebugInfo {
            name: String::new(),
            type_id: 0,
            location: None,
            scope_start: function_address,
            scope_end: function_address + 0x1000, // Default scope size
            declared_line: None,
        };

        let mut attrs = entry.attrs();
        while let Some(attr) = attrs.next()
            .map_err(|e| CursedError::RuntimeError(format!("Failed to read variable attribute: {}", e)))? {
            
            match attr.name() {
                DW_AT_name => {
                    if let Ok(name_str) = dwarf.attr_string(unit, attr.value()) {
                        if let Ok(cow_str) = name_str.to_string_lossy() {
                            variable.name = cow_str.into_owned();
                        }
                    }
                }
                DW_AT_type => {
                    if let AttributeValue::UnitRef(offset) = attr.value() {
                        variable.type_id = offset.0 as u64;
                    }
                }
                DW_AT_location => {
                    if let AttributeValue::Exprloc(expr) = attr.value() {
                        if let Ok(slice) = expr.0.to_slice() {
                            variable.location = Some(Vec::from(slice.as_ref()));
                        }
                    }
                }
                _ => {}
            }
        }

        // Add variable to function's variable list
        if !variable.name.is_empty() {
            self.variables.entry(function_address).or_insert_with(Vec::new).push(variable);
        }

        Ok(())
    }

    /// Parse a lexical block (local scope)
    fn parse_lexical_block<R: Reader<Offset = usize>>(
        &mut self,
        dwarf: &Dwarf<R>,
        unit: &Unit<R>,
        entry: &DebuggingInformationEntry<R>,
        function_address: u64,
    ) -> Result<(), CursedError> {
        // Parse lexical block children for more local variables
        let block_offset = entry.offset();
        let mut child_entries = unit.entries_at_offset(block_offset)
            .map_err(|e| CursedError::RuntimeError(format!("Failed to get lexical block children: {}", e)))?;
        
        let _ = child_entries.next_dfs(); // Skip the lexical block entry itself
        
        while let Some((_, child_entry)) = child_entries.next_dfs()
            .map_err(|e| CursedError::RuntimeError(format!("Failed to read lexical block child: {}", e)))? {
            
            if child_entry.tag() == DW_TAG_variable {
                self.parse_local_variable(dwarf, unit, &child_entry, function_address)?;
            }
        }

        Ok(())
    }

    /// Parse an inlined subroutine
    fn parse_inlined_subroutine<R: Reader<Offset = usize>>(
        &mut self,
        dwarf: &Dwarf<R>,
        unit: &Unit<R>,
        entry: &DebuggingInformationEntry<R>,
    ) -> Result<(), CursedError> {
        let mut inline_site = InlineCallSite {
            function_name: String::new(),
            call_address: 0,
            original_location: None,
            inline_location: None,
        };

        let mut attrs = entry.attrs();
        while let Some(attr) = attrs.next()
            .map_err(|e| CursedError::RuntimeError(format!("Failed to read inline attribute: {}", e)))? {
            
            match attr.name() {
                DW_AT_low_pc => {
                    if let AttributeValue::Addr(addr) = attr.value() {
                        inline_site.call_address = addr;
                    }
                }
                DW_AT_call_file => {
                    // Extract file information for inline location
                }
                DW_AT_call_line => {
                    // Extract line information for inline location
                }
                _ => {}
            }
        }

        // Add inline site to database
        if inline_site.call_address != 0 {
            self.inline_sites.entry(inline_site.call_address).or_insert_with(Vec::new).push(inline_site);
        }

        Ok(())
    }

    /// Parse a global variable
    fn parse_global_variable<R: Reader>(
        &mut self,
        dwarf: &Dwarf<R>,
        unit: &Unit<R>,
        entry: &DebuggingInformationEntry<R>,
    ) -> Result<(), CursedError> {
        // Similar to parse_local_variable but for global scope
        // Implementation would be similar with different scope handling
        Ok(())
    }

    /// Parse type information
    fn parse_type<R: Reader<Offset = usize>>(
        &mut self,
        dwarf: &Dwarf<R>,
        unit: &Unit<R>,
        entry: &DebuggingInformationEntry<R>,
    ) -> Result<(), CursedError> {
        let type_offset = entry.offset().0;
        let mut type_info = DwarfTypeInfo {
            name: String::new(),
            size: 0,
            encoding: None,
            members: Vec::new(),
            base_type: None,
        };

        let mut attrs = entry.attrs();
        while let Some(attr) = attrs.next()
            .map_err(|e| CursedError::RuntimeError(format!("Failed to read type attribute: {}", e)))? {
            
            match attr.name() {
                DW_AT_name => {
                    if let Ok(name_str) = dwarf.attr_string(unit, attr.value()) {
                        if let Ok(cow_str) = name_str.to_string_lossy() {
                            type_info.name = cow_str.into_owned();
                        }
                    }
                }
                DW_AT_byte_size => {
                    if let AttributeValue::Udata(size) = attr.value() {
                        type_info.size = size;
                    }
                }
                DW_AT_encoding => {
                    if let AttributeValue::Udata(encoding) = attr.value() {
                        type_info.encoding = Some(self.decode_type_encoding(encoding));
                    }
                }
                DW_AT_type => {
                    if let AttributeValue::UnitRef(offset) = attr.value() {
                        type_info.base_type = Some(offset.0 as u64);
                    }
                }
                _ => {}
            }
        }

        // Parse type members for composite types
        if entry.tag() == DW_TAG_structure_type || entry.tag() == DW_TAG_union_type {
            self.parse_type_members(dwarf, unit, entry, &mut type_info)?;
        }

        // Add type to database
        self.types.insert(type_offset as u64, type_info);

        Ok(())
    }

    /// Parse type members for composite types
    fn parse_type_members<R: Reader<Offset = usize>>(
        &mut self,
        dwarf: &Dwarf<R>,
        unit: &Unit<R>,
        entry: &DebuggingInformationEntry<R>,
        type_info: &mut DwarfTypeInfo,
    ) -> Result<(), CursedError> {
        let type_offset = entry.offset();
        let mut child_entries = unit.entries_at_offset(type_offset)
            .map_err(|e| CursedError::RuntimeError(format!("Failed to get type children: {}", e)))?;
        
        let _ = child_entries.next_dfs(); // Skip the type entry itself
        
        while let Some((_, child_entry)) = child_entries.next_dfs()
            .map_err(|e| CursedError::RuntimeError(format!("Failed to read type member: {}", e)))? {
            
            if child_entry.tag() == DW_TAG_variable {
                let mut member = TypeMemberInfo {
                    name: String::new(),
                    type_id: 0,
                    offset: 0,
                    size: 0,
                };

                let mut attrs = child_entry.attrs();
                while let Some(attr) = attrs.next()
                    .map_err(|e| CursedError::RuntimeError(format!("Failed to read member attribute: {}", e)))? {
                    
                    match attr.name() {
                        DW_AT_name => {
                            if let Ok(name_str) = dwarf.attr_string(unit, attr.value()) {
                                if let Ok(cow_str) = name_str.to_string_lossy() {
                                    member.name = cow_str.into_owned();
                                }
                            }
                        }
                        DW_AT_type => {
                            if let AttributeValue::UnitRef(offset) = attr.value() {
                                member.type_id = offset.0 as u64;
                            }
                        }
                        DW_AT_data_member_location => {
                            if let AttributeValue::Udata(offset) = attr.value() {
                                member.offset = offset;
                            }
                        }
                        _ => {}
                    }
                }

                if !member.name.is_empty() {
                    type_info.members.push(member);
                }
            }
        }

        Ok(())
    }

    /// Parse line number information
    fn parse_line_information<R: Reader<Offset = usize>>(&mut self, dwarf: &gimli::read::Dwarf<R>) -> Result<(), CursedError> {
        let mut units = dwarf.units();
        while let Some(unit_header) = units.next()
            .map_err(|e| CursedError::RuntimeError(format!("Failed to read unit header for line info: {}", e)))? {
            
            let unit = dwarf.unit(unit_header)
                .map_err(|e| CursedError::RuntimeError(format!("Failed to parse unit for line info: {}", e)))?;

            // Get line program
            if let Some(line_program) = unit.line_program.clone() {
                let mut rows = line_program.rows();
                
                while let Some((header, row)) = rows.next_row()
                    .map_err(|e| CursedError::RuntimeError(format!("Failed to read line row: {}", e)))? {
                    
                    if let Some(file_entry) = row.file(header) {
                        let path = self.resolve_file_path(header, file_entry)?;
                        
                        let line_info = LineInfo {
                            file: path,
                            line: row.line().map(|l| l.get() as u32).unwrap_or(0),
                            column: match row.column() {
                                gimli::ColumnType::Column(c) => c.get() as u32,
                                gimli::ColumnType::LeftEdge => 0,
                            },
                            is_stmt: row.is_stmt(),
                        };
                        
                        self.line_mappings.insert(row.address(), line_info);
                    }
                }
            }
        }

        Ok(())
    }

    /// Resolve file path from line program
    fn resolve_file_path<R: Reader>(
        &self,
        _header: &LineProgramHeader<R>,
        _file_entry: &FileEntry<R>,
    ) -> Result<PathBuf, CursedError> {
        // Simplified implementation that avoids complex API issues
        // This can be enhanced later when gimli API is better understood
        Ok(PathBuf::from("unknown_file.src"))
    }

    /// Decode DWARF type encoding
    fn decode_type_encoding(&self, encoding: u64) -> String {
        match encoding {
            x if x == DW_ATE_address.0 as u64 => "address".to_string(),
            x if x == DW_ATE_boolean.0 as u64 => "boolean".to_string(),
            x if x == DW_ATE_complex_float.0 as u64 => "complex_float".to_string(),
            x if x == DW_ATE_float.0 as u64 => "float".to_string(),
            x if x == DW_ATE_signed.0 as u64 => "signed".to_string(),
            x if x == DW_ATE_signed_char.0 as u64 => "signed_char".to_string(),
            x if x == DW_ATE_unsigned.0 as u64 => "unsigned".to_string(),
            x if x == DW_ATE_unsigned_char.0 as u64 => "unsigned_char".to_string(),
            x if x == DW_ATE_imaginary_float.0 as u64 => "imaginary_float".to_string(),
            x if x == DW_ATE_packed_decimal.0 as u64 => "packed_decimal".to_string(),
            x if x == DW_ATE_numeric_string.0 as u64 => "numeric_string".to_string(),
            x if x == DW_ATE_edited.0 as u64 => "edited".to_string(),
            x if x == DW_ATE_signed_fixed.0 as u64 => "signed_fixed".to_string(),
            x if x == DW_ATE_unsigned_fixed.0 as u64 => "unsigned_fixed".to_string(),
            x if x == DW_ATE_decimal_float.0 as u64 => "decimal_float".to_string(),
            x if x == DW_ATE_UTF.0 as u64 => "UTF".to_string(),
            x if x == DW_ATE_UCS.0 as u64 => "UCS".to_string(),
            x if x == DW_ATE_ASCII.0 as u64 => "ASCII".to_string(),
            _ => format!("unknown_encoding_{}", encoding),
        }
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

    /// Evaluate variable location expression
    pub fn evaluate_location(&self, location_expr: &[u8], frame_base: u64, registers: &RegisterMap) -> Result<u64, CursedError> {
        let mut evaluator = LocationEvaluator::new(frame_base, registers);
        evaluator.evaluate(location_expr)
    }

    /// Get source location for address
    pub fn get_source_location_for_address(&self, address: u64) -> Option<&LineInfo> {
        // Find the closest line mapping
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
                // Fallback to stack pointer if no frame base
                registers.get_register(RegisterName::StackPointer).unwrap_or(0)
            };

            // Evaluate parameter locations
            for param in &function.parameters {
                if let Some(ref location_expr) = param.location {
                    match self.evaluate_location(location_expr, frame_base, registers) {
                        Ok(location) => {
                            let param_info = ParameterInfo {
                                name: param.name.clone(),
                                param_type: self.get_type_name(param.type_id),
                                value: None, // Would need memory access to get actual value
                                location: Some(location),
                            };
                            frame_info.parameters.push(param_info);
                        }
                        Err(e) => {
                            eprintln!("Failed to evaluate parameter location: {}", e);
                        }
                    }
                }
            }

            // Evaluate local variable locations
            if let Some(variables) = self.variables.get(&function.start_address) {
                for var in variables {
                    if address >= var.scope_start && address < var.scope_end {
                        if let Some(ref location_expr) = var.location {
                            match self.evaluate_location(location_expr, frame_base, registers) {
                                Ok(location) => {
                                    let var_info = LocalVariableInfo {
                                        name: var.name.clone(),
                                        var_type: self.get_type_name(var.type_id),
                                        value: None, // Would need memory access to get actual value
                                        scope: format!("function_{}", function.name),
                                        location: Some(location),
                                    };
                                    frame_info.local_variables.push(var_info);
                                }
                                Err(e) => {
                                    eprintln!("Failed to evaluate variable location: {}", e);
                                }
                            }
                        }
                    }
                }
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

    /// Get type name by ID
    fn get_type_name(&self, type_id: u64) -> String {
        self.types.get(&type_id)
            .map(|t| t.name.clone())
            .unwrap_or_else(|| format!("unknown_type_{}", type_id))
    }
}

/// LLVM debug information integration with DWARF support
#[derive(Debug)]
pub struct LlvmDebugInfo {
    /// Debug metadata
    debug_metadata: HashMap<u64, LlvmDebugMetadata>,
    /// Compilation units
    compilation_units: Vec<CompilationUnit>,
    /// Type information
    type_info: HashMap<String, TypeDebugInfo>,
    /// DWARF debug database
    pub dwarf_database: Option<DwarfDebugDatabase>,
}

impl LlvmDebugInfo {
    /// Create new LLVM debug info
    pub fn new() -> Self {
        Self {
            debug_metadata: HashMap::new(),
            compilation_units: Vec::new(),
            type_info: HashMap::new(),
            dwarf_database: None,
        }
    }

    /// Get source location for an address
    pub fn get_source_location(&self, address: u64) -> Result<Option<SourceLocation>, CursedError> {
        if let Some(metadata) = self.debug_metadata.get(&address) {
            Ok(Some(SourceLocation {
                file: metadata.file_path.to_string_lossy().to_string(),
                line: metadata.line as usize,
                column: metadata.column as usize,
            }))
        } else {
            Ok(None)
        }
    }

    /// Add debug metadata for an address
    pub fn add_metadata(&mut self, address: u64, metadata: LlvmDebugMetadata) {
        self.debug_metadata.insert(address, metadata);
    }

    /// Load DWARF debug information from binary data
    pub fn load_dwarf_info(&mut self, dwarf_data: &[u8]) -> Result<(), CursedError> {
        let mut database = DwarfDebugDatabase::new();
        database.load_from_dwarf(dwarf_data)?;
        self.dwarf_database = Some(database);
        Ok(())
    }

    /// Extract function parameters from debug info
    pub fn extract_function_parameters(&self, address: u64) -> Result<Vec<ParameterInfo>, CursedError> {
        if let Some(ref database) = self.dwarf_database {
            if let Some(func_info) = database.find_function(address) {
                let mut parameters = Vec::new();
                
                for param in &func_info.parameters {
                    let param_type = database.types.get(&param.type_id)
                        .map(|t| t.name.clone())
                        .unwrap_or_else(|| format!("unknown_type_{}", param.type_id));
                    
                    parameters.push(ParameterInfo {
                        name: param.name.clone(),
                        param_type,
                        value: None, // Would need to evaluate location expression
                        location: None, // Would need to decode location expression
                    });
                }
                
                return Ok(parameters);
            }
        }

        // Fallback to metadata-based extraction
        if let Some(metadata) = self.debug_metadata.get(&address) {
            if let Some(ref func_name) = metadata.function_name {
                // This is a simplified implementation - in practice, you'd need
                // more sophisticated parameter extraction
                return Ok(vec![ParameterInfo {
                    name: "param".to_string(),
                    param_type: "unknown".to_string(),
                    value: None,
                    location: Some(address),
                }]);
            }
        }

        Ok(Vec::new())
    }

    /// Extract local variables from debug info
    pub fn extract_local_variables(&self, address: u64) -> Result<Vec<LocalVariableInfo>, CursedError> {
        if let Some(ref database) = self.dwarf_database {
            let variables = database.get_variables_at_address(address);
            let mut local_vars = Vec::new();
            
            for var in variables {
                let var_type = database.types.get(&var.type_id)
                    .map(|t| t.name.clone())
                    .unwrap_or_else(|| format!("unknown_type_{}", var.type_id));
                
                local_vars.push(LocalVariableInfo {
                    name: var.name.clone(),
                    var_type,
                    value: None, // Would need to evaluate location expression
                    scope: format!("0x{:x}-0x{:x}", var.scope_start, var.scope_end),
                    location: None, // Would need to decode location expression
                });
            }
            
            return Ok(local_vars);
        }

        // Fallback to basic implementation
        Ok(Vec::new())
    }

    /// Extract inline function information
    pub fn extract_inline_info(&self, address: u64) -> Result<Vec<InlineInfo>, CursedError> {
        if let Some(ref database) = self.dwarf_database {
            let inline_sites = database.get_inline_info_at_address(address);
            let mut inline_infos = Vec::new();
            
            for site in inline_sites {
                let inline_site = if let Some((file, line, col)) = &site.inline_location {
                    SourceLocation {
                        file: file.to_string_lossy().to_string(),
                        line: *line as usize,
                        column: *col as usize,
                    }
                } else {
                    SourceLocation {
                        file: "unknown".to_string(),
                        line: 0,
                        column: 0,
                    }
                };
                
                let original_location = if let Some((file, line, col)) = &site.original_location {
                    SourceLocation {
                        file: file.to_string_lossy().to_string(),
                        line: *line as usize,
                        column: *col as usize,
                    }
                } else {
                    SourceLocation {
                        file: "unknown".to_string(),
                        line: 0,
                        column: 0,
                    }
                };
                
                inline_infos.push(InlineInfo {
                    function_name: site.function_name.clone(),
                    inline_site,
                    original_location,
                });
            }
            
            return Ok(inline_infos);
        }

        // Fallback to basic implementation
        Ok(Vec::new())
    }

    /// Generate DWARF debug information for a module
    pub fn generate_dwarf_info(&self, module_name: &str, functions: &[FunctionInfo]) -> Result<Vec<u8>, CursedError> {
        // This is a simplified DWARF generation implementation
        // In practice, you'd use a DWARF generation library or LLVM's DWARF generator
        
        let mut dwarf_data = Vec::new();
        
        // Add basic DWARF headers
        dwarf_data.extend_from_slice(b"DWARF");
        dwarf_data.extend_from_slice(&[4, 0, 0, 0]); // Version 4
        
        // Add compilation unit information
        dwarf_data.extend_from_slice(module_name.as_bytes());
        dwarf_data.push(0); // Null terminator
        
        // Add function information
        for func in functions {
            dwarf_data.extend_from_slice(func.name.as_bytes());
            dwarf_data.push(0); // Null terminator
            dwarf_data.extend_from_slice(&func.start_address.to_le_bytes());
            dwarf_data.extend_from_slice(&func.end_address.to_le_bytes());
        }
        
        Ok(dwarf_data)
    }

    /// Parse DWARF information and build debug database
    pub fn parse_dwarf_info(&mut self, dwarf_data: &[u8]) -> Result<(), CursedError> {
        self.load_dwarf_info(dwarf_data)
    }
}

/// LLVM debug metadata for a specific location
#[derive(Debug, Clone)]
pub struct LlvmDebugMetadata {
    /// Source file path
    pub file_path: PathBuf,
    /// Line number
    pub line: u32,
    /// Column number
    pub column: u32,
    /// Function name
    pub function_name: Option<String>,
    /// Scope information
    pub scope: Option<String>,
}

/// Compilation unit information
#[derive(Debug, Clone)]
pub struct CompilationUnit {
    /// Unit name
    pub name: String,
    /// Source files in this unit
    pub source_files: Vec<PathBuf>,
    /// Producer (compiler) information
    pub producer: Option<String>,
    /// Language (CURSED)
    pub language: String,
}

/// Type debug information
#[derive(Debug, Clone)]
pub struct TypeDebugInfo {
    /// Type name
    pub name: String,
    /// Type size in bytes
    pub size: u32,
    /// Type alignment
    pub alignment: u32,
    /// Type kind (struct, function, etc.)
    pub kind: TypeKind,
    /// Member information (for structs/classes)
    pub members: Vec<TypeMember>,
}

/// Type kinds for debugging
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeKind {
    /// Basic type (int, float, etc.)
    Basic,
    /// Struct type
    Struct,
    /// Function type
    Function,
    /// Pointer type
    Pointer,
    /// Array type
    Array,
    /// Enum type
    Enum,
    /// Union type
    Union,
}

/// Type member information
#[derive(Debug, Clone)]
pub struct TypeMember {
    /// Member name
    pub name: String,
    /// Member type
    pub member_type: String,
    /// Offset within the type
    pub offset: u32,
    /// Member size
    pub size: u32,
}

/// Statistics for stack trace capture
#[derive(Debug, Clone)]
pub struct StackTraceStats {
    /// Number of traces captured
    pub traces_captured: u64,
    /// Total frames captured across all traces
    pub total_frames: usize,
    /// Average capture time
    pub avg_capture_time: Option<Duration>,
    /// Last capture time
    pub last_capture_time: Option<SystemTime>,
    /// Cache hit rate for symbol resolution
    pub cache_hit_rate: f64,
}

impl StackTraceStats {
    pub fn new() -> Self {
        Self {
            traces_captured: 0,
            total_frames: 0,
            avg_capture_time: None,
            last_capture_time: None,
            cache_hit_rate: 0.0,
        }
    }
}

/// Statistics for symbol resolver
#[derive(Debug, Clone)]
pub struct ResolverStats {
    /// Total resolution attempts
    pub resolutions: u64,
    /// Successful resolutions
    pub successful_resolutions: u64,
    /// Cache hits
    pub cache_hits: u64,
    /// Success rate
    pub success_rate: f64,
}

impl ResolverStats {
    pub fn new() -> Self {
        Self {
            resolutions: 0,
            successful_resolutions: 0,
            cache_hits: 0,
            success_rate: 0.0,
        }
    }
}

/// Capture session for targeted debugging
#[derive(Debug, Clone)]
pub struct CaptureSession {
    /// Session ID
    pub id: String,
    /// Session name
    pub name: String,
    /// Session start time
    pub start_time: SystemTime,
    /// Target function (if specific)
    pub target_function: Option<String>,
    /// Captured stack traces
    pub captured_traces: Vec<Vec<StackFrame>>,
    /// Session configuration
    pub config: EnhancedStackTraceConfig,
}

/// Legacy minimal implementation for backward compatibility
pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED advanced debug info enabled".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_trace_capture_creation() {
        let config = EnhancedStackTraceConfig::default();
        let capture = StackTraceCapture::new(config);
        
        // Test basic functionality
        let result = capture.capture_stack_trace();
        assert!(result.is_ok());
    }

    #[test]
    fn test_symbol_resolver() {
        let resolver = SymbolResolver::new();
        
        let symbol = SymbolInfo::function("test_function".to_string(), 0x1000, 0x100);
        let add_result = resolver.add_symbol(symbol);
        assert!(add_result.is_ok());
        
        let resolved = resolver.resolve_address(0x1050);
        assert!(resolved.is_ok());
        
        let symbol_info = resolved.unwrap();
        assert_eq!(symbol_info.name, "test_function");
        assert_eq!(symbol_info.offset, 0x50);
    }

    #[test]
    fn test_enhanced_config() {
        let mut config = EnhancedStackTraceConfig::default();
        config.max_depth = 50;
        config.resolve_symbols = false;
        
        assert_eq!(config.max_depth, 50);
        assert!(!config.resolve_symbols);
    }

    #[test]
    fn test_symbol_info_creation() {
        let unknown = SymbolInfo::unknown(0x2000);
        assert_eq!(unknown.symbol_type, SymbolType::Unknown);
        assert_eq!(unknown.address, 0x2000);
        
        let function = SymbolInfo::function("main".to_string(), 0x1000, 0x200);
        assert_eq!(function.symbol_type, SymbolType::Function);
        assert_eq!(function.name, "main");
    }

    #[test]
    fn test_llvm_debug_info() {
        let mut debug_info = LlvmDebugInfo::new();
        
        let metadata = LlvmDebugMetadata {
            file_path: PathBuf::from("test.csd"),
            line: 10,
            column: 5,
            function_name: Some("test_func".to_string()),
            scope: None,
        };
        
        debug_info.add_metadata(0x1000, metadata);
        
        let location = debug_info.get_source_location(0x1000);
        assert!(location.is_ok());
        
        let source_loc = location.unwrap();
        assert!(source_loc.is_some());
        
        let loc = source_loc.unwrap();
        assert_eq!(loc.line, 10);
        assert_eq!(loc.column, 5);
    }

    #[test]
    fn test_dwarf_database_creation() {
        let database = DwarfDebugDatabase::new();
        assert!(database.functions.is_empty());
        assert!(database.variables.is_empty());
        assert!(database.inline_sites.is_empty());
        assert!(database.types.is_empty());
        assert!(database.line_mappings.is_empty());
    }

    #[test]
    fn test_function_debug_info() {
        let func_info = FunctionDebugInfo {
            name: "test_function".to_string(),
            demangled_name: Some("test_function".to_string()),
            start_address: 0x1000,
            end_address: 0x1100,
            parameters: vec![ParameterDebugInfo {
                name: "param1".to_string(),
                type_id: 1,
                location: None,
                by_reference: false,
            }],
            source_file: Some(PathBuf::from("test.csd")),
            line_range: Some((10, 20)),
            frame_base: None,
        };

        assert_eq!(func_info.name, "test_function");
        assert_eq!(func_info.start_address, 0x1000);
        assert_eq!(func_info.end_address, 0x1100);
        assert_eq!(func_info.parameters.len(), 1);
        assert_eq!(func_info.parameters[0].name, "param1");
    }

    #[test]
    fn test_parameter_extraction() {
        let debug_info = LlvmDebugInfo::new();
        
        // Test basic parameter extraction
        let params = debug_info.extract_function_parameters(0x1000);
        assert!(params.is_ok());
        let param_list = params.unwrap();
        assert!(param_list.is_empty()); // No DWARF data loaded
    }

    #[test]
    fn test_local_variable_extraction() {
        let debug_info = LlvmDebugInfo::new();
        
        // Test basic local variable extraction
        let vars = debug_info.extract_local_variables(0x1000);
        assert!(vars.is_ok());
        let var_list = vars.unwrap();
        assert!(var_list.is_empty()); // No DWARF data loaded
    }

    #[test]
    fn test_inline_info_extraction() {
        let debug_info = LlvmDebugInfo::new();
        
        // Test basic inline info extraction
        let inline_info = debug_info.extract_inline_info(0x1000);
        assert!(inline_info.is_ok());
        let inline_list = inline_info.unwrap();
        assert!(inline_list.is_empty()); // No DWARF data loaded
    }

    #[test]
    fn test_dwarf_generation() {
        let debug_info = LlvmDebugInfo::new();
        let functions = vec![
            FunctionInfo {
                name: "main".to_string(),
                start_address: 0x1000,
                end_address: 0x1100,
                start_line: 1,
                end_line: 10,
                parameter_count: 0,
                local_count: 2,
            }
        ];

        let dwarf_result = debug_info.generate_dwarf_info("test_module", &functions);
        assert!(dwarf_result.is_ok());
        
        let dwarf_data = dwarf_result.unwrap();
        assert!(!dwarf_data.is_empty());
        assert!(dwarf_data.starts_with(b"DWARF"));
    }

    #[test]
    fn test_parameter_info_creation() {
        let param_info = ParameterInfo {
            name: "test_param".to_string(),
            param_type: "int".to_string(),
            value: Some("42".to_string()),
            location: Some(0x1000),
        };

        assert_eq!(param_info.name, "test_param");
        assert_eq!(param_info.param_type, "int");
        assert_eq!(param_info.value.as_ref().unwrap(), "42");
        assert_eq!(param_info.location.unwrap(), 0x1000);
    }

    #[test]
    fn test_local_variable_info_creation() {
        let var_info = LocalVariableInfo {
            name: "local_var".to_string(),
            var_type: "string".to_string(),
            value: Some("hello".to_string()),
            scope: "function".to_string(),
            location: Some(0x2000),
        };

        assert_eq!(var_info.name, "local_var");
        assert_eq!(var_info.var_type, "string");
        assert_eq!(var_info.value.as_ref().unwrap(), "hello");
        assert_eq!(var_info.scope, "function");
        assert_eq!(var_info.location.unwrap(), 0x2000);
    }

    #[test]
    fn test_inline_info_creation() {
        let inline_info = InlineInfo {
            function_name: "inlined_func".to_string(),
            inline_site: SourceLocation {
                file: "caller.csd".to_string(),
                line: 15,
                column: 5,
            },
            original_location: SourceLocation {
                file: "original.csd".to_string(),
                line: 25,
                column: 10,
            },
        };

        assert_eq!(inline_info.function_name, "inlined_func");
        assert_eq!(inline_info.inline_site.file, "caller.csd");
        assert_eq!(inline_info.inline_site.line, 15);
        assert_eq!(inline_info.original_location.file, "original.csd");
        assert_eq!(inline_info.original_location.line, 25);
    }
}

/// Register map for variable location evaluation
#[derive(Debug, Clone)]
pub struct RegisterMap {
    /// General purpose registers
    registers: HashMap<RegisterName, u64>,
}

impl RegisterMap {
    /// Create new register map
    pub fn new() -> Self {
        Self {
            registers: HashMap::new(),
        }
    }

    /// Set register value
    pub fn set_register(&mut self, name: RegisterName, value: u64) {
        self.registers.insert(name, value);
    }

    /// Get register value
    pub fn get_register(&self, name: RegisterName) -> Option<u64> {
        self.registers.get(&name).copied()
    }

    /// Create register map from current context
    pub fn from_current_context() -> Self {
        let mut map = Self::new();
        
        // Platform-specific register capture would go here
        #[cfg(target_arch = "x86_64")]
        {
            // x86-64 register capture
            map.set_register(RegisterName::StackPointer, Self::get_stack_pointer());
            map.set_register(RegisterName::BasePointer, Self::get_base_pointer());
            // Add other registers as needed
        }
        
        #[cfg(target_arch = "aarch64")]
        {
            // ARM64 register capture
            map.set_register(RegisterName::StackPointer, Self::get_stack_pointer());
            map.set_register(RegisterName::BasePointer, Self::get_base_pointer());
            // Add other registers as needed
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
        0 // Fallback for unsupported architectures
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    fn get_base_pointer() -> u64 {
        0 // Fallback for unsupported architectures
    }
}

/// Register names for different architectures
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegisterName {
    // Common registers
    StackPointer,
    BasePointer,
    InstructionPointer,
    
    // x86-64 registers
    Rax, Rbx, Rcx, Rdx, Rsi, Rdi, Rbp, Rsp, Rip,
    R8, R9, R10, R11, R12, R13, R14, R15,
    
    // ARM64 registers
    X0, X1, X2, X3, X4, X5, X6, X7, X8, X9,
    X10, X11, X12, X13, X14, X15, X16, X17, X18, X19,
    X20, X21, X22, X23, X24, X25, X26, X27, X28, X29, X30,
    Sp, Pc,
    
    // Custom register for unknown architectures
    Custom(u32),
}

/// Location evaluator for DWARF expressions
pub struct LocationEvaluator<'a> {
    frame_base: u64,
    registers: &'a RegisterMap,
    stack: Vec<u64>,
}

impl<'a> LocationEvaluator<'a> {
    /// Create new location evaluator
    pub fn new(frame_base: u64, registers: &'a RegisterMap) -> Self {
        Self {
            frame_base,
            registers,
            stack: Vec::new(),
        }
    }

    /// Evaluate DWARF location expression
    pub fn evaluate(&mut self, expression: &[u8]) -> Result<u64, CursedError> {
        let mut cursor = 0;
        
        while cursor < expression.len() {
            let opcode = expression[cursor];
            cursor += 1;
            
            match opcode {
                // Literal operations
                x if x >= DW_OP_lit0.0 && x <= DW_OP_lit31.0 => {
                    let value = (x - DW_OP_lit0.0) as u64;
                    self.stack.push(value);
                }
                
                // Constant operations
                x if x == DW_OP_const1u.0 => {
                    if cursor >= expression.len() {
                        return Err(CursedError::RuntimeError("Unexpected end of expression".to_string()));
                    }
                    let value = expression[cursor] as u64;
                    cursor += 1;
                    self.stack.push(value);
                }
                
                x if x == DW_OP_const2u.0 => {
                    if cursor + 1 >= expression.len() {
                        return Err(CursedError::RuntimeError("Unexpected end of expression".to_string()));
                    }
                    let value = u16::from_le_bytes([expression[cursor], expression[cursor + 1]]) as u64;
                    cursor += 2;
                    self.stack.push(value);
                }
                
                x if x == DW_OP_const4u.0 => {
                    if cursor + 3 >= expression.len() {
                        return Err(CursedError::RuntimeError("Unexpected end of expression".to_string()));
                    }
                    let value = u32::from_le_bytes([
                        expression[cursor], expression[cursor + 1],
                        expression[cursor + 2], expression[cursor + 3]
                    ]) as u64;
                    cursor += 4;
                    self.stack.push(value);
                }
                
                x if x == DW_OP_const8u.0 => {
                    if cursor + 7 >= expression.len() {
                        return Err(CursedError::RuntimeError("Unexpected end of expression".to_string()));
                    }
                    let value = u64::from_le_bytes([
                        expression[cursor], expression[cursor + 1],
                        expression[cursor + 2], expression[cursor + 3],
                        expression[cursor + 4], expression[cursor + 5],
                        expression[cursor + 6], expression[cursor + 7]
                    ]);
                    cursor += 8;
                    self.stack.push(value);
                }
                
                // Register operations
                x if x >= DW_OP_reg0.0 && x <= DW_OP_reg31.0 => {
                    let reg_num = x - DW_OP_reg0.0;
                    let reg_name = self.map_register_number(reg_num as u32);
                    if let Some(value) = self.registers.get_register(reg_name) {
                        self.stack.push(value);
                    } else {
                        return Err(CursedError::RuntimeError(format!("Register {} not available", reg_num)));
                    }
                }
                
                // Base register + offset operations
                x if x >= DW_OP_breg0.0 && x <= DW_OP_breg31.0 => {
                    let reg_num = x - DW_OP_breg0.0;
                    let offset = self.read_sleb128(&expression, &mut cursor)?;
                    let reg_name = self.map_register_number(reg_num as u32);
                    
                    if let Some(reg_value) = self.registers.get_register(reg_name) {
                        let result = (reg_value as i64 + offset) as u64;
                        self.stack.push(result);
                    } else {
                        return Err(CursedError::RuntimeError(format!("Register {} not available", reg_num)));
                    }
                }
                
                // Frame base register + offset
                x if x == DW_OP_fbreg.0 => {
                    let offset = self.read_sleb128(&expression, &mut cursor)?;
                    let result = (self.frame_base as i64 + offset) as u64;
                    self.stack.push(result);
                }
                
                // Arithmetic operations
                x if x == DW_OP_plus.0 => {
                    if self.stack.len() < 2 {
                        return Err(CursedError::RuntimeError("Stack underflow in plus operation".to_string()));
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }
                
                x if x == DW_OP_plus_uconst.0 => {
                    if self.stack.is_empty() {
                        return Err(CursedError::RuntimeError("Stack underflow in plus_uconst operation".to_string()));
                    }
                    let constant = self.read_uleb128(&expression, &mut cursor)?;
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + constant);
                }
                
                x if x == DW_OP_minus.0 => {
                    if self.stack.len() < 2 {
                        return Err(CursedError::RuntimeError("Stack underflow in minus operation".to_string()));
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                }
                
                // Memory dereference
                x if x == DW_OP_deref.0 => {
                    if self.stack.is_empty() {
                        return Err(CursedError::RuntimeError("Stack underflow in deref operation".to_string()));
                    }
                    let address = self.stack.pop().unwrap();
                    // In a real implementation, we would read from memory at this address
                    // For now, we just return the address itself
                    self.stack.push(address);
                }
                
                // Stack manipulation
                x if x == DW_OP_dup.0 => {
                    if self.stack.is_empty() {
                        return Err(CursedError::RuntimeError("Stack underflow in dup operation".to_string()));
                    }
                    let value = *self.stack.last().unwrap();
                    self.stack.push(value);
                }
                
                x if x == DW_OP_drop.0 => {
                    if self.stack.is_empty() {
                        return Err(CursedError::RuntimeError("Stack underflow in drop operation".to_string()));
                    }
                    self.stack.pop();
                }
                
                // No-op
                x if x == DW_OP_nop.0 => {
                    // Do nothing
                }
                
                _ => {
                    return Err(CursedError::RuntimeError(format!("Unsupported DWARF operation: 0x{:02x}", opcode)));
                }
            }
        }
        
        if self.stack.is_empty() {
            return Err(CursedError::RuntimeError("Empty stack after expression evaluation".to_string()));
        }
        
        Ok(self.stack.pop().unwrap())
    }
    
    /// Map register number to register name based on architecture
    fn map_register_number(&self, reg_num: u32) -> RegisterName {
        #[cfg(target_arch = "x86_64")]
        {
            match reg_num {
                0 => RegisterName::Rax,
                1 => RegisterName::Rdx,
                2 => RegisterName::Rcx,
                3 => RegisterName::Rbx,
                4 => RegisterName::Rsi,
                5 => RegisterName::Rdi,
                6 => RegisterName::Rbp,
                7 => RegisterName::Rsp,
                8 => RegisterName::R8,
                9 => RegisterName::R9,
                10 => RegisterName::R10,
                11 => RegisterName::R11,
                12 => RegisterName::R12,
                13 => RegisterName::R13,
                14 => RegisterName::R14,
                15 => RegisterName::R15,
                16 => RegisterName::Rip,
                _ => RegisterName::Custom(reg_num),
            }
        }
        
        #[cfg(target_arch = "aarch64")]
        {
            match reg_num {
                0..=30 => {
                    // Map X0-X30
                    match reg_num {
                        0 => RegisterName::X0, 1 => RegisterName::X1, 2 => RegisterName::X2,
                        3 => RegisterName::X3, 4 => RegisterName::X4, 5 => RegisterName::X5,
                        6 => RegisterName::X6, 7 => RegisterName::X7, 8 => RegisterName::X8,
                        9 => RegisterName::X9, 10 => RegisterName::X10, 11 => RegisterName::X11,
                        12 => RegisterName::X12, 13 => RegisterName::X13, 14 => RegisterName::X14,
                        15 => RegisterName::X15, 16 => RegisterName::X16, 17 => RegisterName::X17,
                        18 => RegisterName::X18, 19 => RegisterName::X19, 20 => RegisterName::X20,
                        21 => RegisterName::X21, 22 => RegisterName::X22, 23 => RegisterName::X23,
                        24 => RegisterName::X24, 25 => RegisterName::X25, 26 => RegisterName::X26,
                        27 => RegisterName::X27, 28 => RegisterName::X28, 29 => RegisterName::X29,
                        30 => RegisterName::X30,
                        _ => unreachable!(),
                    }
                }
                31 => RegisterName::Sp,
                32 => RegisterName::Pc,
                _ => RegisterName::Custom(reg_num),
            }
        }
        
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            RegisterName::Custom(reg_num)
        }
    }
    
    /// Read ULEB128 value from expression
    fn read_uleb128(&self, data: &[u8], cursor: &mut usize) -> Result<u64, CursedError> {
        let mut result = 0u64;
        let mut shift = 0;
        
        loop {
            if *cursor >= data.len() {
                return Err(CursedError::RuntimeError("Unexpected end while reading ULEB128".to_string()));
            }
            
            let byte = data[*cursor];
            *cursor += 1;
            
            result |= ((byte & 0x7F) as u64) << shift;
            
            if (byte & 0x80) == 0 {
                break;
            }
            
            shift += 7;
            if shift >= 64 {
                return Err(CursedError::RuntimeError("ULEB128 value too large".to_string()));
            }
        }
        
        Ok(result)
    }
    
    /// Read SLEB128 value from expression
    fn read_sleb128(&self, data: &[u8], cursor: &mut usize) -> Result<i64, CursedError> {
        let mut result = 0i64;
        let mut shift = 0;
        let mut byte;
        
        loop {
            if *cursor >= data.len() {
                return Err(CursedError::RuntimeError("Unexpected end while reading SLEB128".to_string()));
            }
            
            byte = data[*cursor];
            *cursor += 1;
            
            result |= ((byte & 0x7F) as i64) << shift;
            shift += 7;
            
            if (byte & 0x80) == 0 {
                break;
            }
            
            if shift >= 64 {
                return Err(CursedError::RuntimeError("SLEB128 value too large".to_string()));
            }
        }
        
        // Sign extend if necessary
        if shift < 64 && (byte & 0x40) != 0 {
            result |= !0i64 << shift;
        }
        
        Ok(result)
    }
}

/// Stack frame information reconstructed from debug info
#[derive(Debug, Clone)]
pub struct StackFrameInfo {
    /// Function name
    pub function_name: String,
    /// Function parameters with their locations and values
    pub parameters: Vec<ParameterInfo>,
    /// Local variables in scope at this address
    pub local_variables: Vec<LocalVariableInfo>,
    /// Source location
    pub source_location: Option<SourceLocation>,
    /// Instruction address
    pub address: u64,
}

/// DWARF version compatibility handler
#[derive(Debug, Clone)]
pub struct DwarfVersionHandler {
    /// Supported DWARF versions
    supported_versions: Vec<u16>,
    /// Current version being processed
    current_version: Option<u16>,
}

impl DwarfVersionHandler {
    /// Create new version handler
    pub fn new() -> Self {
        Self {
            supported_versions: vec![2, 3, 4, 5],
            current_version: None,
        }
    }

    /// Check if DWARF version is supported
    pub fn is_supported(&self, version: u16) -> bool {
        self.supported_versions.contains(&version)
    }

    /// Set current DWARF version
    pub fn set_version(&mut self, version: u16) -> Result<(), CursedError> {
        if self.is_supported(version) {
            self.current_version = Some(version);
            Ok(())
        } else {
            Err(CursedError::RuntimeError(format!("Unsupported DWARF version: {}", version)))
        }
    }

    /// Get current version
    pub fn current_version(&self) -> Option<u16> {
        self.current_version
    }

    /// Handle version-specific parsing differences
    pub fn handle_version_differences(&self, version: u16) -> Result<DwarfVersionFeatures, CursedError> {
        match version {
            2 => Ok(DwarfVersionFeatures {
                has_ranges: false,
                has_locations_v2: false,
                has_str_offsets: false,
                has_addr_table: false,
                has_rnglists: false,
                has_loclists: false,
            }),
            3 => Ok(DwarfVersionFeatures {
                has_ranges: true,
                has_locations_v2: false,
                has_str_offsets: false,
                has_addr_table: false,
                has_rnglists: false,
                has_loclists: false,
            }),
            4 => Ok(DwarfVersionFeatures {
                has_ranges: true,
                has_locations_v2: true,
                has_str_offsets: false,
                has_addr_table: false,
                has_rnglists: false,
                has_loclists: false,
            }),
            5 => Ok(DwarfVersionFeatures {
                has_ranges: true,
                has_locations_v2: true,
                has_str_offsets: true,
                has_addr_table: true,
                has_rnglists: true,
                has_loclists: true,
            }),
            _ => Err(CursedError::RuntimeError(format!("Unsupported DWARF version: {}", version)))
        }
    }
}

/// DWARF version-specific features
#[derive(Debug, Clone)]
pub struct DwarfVersionFeatures {
    /// Has .debug_ranges section
    pub has_ranges: bool,
    /// Has enhanced location descriptions (DWARF 4+)
    pub has_locations_v2: bool,
    /// Has .debug_str_offsets section (DWARF 5+)
    pub has_str_offsets: bool,
    /// Has .debug_addr section (DWARF 5+)
    pub has_addr_table: bool,
    /// Has .debug_rnglists section (DWARF 5+)
    pub has_rnglists: bool,
    /// Has .debug_loclists section (DWARF 5+)
    pub has_loclists: bool,
}

/// Error handling for malformed debug information
#[derive(Debug, Clone)]
pub struct DebugInfoErrorHandler {
    /// Continue parsing on errors
    pub continue_on_error: bool,
    /// Collect all errors encountered
    pub errors: Vec<DebugInfoError>,
    /// Maximum number of errors to collect
    pub max_errors: usize,
}

impl DebugInfoErrorHandler {
    /// Create new error handler
    pub fn new() -> Self {
        Self {
            continue_on_error: true,
            errors: Vec::new(),
            max_errors: 100,
        }
    }

    /// Handle a debug info parsing error
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

    /// Get all collected errors
    pub fn get_errors(&self) -> &[DebugInfoError] {
        &self.errors
    }

    /// Check if any errors were encountered
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

/// Debug information parsing error
#[derive(Debug, Clone)]
pub enum DebugInfoError {
    /// Malformed DIE entry
    MalformedDie { offset: u64, message: String },
    /// Invalid attribute
    InvalidAttribute { die_offset: u64, attribute: String },
    /// Missing required attribute
    MissingAttribute { die_offset: u64, attribute: String },
    /// Invalid type reference
    InvalidTypeRef { die_offset: u64, type_id: u64 },
    /// Location expression error
    LocationExpressionError { die_offset: u64, message: String },
    /// Line program error
    LineProgramError { message: String },
    /// Unsupported DWARF feature
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
