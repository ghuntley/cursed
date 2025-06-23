/// Runtime debugging support for CURSED programming language
///
/// Provides runtime symbol table management, dynamic stack inspection capabilities,
/// variable inspection and value dumping, and breakpoint simulation support.

use crate::error::Error as CursedError;
use crate::runtime::debug_info::{DebugInfo, VariableInfo, EnhancedStackTrace, EnhancedStackFrame};
use crate::debug::enhanced_debug::{DebugInfoRegistry, EnhancedDebugInfo, ScopeInfo};
use crate::stdlib::value::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};
use std::path::{Path, PathBuf};
use std::fmt;
use std::time::{Duration, Instant};

/// Runtime debugging system
#[derive(Debug)]
pub struct RuntimeDebugger {
    /// Debug info registry
    registry: Arc<DebugInfoRegistry>,
    /// Runtime symbol table
    symbol_table: Arc<RwLock<RuntimeSymbolTable>>,
    /// Active stack frames
    stack_frames: Arc<RwLock<Vec<RuntimeStackFrame>>>,
    /// Breakpoint manager
    breakpoints: Arc<RwLock<BreakpointManager>>,
    /// Variable inspector
    inspector: Arc<VariableInspector>,
    /// Debug mode flag
    debug_enabled: bool,
    /// Performance monitoring
    performance_monitor: Arc<Mutex<PerformanceMonitor>>,
}

impl RuntimeDebugger {
    /// Create new runtime debugger
    pub fn new(debug_enabled: bool) -> Self {
        RuntimeDebugger {
            registry: Arc::new(DebugInfoRegistry::new()),
            symbol_table: Arc::new(RwLock::new(RuntimeSymbolTable::new())),
            stack_frames: Arc::new(RwLock::new(Vec::new())),
            breakpoints: Arc::new(RwLock::new(BreakpointManager::new())),
            inspector: Arc::new(VariableInspector::new()),
            debug_enabled,
            performance_monitor: Arc::new(Mutex::new(PerformanceMonitor::new())),
        }
    }

    /// Register debug information
    pub fn register_debug_info(&self, debug_info: EnhancedDebugInfo) -> Result<(), Error> {
        if !self.debug_enabled {
            return Ok(());
        }

        let location_key = format!("{}:{}:{}", 
            debug_info.debug_info.file_path.display(),
            debug_info.debug_info.line,
            debug_info.debug_info.column
        );

        self.registry.register_debug_info(location_key, debug_info)
    }

    /// Enter function scope
    pub fn enter_function(&self, function_name: &str, file_path: &Path, line: u32) -> Result<(), Error> {
        if !self.debug_enabled {
            return Ok(0);
        }

        let start_time = Instant::now();

        // Create runtime stack frame
        let frame = RuntimeStackFrame::new(
            function_name.to_string(),
            file_path.to_path_buf(),
            line,
            HashMap::new(),
        );

        // Add to stack
        let mut stack_frames = self.stack_frames.write()
            .map_err(|_| CursedError::Runtime("Failed to acquire stack frames lock".to_string()))?;
        
        let frame_id = stack_frames.len() as u64;
        stack_frames.push(frame);

        // Update symbol table
        let mut symbol_table = self.symbol_table.write()
            .map_err(|_| CursedError::Runtime("Failed to acquire symbol table lock".to_string()))?;
        
        symbol_table.enter_scope(function_name.to_string());

        // Record performance
        if let Ok(mut monitor) = self.performance_monitor.lock() {
            monitor.record_function_entry(function_name, start_time);
        }

        Ok(frame_id)
    }

    /// Exit function scope
    pub fn exit_function(&self, frame_id: u64) -> Result<(), Error> {
        if !self.debug_enabled {
            return Ok(());
        }

        let start_time = Instant::now();

        // Remove from stack
        let mut stack_frames = self.stack_frames.write()
            .map_err(|_| CursedError::Runtime("Failed to acquire stack frames lock".to_string()))?;
        
        if stack_frames.len() > frame_id as usize {
            let frame = stack_frames.remove(frame_id as usize);
            
            // Record performance
            if let Ok(mut monitor) = self.performance_monitor.lock() {
                monitor.record_function_exit(&frame.function_name, start_time);
            }
        }

        // Exit scope in symbol table
        let mut symbol_table = self.symbol_table.write()
            .map_err(|_| CursedError::Runtime("Failed to acquire symbol table lock".to_string()))?;
        
        symbol_table.exit_scope();

        Ok(())
    }

