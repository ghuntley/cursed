//! Enhanced CURSED REPL with advanced features
//! Integrates multi-line input, tab completion, syntax highlighting, and debugging

use crate::error::CursedError;
use crate::execution::CursedExecutionEngine;
use super::session_manager::SessionManager;
use super::advanced_tab_completion::CursedCompleter;
use super::advanced_multi_line_editor::AdvancedMultiLineEditor;
use super::advanced_syntax_highlighter::{CursedSyntaxHighlighter, create_dark_theme};
use super::interactive_debugger::InteractiveDebugger;

use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use rustyline::error::ReadlineError;
use rustyline::{Editor, DefaultEditor, Helper, Context, Result as RlResult};
use rustyline::completion::Completer;
use rustyline::hint::Hinter;
use rustyline::highlight::Highlighter;
use rustyline::validate::Validator;
use colored::*;

/// REPL configuration
#[derive(Debug, Clone)]
pub struct ReplConfig {
    pub enable_syntax_highlighting: bool,
    pub enable_tab_completion: bool,
    pub enable_multi_line: bool,
    pub enable_debugging: bool,
    pub enable_history: bool,
    pub max_history_size: usize,
    pub auto_indent: bool,
    pub show_hints: bool,
    pub color_theme: String, // "dark" or "light"
}

impl Default for ReplConfig {
    fn default() -> Self {
        Self {
            enable_syntax_highlighting: true,
            enable_tab_completion: true,
            enable_multi_line: true,
            enable_debugging: false, // Disabled by default for performance
            enable_history: true,
            max_history_size: 1000,
            auto_indent: true,
            show_hints: true,
            color_theme: "dark".to_string(),
        }
    }
}

/// Custom helper that implements all Rustyline traits
#[derive(Clone)]
struct CursedReplHelper {
    completer: CursedCompleter,
    highlighter: CursedSyntaxHighlighter,
    config: ReplConfig,
}

impl CursedReplHelper {
    fn new(config: ReplConfig) -> Self {
        let mut highlighter = CursedSyntaxHighlighter::new();
        
        // Set color theme
        if config.color_theme == "dark" {
            highlighter.set_color_scheme(create_dark_theme());
        }
        
        Self {
            completer: CursedCompleter::new(),
            highlighter,
            config,
        }
    }
    
    fn update_variables(&mut self, variables: &HashMap<String, String>) {
        self.completer.update_variables(variables);
    }
    
    fn add_function(&mut self, name: String, parameters: Vec<String>) {
        self.completer.add_function(name, parameters);
    }
}

impl Helper for CursedReplHelper {}

impl Completer for CursedReplHelper {
    type Candidate = <CursedCompleter as Completer>::Candidate;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> RlResult<(usize, Vec<Self::Candidate>)> {
        if self.config.enable_tab_completion {
            self.completer.complete(line, pos, ctx)
        } else {
            Ok((pos, vec![]))
        }
    }
}

impl Hinter for CursedReplHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> {
        if !self.config.show_hints || line.is_empty() || pos != line.len() {
            return None;
        }
        
        // Simple hints based on current input
        let trimmed = line.trim();
        
        if trimmed == "sus" {
            Some(" variable_name type = value".dimmed().to_string())
        } else if trimmed == "slay" {
            Some(" function_name(params) return_type { }".dimmed().to_string())
        } else if trimmed == "vibez.spill" {
            Some("(value)".dimmed().to_string())
        } else if trimmed == "yeet" {
            Some(" \"module_name\"".dimmed().to_string())
        } else if trimmed.starts_with("sus ") && !trimmed.contains('=') {
            Some(" = value".dimmed().to_string())
        } else {
            None
        }
    }
}

