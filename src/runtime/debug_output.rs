/// Enhanced debug output system with Gen Z slang-themed messages
///
/// Provides pretty-printed stack traces, source code context display,
/// variable value inspection, and themed debug messages for CURSED.

use crate::error::{CursedError, SourceLocation};
use crate::runtime::stack_trace::{StackTrace, CallFrame};
// use crate::runtime::debug_info::{EnhancedStackTrace, EnhancedStackFrame, VariableInfo};
use crate::runtime::stack_walker::{ContextualStackWalk, RawStackFrame, SourceFrameInfo};

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fmt;
use std::io::{self, Write};
use colored::{Colorize, ColoredString};
use tracing::{debug, error, info, warn};

/// Configuration for debug output formatting
#[derive(Debug, Clone)]
pub struct DebugOutputConfig {
    /// Use colored output
    pub use_colors: bool,
    /// Show source code context
    pub show_source_context: bool,
    /// Number of context lines around error
    pub context_lines: u32,
    /// Maximum variable value length to display
    pub max_variable_length: usize,
    /// Use Gen Z slang in messages
    pub use_gen_z_slang: bool,
    /// Show instruction pointers
    pub show_instruction_pointers: bool,
    /// Show frame numbers
    pub show_frame_numbers: bool,
    /// Maximum frames to display
    pub max_display_frames: usize,
    /// Compact output mode
    pub compact_mode: bool,
}

impl Default for DebugOutputConfig {
    fn default() -> Self {
        DebugOutputConfig {
            use_colors: true,
            show_source_context: true,
            context_lines: 3,
            max_variable_length: 100,
            use_gen_z_slang: true,
            show_instruction_pointers: false,
            show_frame_numbers: true,
            max_display_frames: 20,
            compact_mode: false,
        }
    }
}

/// Enhanced debug output formatter
pub struct DebugFormatter {
    /// Configuration for output
    config: DebugOutputConfig,
    /// Source file cache for context display
    source_cache: HashMap<PathBuf, Vec<String>>,
}

