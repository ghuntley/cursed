//! Interactive debugger for CURSED REPL
//! Provides step-through debugging, breakpoints, and variable inspection

use crate::error::CursedError;
use std::collections::{HashMap, BTreeMap};
use colored::*;

/// Debugger state
#[derive(Debug, Clone, PartialEq)]
pub enum DebuggerState {
    Running,
    Paused,
    SteppingInto,
    SteppingOver,
    SteppingOut,
    Stopped,
}

/// Breakpoint information
#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub id: u32,
    pub line: usize,
    pub condition: Option<String>,
    pub hit_count: u32,
    pub enabled: bool,
}

/// Call stack frame
#[derive(Debug, Clone)]
pub struct StackFrame {
    pub function_name: String,
    pub line_number: usize,
    pub variables: HashMap<String, String>,
}

/// Variable inspection info
#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub name: String,
    pub value: String,
    pub type_name: String,
    pub scope: String,
    pub is_mutable: bool,
}

/// Interactive debugger for CURSED REPL
pub struct InteractiveDebugger {
    /// Current debugger state
    state: DebuggerState,
    /// Active breakpoints
    breakpoints: BTreeMap<u32, Breakpoint>,
    /// Next breakpoint ID
    next_breakpoint_id: u32,
    /// Call stack
    call_stack: Vec<StackFrame>,
    /// Current line being executed
    current_line: usize,
    /// Variable watches
    watches: HashMap<String, String>,
    /// Execution history
    execution_history: Vec<String>,
    /// Step counter
    step_count: u64,
    /// Whether to show detailed trace
    verbose_trace: bool,
}

impl InteractiveDebugger {
    pub fn new() -> Self {
        Self {
            state: DebuggerState::Running,
            breakpoints: BTreeMap::new(),
            next_breakpoint_id: 1,
            call_stack: Vec::new(),
            current_line: 0,
            watches: HashMap::new(),
            execution_history: Vec::new(),
            step_count: 0,
            verbose_trace: false,
        }
    }
    
    /// Set a breakpoint at the specified line
    pub fn set_breakpoint(&mut self, line: usize, condition: Option<String>) -> u32 {
        let id = self.next_breakpoint_id;
        self.next_breakpoint_id += 1;
        
        let breakpoint = Breakpoint {
            id,
            line,
            condition,
            hit_count: 0,
            enabled: true,
        };
        
        self.breakpoints.insert(id, breakpoint);
        
        println!("{} Breakpoint {} set at line {}", 
            "✓".green(), id, line);
        
        if let Some(cond) = &condition {
            println!("  Condition: {}", cond.cyan());
        }
        
        id
    }
    
    /// Remove a breakpoint
    pub fn remove_breakpoint(&mut self, id: u32) -> bool {
        if let Some(bp) = self.breakpoints.remove(&id) {
            println!("{} Breakpoint {} removed from line {}", 
                "✓".green(), id, bp.line);
            true
        } else {
            println!("{} Breakpoint {} not found", "✗".red(), id);
            false
        }
    }
    
    /// Enable or disable a breakpoint
    pub fn toggle_breakpoint(&mut self, id: u32) -> bool {
        if let Some(bp) = self.breakpoints.get_mut(&id) {
            bp.enabled = !bp.enabled;
            let status = if bp.enabled { "enabled" } else { "disabled" };
            println!("{} Breakpoint {} {} at line {}", 
                "✓".green(), id, status, bp.line);
            true
        } else {
            println!("{} Breakpoint {} not found", "✗".red(), id);
            false
        }
    }
    
    /// List all breakpoints
    pub fn list_breakpoints(&self) {
        if self.breakpoints.is_empty() {
            println!("No breakpoints set");
            return;
        }
        
        println!("{}", "Breakpoints:".cyan().bold());
        for (id, bp) in &self.breakpoints {
            let status = if bp.enabled { "enabled".green() } else { "disabled".red() };
            let hits = if bp.hit_count > 0 { 
                format!(" (hit {} times)", bp.hit_count).dimmed() 
            } else { 
                "".normal() 
            };
            
            println!("  {} {}: Line {} {}{}", 
                id, status, bp.line, 
                bp.condition.as_ref().map(|c| format!("[{}]", c)).unwrap_or_default(),
                hits);
        }
    }
    