    /// Register variable
    pub fn register_variable(
        &self, 
        name: String, 
        value: Value, 
        var_type: String,
        line: u32,
    ) -> Result<(), Error> {
        if !self.debug_enabled {
            return Ok(());
        }

        // Add to symbol table
        let mut symbol_table = self.symbol_table.write()
            .map_err(|_| CursedError::Runtime("Failed to acquire symbol table lock".to_string()))?;
        
        let runtime_var = RuntimeVariable::new(name.clone(), value, var_type, line);
        symbol_table.register_variable(name, runtime_var);

        Ok(())
    }

    /// Update variable value
    pub fn update_variable(&self, name: &str, value: Value) -> Result<(), Error> {
        if !self.debug_enabled {
            return Ok(());
        }

        let mut symbol_table = self.symbol_table.write()
            .map_err(|_| CursedError::Runtime("Failed to acquire symbol table lock".to_string()))?;
        
        symbol_table.update_variable(name, value);
        
        Ok(())
    }

    /// Get variable value
    pub fn get_variable(&self, name: &str) -> Result<(), Error> {
        if !self.debug_enabled {
            return Ok(None);
        }

        let symbol_table = self.symbol_table.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire symbol table lock".to_string()))?;
        
        Ok(symbol_table.get_variable(name).map(|var| var.value.clone()))
    }

    /// Get current stack trace
    pub fn get_stack_trace(&self) -> Result<(), Error> {
        let stack_frames = self.stack_frames.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire stack frames lock".to_string()))?;
        
        Ok(stack_frames.clone())
    }

    /// Inspect variable with detailed information
    pub fn inspect_variable(&self, name: &str) -> Result<(), Error> {
        if !self.debug_enabled {
            return Ok(None);
        }

        let symbol_table = self.symbol_table.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire symbol table lock".to_string()))?;
        
        if let Some(var) = symbol_table.get_variable(name) {
            Ok(Some(self.inspector.inspect_variable(var)?))
        } else {
            Ok(None)
        }
    }

    /// Set breakpoint
    pub fn set_breakpoint(&self, file_path: PathBuf, line: u32) -> Result<(), Error> {
        let mut breakpoints = self.breakpoints.write()
            .map_err(|_| CursedError::Runtime("Failed to acquire breakpoints lock".to_string()))?;
        
        breakpoints.set_breakpoint(file_path, line)
    }

    /// Remove breakpoint
    pub fn remove_breakpoint(&self, breakpoint_id: u64) -> Result<(), Error> {
        let mut breakpoints = self.breakpoints.write()
            .map_err(|_| CursedError::Runtime("Failed to acquire breakpoints lock".to_string()))?;
        
        Ok(breakpoints.remove_breakpoint(breakpoint_id))
    }

    /// Check if breakpoint should trigger
    pub fn check_breakpoint(&self, file_path: &Path, line: u32) -> Result<(), Error> {
        if !self.debug_enabled {
            return Ok(None);
        }

        let breakpoints = self.breakpoints.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire breakpoints lock".to_string()))?;
        
        Ok(breakpoints.check_breakpoint(file_path, line))
    }

    /// Get all variables in current scope
    pub fn get_scope_variables(&self) -> Result<(), Error> {
        let symbol_table = self.symbol_table.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire symbol table lock".to_string()))?;
        
        Ok(symbol_table.get_current_scope_variables())
    }

    /// Generate debug report
    pub fn generate_debug_report(&self) -> Result<(), Error> {
        let stack_trace = self.get_stack_trace()?;
        let scope_vars = self.get_scope_variables()?;
        
        let breakpoints = self.breakpoints.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire breakpoints lock".to_string()))?;
        
        let performance = self.performance_monitor.lock()
            .map_err(|_| CursedError::Runtime("Failed to acquire performance monitor lock".to_string()))?;

        Ok(DebugReport {
            stack_trace,
            scope_variables: scope_vars,
            active_breakpoints: breakpoints.get_all_breakpoints(),
            performance_data: performance.get_summary(),
        })
    }

    /// Get debug registry (for external access)
    pub fn get_registry(&self) -> Arc<DebugInfoRegistry> {
        self.registry.clone()
    }
}

/// Runtime symbol table for variable tracking
#[derive(Debug)]
pub struct RuntimeSymbolTable {
    /// Scope stack
    scope_stack: Vec<String>,
    /// Variables by scope
    variables: HashMap<String, HashMap<String, RuntimeVariable>>,
    /// Current scope depth
    scope_depth: usize,
}