impl Highlighter for CursedReplHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> std::borrow::Cow<'l, str> {
        if self.config.enable_syntax_highlighting {
            match self.highlighter.highlight_line(line) {
                Ok(highlighted) => std::borrow::Cow::Owned(highlighted),
                Err(_) => std::borrow::Cow::Borrowed(line),
            }
        } else {
            std::borrow::Cow::Borrowed(line)
        }
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        _default: bool,
    ) -> std::borrow::Cow<'b, str> {
        std::borrow::Cow::Borrowed(prompt)
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> std::borrow::Cow<'h, str> {
        std::borrow::Cow::Borrowed(hint)
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _forced: bool) -> bool {
        self.config.enable_syntax_highlighting
    }
}

impl Validator for CursedReplHelper {}

/// Enhanced CURSED REPL with all advanced features
pub struct EnhancedCursedRepl {
    session_manager: SessionManager,
    context: HashMap<String, String>,
    execution_engine: CursedExecutionEngine,
    multi_line_editor: AdvancedMultiLineEditor,
    debugger: Option<InteractiveDebugger>,
    config: ReplConfig,
    helper: CursedReplHelper,
    command_counter: u64,
}

impl EnhancedCursedRepl {
    pub fn new() -> Result<Self, CursedError> {
        Self::with_config(ReplConfig::default())
    }
    
    pub fn with_config(config: ReplConfig) -> Result<Self, CursedError> {
        let helper = CursedReplHelper::new(config.clone());
        let debugger = if config.enable_debugging {
            Some(InteractiveDebugger::new())
        } else {
            None
        };
        
        Ok(Self {
            session_manager: SessionManager::new()?,
            context: HashMap::new(),
            execution_engine: CursedExecutionEngine::new()?,
            multi_line_editor: AdvancedMultiLineEditor::new(),
            debugger,
            config,
            helper,
            command_counter: 0,
        })
    }
    