    /// Check if execution should pause at current line
    pub fn should_pause(&mut self, line: usize, variables: &HashMap<String, String>) -> bool {
        self.current_line = line;
        
        match self.state {
            DebuggerState::SteppingInto | DebuggerState::SteppingOver => {
                self.state = DebuggerState::Paused;
                return true;
            }
            DebuggerState::Running => {
                // Check breakpoints
                for bp in self.breakpoints.values_mut() {
                    if bp.enabled && bp.line == line {
                        // Check condition if present
                        if let Some(condition) = &bp.condition {
                            if !self.evaluate_condition(condition, variables) {
                                continue;
                            }
                        }
                        
                        bp.hit_count += 1;
                        self.state = DebuggerState::Paused;
                        
                        println!("{} Breakpoint {} hit at line {}", 
                            "⏸️".yellow(), bp.id, line);
                        
                        return true;
                    }
                }
            }
            _ => {}
        }
        
        false
    }
    
    /// Evaluate a breakpoint condition
    fn evaluate_condition(&self, condition: &str, variables: &HashMap<String, String>) -> bool {
        // Simple condition evaluation
        // For production, this would use the full CURSED expression evaluator
        
        if condition.contains("==") {
            let parts: Vec<&str> = condition.split("==").collect();
            if parts.len() == 2 {
                let left = parts[0].trim();
                let right = parts[1].trim().trim_matches('"');
                
                if let Some(value) = variables.get(left) {
                    return value == right;
                }
            }
        }
        
        if condition.contains(">") {
            let parts: Vec<&str> = condition.split(">").collect();
            if parts.len() == 2 {
                let left = parts[0].trim();
                let right = parts[1].trim();
                
                if let Some(value) = variables.get(left) {
                    if let (Ok(left_num), Ok(right_num)) = (value.parse::<i64>(), right.parse::<i64>()) {
                        return left_num > right_num;
                    }
                }
            }
        }
        
        // Default to true if we can't evaluate
        true
    }
    
    /// Step into the next statement
    pub fn step_into(&mut self) {
        self.state = DebuggerState::SteppingInto;
        self.step_count += 1;
        
        if self.verbose_trace {
            println!("{} Step into ({})", "→".cyan(), self.step_count);
        }
    }
    
    /// Step over the next statement
    pub fn step_over(&mut self) {
        self.state = DebuggerState::SteppingOver;
        self.step_count += 1;
        
        if self.verbose_trace {
            println!("{} Step over ({})", "↷".cyan(), self.step_count);
        }
    }
    
    /// Step out of current function
    pub fn step_out(&mut self) {
        self.state = DebuggerState::SteppingOut;
        self.step_count += 1;
        
        if self.verbose_trace {
            println!("{} Step out ({})", "↗".cyan(), self.step_count);
        }
    }
    
    /// Continue execution
    pub fn continue_execution(&mut self) {
        self.state = DebuggerState::Running;
        
        if self.verbose_trace {
            println!("{} Continue execution", "▶️".green());
        }
    }
    
    /// Stop debugging
    pub fn stop(&mut self) {
        self.state = DebuggerState::Stopped;
        self.call_stack.clear();
        
        println!("{} Debugging stopped", "⏹️".red());
    }
    
    /// Add a variable to watch list
    pub fn add_watch(&mut self, variable: String) {
        self.watches.insert(variable.clone(), String::new());
        println!("{} Added watch: {}", "👀".yellow(), variable.cyan());
    }
    
    /// Remove a variable from watch list
    pub fn remove_watch(&mut self, variable: &str) -> bool {
        if self.watches.remove(variable).is_some() {
            println!("{} Removed watch: {}", "✓".green(), variable.cyan());
            true
        } else {
            println!("{} Watch not found: {}", "✗".red(), variable);
            false
        }
    }
    
    /// List all watched variables
    pub fn list_watches(&self, variables: &HashMap<String, String>) {
        if self.watches.is_empty() {
            println!("No variables being watched");
            return;
        }
        
        println!("{}", "Watched Variables:".cyan().bold());
        for watch_var in self.watches.keys() {
            let value = variables.get(watch_var)
                .map(|v| v.as_str())
                .unwrap_or("<undefined>");
            
            println!("  {} = {}", watch_var.green(), value.yellow());
        }
    }
    
    /// Inspect all variables in current scope
    pub fn inspect_variables(&self, variables: &HashMap<String, String>) {
        if variables.is_empty() {
            println!("No variables in current scope");
            return;
        }
        
        println!("{}", "Variables in Current Scope:".cyan().bold());
        
        let mut sorted_vars: Vec<_> = variables.iter().collect();
        sorted_vars.sort_by_key(|(name, _)| *name);
        
        for (name, value) in sorted_vars {
            let var_info = self.get_variable_info(name, value);
            println!("  {} {} = {} {}", 
                var_info.type_name.blue(),
                name.green(), 
                value.yellow(),
                if var_info.is_mutable { "(mutable)".dimmed() } else { "".normal() });
        }
    }
    
