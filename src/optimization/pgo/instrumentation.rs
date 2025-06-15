/// Code Instrumentation for Profile Collection
/// 
/// Provides instrumentation capabilities for collecting profile data
/// at both source and LLVM IR levels.

use crate::error::{Error, Result};
use crate::optimization::pgo::{PgoConfig, InstrumentationMode};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{info, debug, warn, instrument};

/// Instrumentation manager
#[derive(Debug)]
pub struct InstrumentationManager {
    config: PgoConfig,
    active_instrumentations: HashMap<String, InstrumentationSession>,
    counter_registry: CounterRegistry,
    instrumentation_templates: InstrumentationTemplates,
}

/// Active instrumentation session
#[derive(Debug, Clone)]
struct InstrumentationSession {
    session_id: String,
    target_files: Vec<PathBuf>,
    instrumentation_mode: InstrumentationMode,
    counter_count: u32,
    output_directory: PathBuf,
}

/// Counter registry for tracking instrumentation points
#[derive(Debug, Default)]
struct CounterRegistry {
    function_counters: HashMap<String, u32>,
    basic_block_counters: HashMap<String, u32>,
    edge_counters: HashMap<String, u32>,
    value_counters: HashMap<String, u32>,
    next_counter_id: u32,
}

/// Templates for different instrumentation patterns
#[derive(Debug)]
struct InstrumentationTemplates {
    function_entry_template: String,
    basic_block_template: String,
    edge_template: String,
    value_profiling_template: String,
    runtime_header: String,
}

impl InstrumentationManager {
    /// Create a new instrumentation manager
    #[instrument]
    pub fn new(config: PgoConfig) -> Result<Self> {
        info!("Creating instrumentation manager with mode: {:?}", config.instrumentation_mode);

        let templates = InstrumentationTemplates::new();
        
        Ok(Self {
            config,
            active_instrumentations: HashMap::new(),
            counter_registry: CounterRegistry::default(),
            instrumentation_templates: templates,
        })
    }

    /// Start instrumentation for a session
    #[instrument(skip(self))]
    pub fn start_instrumentation(&mut self, session_id: &str) -> Result<()> {
        if !self.config.enabled {
            return Ok(());
        }

        info!("Starting instrumentation for session: {}", session_id);

        let output_dir = self.config.profile_data_dir.join(format!("instrumentation_{}", session_id));
        fs::create_dir_all(&output_dir).map_err(|e| {
            Error::Other(format!("Failed to create instrumentation directory: {}", e))
        })?;

        let session = InstrumentationSession {
            session_id: session_id.to_string(),
            target_files: Vec::new(),
            instrumentation_mode: self.config.instrumentation_mode.clone(),
            counter_count: 0,
            output_directory: output_dir,
        };

        self.active_instrumentations.insert(session_id.to_string(), session);

        // Generate instrumentation runtime if needed
        self.generate_instrumentation_runtime(session_id)?;

        Ok(())
    }

    /// Stop instrumentation for a session
    #[instrument(skip(self))]
    pub fn stop_instrumentation(&mut self) -> Result<()> {
        info!("Stopping all active instrumentations");

        // Generate final instrumentation data
        for (session_id, session) in &self.active_instrumentations {
            self.finalize_instrumentation_session(session_id, session)?;
        }

        self.active_instrumentations.clear();
        Ok(())
    }

    /// Instrument source code
    #[instrument(skip(self, source_code))]
    pub fn instrument_source_code(&self, source_code: &str, target: &str) -> Result<String> {
        if !self.config.enabled {
            return Ok(source_code.to_string());
        }

        info!("Instrumenting source code for target: {}", target);

        match self.config.instrumentation_mode {
            InstrumentationMode::Frontend => self.instrument_source_frontend(source_code, target),
            InstrumentationMode::Sampling => Ok(source_code.to_string()), // No source changes needed
            InstrumentationMode::Hybrid => self.instrument_source_frontend(source_code, target),
            _ => Ok(source_code.to_string()),
        }
    }