    /// Run the enhanced REPL
    pub fn run_enhanced_repl(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut rl = Editor::<CursedReplHelper, _>::new()?;
        rl.set_helper(Some(self.helper.clone()));
        
        // Load history if enabled
        if self.config.enable_history {
            self.load_history(&mut rl);
        }
        
        self.print_enhanced_welcome();
        
        loop {
            // Get appropriate prompt
            let prompt = if self.config.enable_multi_line && self.multi_line_editor.is_multi_line() {
                self.multi_line_editor.get_prompt()
            } else {
                self.get_main_prompt()
            };
            
            let readline = rl.readline(&prompt);
            
            match readline {
                Ok(line) => {
                    self.command_counter += 1;
                    
                    if line.trim().is_empty() {
                        if self.multi_line_editor.is_multi_line() {
                            // Empty line in multi-line mode - might complete input
                            if let Some(complete_input) = self.multi_line_editor.process_line(line)? {
                                self.process_complete_input(&complete_input)?;
                            }
                        }
                        continue;
                    }
                    
                    // Handle special commands
                    if let Some(response) = self.handle_enhanced_command(&line)? {
                        if response == "exit" {
                            break;
                        }
                        if !response.is_empty() {
                            println!("{}", response);
                        }
                        continue;
                    }
                    
                    // Add to rustyline history
                    if self.config.enable_history {
                        rl.add_history_entry(line.as_str())?;
                    }
                    
                    // Process input (multi-line or single-line)
                    if self.config.enable_multi_line {
                        if let Some(complete_input) = self.multi_line_editor.process_line(line)? {
                            self.process_complete_input(&complete_input)?;
                        }
                    } else {
                        self.process_complete_input(&line)?;
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("{}", "^C".yellow());
                    if self.multi_line_editor.is_multi_line() {
                        self.multi_line_editor.reset();
                        println!("Multi-line input cancelled");
                    }
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    println!("{}", "Goodbye!".cyan());
                    break;
                }
                Err(err) => {
                    println!("{} {}", "Error:".red().bold(), err);
                    break;
                }
            }
        }
        
        // Save history
        if self.config.enable_history {
            self.save_history(&rl);
        }
        
        Ok(())
    }
    
    /// Get the main prompt with indicators
    fn get_main_prompt(&self) -> String {
        let mut prompt = "cursed".cyan().bold().to_string();
        
        // Add indicators
        let mut indicators = Vec::new();
        
        if self.config.enable_debugging && self.debugger.as_ref().map_or(false, |d| d.is_paused()) {
            indicators.push("🔍".to_string());
        }
        
        if self.multi_line_editor.is_multi_line() {
            indicators.push("📝".to_string());
        }
        
        if !indicators.is_empty() {
            prompt.push_str(&format!(" {}", indicators.join("")));
        }
        
        prompt.push_str("> ");
        prompt
    }
    
    /// Process a complete input (single or multi-line)
    fn process_complete_input(&mut self, input: &str) -> Result<(), CursedError> {
        // Update helper with current context
        self.helper.update_variables(&self.context);
        
        // Check for syntax errors if highlighting is enabled
        if self.config.enable_syntax_highlighting {
            let highlighter = &self.helper.highlighter;
            let errors = highlighter.validate_syntax(input);
            if !errors.is_empty() {
                for (start, end, message) in errors {
                    println!("{} Syntax error at position {}-{}: {}", 
                        "⚠️".yellow(), start, end, message);
                }
                // Show highlighted input with errors
                let highlighted = highlighter.highlight_errors(input, &[(0, input.len(), "syntax error".to_string())]);
                println!("  {}", highlighted);
                return Ok(());
            }
        }
        
        // Process with debugger if enabled
        if let Some(ref mut debugger) = self.debugger {
            if debugger.should_pause(self.command_counter as usize, &self.context) {
                println!("{} Execution paused at command {}", 
                    "⏸️".yellow(), self.command_counter);
                self.run_debug_session()?;
                return Ok(());
            }
        }
        
        // Store in session history
        self.session_manager.add_to_history(input.to_string());
        
        // Execute the input
        match self.execution_engine.execute_repl(input) {
            Ok(result) => {
                if !result.trim().is_empty() {
                    println!("{}", result);
                }
                
                // Extract any new variables/functions for completion
                self.extract_definitions(input);
            }
            Err(e) => {
                println!("{} {}", "Error:".red().bold(), e);
                
                // Show helpful error context if syntax highlighting is enabled
                if self.config.enable_syntax_highlighting {
                    let highlighted = self.helper.highlighter.highlight_line(input)
                        .unwrap_or_else(|_| input.to_string());
                    println!("  {}", highlighted);
                }
            }
        }
        
        Ok(())
    }
    
    /// Extract variable and function definitions for tab completion
    fn extract_definitions(&mut self, input: &str) {
        // Simple extraction - in production this would use proper AST analysis
        
        // Extract variable declarations: sus var_name = ...
        if input.trim().starts_with("sus ") {
            if let Some(equals_pos) = input.find('=') {
                let var_part = &input[4..equals_pos].trim(); // Skip "sus "
                if let Some(space_pos) = var_part.find(' ') {
                    let var_name = var_part[..space_pos].trim();
                    if !var_name.is_empty() {
                        let value = input[equals_pos + 1..].trim();
                        self.context.insert(var_name.to_string(), value.to_string());
                    }
                }
            }
        }
        
        // Extract function definitions: slay func_name(...) ...
        if input.trim().starts_with("slay ") {
            if let Some(paren_pos) = input.find('(') {
                let func_part = &input[5..paren_pos].trim(); // Skip "slay "
                let func_name = func_part.split_whitespace().next().unwrap_or("");
                
                if !func_name.is_empty() {
                    // Extract parameters (simplified)
                    let mut params = Vec::new();
                    if let Some(close_paren) = input.find(')') {
                        let param_str = &input[paren_pos + 1..close_paren];
                        for param in param_str.split(',') {
                            let param_name = param.trim().split_whitespace().next().unwrap_or("");
                            if !param_name.is_empty() {
                                params.push(param_name.to_string());
                            }
                        }
                    }
                    
                    self.helper.add_function(func_name.to_string(), params);
                }
            }
        }
    }
    
    /// Handle enhanced REPL commands
    fn handle_enhanced_command(&mut self, input: &str) -> Result<Option<String>, CursedError> {
        let input = input.trim();
        
        // Standard commands
        match input {
            ":quit" | ":exit" | ":q" => return Ok(Some("exit".to_string())),
            ":help" | ":h" => return Ok(Some(self.get_enhanced_help())),
            ":history" => return Ok(Some(self.get_history_text())),
            ":clear" => return Ok(Some(self.clear_screen())),
            ":vars" => return Ok(Some(self.show_variables())),
            ":version" => return Ok(Some(format!("Enhanced CURSED REPL v{}", 
                option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")))),
            _ => {}
        }
        
        // Configuration commands
        if input.starts_with(":config") {
            return self.handle_config_command(input);
        }
        
        // Multi-line commands
        if input.starts_with(":multiline") {
            return self.handle_multiline_command(input);
        }
        
        // Debugging commands
        if input.starts_with(":debug") {
            return self.handle_debug_command(input);
        }
        
        // Syntax highlighting commands
        if input.starts_with(":syntax") {
            return self.handle_syntax_command(input);
        }
        
        // Theme commands
        if input.starts_with(":theme") {
            return self.handle_theme_command(input);
        }
        
        // Unknown command starting with ':'
        if input.starts_with(':') {
            return Ok(Some(format!("Unknown command: {}. Type :help for available commands.", input)));
        }
        
        Ok(None)
    }
    
    /// Handle configuration commands
    fn handle_config_command(&mut self, input: &str) -> Result<Option<String>, CursedError> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.len() == 1 {
            // Show current configuration
            return Ok(Some(format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
                "Current Configuration:".cyan().bold(),
                format!("  Syntax highlighting: {}", if self.config.enable_syntax_highlighting { "on".green() } else { "off".red() }),
                format!("  Tab completion: {}", if self.config.enable_tab_completion { "on".green() } else { "off".red() }),
                format!("  Multi-line input: {}", if self.config.enable_multi_line { "on".green() } else { "off".red() }),
                format!("  Debugging: {}", if self.config.enable_debugging { "on".green() } else { "off".red() }),
                format!("  History: {}", if self.config.enable_history { "on".green() } else { "off".red() }),
                format!("  Auto-indent: {}", if self.config.auto_indent { "on".green() } else { "off".red() }),
                format!("  Hints: {}", if self.config.show_hints { "on".green() } else { "off".red() }),
                format!("  Theme: {}", self.config.color_theme.cyan()),
            )));
        }
        
        if parts.len() >= 3 {
            let setting = parts[1];
            let value = parts[2];
            
            let enabled = match value {
                "on" | "true" | "1" => true,
                "off" | "false" | "0" => false,
                _ => return Ok(Some(format!("Invalid value: {}. Use 'on' or 'off'.", value))),
            };
            
            match setting {
                "syntax" => {
                    self.config.enable_syntax_highlighting = enabled;
                    return Ok(Some(format!("Syntax highlighting {}", if enabled { "enabled" } else { "disabled" })));
                }
                "completion" => {
                    self.config.enable_tab_completion = enabled;
                    return Ok(Some(format!("Tab completion {}", if enabled { "enabled" } else { "disabled" })));
                }
                "multiline" => {
                    self.config.enable_multi_line = enabled;
                    return Ok(Some(format!("Multi-line input {}", if enabled { "enabled" } else { "disabled" })));
                }
                "debug" => {
                    self.config.enable_debugging = enabled;
                    if enabled && self.debugger.is_none() {
                        self.debugger = Some(InteractiveDebugger::new());
                    }
                    return Ok(Some(format!("Debugging {}", if enabled { "enabled" } else { "disabled" })));
                }
                "history" => {
                    self.config.enable_history = enabled;
                    return Ok(Some(format!("History {}", if enabled { "enabled" } else { "disabled" })));
                }
                "hints" => {
                    self.config.show_hints = enabled;
                    return Ok(Some(format!("Hints {}", if enabled { "enabled" } else { "disabled" })));
                }
                _ => return Ok(Some(format!("Unknown setting: {}. Type :help for available settings.", setting))),
            }
        }
        
        Ok(Some("Usage: :config [setting] [on|off]".to_string()))
    }
    
    /// Handle multiline-specific commands
    fn handle_multiline_command(&mut self, input: &str) -> Result<Option<String>, CursedError> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        match parts.get(1) {
            Some("reset") => {
                self.multi_line_editor.reset();
                Ok(Some("Multi-line input reset".to_string()))
            }
            Some("status") => {
                let status = if self.multi_line_editor.is_multi_line() {
                    format!("Multi-line mode: {} ({})", 
                        "active".green(), 
                        self.multi_line_editor.get_state_description())
                } else {
                    format!("Multi-line mode: {}", "inactive".red())
                };
                Ok(Some(status))
            }
            _ => Ok(Some("Usage: :multiline [reset|status]".to_string()))
        }
    }
    
    /// Handle debugging commands
    fn handle_debug_command(&mut self, input: &str) -> Result<Option<String>, CursedError> {
        if self.debugger.is_none() {
            return Ok(Some("Debugging is not enabled. Use ':config debug on' to enable.".to_string()));
        }
        
        let debug_cmd = &input[6..].trim(); // Remove ":debug"
        
        if debug_cmd.is_empty() {
            self.run_debug_session()?;
            return Ok(Some("".to_string()));
        }
        
        // Pass command to debugger
        let debugger = self.debugger.as_mut().unwrap();
        let should_continue = debugger.handle_command(debug_cmd, &self.context)?;
        
        if should_continue {
            Ok(Some("Debugger command executed".to_string()))
        } else {
            Ok(Some("".to_string()))
        }
    }
    
    /// Handle syntax highlighting commands
    fn handle_syntax_command(&mut self, input: &str) -> Result<Option<String>, CursedError> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        match parts.get(1) {
            Some("check") => {
                if parts.len() >= 3 {
                    let code = parts[2..].join(" ");
                    let errors = self.helper.highlighter.validate_syntax(&code);
                    if errors.is_empty() {
                        Ok(Some("Syntax is valid ✓".green().to_string()))
                    } else {
                        let mut result = "Syntax errors found:".red().to_string();
                        for (start, end, message) in errors {
                            result.push_str(&format!("\n  {}-{}: {}", start, end, message));
                        }
                        Ok(Some(result))
                    }
                } else {
                    Ok(Some("Usage: :syntax check <code>".to_string()))
                }
            }
            Some("preview") => {
                if parts.len() >= 3 {
                    let code = parts[2..].join(" ");
                    let highlighted = self.helper.highlighter.preview_highlighting(&code);
                    Ok(Some(format!("Preview:\n{}", highlighted)))
                } else {
                    Ok(Some("Usage: :syntax preview <code>".to_string()))
                }
            }
            _ => Ok(Some("Usage: :syntax [check|preview] <code>".to_string()))
        }
    }
    
