/// PostgreSQL COPY protocol implementation for bulk operations in CURSED
/// 
/// This module provides support for PostgreSQL's COPY protocol which allows
/// high-performance bulk data import/export operations.

use std::io::{Read, Write, BufRead, BufReader, BufWriter};
use std::sync::{Arc, Mutex};
use super::{PostgreSQLConnection, PostgreSQLError};
use super::super::{DatabaseError, SqlValue};
use super::ffi::SafePGconn;

/// fr fr COPY operation direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CopyDirection {
    /// Copy data from client to server (COPY ... FROM)
    In,
    /// Copy data from server to client (COPY ... TO)
    Out,
    /// Bidirectional copy (COPY ... WITH BINARY)
    Both,
}

/// fr fr COPY data format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CopyFormat {
    /// Text format (default)
    Text,
    /// Binary format (more efficient)
    Binary,
    /// CSV format
    Csv,
}

impl CopyFormat {
    /// slay Convert to PostgreSQL format string
    pub fn to_pg_string(&self) -> &'static str {
        match self {
            Self::Text => "TEXT",
            Self::Binary => "BINARY",
            Self::Csv => "CSV",
        }
    }
}

/// fr fr COPY options for fine-tuning operations
#[derive(Debug, Clone)]
pub struct CopyOptions {
    /// fr fr Data format
    pub format: CopyFormat,
    /// fr fr Field delimiter (for text/CSV)
    pub delimiter: Option<String>,
    /// fr fr Null value representation
    pub null_string: Option<String>,
    /// fr fr Quote character (for CSV)
    pub quote: Option<char>,
    /// fr fr Escape character (for CSV)
    pub escape: Option<char>,
    /// fr fr Include header row (for CSV)
    pub header: bool,
    /// fr fr Force quote all fields (for CSV)
    pub force_quote: bool,
    /// fr fr Encoding
    pub encoding: Option<String>,
    /// fr fr Specific columns to copy
    pub columns: Vec<String>,
}

impl Default for CopyOptions {
    fn default() -> Self {
        Self {
            format: CopyFormat::Text,
            delimiter: None,
            null_string: None,
            quote: None,
            escape: None,
            header: false,
            force_quote: false,
            encoding: None,
            columns: Vec::new(),
        }
    }
}

impl CopyOptions {
    /// slay Create options for text format
    pub fn text() -> Self {
        Self {
            format: CopyFormat::Text,
            ..Default::default()
        }
    }
    
    /// slay Create options for binary format
    pub fn binary() -> Self {
        Self {
            format: CopyFormat::Binary,
            ..Default::default()
        }
    }
    
    /// slay Create options for CSV format
    pub fn csv() -> Self {
        Self {
            format: CopyFormat::Csv,
            header: true,
            ..Default::default()
        }
    }
    
    /// slay Set delimiter
    pub fn delimiter(mut self, delim: String) -> Self {
        self.delimiter = Some(delim);
        self
    }
    
    /// slay Set null string
    pub fn null_string(mut self, null_str: String) -> Self {
        self.null_string = Some(null_str);
        self
    }
    
    /// slay Set quote character
    pub fn quote(mut self, quote_char: char) -> Self {
        self.quote = Some(quote_char);
        self
    }
    
    /// slay Include header
    pub fn with_header(mut self) -> Self {
        self.header = true;
        self
    }
    
    /// slay Set columns
    pub fn columns(mut self, cols: Vec<String>) -> Self {
        self.columns = cols;
        self
    }
    
    /// slay Build PostgreSQL COPY options string
    pub fn to_pg_options(&self) -> String {
        let mut options = Vec::new();
        
        options.push(format!("FORMAT {}", self.format.to_pg_string()));
        
        if let Some(ref delim) = self.delimiter {
            options.push(format!("DELIMITER '{}'", delim));
        }
        
        if let Some(ref null_str) = self.null_string {
            options.push(format!("NULL '{}'", null_str));
        }
        
        if let Some(quote_char) = self.quote {
            options.push(format!("QUOTE '{}'", quote_char));
        }
        
        if let Some(escape_char) = self.escape {
            options.push(format!("ESCAPE '{}'", escape_char));
        }
        
        if self.header {
            options.push("HEADER".to_string());
        }
        
        if self.force_quote {
            options.push("FORCE_QUOTE *".to_string());
        }
        
        if let Some(ref encoding) = self.encoding {
            options.push(format!("ENCODING '{}'", encoding));
        }
        
        if options.is_empty() {
            String::new()
        } else {
            format!("WITH ({})", options.join(", "))
        }
    }
}