    /// Frontend source-level instrumentation
    #[instrument(skip(self, source_code))]
    fn instrument_source_frontend(&self, source_code: &str, target: &str) -> Result<String> {
        let mut instrumented_code = String::new();
        
        // Add runtime includes at the top
        instrumented_code.push_str(&self.instrumentation_templates.runtime_header);
        instrumented_code.push('\n');

        // Parse and instrument the source code
        let lines: Vec<&str> = source_code.lines().collect();
        let mut in_function = false;
        let mut current_function = String::new();
        let mut brace_count = 0;

        for line in lines {
            let trimmed = line.trim();
            
            // Detect function declarations
            if self.is_function_declaration(trimmed) {
                current_function = self.extract_function_name(trimmed);
                in_function = true;
                instrumented_code.push_str(line);
                instrumented_code.push('\n');
                continue;
            }

            // Add function entry instrumentation
            if in_function && trimmed == "{" {
                instrumented_code.push_str(line);
                instrumented_code.push('\n');
                
                let counter_name = format!("__prof_func_{}", current_function);
                let instrumentation = self.instrumentation_templates.function_entry_template
                    .replace("{COUNTER_NAME}", &counter_name)
                    .replace("{FUNCTION_NAME}", &current_function);
                
                instrumented_code.push_str("    ");
                instrumented_code.push_str(&instrumentation);
                instrumented_code.push('\n');
                
                brace_count += 1;
                continue;
            }

            // Track brace nesting
            if trimmed == "{" {
                brace_count += 1;
            } else if trimmed == "}" {
                brace_count -= 1;
                if brace_count == 0 {
                    in_function = false;
                }
            }

            // Add basic block instrumentation for control flow statements
            if in_function && self.is_control_flow_statement(trimmed) {
                let block_id = format!("{}_bb_{}", current_function, self.generate_block_id());
                let instrumentation = self.instrumentation_templates.basic_block_template
                    .replace("{BLOCK_ID}", &block_id);
                
                instrumented_code.push_str("    ");
                instrumented_code.push_str(&instrumentation);
                instrumented_code.push('\n');
            }

            instrumented_code.push_str(line);
            instrumented_code.push('\n');
        }

        debug!("Instrumented source code for target: {}", target);
        Ok(instrumented_code)
    }

    /// Generate instrumentation runtime code
    #[instrument(skip(self))]
    fn generate_instrumentation_runtime(&self, session_id: &str) -> Result<()> {
        let session = self.active_instrumentations.get(session_id)
            .ok_or_else(|| Error::Other("Session not found".to_string()))?;

        let runtime_path = session.output_directory.join("pgo_runtime.c");
        
        let runtime_code = self.generate_runtime_code()?;
        
        fs::write(&runtime_path, runtime_code).map_err(|e| {
            Error::Other(format!("Failed to write runtime code: {}", e))
        })?;

        // Generate header file
        let header_path = session.output_directory.join("pgo_runtime.h");
        let header_code = self.generate_runtime_header()?;
        
        fs::write(&header_path, header_code).map_err(|e| {
            Error::Other(format!("Failed to write runtime header: {}", e))
        })?;

        debug!("Generated instrumentation runtime for session: {}", session_id);
        Ok(())
    }

