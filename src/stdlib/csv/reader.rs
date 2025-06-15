/// CSV Reader implementation with comprehensive configuration options
use std::io::{self, BufRead, BufReader};
use std::collections::VecDeque;
use crate::stdlib::csv::error::{CsvError, CsvResult, ParseError, parse_error, field_count_mismatch};

/// Configuration for CSV reader
#[derive(Debug, Clone)]
pub struct ReaderConfig {
    /// Field delimiter character (default: ',')
    pub comma: char,
    
    /// Comment character (default: None)
    pub comment: Option<char>,
    
    /// Expected number of fields per record (-1 for variable)
    pub fields_per_record: i32,
    
    /// Allow lazy quotes (less strict quote parsing)
    pub lazy_quotes: bool,
    
    /// Trim leading space in fields
    pub trim_leading_space: bool,
    
    /// Reuse record slice for memory efficiency
    pub reuse_record: bool,
    
    /// Quote character (default: '"')
    pub quote: char,
    
    /// Buffer size for reading
    pub buffer_size: usize,
}

impl Default for ReaderConfig {
    fn default() -> Self {
        Self {
            comma: ',',
            comment: None,
            fields_per_record: -1, // Variable field count
            lazy_quotes: false,
            trim_leading_space: false,
            reuse_record: false,
            quote: '"',
            buffer_size: 8192,
        }
    }
}

/// CSV Reader that reads records from a CSV-encoded source
pub struct Reader<R: io::Read> {
    /// Internal buffered reader
    reader: BufReader<R>,
    
    /// Configuration
    config: ReaderConfig,
    
    /// Current line number for error reporting
    line_number: usize,
    
    /// Current column number for error reporting
    column_number: usize,
    
    /// Reusable record buffer
    record_buffer: Vec<String>,
    
    /// Field buffer for parsing
    field_buffer: String,
    
    /// Raw line buffer
    line_buffer: String,
    
    /// Whether we've read any records yet
    started: bool,
    
    /// Number of fields in first record (for validation)
    first_record_fields: Option<usize>,
}

impl<R: io::Read> Reader<R> {
    /// Create a new CSV reader with default configuration
    pub fn new(reader: R) -> Self {
        Self {
            reader: BufReader::new(reader),
            config: ReaderConfig::default(),
            line_number: 0,
            column_number: 0,
            record_buffer: Vec::new(),
            field_buffer: String::new(),
            line_buffer: String::new(),
            started: false,
            first_record_fields: None,
        }
    }
    
    /// Create a new CSV reader with custom configuration
    pub fn with_config(reader: R, config: ReaderConfig) -> Self {
        Self {
            reader: BufReader::new(reader),
            config,
            line_number: 0,
            column_number: 0,
            record_buffer: Vec::new(),
            field_buffer: String::new(),
            line_buffer: String::new(),
            started: false,
            first_record_fields: None,
        }
    }
    
    /// Set the field delimiter character
    pub fn comma(mut self, c: char) -> Self {
        self.config.comma = c;
        self
    }
    
    /// Set the comment character
    pub fn comment(mut self, c: char) -> Self {
        self.config.comment = Some(c);
        self
    }
    
    /// Set the expected number of fields per record
    pub fn fields_per_record(mut self, n: i32) -> Self {
        self.config.fields_per_record = n;
        self
    }
    
    /// Enable or disable lazy quotes
    pub fn lazy_quotes(mut self, enable: bool) -> Self {
        self.config.lazy_quotes = enable;
        self
    }
    
    /// Enable or disable trimming of leading space
    pub fn trim_leading_space(mut self, enable: bool) -> Self {
        self.config.trim_leading_space = enable;
        self
    }
    
    /// Enable or disable record reuse
    pub fn reuse_record(mut self, enable: bool) -> Self {
        self.config.reuse_record = enable;
        self
    }
    
    /// Get the current comma character
    pub fn get_comma(&self) -> char {
        self.config.comma
    }
    
    /// Get the current comment character
    pub fn get_comment(&self) -> Option<char> {
        self.config.comment
    }
    
