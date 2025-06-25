/// DropZ - Basic I/O primitives with Gen Z flair 📤
/// 
/// This module provides essential input/output operations using CURSED language
/// conventions and Gen Z naming. All functions work with CURSED types and provide
/// the I/O foundation for interactive applications.
/// 
/// # Why DropZ matters:
/// - Essential for any interactive application
/// - Provides type-safe I/O operations with CURSED semantics  
/// - Includes modern I/O patterns with Gen Z naming
/// - Optimized for performance while maintaining ease of use

// use crate::stdlib::io::{self, IoError, IoResult};
use crate::error::CursedError;
use std::collections::HashMap;
use std::io::{Write, BufRead, BufReader, BufWriter};
use std::fs::File;

/// CursedError type for DropZ operations
pub type DropzError = IoError;

/// Result type for DropZ operations
pub type DropzResult<T> = IoResult<T>;

/// Tea type alias for CURSED strings
pub type Tea = String;

/// Normie type alias for CURSED int32  
pub type Normie = i32;

/// Thicc type alias for CURSED int64
pub type Thicc = i64;

/// Chonky type alias for CURSED f64
pub type Chonky = f64;

// ================================
// BASIC OUTPUT OPERATIONS (PRINT VIBES)
// ================================

/// Print tea without newline (drop some text vibes)
/// 
/// # Examples
/// ```cursed
/// drop_tea("Hello");
/// drop_tea(" World"); // Output: "Hello World"
/// ```
pub fn drop_tea(text: &str) -> DropzResult<()> {
    io::print(text)
/// Print tea with newline (drop line vibes)
/// 
/// # Examples
/// ```cursed
/// drop_line_tea("Hello World"); // Output: "Hello World\n"
/// ```
pub fn drop_line_tea(text: &str) -> DropzResult<()> {
    io::println(text)
/// Print error tea without newline (drop error vibes)
/// 
/// # Examples
/// ```cursed
/// drop_error_tea("Warning: ");
/// drop_error_tea("Something sus"); // Output to stderr
/// ```
pub fn drop_error_tea(text: &str) -> DropzResult<()> {
    io::eprint(text)
/// Print error tea with newline (drop error line vibes)
/// 
/// # Examples
/// ```cursed
/// drop_error_line_tea("CursedError: File not found"); // Output to stderr with newline
/// ```
pub fn drop_error_line_tea(text: &str) -> DropzResult<()> {
    io::eprintln(text)
/// Print formatted tea (drop formatted vibes)
/// 
/// # Examples
/// ```cursed
/// drop_formatted_tea("Hello {}, you are {} years old", &["Alice", "25"]);
/// ```
pub fn drop_formatted_tea(format: &str, args: &[&str]) -> DropzResult<()> {
    // Convert &[&str] to &[Value] for compatibility with io::printf
//     let values: Vec<crate::stdlib::value::Value> = args.iter()
//         .map(|&s| crate::stdlib::value::Value::String(s.to_string()))
        .collect();
    io::printf(format, &values)
/// Flush output buffers (force drop vibes)
/// 
/// # Examples
/// ```cursed
/// drop_tea("Processing...");
/// flush_drops(); // Ensure output is visible immediately
/// ```
pub fn flush_drops() -> DropzResult<()> {
    io::flush()
// ================================
// BASIC INPUT OPERATIONS (CATCH VIBES)
// ================================

/// Read line from input (catch line vibes)
/// 
/// # Examples
/// ```cursed
/// facts input = catch_line_tea(); // Reads user input until newline
/// ```
pub fn catch_line_tea() -> DropzResult<Tea> {
    io::read_line()
/// Read single character from input (catch char vibes)
/// 
/// # Examples
/// ```cursed
/// facts ch = catch_char_vibes(); // Reads single character
/// ```
pub fn catch_char_vibes() -> DropzResult<char> {
    io::read_char()
/// Read until delimiter (catch until vibes)
/// 
/// # Examples
/// ```cursed
/// facts text = catch_until_vibes('|'); // Read until pipe character
/// ```
pub fn catch_until_vibes(delimiter: char) -> DropzResult<Tea> {
    io::read_until(delimiter)
/// Read all input until EOF (catch all vibes)
/// 
/// # Examples
/// ```cursed
/// facts all_input = catch_all_tea(); // Read everything until EOF
/// ```
pub fn catch_all_tea() -> DropzResult<Tea> {
    io::read_all()
// ================================
// INTERACTIVE INPUT OPERATIONS (VIBE CHECK)
// ================================

/// Show prompt and read input (vibe check vibes)
/// 
/// # Examples
/// ```cursed
/// facts name = vibe_check_tea("Enter your name: ");
/// ```
pub fn vibe_check_tea(prompt: &str) -> DropzResult<Tea> {
    io::prompt(prompt)
/// Ask yes/no question (vibe check bool vibes)
/// 
/// # Examples
/// ```cursed
/// facts proceed = vibe_check_bool("Continue? (y/n): ");
/// ```
pub fn vibe_check_bool(prompt: &str) -> DropzResult<bool> {
    io::confirm(prompt)
/// Read password without echo (vibe check secret vibes)
/// 
/// # Examples
/// ```cursed
/// facts password = vibe_check_secret("Enter password: ");
/// ```
pub fn vibe_check_secret(prompt: &str) -> DropzResult<Tea> {
    io::read_password(prompt)
/// Select from options (vibe check choice vibes)
/// 
/// # Examples
/// ```cursed
/// facts options = vec!["Option 1".to_string(), "Option 2".to_string()];
/// facts choice = vibe_check_choice("Choose an option:", &options);
/// ```
pub fn vibe_check_choice(prompt: &str, options: &[String]) -> DropzResult<usize> {
    io::select(prompt, options)
/// Select multiple options (vibe check multiple vibes)
/// 
/// # Examples
/// ```cursed
/// facts options = vec!["Feature A".to_string(), "Feature B".to_string()];
/// facts choices = vibe_check_multiple("Select features:", &options);
/// ```
pub fn vibe_check_multiple(prompt: &str, options: &[String]) -> DropzResult<Vec<usize>> {
    io::multi_select(prompt, options)
// ================================
// NUMBER INPUT OPERATIONS (NUMERIC VIBES)
// ================================

/// Read normie number from input (catch normie vibes)
/// 
/// # Examples
/// ```cursed
/// facts num = catch_normie_vibes("Enter a number: ");
/// ```
pub fn catch_normie_vibes(prompt: &str) -> DropzResult<Normie> {
    let input = vibe_check_tea(prompt)?;
    input.trim().parse::<Normie>()
        .map_err(|_| DropzError::InvalidInput(format!("Invalid normie number: {}", input)))
/// Read thicc number from input (catch thicc vibes)
/// 
/// # Examples
/// ```cursed
/// facts big_num = catch_thicc_vibes("Enter a big number: ");
/// ```
pub fn catch_thicc_vibes(prompt: &str) -> DropzResult<Thicc> {
    let input = vibe_check_tea(prompt)?;
    input.trim().parse::<Thicc>()
        .map_err(|_| DropzError::InvalidInput(format!("Invalid thicc number: {}", input)))
/// Read chonky number from input (catch chonky vibes)
/// 
/// # Examples
/// ```cursed
/// facts decimal = catch_chonky_vibes("Enter a decimal: ");
/// ```
pub fn catch_chonky_vibes(prompt: &str) -> DropzResult<Chonky> {
    let input = vibe_check_tea(prompt)?;
    input.trim().parse::<Chonky>()
        .map_err(|_| DropzError::InvalidInput(format!("Invalid chonky number: {}", input)))
// ================================
// BUFFERED I/O OPERATIONS (STREAMING VIBES)
// ================================

/// Buffered reader for efficient input (stream catcher vibes)
/// 
/// # Examples
/// ```cursed
/// facts reader = StreamCatcherVibes::new(file);
/// facts line = reader.catch_line();
/// ```
pub struct StreamCatcherVibes<R: std::io::Read> {
impl<R: std::io::Read> StreamCatcherVibes<R> {
    /// Create new buffered reader (new stream catcher vibes)
    pub fn new(reader: R) -> Self {
        Self {
        }
    }
    
    /// Read line from buffered reader (catch line from stream vibes)
    pub fn catch_line(&mut self) -> DropzResult<Tea> {
        let mut line = String::new();
        self.reader.read_line(&mut line)
            .map_err(|e| DropzError::General(e.to_string()))?;
        
        // Remove trailing newline if present
        if line.ends_with('\n') {
            line.pop();
            if line.ends_with('\r') {
                line.pop();
            }
        }
        
        Ok(line)
    /// Read all lines from buffered reader (catch all lines vibes)
    pub fn catch_all_lines(&mut self) -> DropzResult<Vec<Tea>> {
        let mut lines = Vec::new();
        loop {
            let mut line = String::new();
            let bytes_read = self.reader.read_line(&mut line)
                .map_err(|e| DropzError::General(e.to_string()))?;
            
            if bytes_read == 0 {
                break; // EOF
            // Remove trailing newline if present
            if line.ends_with('\n') {
                line.pop();
                if line.ends_with('\r') {
                    line.pop();
                }
            }
            
            lines.push(line);
        }
        Ok(lines)
    /// Check if more data is available (has more vibes)
    pub fn has_more_vibes(&mut self) -> DropzResult<bool> {
        match self.reader.fill_buf() {
        }
    }
/// Buffered writer for efficient output (stream dropper vibes)
/// 
/// # Examples
/// ```cursed
/// facts writer = StreamDropperVibes::new(file);
/// writer.drop_line("Hello World");
/// writer.flush_it();
/// ```
pub struct StreamDropperVibes<W: std::io::Write> {
impl<W: std::io::Write> StreamDropperVibes<W> {
    /// Create new buffered writer (new stream dropper vibes)
    pub fn new(writer: W) -> Self {
        Self {
        }
    }
    
    /// Write tea to buffered writer (drop to stream vibes)
    pub fn drop_tea(&mut self, text: &str) -> DropzResult<()> {
        self.writer.write_all(text.as_bytes())
            .map_err(|e| DropzError::General(e.to_string()))
    /// Write tea with newline to buffered writer (drop line to stream vibes)
    pub fn drop_line(&mut self, text: &str) -> DropzResult<()> {
        writeln!(self.writer, "{}", text)
            .map_err(|e| DropzError::General(e.to_string()))
    /// Flush buffered writer (force stream drop vibes)
    pub fn flush_it(&mut self) -> DropzResult<()> {
        self.writer.flush()
            .map_err(|e| DropzError::General(e.to_string()))
    /// Write multiple lines (drop multiple lines vibes)
    pub fn drop_lines(&mut self, lines: &[&str]) -> DropzResult<()> {
        for line in lines {
            self.drop_line(line)?;
        }
        Ok(())
    /// Write formatted tea (drop formatted to stream vibes)
    pub fn drop_formatted(&mut self, format: &str, args: &[&str]) -> DropzResult<()> {
        let formatted = simple_format(format, args)?;
        self.drop_tea(&formatted)
    }
}

// ================================
// PROGRESS OPERATIONS (PROGRESS VIBES)
// ================================

/// Progress bar for visual feedback (progress vibes)
/// 
/// # Examples
/// ```cursed
/// facts progress = ProgressVibes::new(100, 50);
/// progress.update_it(50);
/// progress.finish_it();
/// ```
pub struct ProgressVibes {
impl ProgressVibes {
    /// Create new progress bar (new progress vibes)
    pub fn new(total: usize, width: usize) -> Self {
        Self {
        }
    }
    
    /// Update progress (update vibes)
    pub fn update_it(&mut self, current: usize) -> DropzResult<()> {
        self.current = current.min(self.total);
        self.render_it()
    /// Set progress message (set message vibes)
    pub fn set_message(&mut self, message: Tea) {
        self.message = message;
    /// Render progress bar (show vibes)
    fn render_it(&self) -> DropzResult<()> {
        let percentage = if self.total > 0 {
            (self.current * 100) / self.total
        } else {
            0
        
        let filled = if self.total > 0 {
            (self.current * self.width) / self.total
        } else {
            0
        
        let empty = self.width.saturating_sub(filled);
        
        let bar = format!(
            "\r{} [{}{}] {}% ({}/{})",
            self.total
        );
        
        drop_tea(&bar)?;
        flush_drops()
    /// Finish progress bar (complete vibes)
    pub fn finish_it(&mut self) -> DropzResult<()> {
        self.current = self.total;
        self.render_it()?;
        drop_line_tea("")
    /// Increment progress (step vibes)
    pub fn step_it(&mut self) -> DropzResult<()> {
        self.update_it(self.current + 1)
    }
}

/// Create new progress bar (convenience function)
/// 
/// # Examples
/// ```cursed
/// facts progress = new_progress_vibes(100, 50);
/// ```
pub fn new_progress_vibes(total: usize, width: usize) -> ProgressVibes {
    ProgressVibes::new(total, width)
// ================================
// PAGINATED OUTPUT (PAGINATION VIBES)
// ================================

/// Display content with pagination (paged drops vibes)
/// 
/// # Examples
/// ```cursed
/// facts lines = vec!["Line 1".to_string(), "Line 2".to_string()];
/// paginate_drops(&lines, 10);
/// ```
pub fn paginate_drops(lines: &[Tea], page_size: usize) -> DropzResult<()> {
    if lines.is_empty() {
        return Ok(());
    let mut start = 0;
    loop {
        let end = (start + page_size).min(lines.len());
        
        // Display current page
        for line in &lines[start..end] {
            drop_line_tea(line)?;
        // Check if we're done
        if end >= lines.len() {
            break;
        // Ask user to continue
        drop_tea(&format!("Page {}/{} - Press Enter to continue, 'q' to quit: ", 
                         (start / page_size) + 1, 
                         (lines.len() + page_size - 1) / page_size))?;
        flush_drops()?;
        
        let input = catch_line_tea()?;
        if input.trim().to_lowercase() == "q" {
            break;
        start = end;
    Ok(())
// ================================
// UTILITY FUNCTIONS
// ================================

/// Simple format function for basic string interpolation
fn simple_format(format: &str, args: &[&str]) -> DropzResult<Tea> {
    let mut result = format.to_string();
    
    for (i, arg) in args.iter().enumerate() {
        let placeholder = format!("{{{}}}", i);
        result = result.replace(&placeholder, arg);
    // Also support {} placeholder
    let mut arg_index = 0;
    while let Some(pos) = result.find("{}") {
        if arg_index < args.len() {
            result.replace_range(pos..pos+2, args[arg_index]);
            arg_index += 1;
        } else {
            break;
        }
    }
    
    Ok(result)
/// Clear screen (clear vibes)
/// 
/// # Examples
/// ```cursed
/// clear_drops(); // Clears the terminal screen
/// ```
pub fn clear_drops() -> DropzResult<()> {
    if cfg!(target_os = "windows") {
        drop_tea("\x1B[2J\x1B[1;1H")
    } else {
        drop_tea("\x1B[2J\x1B[H")
    }
}

/// Move cursor to position (move cursor vibes)
/// 
/// # Examples
/// ```cursed
/// move_cursor_vibes(10, 5); // Move to row 10, column 5
/// ```
pub fn move_cursor_vibes(row: usize, col: usize) -> DropzResult<()> {
    drop_tea(&format!("\x1B[{};{}H", row, col))
/// Hide cursor (hide cursor vibes)
/// 
/// # Examples
/// ```cursed
/// hide_cursor_vibes();
/// ```
pub fn hide_cursor_vibes() -> DropzResult<()> {
    drop_tea("\x1B[?25l")
/// Show cursor (show cursor vibes)
/// 
/// # Examples
/// ```cursed
/// show_cursor_vibes();
/// ```
pub fn show_cursor_vibes() -> DropzResult<()> {
    drop_tea("\x1B[?25h")
/// Get terminal size (terminal size vibes)
/// 
/// # Examples
/// ```cursed
/// facts (width, height) = get_terminal_size_vibes();
/// ```
pub fn get_terminal_size_vibes() -> DropzResult<(usize, usize)> {
    // Basic implementation - in real world this would use termios/winapi
    Ok((80, 24)) // Default terminal size
/// Set text color (color vibes)
/// 
/// # Examples
/// ```cursed
/// set_text_color_vibes("red");
/// drop_line_tea("This is red text");
/// reset_text_color_vibes();
/// ```
pub fn set_text_color_vibes(color: &str) -> DropzResult<()> {
    let color_code = match color.to_lowercase().as_str() {
    
    drop_tea(&format!("\x1B[{}m", color_code))
/// Reset text color (reset color vibes)
/// 
/// # Examples
/// ```cursed
/// reset_text_color_vibes();
/// ```
pub fn reset_text_color_vibes() -> DropzResult<()> {
    drop_tea("\x1B[0m")
/// Module initialization function
pub fn init_dropz() -> DropzResult<()> {
    // Initialize any global state for DropZ module
    Ok(())
/// Get module statistics and information
pub fn get_dropz_stats() -> HashMap<String, String> {
    let mut stats = HashMap::new();
    stats.insert("version".to_string(), "1.0.0".to_string());
    stats.insert("operations".to_string(), "Input, Output, Buffered I/O, Progress, Pagination".to_string());
    stats.insert("features".to_string(), "Gen Z naming, CURSED types, interactive I/O".to_string());
    stats.insert("types".to_string(), "tea, normie, thicc, chonky".to_string());
    stats