    /// Generate runtime implementation code
    fn generate_runtime_code(&self) -> Result<String> {
        let runtime_code = r#"
// PGO Runtime Implementation
#include "pgo_runtime.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdatomic.h>

// Global profile data structure
static struct {
    atomic_uint_fast64_t *function_counters;
    atomic_uint_fast64_t *basic_block_counters;
    atomic_uint_fast64_t *edge_counters;
    atomic_uint_fast64_t *value_counters;
    unsigned int num_functions;
    unsigned int num_blocks;
    unsigned int num_edges;
    unsigned int num_values;
    const char **function_names;
    const char **block_names;
    const char **edge_names;
    const char **value_names;
} pgo_data = {0};

// Initialize profiling data structures
void __pgo_init(unsigned int num_funcs, unsigned int num_blocks, 
                unsigned int num_edges, unsigned int num_values) {
    pgo_data.num_functions = num_funcs;
    pgo_data.num_blocks = num_blocks;
    pgo_data.num_edges = num_edges;
    pgo_data.num_values = num_values;
    
    if (num_funcs > 0) {
        pgo_data.function_counters = calloc(num_funcs, sizeof(atomic_uint_fast64_t));
        pgo_data.function_names = calloc(num_funcs, sizeof(char*));
    }
    
    if (num_blocks > 0) {
        pgo_data.basic_block_counters = calloc(num_blocks, sizeof(atomic_uint_fast64_t));
        pgo_data.block_names = calloc(num_blocks, sizeof(char*));
    }
    
    if (num_edges > 0) {
        pgo_data.edge_counters = calloc(num_edges, sizeof(atomic_uint_fast64_t));
        pgo_data.edge_names = calloc(num_edges, sizeof(char*));
    }
    
    if (num_values > 0) {
        pgo_data.value_counters = calloc(num_values, sizeof(atomic_uint_fast64_t));
        pgo_data.value_names = calloc(num_values, sizeof(char*));
    }
    
    // Register exit handler to write profile data
    atexit(__pgo_write_profile);
}

// Increment function counter
void __pgo_increment_function(unsigned int counter_id) {
    if (counter_id < pgo_data.num_functions && pgo_data.function_counters) {
        atomic_fetch_add_explicit(&pgo_data.function_counters[counter_id], 1, 
                                  memory_order_relaxed);
    }
}

// Increment basic block counter
void __pgo_increment_block(unsigned int counter_id) {
    if (counter_id < pgo_data.num_blocks && pgo_data.basic_block_counters) {
        atomic_fetch_add_explicit(&pgo_data.basic_block_counters[counter_id], 1,
                                  memory_order_relaxed);
    }
}

// Increment edge counter
void __pgo_increment_edge(unsigned int counter_id) {
    if (counter_id < pgo_data.num_edges && pgo_data.edge_counters) {
        atomic_fetch_add_explicit(&pgo_data.edge_counters[counter_id], 1,
                                  memory_order_relaxed);
    }
}

// Record value profile
void __pgo_record_value(unsigned int site_id, uint64_t value) {
    if (site_id < pgo_data.num_values && pgo_data.value_counters) {
        // Simplified: just count occurrences
        atomic_fetch_add_explicit(&pgo_data.value_counters[site_id], 1,
                                  memory_order_relaxed);
    }
}

// Register function name
void __pgo_register_function(unsigned int counter_id, const char *name) {
    if (counter_id < pgo_data.num_functions && pgo_data.function_names) {
        pgo_data.function_names[counter_id] = name;
    }
}

// Write profile data to file
void __pgo_write_profile(void) {
    const char *profile_file = getenv("PGO_PROFILE_FILE");
    if (!profile_file) {
        profile_file = "default.profraw";
    }
    
    FILE *f = fopen(profile_file, "w");
    if (!f) {
        return;
    }
    
    // Write function profiles
    for (unsigned int i = 0; i < pgo_data.num_functions; i++) {
        uint64_t count = atomic_load_explicit(&pgo_data.function_counters[i],
                                             memory_order_relaxed);
        const char *name = pgo_data.function_names[i] ? 
                          pgo_data.function_names[i] : "unknown";
        fprintf(f, "func:%s %lu 0\n", name, count);
    }
    
    // Write basic block profiles
    for (unsigned int i = 0; i < pgo_data.num_blocks; i++) {
        uint64_t count = atomic_load_explicit(&pgo_data.basic_block_counters[i],
                                             memory_order_relaxed);
        const char *name = pgo_data.block_names[i] ? 
                          pgo_data.block_names[i] : "unknown";
        fprintf(f, "bb:%s %lu\n", name, count);
    }
    
    // Write edge profiles
    for (unsigned int i = 0; i < pgo_data.num_edges; i++) {
        uint64_t count = atomic_load_explicit(&pgo_data.edge_counters[i],
                                             memory_order_relaxed);
        const char *name = pgo_data.edge_names[i] ? 
                          pgo_data.edge_names[i] : "unknown";
        fprintf(f, "edge:%s %lu\n", name, count);
    }
    
    // Write value profiles
    for (unsigned int i = 0; i < pgo_data.num_values; i++) {
        uint64_t count = atomic_load_explicit(&pgo_data.value_counters[i],
                                             memory_order_relaxed);
        const char *name = pgo_data.value_names[i] ? 
                          pgo_data.value_names[i] : "unknown";
        fprintf(f, "value:%s:0 %lu\n", name, count);
    }
    
    fclose(f);
}

// Force profile write (for manual control)
void __pgo_flush_profile(void) {
    __pgo_write_profile();
}
"#;

        Ok(runtime_code.to_string())
    }

    /// Generate runtime header file
    fn generate_runtime_header(&self) -> Result<String> {
        let header_code = r#"
#ifndef PGO_RUNTIME_H
#define PGO_RUNTIME_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Runtime initialization
void __pgo_init(unsigned int num_funcs, unsigned int num_blocks, 
                unsigned int num_edges, unsigned int num_values);

// Counter increment functions
void __pgo_increment_function(unsigned int counter_id);
void __pgo_increment_block(unsigned int counter_id);
void __pgo_increment_edge(unsigned int counter_id);

// Value profiling
void __pgo_record_value(unsigned int site_id, uint64_t value);

// Name registration
void __pgo_register_function(unsigned int counter_id, const char *name);

// Profile data output
void __pgo_write_profile(void);
void __pgo_flush_profile(void);

#ifdef __cplusplus
}
#endif

#endif // PGO_RUNTIME_H
"#;