impl DebugFormatter {
    /// Create a new debug formatter
    pub fn new() -> Self {
        DebugFormatter {
            config: DebugOutputConfig::default(),
            source_cache: HashMap::new(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: DebugOutputConfig) -> Self {
        DebugFormatter {
            config,
            source_cache: HashMap::new(),
        }
    }

    /// Format a stack trace with enhanced output
    pub fn format_stack_trace(&mut self, trace: &StackTrace) -> crate::error::Result<()> {
        let mut output = String::new();
        
        // Header with Gen Z flair
        if self.config.use_gen_z_slang {
            output.push_str(&self.gen_z_header("💥 Stack Trace (this ain't it chief) 💥"));
        } else {
            output.push_str("Stack Trace:\n");
        }
        
        output.push_str(&format!("Trace ID: {}\n", trace.trace_id));
        output.push_str(&format!("Thread: {:?}\n", trace.thread_id));
        
        if let Some(goroutine_id) = trace.goroutine_id {
            output.push_str(&format!("Goroutine: #{}\n", goroutine_id));
        }
        
        output.push('\n');
        
        // Format each frame
        let max_frames = std::cmp::min(trace.frames.len(), self.config.max_display_frames);
        for (index, frame) in trace.frames.iter().take(max_frames).enumerate() {
            output.push_str(&self.format_call_frame(frame, index)?);
            output.push('\n');
        }
        
        if trace.frames.len() > self.config.max_display_frames {
            let remaining = trace.frames.len() - self.config.max_display_frames;
            if self.config.use_gen_z_slang {
                output.push_str(&format!("... and {} more frames (we're not gonna show all that bestie) 💅\n", remaining));
            } else {
                output.push_str(&format!("... {} more frames\n", remaining));
            }
        }
        
        if trace.truncated {
            if self.config.use_gen_z_slang {
                output.push_str("⚠️ Stack was truncated (too deep, no cap) ⚠️\n");
            } else {
                output.push_str("Warning: Stack trace was truncated\n");
            }
        }
        
        Ok(output)
    }

    /// Format an enhanced stack trace
    pub fn format_enhanced_stack_trace(&mut self, trace: &EnhancedStackTrace) -> crate::error::Result<()> {
        let mut output = String::new();
        
        // Header
        if self.config.use_gen_z_slang {
            output.push_str(&self.gen_z_header("🔍 Enhanced Stack Trace (we're about to spill ALL the tea) 🔍"));
        } else {
            output.push_str("Enhanced Stack Trace:\n");
        }
        
        output.push_str(&format!("Thread: {:?}\n", trace.thread_id));
        if let Some(goroutine_id) = trace.goroutine_id {
            output.push_str(&format!("Goroutine: #{}\n", goroutine_id));
        }
        output.push('\n');
        
        // Format each enhanced frame
        let max_frames = std::cmp::min(trace.frames.len(), self.config.max_display_frames);
        for frame in trace.frames.iter().take(max_frames) {
            output.push_str(&self.format_enhanced_frame(frame)?);
            output.push('\n');
        }
        
        Ok(output)
    }

    /// Format a contextual stack walk
    pub fn format_contextual_stack_walk(&mut self, walk: &ContextualStackWalk) -> crate::error::Result<()> {
        let mut output = String::new();
        
        // Header
        if self.config.use_gen_z_slang {
            output.push_str(&self.gen_z_header("🚶 Stack Walk (let's see where we've been) 🚶"));
        } else {
            output.push_str("Stack Walk:\n");
        }
        
        output.push_str(&format!("Thread: {:?}\n", walk.thread_id));
        if let Some(goroutine_id) = walk.goroutine_id {
            output.push_str(&format!("Goroutine: #{}\n", goroutine_id));
        }
        output.push('\n');
        
        // Format each raw frame
        let max_frames = std::cmp::min(walk.frames.len(), self.config.max_display_frames);
        for (index, frame) in walk.frames.iter().take(max_frames).enumerate() {
            output.push_str(&self.format_raw_frame(frame, index)?);
            output.push('\n');
        }
        
        // Summary
        let cursed_frames = walk.cursed_frames().len();
        if cursed_frames > 0 {
            if self.config.use_gen_z_slang {
                output.push_str(&format!("✨ Found {} CURSED frames (that's the good stuff) ✨\n", cursed_frames));
            } else {
                output.push_str(&format!("CURSED frames: {}\n", cursed_frames));
            }
        }
        
        Ok(output)
    }

    /// Format a single call frame
    fn format_call_frame(&mut self, frame: &CallFrame, index: usize) -> crate::error::Result<()> {
        let mut output = String::new();
        
        // Frame header
        if self.config.show_frame_numbers {
            output.push_str(&self.colorize_frame_number(&format!("#{}: ", index)));
        }
        
        // Function name with styling
        let function_display = self.colorize_function_name(&frame.function_name);
        output.push_str(&function_display);
        
        // Module information
        if let Some(module) = &frame.module_name {
            output.push_str(&format!(" ({})", self.colorize_module_name(module)));
        }
        
        // Source location
        if let Some(location) = &frame.source_location {
            output.push_str(&format!(" at {}", self.format_source_location(location)));
            
            // Show source context if configured
            if self.config.show_source_context {
                if let Ok(context) = self.get_source_context(location) {
                    output.push_str(&format!("\n{}", context));
                }
            }
        }
        
        // Parameters
        if !frame.parameters.is_empty() && !self.config.compact_mode {
            output.push_str("\n  Parameters:");
            for (name, value) in &frame.parameters {
                let truncated_value = self.truncate_value(value);
                output.push_str(&format!("\n    {}: {}", 
                    self.colorize_variable_name(name),
                    self.colorize_variable_value(&truncated_value)
                ));
            }
        }
        
        // Local variables
        if !frame.local_variables.is_empty() && !self.config.compact_mode {
            output.push_str("\n  Local variables:");
            for (name, value) in &frame.local_variables {
                let truncated_value = self.truncate_value(value);
                output.push_str(&format!("\n    {}: {}", 
                    self.colorize_variable_name(name),
                    self.colorize_variable_value(&truncated_value)
                ));
            }
        }
        
        Ok(output)
    }

    /// Format an enhanced stack frame
    fn format_enhanced_frame(&mut self, frame: &EnhancedStackFrame) -> crate::error::Result<()> {
        let mut output = String::new();
        
        // Frame header
        if self.config.show_frame_numbers {
            output.push_str(&self.colorize_frame_number(&format!("#{}: ", frame.frame_index)));
        }
        
        // Function and file info
        let function_display = self.colorize_function_name(&frame.debug_info.function_name);
        output.push_str(&function_display);
        
        if let Some(module) = &frame.debug_info.module_name {
            output.push_str(&format!(" ({})", self.colorize_module_name(module)));
        }
        
        output.push_str(&format!(" at {}:{}:{}", 
            frame.debug_info.file_path.display(),
            frame.debug_info.line,
            frame.debug_info.column
        ));
        
        // Inlined information
        if frame.is_inlined {
            output.push_str(&self.colorize_special(" [inlined]"));
        }
        
        // Optimization level
        if let Some(opt_level) = &frame.optimization_level {
            output.push_str(&format!(" ({})", self.colorize_special(opt_level)));
        }
        
        // Call site information
        if let Some(call_site) = &frame.call_site {
            output.push_str(&format!("\n    called from {}:{}", 
                call_site.file_path.display(),
                call_site.line
            ));
        }
        
        // Local variables with enhanced info
        if !frame.local_variables.is_empty() && !self.config.compact_mode {
            output.push_str("\n  Local variables:");
            for var in frame.local_variables.values() {
                output.push_str(&format!("\n    {}", self.format_variable_info(var)));
            }
        }
        
        // Show source context
        if self.config.show_source_context {
            let location = SourceLocation::new(frame.debug_info.line as usize, frame.debug_info.column as usize)
                .with_file(&frame.debug_info.file_path.to_string_lossy());
            
            if let Ok(context) = self.get_source_context(&location) {
                output.push_str(&format!("\n{}", context));
            }
        }
        
        Ok(output)
    }

    /// Format a raw stack frame
    fn format_raw_frame(&self, frame: &RawStackFrame, index: usize) -> crate::error::Result<()> {
        let mut output = String::new();
        
        // Frame header
        if self.config.show_frame_numbers {
            output.push_str(&self.colorize_frame_number(&format!("#{}: ", index)));
        }
        
        // Instruction pointer
        if self.config.show_instruction_pointers {
            output.push_str(&format!("0x{:x} ", frame.instruction_pointer));
        }
        
        // Symbol name
        if let Some(symbol) = &frame.symbol_name {
            output.push_str(&self.colorize_function_name(symbol));
        } else {
            output.push_str(&self.colorize_unknown("<unknown>"));
        }
        
        // Source information
        if let Some(source) = &frame.source_info {
            output.push_str(&format!(" at {}:{}", 
                source.file_path.display(),
                source.line
            ));
            
            if let Some(column) = source.column {
                output.push_str(&format!(":{}", column));
            }
            
            if let Some(module) = &source.module_name {
                output.push_str(&format!(" ({})", self.colorize_module_name(module)));
            }
        }
        
        // CURSED frame indicator
        if frame.is_cursed_frame {
            if self.config.use_gen_z_slang {
                output.push_str(&self.colorize_special(" [CURSED - this is our code bestie] ✨"));
            } else {
                output.push_str(&self.colorize_special(" [CURSED]"));
            }
        }
        
        Ok(output)
    }

    /// Format variable information
    fn format_variable_info(&self, var: &VariableInfo) -> String {
        let mut output = String::new();
        
        if var.is_mutable {
            output.push_str("mut ");
        }
        
        output.push_str(&self.colorize_variable_name(&var.name));
        output.push_str(": ");
        output.push_str(&self.colorize_type_name(&var.type_name));
        
        if let Some(value) = &var.value {
            let truncated_value = self.truncate_value(value);
            output.push_str(" = ");
            output.push_str(&self.colorize_variable_value(&truncated_value));
        }
        
        if let Some(location) = &var.location {
            output.push_str(&format!(" @ {}", self.colorize_location(location)));
        }
        
        if var.scope_depth > 0 {
            output.push_str(&format!(" (scope: {})", var.scope_depth));
        }
        
        output
    }

    /// Get source code context around a location
    fn get_source_context(&mut self, location: &SourceLocation) -> crate::error::Result<()> {
        if !self.config.show_source_context {
            return Ok(String::new());
        }
        
        let file_path = if let Some(file) = &location.file {
            PathBuf::from(file)
        } else {
            return Ok(String::new());
        };
        
        // Load source file if not cached
        if !self.source_cache.contains_key(&file_path) {
            match std::fs::read_to_string(&file_path) {
                Ok(content) => {
                    let lines: Vec<String> = content.split("\n").map(|l| l.to_string()).collect();
                    self.source_cache.insert(file_path.clone(), lines);
                }
                Err(_) => {
                    return Ok(String::new());
                }
            }
        }
        
        let lines = self.source_cache.get(&file_path).unwrap();
        let line_number = location.line.saturating_sub(1); // Convert to 0-based
        
        let start_line = line_number.saturating_sub(self.config.context_lines as usize);
        let end_line = std::cmp::min(line_number + self.config.context_lines as usize + 1, lines.len());
        
        let mut context = String::new();
        context.push_str("  Source context:\n");
        
        for (i, line) in lines[start_line..end_line].iter().enumerate() {
            let current_line = start_line + i + 1; // Convert back to 1-based
            let is_error_line = current_line == location.line;
            
            let line_marker = if is_error_line { ">" } else { " " };
            let line_color = if is_error_line {
                self.colorize_error_line(&format!("{} {:4} | {}", line_marker, current_line, line))
            } else {
                self.colorize_context_line(&format!("{} {:4} | {}", line_marker, current_line, line))
            };
            
            context.push_str(&format!("    {}\n", line_color));
        }
        
        Ok(context)
    }

    /// Truncate variable values if too long
    fn truncate_value(&self, value: &str) -> String {
        if value.len() <= self.config.max_variable_length {
            value.to_string()
        } else {
            format!("{}...", &value[..self.config.max_variable_length - 3])
        }
    }

    /// Format source location
    fn format_source_location(&self, location: &SourceLocation) -> String {
        if let Some(file) = &location.file {
            format!("{}:{}:{}", file, location.line, location.column)
        } else {
            format!("{}:{}", location.line, location.column)
        }
    }

    /// Generate Gen Z themed header
    fn gen_z_header(&self, text: &str) -> String {
        let separator = "═".repeat(text.len());
        format!("{}\n{}\n{}\n", separator, text, separator)
    }

    // Colorization methods
    fn colorize_frame_number(&self, text: &str) -> String {
        if self.config.use_colors {
            text.bright_blue().to_string()
        } else {
            text.to_string()
        }
    }

    fn colorize_function_name(&self, text: &str) -> String {
        if self.config.use_colors {
            if self.is_cursed_function(text) {
                text.bright_magenta().bold().to_string()
            } else {
                text.bright_yellow().to_string()
            }
        } else {
            text.to_string()
        }
    }

    fn colorize_module_name(&self, text: &str) -> String {
        if self.config.use_colors {
            text.bright_cyan().to_string()
        } else {
            text.to_string()
        }
    }

    fn colorize_variable_name(&self, text: &str) -> String {
        if self.config.use_colors {
            text.bright_green().to_string()
        } else {
            text.to_string()
        }
    }

    fn colorize_variable_value(&self, text: &str) -> String {
        if self.config.use_colors {
            text.white().to_string()
        } else {
            text.to_string()
        }
    }

    fn colorize_type_name(&self, text: &str) -> String {
        if self.config.use_colors {
            text.bright_blue().to_string()
        } else {
            text.to_string()
        }
    }

    fn colorize_location(&self, text: &str) -> String {
        if self.config.use_colors {
            text.dimmed().to_string()
        } else {
            text.to_string()
        }
    }

    fn colorize_special(&self, text: &str) -> String {
        if self.config.use_colors {
            text.bright_magenta().to_string()
        } else {
            text.to_string()
        }
    }

    fn colorize_unknown(&self, text: &str) -> String {
        if self.config.use_colors {
            text.dimmed().to_string()
        } else {
            text.to_string()
        }
    }

    fn colorize_error_line(&self, text: &str) -> String {
        if self.config.use_colors {
            text.red().bold().to_string()
        } else {
            text.to_string()
        }
    }

    fn colorize_context_line(&self, text: &str) -> String {
        if self.config.use_colors {
            text.dimmed().to_string()
        } else {
            text.to_string()
        }
    }

    /// Check if function name contains CURSED keywords
    fn is_cursed_function(&self, function_name: &str) -> bool {
        let cursed_keywords = [
            "slay", "yolo", "periodt", "lowkey", "highkey", "sus", "facts",
            "bestie", "flex", "vibe_check", "mood", "basic", "stan", "tea"
        ];
        
        cursed_keywords.iter().any(|keyword| function_name.contains(keyword))
    }

    /// Print debug output to writer
    pub fn print_to_writer<W: Write>(&mut self, output: &str, writer: &mut W) -> crate::error::Result<()> {
        writeln!(writer, "{}", output)
            .map_err(|e| CursedError::Runtime(format!("Failed to write debug output: {}", e)))
    }

    /// Print to stdout
    pub fn print(&mut self, output: &str) -> crate::error::Result<()> {
        let mut stdout = io::stdout();
        self.print_to_writer(output, &mut stdout)
    }

    /// Print to stderr
    pub fn print_error(&mut self, output: &str) -> crate::error::Result<()> {
        let mut stderr = io::stderr();
        self.print_to_writer(output, &mut stderr)
    }
}

impl Default for DebugFormatter {
    fn default() -> Self {
        Self::new()
    }
}

/// Gen Z themed error messages
pub struct GenZMessages;

impl GenZMessages {
    /// Get a Gen Z themed panic message
    pub fn panic_message(error_type: &str) -> String {
        match error_type {
            "null_pointer" => "💀 Bestie tried to use a null pointer - that's not gonna work chief 💀".to_string(),
            "index_out_of_bounds" => "📍 Array index said 'I'm gonna touch grass' - it's out of bounds bestie 📍".to_string(),
            "division_by_zero" => "🧮 Math teacher is NOT happy - you can't divide by zero periodt 🧮".to_string(),
            "stack_overflow" => "📚 Stack said 'I can't even' and overflowed - too much recursion bestie 📚".to_string(),
            "type_mismatch" => "🔄 Types are NOT vibing together - this ain't it chief 🔄".to_string(),
            "memory_error" => "🧠 Memory said 'no cap' and refused to work - we're out of space bestie 🧠".to_string(),
            "io_error" => "💾 File operations are NOT passing the vibe check - I/O error detected 💾".to_string(),
            _ => format!("💥 Something went wrong bestie - {} error detected (we're not vibing) 💥", error_type),
        }
    }

    /// Get a Gen Z themed success message
    pub fn success_message(operation: &str) -> String {
        match operation {
            "compilation" => "✨ Code compilation was absolutely SENDING - no cap bestie! ✨".to_string(),
            "execution" => "🚀 Program executed successfully - that's some good tea right there! 🚀".to_string(),
            "test_pass" => "🎯 Tests are passing the vibe check - periodt! 🎯".to_string(),
            "gc_collection" => "🗑️ Garbage collector cleaned house - very demure, very mindful 🗑️".to_string(),
            _ => format!("✅ {} completed successfully - we love to see it! ✅", operation),
        }
    }

    /// Get a Gen Z themed warning message
    pub fn warning_message(warning_type: &str) -> String {
        match warning_type {
            "performance" => "⚠️ Performance is giving 'slow vibes' - might want to optimize bestie ⚠️".to_string(),
            "deprecated" => "👴 Using deprecated features - time to touch grass and update your code 👴".to_string(),
            "unused_variable" => "👻 Variable is literally ghosting the code - it's unused bestie 👻".to_string(),
            "type_inference" => "🤔 Type inference is confused - be more specific no cap 🤔".to_string(),
            _ => format!("⚠️ {} warning detected - proceed with caution bestie ⚠️", warning_type),
        }
    }
}

/// Convenience functions for common debug output scenarios
pub fn format_panic_trace(trace: &StackTrace) -> crate::error::Result<()> {
    let mut formatter = DebugFormatter::new();
    let mut output = String::new();
    
    if formatter.config.use_gen_z_slang {
        output.push_str("💥 PANIC DETECTED - This code is NOT it bestie! 💥\n\n");
    } else {
        output.push_str("PANIC: Stack trace follows\n\n");
    }
    
    output.push_str(&formatter.format_stack_trace(trace)?);
    Ok(output)
}

pub fn format_error_with_context(error: &CursedError, trace: Option<&StackTrace>) -> crate::error::Result<()> {
    let mut formatter = DebugFormatter::new();
    let mut output = String::new();
    
    if formatter.config.use_gen_z_slang {
        output.push_str("❌ ERROR ALERT - Something's not giving what it's supposed to give ❌\n\n");
    } else {
        output.push_str("ERROR:\n\n");
    }
    
    output.push_str(&format!("Message: {}\n", error));
    
    if let Some(trace) = trace {
        output.push_str("\nStack trace:\n");
        output.push_str(&formatter.format_stack_trace(trace)?);
    }
    
    Ok(output)
}

/// Print a Gen Z themed message to stdout
pub fn print_gen_z_message(category: &str, message_type: &str) {
    let message = match category {
        "panic" => GenZMessages::panic_message(message_type),
        "success" => GenZMessages::success_message(message_type),
        "warning" => GenZMessages::warning_message(message_type),
        _ => format!("📢 {} message: {} 📢", category, message_type),
    };
    
    println!("{}", message);
}