    /// Handle theme commands
    fn handle_theme_command(&mut self, input: &str) -> Result<Option<String>, CursedError> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.len() >= 2 {
            let theme = parts[1];
            match theme {
                "dark" => {
                    self.config.color_theme = "dark".to_string();
                    self.helper.highlighter.set_color_scheme(create_dark_theme());
                    Ok(Some("Switched to dark theme".to_string()))
                }
                "light" => {
                    self.config.color_theme = "light".to_string();
                    self.helper.highlighter.set_color_scheme(super::advanced_syntax_highlighter::create_light_theme());
                    Ok(Some("Switched to light theme".to_string()))
                }
                _ => Ok(Some("Available themes: dark, light".to_string()))
            }
        } else {
            Ok(Some(format!("Current theme: {}", self.config.color_theme.cyan())))
        }
    }
    
    /// Run interactive debug session
    fn run_debug_session(&mut self) -> Result<(), CursedError> {
        if let Some(ref mut debugger) = self.debugger {
            println!("{}", "=== Interactive Debug Session ===".cyan().bold());
            println!("Type 'help' for debugger commands, 'quit' to exit debug mode");
            
            let stdin = io::stdin();
            let mut stdout = io::stdout();
            
            loop {
                print!("{} ", "(debug)".yellow().bold());
                stdout.flush().unwrap();
                
                let mut debug_input = String::new();
                if stdin.read_line(&mut debug_input).is_err() {
                    break;
                }
                
                let should_continue = debugger.handle_command(debug_input.trim(), &self.context)?;
                if should_continue {
                    break;
                }
            }
            
            println!("{}", "=== End Debug Session ===".cyan().bold());
        }
        
        Ok(())
    }
    
    /// Print enhanced welcome message
    fn print_enhanced_welcome(&self) {
        println!("{}", "🔥 Enhanced CURSED REPL".cyan().bold());
        println!("{}", "Advanced interactive CURSED language shell".dimmed());
        println!();
        
        // Show enabled features
        let mut features = Vec::new();
        if self.config.enable_syntax_highlighting { features.push("syntax highlighting".green()); }
        if self.config.enable_tab_completion { features.push("tab completion".blue()); }
        if self.config.enable_multi_line { features.push("multi-line input".magenta()); }
        if self.config.enable_debugging { features.push("debugging".red()); }
        if self.config.enable_history { features.push("history".yellow()); }
        
        if !features.is_empty() {
            println!("Features: {}", features.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(", "));
        }
        
        println!("Type :help for help, :quit to exit");
        println!();
    }
    
    /// Get enhanced help text
    fn get_enhanced_help(&self) -> String {
        format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            "Enhanced CURSED REPL Commands:".cyan().bold(),
            "",
            "Basic Commands:".green().bold(),
            "  :help, :h         - Show this help message",
            "  :quit, :exit, :q  - Exit the REPL",
            "  :history          - Show command history",
            "  :clear            - Clear the screen",
            "  :vars             - Show current variables",
            "  :version          - Show version information",
            "",
            "Configuration:".blue().bold(),
            "  :config                    - Show current configuration",
            "  :config <setting> <on|off> - Toggle settings",
            "  :theme <dark|light>        - Change color theme",
            "",
            "Multi-line Input:".magenta().bold(),
            "  :multiline status          - Show multi-line status",
            "  :multiline reset           - Reset multi-line input",
            "",
            "Debugging:".red().bold(),
            "  :debug                     - Enter debug mode",
            "  :debug help                - Show debug commands",
            "",
            "Syntax Tools:".yellow().bold(),
            "  :syntax check <code>       - Check syntax",
            "  :syntax preview <code>     - Preview highlighting",
            "",
            "CURSED Language:".cyan().bold(),
            "  Variables:  sus x drip = 42",
            "  Functions:  slay add(a drip, b drip) drip { damn a + b }",
            "  Arrays:     sus arr []drip = [1, 2, 3]",
            "  Print:      vibez.spill(\"Hello, world!\")",
            "  Import:     yeet \"module_name\"",
        )
    }
    
    /// Get history text
    fn get_history_text(&self) -> String {
        let history = self.session_manager.get_history();
        if history.is_empty() {
            "No history available".dimmed().to_string()
        } else {
            let mut result = "Command History:".cyan().bold().to_string();
            let recent_history = if history.len() > 20 {
                &history[history.len() - 20..]
            } else {
                history
            };
            
            for (i, entry) in recent_history.iter().enumerate() {
                let line_num = history.len() - recent_history.len() + i + 1;
                result.push_str(&format!("\n{:3}: {}", line_num, entry));
            }
            
            if history.len() > 20 {
                result.push_str(&format!("\n... and {} more entries", history.len() - 20));
            }
            
            result
        }
    }
    
    /// Clear screen
    fn clear_screen(&self) -> String {
        print!("\x1B[2J\x1B[1;1H");
        "".to_string()
    }
    
    /// Show variables
    fn show_variables(&self) -> String {
        if self.context.is_empty() {
            "No variables defined".dimmed().to_string()
        } else {
            let mut result = "Variables:".cyan().bold().to_string();
            let mut sorted_vars: Vec<_> = self.context.iter().collect();
            sorted_vars.sort_by_key(|(name, _)| *name);
            
            for (key, value) in sorted_vars {
                result.push_str(&format!("\n  {} = {}", key.green(), value.yellow()));
            }
            result
        }
    }
    
    /// Load history from file
    fn load_history(&self, rl: &mut Editor<CursedReplHelper, rustyline::DefaultHistory>) {
        let history_path = dirs::home_dir()
            .map(|mut path| {
                path.push(".cursed_history");
                path
            })
            .unwrap_or_else(|| std::path::PathBuf::from(".cursed_history"));
        
        if let Err(e) = rl.load_history(&history_path) {
            match e {
                ReadlineError::Io(ref io_err) if io_err.kind() == std::io::ErrorKind::NotFound => {
                    // File not found is expected on first run
                }
                _ => {
                    println!("{} Failed to load history: {}", "Warning".yellow(), e);
                }
            }
        }
    }
    
    /// Save history to file
    fn save_history(&self, rl: &Editor<CursedReplHelper, rustyline::DefaultHistory>) {
        let history_path = dirs::home_dir()
            .map(|mut path| {
                path.push(".cursed_history");
                path
            })
            .unwrap_or_else(|| std::path::PathBuf::from(".cursed_history"));
        
        if let Err(e) = rl.save_history(&history_path) {
            println!("{} Failed to save history: {}", "Warning".yellow(), e);
        }
    }
    
    /// Load startup file
    pub fn load_startup_file(&mut self, path: &str) -> Result<(), CursedError> {
        let content = fs::read_to_string(path)
            .map_err(|e| CursedError::Io(format!("Failed to read startup file: {}", e)))?;
        
        println!("{} Loading startup file: {}", "📄".blue(), path);
        
        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("fr fr") {
                continue; // Skip empty lines and comments
            }
            
            println!("  {}:{} {}", line_num + 1, "→".cyan(), line.dimmed());
            
            match self.execution_engine.execute(line) {
                Ok(_) => {
                    self.extract_definitions(line);
                }
                Err(e) => {
                    println!("{} Error in startup file line {}: {}", 
                        "✗".red(), line_num + 1, e);
                }
            }
        }
        
        println!("{} Startup file loaded", "✓".green());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_repl_creation() {
        let repl = EnhancedCursedRepl::new();
        assert!(repl.is_ok());
    }

    #[test]
    fn test_config_handling() {
        let mut repl = EnhancedCursedRepl::new().unwrap();
        
        // Test config display
        let result = repl.handle_enhanced_command(":config").unwrap();
        assert!(result.is_some());
        assert!(result.unwrap().contains("Configuration"));
        
        // Test config setting
        let result = repl.handle_enhanced_command(":config syntax off").unwrap();
        assert!(result.is_some());
        assert!(!repl.config.enable_syntax_highlighting);
    }

    #[test]
    fn test_variable_extraction() {
        let mut repl = EnhancedCursedRepl::new().unwrap();
        
        repl.extract_definitions("sus test_var drip = 42");
        assert!(repl.context.contains_key("test_var"));
        assert_eq!(repl.context.get("test_var"), Some(&"42".to_string()));
    }
}