impl RuntimeSymbolTable {
    /// Create new symbol table
    pub fn new() -> Self {
        RuntimeSymbolTable {
            scope_stack: vec!["global".to_string()],
            variables: HashMap::new(),
            scope_depth: 0,
        }
    }

    /// Enter new scope
    pub fn enter_scope(&mut self, scope_name: String) {
        self.scope_stack.push(scope_name.clone());
        self.scope_depth += 1;
        
        if !self.variables.contains_key(&scope_name) {
            self.variables.insert(scope_name, HashMap::new());
        }
    }

    /// Exit current scope
    pub fn exit_scope(&mut self) {
        if self.scope_stack.len() > 1 {
            self.scope_stack.pop();
            self.scope_depth = self.scope_depth.saturating_sub(1);
        }
    }

    /// Register variable in current scope
    pub fn register_variable(&mut self, name: String, variable: RuntimeVariable) {
        if let Some(current_scope) = self.scope_stack.last() {
            if let Some(scope_vars) = self.variables.get_mut(current_scope) {
                scope_vars.insert(name, variable);
            }
        }
    }

    /// Update variable value
    pub fn update_variable(&mut self, name: &str, value: Value) {
        // Search in reverse scope order (most recent first)
        for scope_name in self.scope_stack.iter().rev() {
            if let Some(scope_vars) = self.variables.get_mut(scope_name) {
                if let Some(var) = scope_vars.get_mut(name) {
                    var.value = value;
                    return;
                }
            }
        }
    }

    /// Get variable (searches scopes in reverse order)
    pub fn get_variable(&self, name: &str) -> Option<&RuntimeVariable> {
        for scope_name in self.scope_stack.iter().rev() {
            if let Some(scope_vars) = self.variables.get(scope_name) {
                if let Some(var) = scope_vars.get(name) {
                    return Some(var);
                }
            }
        }
        None
    }

    /// Get all variables in current scope
    pub fn get_current_scope_variables(&self) -> HashMap<String, Value> {
        if let Some(current_scope) = self.scope_stack.last() {
            if let Some(scope_vars) = self.variables.get(current_scope) {
                return scope_vars.iter()
                    .map(|(name, var)| (name.clone(), var.value.clone()))
                    .collect();
            }
        }
        HashMap::new()
    }
}

/// Runtime variable information
#[derive(Debug, Clone)]
pub struct RuntimeVariable {
    /// Variable name
    pub name: String,
    /// Variable value
    pub value: Value,
    /// Variable type
    pub var_type: String,
    /// Declaration line
    pub declaration_line: u32,
    /// Last modified time
    pub last_modified: Instant,
}

impl RuntimeVariable {
    /// Create new runtime variable
    pub fn new(name: String, value: Value, var_type: String, declaration_line: u32) -> Self {
        RuntimeVariable {
            name,
            value,
            var_type,
            declaration_line,
            last_modified: Instant::now(),
        }
    }
}

/// Runtime stack frame information
#[derive(Debug, Clone)]
pub struct RuntimeStackFrame {
    /// Function name
    pub function_name: String,
    /// Source file
    pub file_path: PathBuf,
    /// Current line
    pub line: u32,
    /// Local variables
    pub local_variables: HashMap<String, Value>,
    /// Frame creation time
    pub created_at: Instant,
}

impl RuntimeStackFrame {
    /// Create new runtime stack frame
    pub fn new(
        function_name: String,
        file_path: PathBuf,
        line: u32,
        local_variables: HashMap<String, Value>,
    ) -> Self {
        RuntimeStackFrame {
            function_name,
            file_path,
            line,
            local_variables,
            created_at: Instant::now(),
        }
    }
}

/// Variable inspector for detailed variable analysis
#[derive(Debug)]
pub struct VariableInspector {
    /// Maximum inspection depth for recursive structures
    max_depth: usize,
}

impl VariableInspector {
    /// Create new variable inspector
    pub fn new() -> Self {
        VariableInspector {
            max_depth: 10,
        }
    }

    /// Inspect variable with detailed information
    pub fn inspect_variable(&self, variable: &RuntimeVariable) -> Result<(), Error> {
        let size_estimate = self.estimate_size(&variable.value);
        let type_info = self.analyze_type(&variable.value);
        let contents = self.dump_contents(&variable.value, 0)?;

        Ok(VariableInspection {
            name: variable.name.clone(),
            var_type: variable.var_type.clone(),
            value: variable.value.clone(),
            size_estimate,
            type_info,
            contents,
            declaration_line: variable.declaration_line,
            last_modified: variable.last_modified,
        })
    }

