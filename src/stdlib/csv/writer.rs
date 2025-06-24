use crate::error::Error;
/// CSV Writer implementation with comprehensive configuration options
use std::io::{self, Write, BufWriter};
use crate::stdlib::csv::error::{CsvError, CsvResult, io_error};

/// Configuration for CSV writer
#[derive(Debug, Clone)]
pub struct WriterConfig {
    /// Field delimiter character (default: ',')
    pub comma: char,
    
    /// Quote character (default: '"')
    pub quote: char,
    
    /// Use CRLF line endings instead of LF
    pub use_crlf: bool,
    
    /// Always quote fields
    pub always_quote: bool,
    
    /// Quote fields containing special characters
    pub quote_special: bool,
    
    /// Buffer size for writing
    pub buffer_size: usize,
}

impl Default for WriterConfig {
    fn default() -> Self {
        Self {
            comma: ',',
            quote: '"',
            use_crlf: false,
            always_quote: false,
            quote_special: true,
            buffer_size: 8192,
        }
    }
}

/// CSV Writer that writes records to a CSV-encoded destination
pub struct Writer<W: io::Write> {
    /// Internal buffered writer
    writer: BufWriter<W>,
    
    /// Configuration
    config: WriterConfig,
    
    /// Whether we've written any records yet
    started: bool,
    
    /// Buffer for building output lines
    line_buffer: String,
    
    /// Error state
    error: Option<CsvError>,
}

impl<W: io::Write> Writer<W> {
    /// Create a new CSV writer with default configuration
    pub fn new(writer: W) -> Self {
        Self {
            writer: BufWriter::new(writer),
            config: WriterConfig::default(),
            started: false,
            line_buffer: String::new(),
            error: None,
        }
    }
    
    /// Create a new CSV writer with custom configuration
    pub fn with_config(writer: W, config: WriterConfig) -> Self {
        Self {
            writer: BufWriter::new(writer),
            config,
            started: false,
            line_buffer: String::new(),
            error: None,
        }
    }
    
    /// Set the field delimiter character
    pub fn comma(mut self, c: char) -> Self {
        self.config.comma = c;
        self
    }
    
    /// Set the quote character
    pub fn quote(mut self, c: char) -> Self {
        self.config.quote = c;
        self
    }
    
    /// Enable or disable CRLF line endings
    pub fn use_crlf(mut self, enable: bool) -> Self {
        self.config.use_crlf = enable;
        self
    }
    
    /// Enable or disable always quoting fields
    pub fn always_quote(mut self, enable: bool) -> Self {
        self.config.always_quote = enable;
        self
    }
    
    /// Enable or disable quoting special characters
    pub fn quote_special(mut self, enable: bool) -> Self {
        self.config.quote_special = enable;
        self
    }
    
    /// Get the current comma character
    pub fn get_comma(&self) -> char {
        self.config.comma
    }
    
    /// Get the current quote character
    pub fn get_quote(&self) -> char {
        self.config.quote
    }
    
    /// Write a single record to the CSV
    pub fn write(&mut self, record: &[String]) -> CsvResult<()> {
        if let Some(ref err) = self.error {
            return Err(err.clone());
        }
        
        self.line_buffer.clear();
        
        for (i, field) in record.iter().enumerate() {
            if i > 0 {
                self.line_buffer.push(self.config.comma);
            }
            
            let formatted_field = self.format_field(field);
            self.line_buffer.push_str(&formatted_field);
        }
        
        // Add line ending
        if self.config.use_crlf {
            self.line_buffer.push_str("\r\n");
        } else {
            self.line_buffer.push('\n');
        }
        
        // Write the line
        match self.writer.write_all(self.line_buffer.as_bytes()) {
            Ok(()) => {
                self.started = true;
                Ok(())
            },
            Err(e) => {
                let csv_error = CsvError::from(e);
                self.error = Some(csv_error.clone());
                Err(csv_error)
            }
        }
    }
    
    /// Write multiple records to the CSV
    pub fn write_all(&mut self, records: &[Vec<String>]) -> CsvResult<()> {
        for record in records {
            self.write(record)?;
        }
        Ok(())
    }
    
    /// Flush the writer
    pub fn flush(&mut self) -> CsvResult<()> {
        if let Some(ref err) = self.error {
            return Err(err.clone());
        }
        
        match self.writer.flush() {
            Ok(()) => Ok(()),
            Err(e) => {
                let csv_error = CsvError::from(e);
                self.error = Some(csv_error.clone());
                Err(csv_error)
            }
        }
    }
    
    /// Get the error state
    pub fn error(&self) -> Option<&CsvError> {
        self.error.as_ref()
    }
    
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
        }
        
        // Quote if field contains special characters
        field.contains(self.config.comma) ||
        field.contains(self.config.quote) ||
        field.contains('\n') ||
        field.contains('\r') ||
        field.starts_with(' ') ||
        field.ends_with(' ')
    }
    
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
    }
    
    /// Get configuration
    pub fn config(&self) -> &WriterConfig {
        &self.config
    }
    
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
}

