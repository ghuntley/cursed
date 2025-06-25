use crate::error::CursedError;
/// CSV Writer implementation with comprehensive configuration options
use std::io::{self, Write, BufWriter};
// use crate::stdlib::csv::error::{CsvError, CsvResult, io_error};

/// Configuration for CSV writer
#[derive(Debug, Clone)]
pub struct WriterConfig {
    /// Field delimiter character (default: ',')
    
    /// Quote character (default: '"')
    
    /// Use CRLF line endings instead of LF
    
    /// Always quote fields
    
    /// Quote fields containing special characters
    
    /// Buffer size for writing
impl Default for WriterConfig {
    fn default() -> Self {
        Self {
        }
    }
/// CSV Writer that writes records to a CSV-encoded destination
pub struct Writer<W: io::Write> {
    /// Internal buffered writer
    
    /// Configuration
    
    /// Whether we've written any records yet
    
    /// Buffer for building output lines
    
    /// CursedError state
impl<W: io::Write> Writer<W> {
    /// Create a new CSV writer with default configuration
    pub fn new(writer: W) -> Self {
        Self {
        }
    }
    
    /// Create a new CSV writer with custom configuration
    pub fn with_config(writer: W, config: WriterConfig) -> Self {
        Self {
        }
    }
    
    /// Set the field delimiter character
    pub fn comma(mut self, c: char) -> Self {
        self.config.comma = c;
        self
    /// Set the quote character
    pub fn quote(mut self, c: char) -> Self {
        self.config.quote = c;
        self
    /// Enable or disable CRLF line endings
    pub fn use_crlf(mut self, enable: bool) -> Self {
        self.config.use_crlf = enable;
        self
    /// Enable or disable always quoting fields
    pub fn always_quote(mut self, enable: bool) -> Self {
        self.config.always_quote = enable;
        self
    /// Enable or disable quoting special characters
    pub fn quote_special(mut self, enable: bool) -> Self {
        self.config.quote_special = enable;
        self
    /// Get the current comma character
    pub fn get_comma(&self) -> char {
        self.config.comma
    /// Get the current quote character
    pub fn get_quote(&self) -> char {
        self.config.quote
    /// Write a single record to the CSV
    pub fn write(&mut self, record: &[String]) -> CsvResult<()> {
        if let Some(ref err) = self.error {
            return Err(err.clone());
        self.line_buffer.clear();
        
        for (i, field) in record.iter().enumerate() {
            if i > 0 {
                self.line_buffer.push(self.config.comma);
            let formatted_field = self.format_field(field);
            self.line_buffer.push_str(&formatted_field);
        // Add line ending
        if self.config.use_crlf {
            self.line_buffer.push_str("\r\n");
        } else {
            self.line_buffer.push('\n');
        // Write the line
        match self.writer.write_all(self.line_buffer.as_bytes()) {
            Ok(()) => {
                self.started = true;
                Ok(())
            Err(e) => {
                let csv_error = CsvError::from(e);
                self.error = Some(csv_error.clone());
                Err(csv_error)
            }
        }
    /// Write multiple records to the CSV
    pub fn write_all(&mut self, records: &[Vec<String>]) -> CsvResult<()> {
        for record in records {
            self.write(record)?;
        }
        Ok(())
    /// Flush the writer
    pub fn flush(&mut self) -> CsvResult<()> {
        if let Some(ref err) = self.error {
            return Err(err.clone());
        match self.writer.flush() {
            Err(e) => {
                let csv_error = CsvError::from(e);
                self.error = Some(csv_error.clone());
                Err(csv_error)
            }
        }
    /// Get the error state
    pub fn error(&self) -> Option<&CsvError> {
        self.error.as_ref()
    /// Format a field for output, adding quotes if necessary
    fn format_field(&self, field: &str) -> String {
        if self.config.always_quote || self.needs_quoting(field) {
            self.quote_field(field)
        } else {
            field.to_string()
        }
    }
    
    /// Check if a field needs quoting
    fn needs_quoting(&self, field: &str) -> bool {
        if !self.config.quote_special {
            return false;
        // Quote if field contains special characters
        field.contains(self.config.comma) ||
        field.contains(self.config.quote) ||
        field.contains('\n') ||
        field.contains('\r') ||
        field.starts_with(' ') ||
        field.ends_with(' ')
    /// Quote a field, escaping internal quotes
    fn quote_field(&self, field: &str) -> String {
        let mut result = String::new();
        result.push(self.config.quote);
        
        for ch in field.chars() {
            if ch == self.config.quote {
                // Escape quote by doubling it
                result.push(self.config.quote);
                result.push(self.config.quote);
            } else {
                result.push(ch);
            }
        }
        
        result.push(self.config.quote);
        result
    /// Get configuration
    pub fn config(&self) -> &WriterConfig {
        &self.config
    /// Check if any records have been written
    pub fn has_written(&self) -> bool {
        self.started
    }
}

// Convenience methods for compatibility with spec
impl<W: io::Write> Writer<W> {
    /// Get the comma character (compatibility method)
    pub fn comma(&self) -> char {
        self.config.comma
    }
}

/// Helper function to write CSV data to a string
pub fn write_to_string(records: &[Vec<String>]) -> CsvResult<String> {
    let mut buf = Vec::new();
    {
        let mut writer = Writer::new(&mut buf);
        writer.write_all(records)?;
        writer.flush()?;
    }
    String::from_utf8(buf).map_err(|e| CsvError::from(e.utf8_error()))
/// Helper function to write CSV data to a string with custom delimiter
pub fn write_to_string_with_delimiter(records: &[Vec<String>], delimiter: char) -> CsvResult<String> {
    let mut buf = Vec::new();
    {
        let mut writer = Writer::new(&mut buf).comma(delimiter);
        writer.write_all(records)?;
        writer.flush()?;
    }
    String::from_utf8(buf).map_err(|e| CsvError::from(e.utf8_error()))