    /// Estimate variable size in bytes
    fn estimate_size(&self, value: &Value) -> usize {
        match value {
            Value::Null => 0,
            Value::Bool(_) => 1,
            Value::Integer(_) => 8,
            Value::Number(_) => 8,
            Value::String(s) => s.len(),
            Value::Array(arr) => arr.iter().map(|v| self.estimate_size(v)).sum::<usize>() + 8,
            Value::Object(obj) => obj.iter().map(|(k, v)| k.len() + self.estimate_size(v)).sum::<usize>() + 8,
            Value::Bytes(b) => b.len(), // Bytes size estimate
        }
    }

    /// Analyze type information
    fn analyze_type(&self, value: &Value) -> TypeAnalysis {
        match value {
            Value::Null => TypeAnalysis::new("nil", false, false),
            Value::Bool(_) => TypeAnalysis::new("facts", false, false),
            Value::Integer(_) => TypeAnalysis::new("sus", false, false),
            Value::Number(_) => TypeAnalysis::new("vibes", false, false),
            Value::String(_) => TypeAnalysis::new("tea", false, false),
            Value::Array(_) => TypeAnalysis::new("array", true, false),
            Value::Object(_) => TypeAnalysis::new("object", true, true),
            Value::Bytes(_) => TypeAnalysis::new("bytes", false, false),
        }
    }

    /// Dump variable contents
    fn dump_contents(&self, value: &Value, depth: usize) -> Result<(), Error> {
        if depth > self.max_depth {
            return Ok("... (max depth reached)".to_string());
        }

        match value {
            Value::Null => Ok("nil".to_string()),
            Value::Bool(b) => Ok(b.to_string()),
            Value::Integer(i) => Ok(i.to_string()),
            Value::Number(f) => Ok(f.to_string()),
            Value::String(s) => Ok(format!("\"{}\"", s)),
            Value::Array(arr) => {
                let items: Result<Vec<String>, _> = arr.iter()
                    .take(10) // Limit array elements shown
                    .map(|v| self.dump_contents(v, depth + 1))
                    .collect();
                
                let items = items?;
                let mut result = format!("[{}]", items.join(", "));
                
                if arr.len() > 10 {
                    result.push_str(&format!(" ... ({} more items)", arr.len() - 10));
                }
                
                Ok(result)
            }
            Value::Object(obj) => {
                let mut items = Vec::new();
                for (k, v) in obj.iter().take(10) {
                    let value_str = self.dump_contents(v, depth + 1)?;
                    items.push(format!("{}: {}", k, value_str));
                }
                
                let mut result = format!("{{{}}}", items.join(", "));
                
                if obj.len() > 10 {
                    result.push_str(&format!(" ... ({} more fields)", obj.len() - 10));
                }
                
                Ok(result)
            }
            Value::Bytes(b) => Ok(format!("bytes[{}]", b.len())),
        }
    }
}

/// Variable inspection result
#[derive(Debug, Clone)]
pub struct VariableInspection {
    pub name: String,
    pub var_type: String,
    pub value: Value,
    pub size_estimate: usize,
    pub type_info: TypeAnalysis,
    pub contents: String,
    pub declaration_line: u32,
    pub last_modified: Instant,
}

/// Type analysis information
#[derive(Debug, Clone)]
pub struct TypeAnalysis {
    pub type_name: String,
    pub is_collection: bool,
    pub is_complex: bool,
}

impl TypeAnalysis {
    fn new(type_name: &str, is_collection: bool, is_complex: bool) -> Self {
        TypeAnalysis {
            type_name: type_name.to_string(),
            is_collection,
            is_complex,
        }
    }
}

/// Breakpoint manager
#[derive(Debug)]
pub struct BreakpointManager {
    /// Active breakpoints
    breakpoints: HashMap<u64, Breakpoint>,
    /// Next breakpoint ID
    next_id: u64,
}

impl BreakpointManager {
    /// Create new breakpoint manager
    pub fn new() -> Self {
        BreakpointManager {
            breakpoints: HashMap::new(),
            next_id: 1,
        }
    }

    /// Set breakpoint
    pub fn set_breakpoint(&mut self, file_path: PathBuf, line: u32) -> Result<(), Error> {
        let id = self.next_id;
        self.next_id += 1;

        let breakpoint = Breakpoint::new(id, file_path, line);
        self.breakpoints.insert(id, breakpoint);

        Ok(id)
    }