/// fr fr COPY manager for handling bulk operations
#[derive(Debug)]
pub struct CopyManager {
    /// fr fr Connection handle
    conn: Arc<Mutex<SafePGconn>>,
}

impl CopyManager {
    /// slay Create a new COPY manager
    pub fn new(conn: Arc<Mutex<SafePGconn>>) -> Self {
        Self { conn }
    }
    
    /// slay Copy data from reader to table
    pub fn copy_in<R: Read>(
        &self,
        table: &str,
        reader: R,
        options: CopyOptions,
    ) -> crate::error::Result<()> {
        let copy_sql = self.build_copy_in_sql(table, &options);
        
        let conn = self.conn.lock().map_err(|_| {
            PostgreSQLError::connection_error("Failed to acquire connection lock")
        })?;
        
        // Start COPY operation
        let result = conn.exec(&copy_sql)
            .map_err(|e| PostgreSQLError::query_error(&format!("Failed to start COPY IN: {}", e)))?;
        
        // Stream data
        let mut buf_reader = BufReader::new(reader);
        let mut bytes_copied = 0u64;
        
        match options.format {
            CopyFormat::Text | CopyFormat::Csv => {
                self.copy_text_data(&conn, &mut buf_reader, &mut bytes_copied)?;
            }
            CopyFormat::Binary => {
                self.copy_binary_data(&conn, &mut buf_reader, &mut bytes_copied)?;
            }
        }
        
        // End COPY operation
        unsafe {
            use super::ffi::PQputCopyEnd;
            let result_code = PQputCopyEnd(conn.as_ptr(), std::ptr::null());
            if result_code != 1 {
                return Err(PostgreSQLError::query_error("Failed to end COPY operation"));
            }
        }
        
        Ok(bytes_copied)
    }
    
    /// slay Copy data from table to writer
    pub fn copy_out<W: Write>(
        &self,
        table: &str,
        writer: W,
        options: CopyOptions,
    ) -> crate::error::Result<()> {
        let copy_sql = self.build_copy_out_sql(table, &options);
        
        let conn = self.conn.lock().map_err(|_| {
            PostgreSQLError::connection_error("Failed to acquire connection lock")
        })?;
        
        // Start COPY operation
        let result = conn.exec(&copy_sql)
            .map_err(|e| PostgreSQLError::query_error(&format!("Failed to start COPY OUT: {}", e)))?;
        
        // Read data
        let mut buf_writer = BufWriter::new(writer);
        let mut bytes_copied = 0u64;
        
        loop {
            unsafe {
                use super::ffi::PQgetCopyData;
                use std::ffi::CStr;
                
                let mut buffer: *mut i8 = std::ptr::null_mut();
                let result = PQgetCopyData(conn.as_ptr(), &mut buffer, 0);
                
                if result == -1 {
                    // End of data
                    break;
                } else if result == -2 {
                    // CursedError
                    return Err(PostgreSQLError::query_error("CursedError reading COPY data"));
                } else if result > 0 {
                    // Got data
                    let data_slice = std::slice::from_raw_parts(buffer as *const u8, result as usize);
                    buf_writer.write_all(data_slice)
                        .map_err(|e| PostgreSQLError::query_error(&format!("Failed to write data: {}", e)))?;
                    bytes_copied += result as u64;
                    
                    // Free the buffer
                    super::ffi::PQfreemem(buffer as *mut std::ffi::c_void);
                }
            }
        }
        
        buf_writer.flush()
            .map_err(|e| PostgreSQLError::query_error(&format!("Failed to flush writer: {}", e)))?;
        
        Ok(bytes_copied)
    }
    