    /// Get detailed information about a variable
    fn get_variable_info(&self, name: &str, value: &str) -> VariableInfo {
        // Infer type from value (simplified)
        let type_name = if value.parse::<i64>().is_ok() {
            "drip".to_string() // Integer
        } else if value.parse::<f64>().is_ok() {
            "flex".to_string() // Float
        } else if value == "based" || value == "cringe" {
            "lit".to_string() // Boolean
        } else if value.starts_with('"') && value.ends_with('"') {
            "tea".to_string() // String
        } else if value.starts_with('[') && value.ends_with(']') {
            "array".to_string() // Array
        } else {
            "vibe".to_string() // Unknown/Any
        };
        
        VariableInfo {
            name: name.to_string(),
            value: value.to_string(),
            type_name,
            scope: "local".to_string(), // Simplified
            is_mutable: true, // Simplified - would need actual scope analysis
        }
    }
    
    /// Show call stack
    pub fn show_call_stack(&self) {
        if self.call_stack.is_empty() {
            println!("No call stack (not in function)");
            return;
        }
        
        println!("{}", "Call Stack:".cyan().bold());
        for (depth, frame) in self.call_stack.iter().enumerate() {
            let prefix = if depth == 0 { "→" } else { " " };
            println!("  {}{}: {} (line {})", 
                prefix.cyan(), 
                depth, 
                frame.function_name.green(), 
                frame.line_number);
        }
    }
    
    /// Enter a function (update call stack)
    pub fn enter_function(&mut self, function_name: String, line: usize, variables: HashMap<String, String>) {
        let frame = StackFrame {
            function_name: function_name.clone(),
            line_number: line,
            variables,
        };
        
        self.call_stack.push(frame);
        
        if self.verbose_trace {
            println!("{} Entering function: {} at line {}", 
                "→".green(), function_name.cyan(), line);
        }
    }
    
    /// Exit a function (update call stack)
    pub fn exit_function(&mut self) -> Option<StackFrame> {
        let frame = self.call_stack.pop();
        
        if let Some(ref f) = frame {
            if self.verbose_trace {
                println!("{} Exiting function: {}", 
                    "←".red(), f.function_name.cyan());
            }
        }
        
        frame
    }
    
    /// Get current state
    pub fn get_state(&self) -> &DebuggerState {
        &self.state
    }
    
    /// Check if currently paused
    pub fn is_paused(&self) -> bool {
        matches!(self.state, DebuggerState::Paused)
    }
    
    /// Enable/disable verbose tracing
    pub fn set_verbose_trace(&mut self, enabled: bool) {
        self.verbose_trace = enabled;
        let status = if enabled { "enabled" } else { "disabled" };
        println!("{} Verbose trace {}", "✓".green(), status);
    }
    
    /// Handle debugger commands
    pub fn handle_command(&mut self, command: &str, variables: &HashMap<String, String>) -> Result<bool, CursedError> {
        let parts: Vec<&str> = command.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Ok(false);
        }
        