    /// Remove breakpoint
    pub fn remove_breakpoint(&mut self, breakpoint_id: u64) -> bool {
        self.breakpoints.remove(&breakpoint_id).is_some()
    }

    /// Check if breakpoint should trigger
    pub fn check_breakpoint(&self, file_path: &Path, line: u32) -> Option<Breakpoint> {
        self.breakpoints.values()
            .find(|bp| bp.file_path == file_path && bp.line == line)
            .cloned()
    }

    /// Get all breakpoints
    pub fn get_all_breakpoints(&self) -> Vec<Breakpoint> {
        self.breakpoints.values().cloned().collect()
    }
}

/// Breakpoint information
#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub id: u64,
    pub file_path: PathBuf,
    pub line: u32,
    pub enabled: bool,
    pub hit_count: u32,
    pub created_at: Instant,
}

impl Breakpoint {
    /// Create new breakpoint
    pub fn new(id: u64, file_path: PathBuf, line: u32) -> Self {
        Breakpoint {
            id,
            file_path,
            line,
            enabled: true,
            hit_count: 0,
            created_at: Instant::now(),
        }
    }

    /// Record breakpoint hit
    pub fn hit(&mut self) {
        self.hit_count += 1;
    }
}

/// Performance monitoring for debugging
#[derive(Debug)]
pub struct PerformanceMonitor {
    /// Function call counts
    function_calls: HashMap<String, u64>,
    /// Function execution times
    function_times: HashMap<String, Vec<Duration>>,
    /// Total debugging overhead
    debug_overhead: Duration,
}

impl PerformanceMonitor {
    /// Create new performance monitor
    pub fn new() -> Self {
        PerformanceMonitor {
            function_calls: HashMap::new(),
            function_times: HashMap::new(),
            debug_overhead: Duration::new(0, 0),
        }
    }

    /// Record function entry
    pub fn record_function_entry(&mut self, function_name: &str, _start_time: Instant) {
        *self.function_calls.entry(function_name.to_string()).or_insert(0) += 1;
    }

    /// Record function exit
    pub fn record_function_exit(&mut self, function_name: &str, start_time: Instant) {
        let duration = start_time.elapsed();
        self.function_times.entry(function_name.to_string())
            .or_insert_with(Vec::new)
            .push(duration);
    }

    /// Get performance summary
    pub fn get_summary(&self) -> PerformanceSummary {
        let total_calls: u64 = self.function_calls.values().sum();
        let total_time: Duration = self.function_times.values()
            .flatten()
            .sum();

        PerformanceSummary {
            total_function_calls: total_calls,
            total_execution_time: total_time,
            debug_overhead: self.debug_overhead,
            function_call_counts: self.function_calls.clone(),
        }
    }
}

/// Performance summary
#[derive(Debug, Clone)]
pub struct PerformanceSummary {
    pub total_function_calls: u64,
    pub total_execution_time: Duration,
    pub debug_overhead: Duration,
    pub function_call_counts: HashMap<String, u64>,
}

/// Debug report
#[derive(Debug)]
pub struct DebugReport {
    pub stack_trace: Vec<RuntimeStackFrame>,
    pub scope_variables: HashMap<String, Value>,
    pub active_breakpoints: Vec<Breakpoint>,
    pub performance_data: PerformanceSummary,
}