    /// slay Copy data from SQL query to writer
    pub fn copy_query_out<W: Write>(
        &self,
        query: &str,
        writer: W,
        options: CopyOptions,
    ) -> crate::error::Result<()> {
        let copy_sql = format!("COPY ({}) TO STDOUT {}", query, options.to_pg_options());
        
        let conn = self.conn.lock().map_err(|_| {
            PostgreSQLError::connection_error("Failed to acquire connection lock")
        })?;
        
        // Start COPY operation
        let result = conn.exec(&copy_sql)
            .map_err(|e| PostgreSQLError::query_error(&format!("Failed to start COPY query: {}", e)))?;
        
        // Read data (same as copy_out)
        let mut buf_writer = BufWriter::new(writer);
        let mut bytes_copied = 0u64;
        
        loop {
            unsafe {
                use super::ffi::PQgetCopyData;
                
                let mut buffer: *mut i8 = std::ptr::null_mut();
                let result = PQgetCopyData(conn.as_ptr(), &mut buffer, 0);
                
                if result == -1 {
                    break;
                } else if result == -2 {
                    return Err(PostgreSQLError::query_error("CursedError reading COPY data"));
                } else if result > 0 {
                    let data_slice = std::slice::from_raw_parts(buffer as *const u8, result as usize);
                    buf_writer.write_all(data_slice)
                        .map_err(|e| PostgreSQLError::query_error(&format!("Failed to write data: {}", e)))?;
                    bytes_copied += result as u64;
                    
                    super::ffi::PQfreemem(buffer as *mut std::ffi::c_void);
                }
            }
        }
        
        buf_writer.flush()
            .map_err(|e| PostgreSQLError::query_error(&format!("Failed to flush writer: {}", e)))?;
        
        Ok(bytes_copied)
    }
    
    /// slay Copy structured data (Vec<Vec<SqlValue>>) to table
    pub fn copy_data_in(
        &self,
        table: &str,
        data: Vec<Vec<SqlValue>>,
        options: CopyOptions,
    ) -> crate::error::Result<()> {
        // Convert structured data to format suitable for COPY
        let formatted_data = self.format_data_for_copy(data, &options)?;
        let cursor = std::io::Cursor::new(formatted_data);
        
        self.copy_in(table, cursor, options)
    }
    
    /// slay Build COPY IN SQL statement
    fn build_copy_in_sql(&self, table: &str, options: &CopyOptions) -> String {
        let columns_part = if options.columns.is_empty() {
            String::new()
        } else {
            format!(" ({})", options.columns.join(", "))
        };
        
        format!("COPY {}{} FROM STDIN {}", table, columns_part, options.to_pg_options())
    }
    
    /// slay Build COPY OUT SQL statement
    fn build_copy_out_sql(&self, table: &str, options: &CopyOptions) -> String {
        let columns_part = if options.columns.is_empty() {
            String::new()
        } else {
            format!(" ({})", options.columns.join(", "))
        };
        
        format!("COPY {}{} TO STDOUT {}", table, columns_part, options.to_pg_options())
    }
    
    /// slay Copy text data to PostgreSQL
    fn copy_text_data<R: BufRead>(
        &self,
        conn: &SafePGconn,
        reader: &mut R,
        bytes_copied: &mut u64,
    ) -> crate::error::Result<()> {
        let mut line = String::new();
        
        loop {
            line.clear();
            let bytes_read = reader.read_line(&mut line)
                .map_err(|e| PostgreSQLError::query_error(&format!("Failed to read line: {}", e)))?;
            
            if bytes_read == 0 {
                break; // EOF
            }
            
            // Send line to PostgreSQL
            unsafe {
                use super::ffi::PQputCopyData;
                use std::ffi::CString;
                
                let c_line = CString::new(line.clone())
                    .map_err(|_| PostgreSQLError::query_error("Invalid line data"))?;
                
                let result = PQputCopyData(conn.as_ptr(), c_line.as_ptr(), line.len() as i32);
                if result != 1 {
                    return Err(PostgreSQLError::query_error("Failed to send COPY data"));
                }
            }
            
            *bytes_copied += bytes_read as u64;
        }
        
        Ok(())
    }
    