        match parts[0] {
            "continue" | "c" => {
                self.continue_execution();
                Ok(true)
            }
            "step" | "s" => {
                self.step_into();
                Ok(true)
            }
            "next" | "n" => {
                self.step_over();
                Ok(true)
            }
            "finish" | "f" => {
                self.step_out();
                Ok(true)
            }
            "break" | "b" => {
                if parts.len() >= 2 {
                    if let Ok(line) = parts[1].parse::<usize>() {
                        let condition = if parts.len() > 2 {
                            Some(parts[2..].join(" "))
                        } else {
                            None
                        };
                        self.set_breakpoint(line, condition);
                    } else {
                        println!("{} Invalid line number: {}", "✗".red(), parts[1]);
                    }
                } else {
                    println!("{} Usage: break <line> [condition]", "?".yellow());
                }
                Ok(false)
            }
            "delete" | "d" => {
                if parts.len() >= 2 {
                    if let Ok(id) = parts[1].parse::<u32>() {
                        self.remove_breakpoint(id);
                    } else {
                        println!("{} Invalid breakpoint ID: {}", "✗".red(), parts[1]);
                    }
                } else {
                    println!("{} Usage: delete <breakpoint_id>", "?".yellow());
                }
                Ok(false)
            }
            "list" | "l" => {
                self.list_breakpoints();
                Ok(false)
            }
            "watch" | "w" => {
                if parts.len() >= 2 {
                    self.add_watch(parts[1].to_string());
                } else {
                    self.list_watches(variables);
                }
                Ok(false)
            }
            "unwatch" | "uw" => {
                if parts.len() >= 2 {
                    self.remove_watch(parts[1]);
                } else {
                    println!("{} Usage: unwatch <variable>", "?".yellow());
                }
                Ok(false)
            }
            "vars" | "v" => {
                self.inspect_variables(variables);
                Ok(false)
            }
            "stack" | "st" => {
                self.show_call_stack();
                Ok(false)
            }
            "trace" | "t" => {
                if parts.len() >= 2 {
                    match parts[1] {
                        "on" => self.set_verbose_trace(true),
                        "off" => self.set_verbose_trace(false),
                        _ => println!("{} Usage: trace [on|off]", "?".yellow()),
                    }
                } else {
                    self.set_verbose_trace(!self.verbose_trace);
                }
                Ok(false)
            }
            "quit" | "q" => {
                self.stop();
                Ok(true)
            }
            "help" | "h" => {
                self.show_debug_help();
                Ok(false)
            }
            _ => {
                println!("{} Unknown debugger command: {}. Type 'help' for available commands.", 
                    "?".yellow(), parts[0]);
                Ok(false)
            }
        }
    }
    
    /// Show debugger help
    fn show_debug_help(&self) {
        println!("{}", "Interactive Debugger Commands:".cyan().bold());
        println!("  {} continue, c        - Continue execution", "▶️".green());
        println!("  {} step, s           - Step into next statement", "→".cyan());
        println!("  {} next, n           - Step over next statement", "↷".cyan());
        println!("  {} finish, f         - Step out of current function", "↗".cyan());
        println!("  {} break, b <line>   - Set breakpoint at line", "🔴".red());
        println!("  {} delete, d <id>    - Remove breakpoint", "❌".red());
        println!("  {} list, l           - List all breakpoints", "📋".blue());
        println!("  {} watch, w <var>    - Watch a variable", "👀".yellow());
        println!("  {} unwatch, uw <var> - Stop watching variable", "👁️".dimmed());
        println!("  {} vars, v           - Inspect all variables", "🔍".blue());
        println!("  {} stack, st         - Show call stack", "📚".blue());
        println!("  {} trace, t [on|off] - Toggle verbose trace", "🔊".yellow());
        println!("  {} quit, q           - Stop debugging", "⏹️".red());
        println!("  {} help, h           - Show this help", "❓".blue());
    }
    
    /// Get execution summary
    pub fn get_execution_summary(&self) -> String {
        format!("Steps: {}, Breakpoints: {}, Stack depth: {}", 
            self.step_count, 
            self.breakpoints.len(),
            self.call_stack.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoint_management() {
        let mut debugger = InteractiveDebugger::new();
        
        // Set breakpoint
        let id = debugger.set_breakpoint(10, None);
        assert_eq!(id, 1);
        assert_eq!(debugger.breakpoints.len(), 1);
        
        // Remove breakpoint
        assert!(debugger.remove_breakpoint(id));
        assert_eq!(debugger.breakpoints.len(), 0);
    }

    #[test]
    fn test_stepping() {
        let mut debugger = InteractiveDebugger::new();
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), "42".to_string());
        
        // Should not pause initially
        assert!(!debugger.should_pause(1, &vars));
        
        // Step into
        debugger.step_into();
        assert!(debugger.should_pause(2, &vars));
        assert!(debugger.is_paused());
    }

    #[test]
    fn test_watch_variables() {
        let mut debugger = InteractiveDebugger::new();
        
        debugger.add_watch("test_var".to_string());
        assert!(debugger.watches.contains_key("test_var"));
        
        assert!(debugger.remove_watch("test_var"));
        assert!(!debugger.watches.contains_key("test_var"));
    }

    #[test]
    fn test_call_stack() {
        let mut debugger = InteractiveDebugger::new();
        let vars = HashMap::new();
        
        debugger.enter_function("main".to_string(), 1, vars.clone());
        debugger.enter_function("helper".to_string(), 5, vars);
        
        assert_eq!(debugger.call_stack.len(), 2);
        
        let frame = debugger.exit_function();
        assert!(frame.is_some());
        assert_eq!(frame.unwrap().function_name, "helper");
        assert_eq!(debugger.call_stack.len(), 1);
    }
}