impl fmt::Display for DebugReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Debug Report ===")?;
        
        writeln!(f, "\nStack Trace ({} frames):", self.stack_trace.len())?;
        for (i, frame) in self.stack_trace.iter().enumerate() {
            writeln!(f, "  {}: {} at {}:{}", 
                i, frame.function_name, 
                frame.file_path.display(), frame.line)?;
        }
        
        writeln!(f, "\nScope Variables ({}):", self.scope_variables.len())?;
        for (name, value) in &self.scope_variables {
            writeln!(f, "  {}: {:?}", name, value)?;
        }
        
        writeln!(f, "\nActive Breakpoints ({}):", self.active_breakpoints.len())?;
        for bp in &self.active_breakpoints {
            writeln!(f, "  {}: {}:{} (hits: {})", 
                bp.id, bp.file_path.display(), bp.line, bp.hit_count)?;
        }
        
        writeln!(f, "\nPerformance:")?;
        writeln!(f, "  Total function calls: {}", self.performance_data.total_function_calls)?;
        writeln!(f, "  Total execution time: {:?}", self.performance_data.total_execution_time)?;
        writeln!(f, "  Debug overhead: {:?}", self.performance_data.debug_overhead)?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_debugger_creation() {
        let debugger = RuntimeDebugger::new(true);
        assert!(debugger.debug_enabled);
    }

    #[test]
    fn test_function_scope_management() {
        let debugger = RuntimeDebugger::new(true);
        
        let frame_id = debugger.enter_function("test_func", Path::new("test.csd"), 42);
        assert!(frame_id.is_ok());
        
        let exit_result = debugger.exit_function(frame_id.unwrap());
        assert!(exit_result.is_ok());
    }

    #[test]
    fn test_variable_registration() {
        let debugger = RuntimeDebugger::new(true);
        
        let _ = debugger.enter_function("test_func", Path::new("test.csd"), 42);
        let result = debugger.register_variable(
            "test_var".to_string(),
            Value::Integer(42),
            "sus".to_string(),
            10,
        );
        
        assert!(result.is_ok());
        
        let value = debugger.get_variable("test_var");
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), Some(Value::Integer(42)));
    }

    #[test]
    fn test_variable_inspection() {
        let debugger = RuntimeDebugger::new(true);
        
        let _ = debugger.enter_function("test_func", Path::new("test.csd"), 42);
        let _ = debugger.register_variable(
            "test_var".to_string(),
            Value::String("test value".to_string()),
            "tea".to_string(),
            10,
        );
        
        let inspection = debugger.inspect_variable("test_var");
        assert!(inspection.is_ok());
        
        let inspection = inspection.unwrap();
        assert!(inspection.is_some());
        
        let inspection = inspection.unwrap();
        assert_eq!(inspection.name, "test_var");
        assert_eq!(inspection.var_type, "tea");
    }

    #[test]
    fn test_breakpoint_management() {
        let debugger = RuntimeDebugger::new(true);
        
        let bp_id = debugger.set_breakpoint(PathBuf::from("test.csd"), 42);
        assert!(bp_id.is_ok());
        
        let check = debugger.check_breakpoint(Path::new("test.csd"), 42);
        assert!(check.is_ok());
        assert!(check.unwrap().is_some());
        
        let removed = debugger.remove_breakpoint(bp_id.unwrap());
        assert!(removed.is_ok());
        assert!(removed.unwrap());
    }

    #[test]
    fn test_stack_trace_generation() {
        let debugger = RuntimeDebugger::new(true);
        
        let _ = debugger.enter_function("func1", Path::new("test.csd"), 10);
        let _ = debugger.enter_function("func2", Path::new("test.csd"), 20);
        
        let stack_trace = debugger.get_stack_trace();
        assert!(stack_trace.is_ok());
        
        let frames = stack_trace.unwrap();
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].function_name, "func1");
        assert_eq!(frames[1].function_name, "func2");
    }

    #[test]
    fn test_debug_report_generation() {
        let debugger = RuntimeDebugger::new(true);
        
        let _ = debugger.enter_function("test_func", Path::new("test.csd"), 42);
        let _ = debugger.register_variable(
            "test_var".to_string(),
            Value::Integer(42),
            "sus".to_string(),
            10,
        );
        let _ = debugger.set_breakpoint(PathBuf::from("test.csd"), 42);
        
        let report = debugger.generate_debug_report();
        assert!(report.is_ok());
        
        let report = report.unwrap();
        assert!(!report.stack_trace.is_empty());
        assert!(!report.scope_variables.is_empty());
        assert!(!report.active_breakpoints.is_empty());
    }

    #[test]
    fn test_symbol_table_scoping() {
        let mut symbol_table = RuntimeSymbolTable::new();
        
        symbol_table.enter_scope("function1".to_string());
        let var1 = RuntimeVariable::new("var1".to_string(), Value::Integer(1), "sus".to_string(), 1);
        symbol_table.register_variable("var1".to_string(), var1);
        
        symbol_table.enter_scope("function2".to_string());
        let var2 = RuntimeVariable::new("var2".to_string(), Value::Integer(2), "sus".to_string(), 2);
        symbol_table.register_variable("var2".to_string(), var2);
        
        // Should find var2 in current scope
        assert!(symbol_table.get_variable("var2").is_some());
        // Should find var1 in parent scope
        assert!(symbol_table.get_variable("var1").is_some());
        
        symbol_table.exit_scope();
        
        // Should still find var1
        assert!(symbol_table.get_variable("var1").is_some());
        // Should not find var2 (out of scope)
        assert!(symbol_table.get_variable("var2").is_none());
    }
}