    /// Read a single record from the CSV
    pub fn read(&mut self) -> CsvResult<Option<Vec<String>>> {
        loop {
            self.line_buffer.clear();
            let bytes_read = self.reader.read_line(&mut self.line_buffer)?;
            
            if bytes_read == 0 {
                return Ok(None); // EOF
            }
            
            self.line_number += 1;
            self.column_number = 0;
            
            // Remove trailing newline
            if self.line_buffer.ends_with('\n') {
                self.line_buffer.pop();
                if self.line_buffer.ends_with('\r') {
                    self.line_buffer.pop();
                }
            }
            
            // Skip empty lines
            if self.line_buffer.trim().is_empty() {
                continue;
            }
            
            // Skip comment lines
            if let Some(comment_char) = self.config.comment {
                if self.line_buffer.trim_start().starts_with(comment_char) {
                    continue;
                }
            }
            
            // Parse the line
            match self.parse_line(&self.line_buffer) {
                Ok(record) => {
                    // Validate field count
                    if let Err(e) = self.validate_field_count(&record) {
                        return Err(e);
                    }
                    
                    return Ok(Some(record));
                },
                Err(e) => return Err(e),
            }
        }
    }
    
    /// Read all records from the CSV
    pub fn read_all(&mut self) -> CsvResult<Vec<Vec<String>>> {
        let mut records = Vec::new();
        
        while let Some(record) = self.read()? {
            records.push(record);
        }
        
        Ok(records)
    }
    
    /// Parse a single line into fields
    fn parse_line(&mut self, line: &str) -> CsvResult<Vec<String>> {
        let mut fields = if self.config.reuse_record {
            self.record_buffer.clear();
            &mut self.record_buffer
        } else {
            &mut Vec::new()
        };
        
        let mut chars = line.char_indices().peekable();
        let mut field_start = 0;
        let mut in_quotes = false;
        let mut field_end = 0;
        
        self.field_buffer.clear();
        
        while let Some((pos, ch)) = chars.next() {
            self.column_number = pos + 1;
            
            match ch {
                c if c == self.config.quote => {
                    if in_quotes {
                        // Check if this is an escaped quote
                        if let Some((_, next_ch)) = chars.peek() {
                            if *next_ch == self.config.quote {
                                // Escaped quote, consume the next quote and add one quote to field
                                chars.next();
                                self.field_buffer.push(self.config.quote);
                                continue;
                            }
                        }
                        // End of quoted field
                        in_quotes = false;
                    } else {
                        // Start of quoted field (only valid at field start)
                        if self.field_buffer.is_empty() || self.config.lazy_quotes {
                            in_quotes = true;
                        } else if !self.config.lazy_quotes {
                            return Err(parse_error(
                                self.line_number,
                                self.column_number,
                                "quote character in middle of field"
                            ));
                        } else {
                            self.field_buffer.push(ch);
                        }
                    }
                },
                c if c == self.config.comma && !in_quotes => {
                    // End of field
                    let field_value = if self.config.trim_leading_space {
                        self.field_buffer.trim_start().to_string()
                    } else {
                        self.field_buffer.clone()
                    };
                    fields.push(field_value);
                    self.field_buffer.clear();
                },
                _ => {
                    self.field_buffer.push(ch);
                }
            }
        }
        
        // Add the last field
        if in_quotes && !self.config.lazy_quotes {
            return Err(parse_error(
                self.line_number,
                self.column_number,
                "unterminated quoted field"
            ));
        }
        
        let field_value = if self.config.trim_leading_space {
            self.field_buffer.trim_start().to_string()
        } else {
            self.field_buffer.clone()
        };
        fields.push(field_value);
        
        // Convert to owned Vec if using reuse_record
        if self.config.reuse_record {
            Ok(fields.clone())
        } else {
            Ok(fields.clone())
        }
    }
    
    /// Validate field count against configuration and first record
    fn validate_field_count(&mut self, record: &[String]) -> CsvResult<()> {
        let field_count = record.len();
        
        // Check against configured fields per record
        if self.config.fields_per_record > 0 {
            let expected = self.config.fields_per_record as usize;
            if field_count != expected {
                return Err(field_count_mismatch(expected, field_count, self.line_number));
            }
        }
        
        // Check against first record if this is not the first
        if !self.started {
            self.first_record_fields = Some(field_count);
            self.started = true;
        } else if self.config.fields_per_record < 0 {
            // Only validate against first record if fields_per_record is not set
            if let Some(expected) = self.first_record_fields {
                if field_count != expected {
                    return Err(field_count_mismatch(expected, field_count, self.line_number));
                }
            }
        }
        
        Ok(())
    }
    
    /// Get current line number
    pub fn line_number(&self) -> usize {
        self.line_number
    }
    
    /// Get current column number
    pub fn column_number(&self) -> usize {
        self.column_number
    }
    
    /// Get configuration
    pub fn config(&self) -> &ReaderConfig {
        &self.config
    }
}