    /// slay Copy binary data to PostgreSQL
    fn copy_binary_data<R: Read>(
        &self,
        conn: &SafePGconn,
        reader: &mut R,
        bytes_copied: &mut u64,
    ) -> crate::error::Result<()> {
        let mut buffer = [0u8; 8192]; // 8KB buffer
        
        loop {
            let bytes_read = reader.read(&mut buffer)
                .map_err(|e| PostgreSQLError::query_error(&format!("Failed to read data: {}", e)))?;
            
            if bytes_read == 0 {
                break; // EOF
            }
            
            // Send data to PostgreSQL
            unsafe {
                use super::ffi::PQputCopyData;
use crate::error::CursedError;
                
                let result = PQputCopyData(
                    conn.as_ptr(),
                    buffer.as_ptr() as *const i8,
                    bytes_read as i32
                );
                
                if result != 1 {
                    return Err(PostgreSQLError::query_error("Failed to send binary COPY data"));
                }
            }
            
            *bytes_copied += bytes_read as u64;
        }
        
        Ok(())
    }
    
    /// slay Format structured data for COPY operation
    fn format_data_for_copy(
        &self,
        data: Vec<Vec<SqlValue>>,
        options: &CopyOptions,
    ) -> crate::error::Result<()> {
        let mut result = Vec::new();
        
        match options.format {
            CopyFormat::Text => {
                let delimiter = options.delimiter.as_deref().unwrap_or("\t");
                let null_str = options.null_string.as_deref().unwrap_or("\\N");
                
                for row in data {
                    let formatted_row = row.iter()
                        .map(|value| self.format_value_for_text(value, null_str))
                        .collect::<Result<Vec<_>, _>>()?
                        .join(delimiter);
                    
                    result.extend_from_slice(formatted_row.as_bytes());
                    result.push(b'\n');
                }
            }
            CopyFormat::Csv => {
                let delimiter = options.delimiter.as_deref().unwrap_or(",");
                let null_str = options.null_string.as_deref().unwrap_or("");
                let quote_char = options.quote.unwrap_or('"');
                
                for row in data {
                    let formatted_row = row.iter()
                        .map(|value| self.format_value_for_csv(value, null_str, quote_char))
                        .collect::<Result<Vec<_>, _>>()?
                        .join(delimiter);
                    
                    result.extend_from_slice(formatted_row.as_bytes());
                    result.push(b'\n');
                }
            }
            CopyFormat::Binary => {
                // Binary format is more complex and requires proper binary encoding
                return Err(PostgreSQLError::query_error("Binary format not implemented for structured data"));
            }
        }
        
        Ok(result)
    }
    
    /// slay Format SqlValue for text COPY format
    fn format_value_for_text(&self, value: &SqlValue, null_str: &str) -> crate::error::Result<()> {
        match value {
            SqlValue::Null => Ok(null_str.to_string()),
            SqlValue::Boolean(b) => Ok(if *b { "t".to_string() } else { "f".to_string() }),
            SqlValue::Integer(i) => Ok(i.to_string()),
            SqlValue::Float(f) => Ok(f.to_string()),
            SqlValue::String(s) => {
                // Escape special characters
                let escaped = s.replace("\\", "\\\\")
                    .replace("\t", "\\t")
                    .replace("\n", "\\n")
                    .replace("\r", "\\r");
                Ok(escaped)
            }
            SqlValue::Bytes(b) => {
                // Convert to hex format
                let hex = b.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
                Ok(format!("\\x{}", hex))
            }
            SqlValue::Json(j) => Ok(j.to_string()),
            SqlValue::Timestamp(t) => {
                let duration = t.duration_since(std::time::UNIX_EPOCH)
                    .map_err(|_| PostgreSQLError::query_error("Invalid timestamp"))?;
                Ok(duration.as_secs().to_string())
            }
        }
    }
    
    /// slay Format SqlValue for CSV COPY format
    fn format_value_for_csv(&self, value: &SqlValue, null_str: &str, quote_char: char) -> crate::error::Result<()> {
        match value {
            SqlValue::Null => Ok(null_str.to_string()),
            SqlValue::Boolean(b) => Ok(if *b { "true".to_string() } else { "false".to_string() }),
            SqlValue::Integer(i) => Ok(i.to_string()),
            SqlValue::Float(f) => Ok(f.to_string()),
            SqlValue::String(s) => {
                // Quote and escape for CSV
                if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
                    let escaped = s.replace(&quote_char.to_string(), &format!("{}{}", quote_char, quote_char));
                    Ok(format!("{}{}{}", quote_char, escaped, quote_char))
                } else {
                    Ok(s.clone())
                }
            }
            SqlValue::Bytes(b) => {
                let hex = b.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
                Ok(format!("\\x{}", hex))
            }
            SqlValue::Json(j) => {
                let json_str = j.to_string();
                if json_str.contains(',') || json_str.contains('"') {
                    let escaped = json_str.replace(&quote_char.to_string(), &format!("{}{}", quote_char, quote_char));
                    Ok(format!("{}{}{}", quote_char, escaped, quote_char))
                } else {
                    Ok(json_str)
                }
            }
            SqlValue::Timestamp(t) => {
                let duration = t.duration_since(std::time::UNIX_EPOCH)
                    .map_err(|_| PostgreSQLError::query_error("Invalid timestamp"))?;
                Ok(duration.as_secs().to_string())
            }
        }
    }
}

