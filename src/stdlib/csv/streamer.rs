use crate::error::CursedError;
/// CSV Streamer for processing large files with minimal memory usage
use std::io;
// use crate::stdlib::csv::reader::Reader;
// use crate::stdlib::csv::error::{CsvError, CsvResult};

/// Function type for processing records in streaming mode
pub type StreamProcessor = Box<dyn FnMut(&[String], &[String]) -> CsvResult<()>>;

/// CSV Streamer that processes records one at a time with minimal memory usage
pub struct Streamer<R: io::Read> {
    /// Underlying CSV reader
    
    /// Header row (cached after first read)
    
    /// Batch size for processing
    
    /// Whether to include header in processing
    
    /// Total records processed
    
    /// CursedError state
impl<R: io::Read> Streamer<R> {
    /// Create a new streamer with default configuration
    pub fn new(reader: R) -> Self {
        Self {
        }
    }
    
    /// Create a new streamer with custom CSV reader
    pub fn with_reader(reader: Reader<R>) -> Self {
        Self {
        }
    }
    
    /// Set the batch size for processing
    pub fn batch_size(mut self, size: usize) -> Self {
        self.batch_size = size.max(1); // Ensure minimum batch size of 1
        self
    /// Set whether to include header in processing
    pub fn include_header(mut self, include: bool) -> Self {
        self.include_header = include;
        self
    /// Configure the underlying reader
    pub fn comma(mut self, c: char) -> Self {
        self.reader = self.reader.comma(c);
        self
    pub fn comment(mut self, c: char) -> Self {
        self.reader = self.reader.comment(c);
        self
    pub fn trim_leading_space(mut self, enable: bool) -> Self {
        self.reader = self.reader.trim_leading_space(enable);
        self
    /// Process all records using the provided processor function
    pub fn process<F>(&mut self, mut processor: F) -> CsvResult<usize>
    where
    {
        // Read header if not already read
        if self.header.is_none() {
            match self.reader.read()? {
                Some(header) => {
                    self.header = Some(header.clone());
                    
                    // Process header if requested
                    if self.include_header {
                        let empty_header: Vec<String> = Vec::new();
                        if let Err(e) = processor(&header, &empty_header) {
                            self.error = Some(e.clone());
                            return Err(e);
                        }
                        self.records_processed += 1;
                    }
                None => return Ok(0), // Empty file
            }
        }
        
        let header = self.header.as_ref().unwrap();
        
        // Process records one by one
        while let Some(record) = self.reader.read()? {
            if let Err(e) = processor(&record, header) {
                self.error = Some(e.clone());
                return Err(e);
            }
            self.records_processed += 1;
        Ok(self.records_processed)
    /// Process records in batches for better performance
    pub fn process_batched<F>(&mut self, mut processor: F) -> CsvResult<usize>
    where
    {
        // Read header if not already read
        if self.header.is_none() {
            match self.reader.read()? {
                Some(header) => {
                    self.header = Some(header);
                None => return Ok(0), // Empty file
            }
        }
        
        let header = self.header.as_ref().unwrap();
        let mut batch = Vec::with_capacity(self.batch_size);
        
        while let Some(record) = self.reader.read()? {
            batch.push(record);
            
            if batch.len() >= self.batch_size {
                if let Err(e) = processor(&batch, header) {
                    self.error = Some(e.clone());
                    return Err(e);
                }
                self.records_processed += batch.len();
                batch.clear();
            }
        }
        
        // Process remaining records in the last batch
        if !batch.is_empty() {
            if let Err(e) = processor(&batch, header) {
                self.error = Some(e.clone());
                return Err(e);
            }
            self.records_processed += batch.len();
        Ok(self.records_processed)
    /// Stream records and collect them into a vector (for compatibility)
    pub fn collect(&mut self) -> CsvResult<Vec<Vec<String>>> {
        let mut records = Vec::new();
        
        self.process(|record, _header| {
            records.push(record.to_vec());
            Ok(())
        })?;
        
        Ok(records)
    /// Stream records and apply a transformation function
    pub fn map<F, T>(&mut self, mut mapper: F) -> CsvResult<Vec<T>>
    where
    {
        let mut results = Vec::new();
        
        self.process(|record, header| {
            let result = mapper(record, header)?;
            results.push(result);
            Ok(())
        })?;
        
        Ok(results)
    /// Stream records and filter them based on a predicate
    pub fn filter<F>(&mut self, mut predicate: F) -> CsvResult<Vec<Vec<String>>>
    where
    {
        let mut filtered_records = Vec::new();
        
        self.process(|record, header| {
            if predicate(record, header)? {
                filtered_records.push(record.to_vec());
            }
            Ok(())
        })?;
        
        Ok(filtered_records)
    /// Count records without storing them in memory
    pub fn count(&mut self) -> CsvResult<usize> {
        let mut count = 0;
        
        self.process(|_record, _header| {
            count += 1;
            Ok(())
        })?;
        
        Ok(count)
    /// Get statistics about the streaming process
    pub fn statistics(&self) -> StreamingStats {
        StreamingStats {
        }
    }
    
    /// Get the header if available
    pub fn header(&self) -> Option<&Vec<String>> {
        self.header.as_ref()
    /// Get any error that occurred
    pub fn error(&self) -> Option<&CsvError> {
        self.error.as_ref()
    /// Get the number of records processed so far
    pub fn records_processed(&self) -> usize {
        self.records_processed
    /// Get the current batch size
    pub fn get_batch_size(&self) -> usize {
        self.batch_size
    /// Reset the streamer for reuse (if underlying reader supports it)
    pub fn reset(&mut self) {
        self.header = None;
        self.records_processed = 0;
        self.error = None;
    }
}

/// Statistics about the streaming process
#[derive(Debug, Clone)]
pub struct StreamingStats {
    /// Number of records processed
    
    /// Whether a header was found
    
    /// Number of columns in header
    
    /// Batch size used for processing
    
    /// Whether an error occurred
impl StreamingStats {
    /// Get a summary string of the statistics
    pub fn summary(&self) -> String {
        format!(
            self.error_occurred
        )
    }
}

// Convenience methods for compatibility with spec
impl<R: io::Read> Streamer<R> {
    /// Get the batch size (compatibility method)
    pub fn batch_size(&self) -> usize {
        self.batch_size
    }
}