// Convenience methods for compatibility with spec
impl<R: io::Read> Reader<R> {
    /// Get the comma character (compatibility method)
    pub fn comma(&self) -> char {
        self.config.comma
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_basic_csv_reading() {
        let csv_data = "name,age,city\nAlice,30,New York\nBob,25,San Francisco";
        let cursor = Cursor::new(csv_data);
        let mut reader = Reader::new(cursor);
        
        let records = reader.read_all().unwrap();
        assert_eq!(records.len(), 3);
        assert_eq!(records[0], vec!["name", "age", "city"]);
        assert_eq!(records[1], vec!["Alice", "30", "New York"]);
        assert_eq!(records[2], vec!["Bob", "25", "San Francisco"]);
    }

    #[test]
    fn test_quoted_fields() {
        let csv_data = r#"name,description
"Smith, John","Senior Developer, Backend"
"Jane Doe","UX Designer""#;
        let cursor = Cursor::new(csv_data);
        let mut reader = Reader::new(cursor);
        
        let records = reader.read_all().unwrap();
        assert_eq!(records.len(), 3);
        assert_eq!(records[1], vec!["Smith, John", "Senior Developer, Backend"]);
        assert_eq!(records[2], vec!["Jane Doe", "UX Designer"]);
    }

    #[test]
    fn test_escaped_quotes() {
        let csv_data = r#"text
"He said ""Hello"" to me""#;
        let cursor = Cursor::new(csv_data);
        let mut reader = Reader::new(cursor);
        
        let records = reader.read_all().unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[1], vec![r#"He said "Hello" to me"#]);
    }

    #[test]
    fn test_custom_delimiter() {
        let tsv_data = "name\tage\tcity\nAlice\t30\tNew York\nBob\t25\tSan Francisco";
        let cursor = Cursor::new(tsv_data);
        let mut reader = Reader::new(cursor).comma('\t');
        
        let records = reader.read_all().unwrap();
        assert_eq!(records.len(), 3);
        assert_eq!(records[1], vec!["Alice", "30", "New York"]);
    }

    #[test]
    fn test_comments() {
        let csv_data = "# This is a comment\nname,age\n# Another comment\nAlice,30\nBob,25";
        let cursor = Cursor::new(csv_data);
        let mut reader = Reader::new(cursor).comment('#');
        
        let records = reader.read_all().unwrap();
        assert_eq!(records.len(), 3); // header + 2 data rows
        assert_eq!(records[0], vec!["name", "age"]);
    }

    #[test]
    fn test_trim_leading_space() {
        let csv_data = "name, age,  city\nAlice,  30,   New York";
        let cursor = Cursor::new(csv_data);
        let mut reader = Reader::new(cursor).trim_leading_space(true);
        
        let records = reader.read_all().unwrap();
        assert_eq!(records[0], vec!["name", "age", "city"]);
        assert_eq!(records[1], vec!["Alice", "30", "New York"]);
    }

    #[test]
    fn test_field_count_validation() {
        let csv_data = "name,age\nAlice,30,extra";
        let cursor = Cursor::new(csv_data);
        let mut reader = Reader::new(cursor);
        
        let result = reader.read_all();
        assert!(result.is_err());
        
        if let Err(CsvError::FieldCountMismatch { expected, actual, line }) = result {
            assert_eq!(expected, 2);
            assert_eq!(actual, 3);
            assert_eq!(line, 2);
        } else {
            panic!("Expected FieldCountMismatch error");
        }
    }

    #[test]
    fn test_lazy_quotes() {
        let csv_data = r#"text
He said "Hello" to me"#;
        let cursor = Cursor::new(csv_data);
        let mut reader = Reader::new(cursor).lazy_quotes(true);
        
        let records = reader.read_all().unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[1], vec![r#"He said "Hello" to me"#]);
    }

    #[test]
    fn test_empty_lines_and_whitespace() {
        let csv_data = "name,age\n\nAlice,30\n\n\nBob,25\n";
        let cursor = Cursor::new(csv_data);
        let mut reader = Reader::new(cursor);
        
        let records = reader.read_all().unwrap();
        assert_eq!(records.len(), 3); // header + 2 data rows
    }

    #[test]
    fn test_configuration_methods() {
        let cursor = Cursor::new("a,b\n1,2");
        let reader = Reader::new(cursor)
            .comma(';')
            .comment('#')
            .fields_per_record(2)
            .lazy_quotes(true)
            .trim_leading_space(true)
            .reuse_record(true);
        
        assert_eq!(reader.config.comma, ';');
        assert_eq!(reader.config.comment, Some('#'));
        assert_eq!(reader.config.fields_per_record, 2);
        assert_eq!(reader.config.lazy_quotes, true);
        assert_eq!(reader.config.trim_leading_space, true);
        assert_eq!(reader.config.reuse_record, true);
    }
}