/// fr fr Bulk operations helper for common COPY scenarios
pub struct BulkOperations {
    copy_manager: CopyManager,
}

impl BulkOperations {
    /// slay Create a new bulk operations helper
    pub fn new(conn: Arc<Mutex<SafePGconn>>) -> Self {
        Self {
            copy_manager: CopyManager::new(conn),
        }
    }
    
    /// slay Bulk insert from CSV file
    pub fn bulk_insert_csv<R: Read>(
        &self,
        table: &str,
        reader: R,
        has_header: bool,
        delimiter: Option<String>,
    ) -> crate::error::Result<()> {
        let options = CopyOptions::csv()
            .delimiter(delimiter.unwrap_or_else(|| ",".to_string()));
        
        let options = if has_header {
            options.with_header()
        } else {
            options
        };
        
        self.copy_manager.copy_in(table, reader, options)
    }
    
    /// slay Bulk export to CSV file
    pub fn bulk_export_csv<W: Write>(
        &self,
        table: &str,
        writer: W,
        include_header: bool,
        delimiter: Option<String>,
    ) -> crate::error::Result<()> {
        let options = CopyOptions::csv()
            .delimiter(delimiter.unwrap_or_else(|| ",".to_string()));
        
        let options = if include_header {
            options.with_header()
        } else {
            options
        };
        
        self.copy_manager.copy_out(table, writer, options)
    }
    
    /// slay Bulk insert structured data
    pub fn bulk_insert_data(
        &self,
        table: &str,
        data: Vec<Vec<SqlValue>>,
        columns: Option<Vec<String>>,
    ) -> crate::error::Result<()> {
        let options = if let Some(cols) = columns {
            CopyOptions::text().columns(cols)
        } else {
            CopyOptions::text()
        };
        
        self.copy_manager.copy_data_in(table, data, options)
    }
    
    /// slay Bulk export query results
    pub fn bulk_export_query<W: Write>(
        &self,
        query: &str,
        writer: W,
        format: CopyFormat,
    ) -> crate::error::Result<()> {
        let options = match format {
            CopyFormat::Text => CopyOptions::text(),
            CopyFormat::Csv => CopyOptions::csv().with_header(),
            CopyFormat::Binary => CopyOptions::binary(),
        };
        
        self.copy_manager.copy_query_out(query, writer, options)
    }
}

/// fr fr COPY statistics for monitoring bulk operations
#[derive(Debug, Clone, Default)]
pub struct CopyStats {
    /// fr fr Total bytes copied
    pub bytes_copied: u64,
    /// fr fr Total rows processed
    pub rows_processed: u64,
    /// fr fr Operation duration
    pub duration: std::time::Duration,
    /// fr fr Average throughput (bytes/sec)
    pub throughput_bps: f64,
    /// fr fr Average rows per second
    pub rows_per_second: f64,
}

impl CopyStats {
    /// slay Create new stats
    pub fn new(bytes: u64, rows: u64, duration: std::time::Duration) -> Self {
        let duration_secs = duration.as_secs_f64();
        let throughput_bps = if duration_secs > 0.0 { bytes as f64 / duration_secs } else { 0.0 };
        let rows_per_second = if duration_secs > 0.0 { rows as f64 / duration_secs } else { 0.0 };
        
        Self {
            bytes_copied: bytes,
            rows_processed: rows,
            duration,
            throughput_bps,
            rows_per_second,
        }
    }
    
    /// slay Get throughput in MB/s
    pub fn throughput_mbps(&self) -> f64 {
        self.throughput_bps / (1024.0 * 1024.0)
    }
}