        Ok(header_code.to_string())
    }

    /// Finalize instrumentation session
    fn finalize_instrumentation_session(&self, session_id: &str, session: &InstrumentationSession) -> Result<()> {
        debug!("Finalizing instrumentation session: {}", session_id);

        // Generate initialization code with actual counter counts
        let init_code = format!(
            "// Auto-generated PGO initialization\n\
             #include \"pgo_runtime.h\"\n\
             __attribute__((constructor))\n\
             void __pgo_init_counters(void) {{\n\
                 __pgo_init({}, {}, {}, {});\n\
             }}\n",
            self.counter_registry.function_counters.len(),
            self.counter_registry.basic_block_counters.len(),
            self.counter_registry.edge_counters.len(),
            self.counter_registry.value_counters.len()
        );

        let init_path = session.output_directory.join("pgo_init.c");
        fs::write(&init_path, init_code).map_err(|e| {
            Error::Other(format!("Failed to write initialization code: {}", e))
        })?;

        Ok(())
    }

    /// Update configuration
    pub fn update_config(&mut self, new_config: PgoConfig) -> Result<()> {
        self.config = new_config;
        Ok(())
    }

    // Helper methods for source code analysis

    fn is_function_declaration(&self, line: &str) -> bool {
        // Simplified function detection for CURSED syntax
        line.contains("slay ") && line.contains("(") && !line.contains(";")
    }

    fn extract_function_name(&self, line: &str) -> String {
        // Extract function name from CURSED function declaration
        if let Some(start) = line.find("slay ") {
            let after_slay = &line[start + 5..];
            if let Some(paren_pos) = after_slay.find('(') {
                let name = after_slay[..paren_pos].trim();
                return name.to_string();
            }
        }
        "unknown".to_string()
    }

    fn is_control_flow_statement(&self, line: &str) -> bool {
        line.contains("lowkey") || line.contains("highkey") || 
        line.contains("periodt") || line.contains("bestie") ||
        line.contains("flex") || line.contains("yolo")
    }

    fn generate_block_id(&self) -> u32 {
        // Generate unique block ID
        rand::random::<u32>()
    }
}

impl InstrumentationTemplates {
    fn new() -> Self {
        Self {
            function_entry_template: "__pgo_increment_function({COUNTER_ID}); __pgo_register_function({COUNTER_ID}, \"{FUNCTION_NAME}\");".to_string(),
            basic_block_template: "__pgo_increment_block({COUNTER_ID});".to_string(),
            edge_template: "__pgo_increment_edge({COUNTER_ID});".to_string(),
            value_profiling_template: "__pgo_record_value({SITE_ID}, {VALUE});".to_string(),
            runtime_header: "#include \"pgo_runtime.h\"".to_string(),
        }
    }
}

impl CounterRegistry {
    fn allocate_function_counter(&mut self, function_name: &str) -> u32 {
        let counter_id = self.next_counter_id;
        self.function_counters.insert(function_name.to_string(), counter_id);
        self.next_counter_id += 1;
        counter_id
    }

    fn allocate_block_counter(&mut self, block_name: &str) -> u32 {
        let counter_id = self.next_counter_id;
        self.basic_block_counters.insert(block_name.to_string(), counter_id);
        self.next_counter_id += 1;
        counter_id
    }

    fn allocate_edge_counter(&mut self, edge_name: &str) -> u32 {
        let counter_id = self.next_counter_id;
        self.edge_counters.insert(edge_name.to_string(), counter_id);
        self.next_counter_id += 1;
        counter_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instrumentation_manager_creation() {
        let config = PgoConfig::default();
        let manager = InstrumentationManager::new(config);
        assert!(manager.is_ok());
    }

    #[test]
    fn test_function_detection() {
        let config = PgoConfig::default();
        let manager = InstrumentationManager::new(config).unwrap();
        
        assert!(manager.is_function_declaration("slay main()"));
        assert!(manager.is_function_declaration("  slay compute_value(x: i32) -> i32"));
        assert!(!manager.is_function_declaration("let x = 5;"));
        assert!(!manager.is_function_declaration("slay test_func(); // declaration only"));
    }

    #[test]
    fn test_function_name_extraction() {
        let config = PgoConfig::default();
        let manager = InstrumentationManager::new(config).unwrap();
        
        assert_eq!(manager.extract_function_name("slay main()"), "main");
        assert_eq!(manager.extract_function_name("  slay compute_value(x: i32)"), "compute_value");
        assert_eq!(manager.extract_function_name("invalid"), "unknown");
    }

    #[test]
    fn test_control_flow_detection() {
        let config = PgoConfig::default();
        let manager = InstrumentationManager::new(config).unwrap();
        
        assert!(manager.is_control_flow_statement("lowkey (x > 0)"));
        assert!(manager.is_control_flow_statement("  highkey"));
        assert!(manager.is_control_flow_statement("periodt;"));
        assert!(!manager.is_control_flow_statement("let x = 5;"));
    }

    #[test]
    fn test_counter_registry() {
        let mut registry = CounterRegistry::default();
        
        let func_id = registry.allocate_function_counter("test_func");
        let block_id = registry.allocate_block_counter("test_block");
        
        assert_eq!(func_id, 0);
        assert_eq!(block_id, 1);
        assert_eq!(registry.function_counters.get("test_func"), Some(&0));
        assert_eq!(registry.basic_block_counters.get("test_block"), Some(&1));
    }
}