/// Helper function to write CSV data to a string with custom delimiter
pub fn write_to_string_with_delimiter(records: &[Vec<String>], delimiter: char) -> CsvResult<String> {
    let mut buf = Vec::new();
    {
        let mut writer = Writer::new(&mut buf).comma(delimiter);
        writer.write_all(records)?;
        writer.flush()?;
    }
    String::from_utf8(buf).map_err(|e| CsvError::from(e.utf8_error()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_basic_csv_writing() {
        let mut buf = Vec::new();
        {
            let mut writer = Writer::new(&mut buf);
            
            let records = vec![
                vec!["name".to_string(), "age".to_string(), "city".to_string()],
                vec!["Alice".to_string(), "30".to_string(), "New York".to_string()],
                vec!["Bob".to_string(), "25".to_string(), "San Francisco".to_string()],
            ];
            
            writer.write_all(&records).unwrap();
            writer.flush().unwrap();
        }
        
        let output = String::from_utf8(buf).unwrap();
        let expected = "name,age,city\nAlice,30,New York\nBob,25,San Francisco\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_quoted_fields() {
        let mut buf = Vec::new();
        {
            let mut writer = Writer::new(&mut buf);
            
            let records = vec![
                vec!["name".to_string(), "description".to_string()],
                vec!["Smith, John".to_string(), "Senior Developer, Backend".to_string()],
                vec!["Jane Doe".to_string(), "UX Designer".to_string()],
            ];
            
            writer.write_all(&records).unwrap();
            writer.flush().unwrap();
        }
        
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("\"Smith, John\""));
        assert!(output.contains("\"Senior Developer, Backend\""));
    }

    #[test]
    fn test_escaped_quotes() {
        let mut buf = Vec::new();
        {
            let mut writer = Writer::new(&mut buf);
            
            let records = vec![
                vec!["text".to_string()],
                vec!["He said \"Hello\" to me".to_string()],
            ];
            
            writer.write_all(&records).unwrap();
            writer.flush().unwrap();
        }
        
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("\"He said \"\"Hello\"\" to me\""));
    }

    #[test]
    fn test_custom_delimiter() {
        let mut buf = Vec::new();
        {
            let mut writer = Writer::new(&mut buf).comma('\t');
            
            let records = vec![
                vec!["name".to_string(), "age".to_string(), "city".to_string()],
                vec!["Alice".to_string(), "30".to_string(), "New York".to_string()],
            ];
            
            writer.write_all(&records).unwrap();
            writer.flush().unwrap();
        }
        
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("name\tage\tcity"));
        assert!(output.contains("Alice\t30\tNew York"));
    }

    #[test]
    fn test_crlf_line_endings() {
        let mut buf = Vec::new();
        {
            let mut writer = Writer::new(&mut buf).use_crlf(true);
            
            let records = vec![
                vec!["name".to_string(), "age".to_string()],
                vec!["Alice".to_string(), "30".to_string()],
            ];
            
            writer.write_all(&records).unwrap();
            writer.flush().unwrap();
        }
        
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("\r\n"));
    }

    #[test]
    fn test_always_quote() {
        let mut buf = Vec::new();
        {
            let mut writer = Writer::new(&mut buf).always_quote(true);
            
            let records = vec![
                vec!["name".to_string(), "age".to_string()],
                vec!["Alice".to_string(), "30".to_string()],
            ];
            
            writer.write_all(&records).unwrap();
            writer.flush().unwrap();
        }
        
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("\"name\",\"age\""));
        assert!(output.contains("\"Alice\",\"30\""));
    }

    #[test]
    fn test_write_single_record() {
        let mut buf = Vec::new();
        {
            let mut writer = Writer::new(&mut buf);
            
            writer.write(&vec!["Alice".to_string(), "30".to_string()]).unwrap();
            writer.write(&vec!["Bob".to_string(), "25".to_string()]).unwrap();
            writer.flush().unwrap();
        }
        
        let output = String::from_utf8(buf).unwrap();
        assert_eq!(output, "Alice,30\nBob,25\n");
    }

    #[test]
    fn test_error_handling() {
        // Test writing to a failing writer
        struct FailingWriter;
        impl io::Write for FailingWriter {
            fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
                Err(io::Error::new(io::ErrorKind::BrokenPipe, "writer failed"))
            }
            
            fn flush(&mut self) -> io::Result<()> {
                Err(io::Error::new(io::ErrorKind::BrokenPipe, "writer failed"))
            }
        }
        
        let mut writer = Writer::new(FailingWriter);
        let result = writer.write(&vec!["test".to_string()]);
        assert!(result.is_err());
        assert!(writer.error().is_some());
    }

    #[test]
    fn test_configuration_methods() {
        let buf = Vec::new();
        let writer = Writer::new(buf)
            .comma(';')
            .quote('\'')
            .use_crlf(true)
            .always_quote(true)
            .quote_special(false);
        
        assert_eq!(writer.config.comma, ';');
        assert_eq!(writer.config.quote, '\'');
        assert_eq!(writer.config.use_crlf, true);
        assert_eq!(writer.config.always_quote, true);
        assert_eq!(writer.config.quote_special, false);
    }

    #[test]
    fn test_helper_functions() {
        let records = vec![
            vec!["name".to_string(), "age".to_string()],
            vec!["Alice".to_string(), "30".to_string()],
        ];
        
        let output = write_to_string(&records).unwrap();
        assert_eq!(output, "name,age\nAlice,30\n");
        
        let output_tsv = write_to_string_with_delimiter(&records, '\t').unwrap();
        assert_eq!(output_tsv, "name\tage\nAlice\t30\n");
    }
}
