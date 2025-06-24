use crate::error::Error;
/// CSV Streamer for processing large files with minimal memory usage
use std::io;
use crate::stdlib::csv::reader::Reader;
use crate::stdlib::csv::error::{CsvError, CsvResult};

/// Function type for processing records in streaming mode
pub type StreamProcessor = Box<dyn FnMut(&[String], &[String]) -> CsvResult<()>>;

/// CSV Streamer that processes records one at a time with minimal memory usage
pub struct Streamer<R: io::Read> {
    /// Underlying CSV reader
    reader: Reader<R>,
    
    /// Header row (cached after first read)
    header: Option<Vec<String>>,
    
    /// Batch size for processing
    batch_size: usize,
    
    /// Whether to include header in processing
    include_header: bool,
    
    /// Total records processed
    records_processed: usize,
    
    /// Error state
    error: Option<CsvError>,
}

impl<R: io::Read> Streamer<R> {
    /// Create a new streamer with default configuration
    pub fn new(reader: R) -> Self {
        Self {
            reader: Reader::new(reader),
            header: None,
            batch_size: 1000,
            include_header: false,
            records_processed: 0,
            error: None,
        }
    }
    
    /// Create a new streamer with custom CSV reader
    pub fn with_reader(reader: Reader<R>) -> Self {
        Self {
            reader,
            header: None,
            batch_size: 1000,
            include_header: false,
            records_processed: 0,
            error: None,
        }
    }
    
    /// Set the batch size for processing
    pub fn batch_size(mut self, size: usize) -> Self {
        self.batch_size = size.max(1); // Ensure minimum batch size of 1
        self
    }
    
    /// Set whether to include header in processing
    pub fn include_header(mut self, include: bool) -> Self {
        self.include_header = include;
        self
    }
    
    /// Configure the underlying reader
    pub fn comma(mut self, c: char) -> Self {
        self.reader = self.reader.comma(c);
        self
    }
    
    pub fn comment(mut self, c: char) -> Self {
        self.reader = self.reader.comment(c);
        self
    }
    
    pub fn trim_leading_space(mut self, enable: bool) -> Self {
        self.reader = self.reader.trim_leading_space(enable);
        self
    }
    
    /// Process all records using the provided processor function
    pub fn process<F>(&mut self, mut processor: F) -> CsvResult<usize>
    where
        F: FnMut(&[String], &[String]) -> CsvResult<()>,
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
                },
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
        }
        
        Ok(self.records_processed)
    }
    
    /// Process records in batches for better performance
    pub fn process_batched<F>(&mut self, mut processor: F) -> CsvResult<usize>
    where
        F: FnMut(&[Vec<String>], &[String]) -> CsvResult<()>,
    {
        // Read header if not already read
        if self.header.is_none() {
            match self.reader.read()? {
                Some(header) => {
                    self.header = Some(header);
                },
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
        }
        
        Ok(self.records_processed)
    }
    
    /// Stream records and collect them into a vector (for compatibility)
    pub fn collect(&mut self) -> CsvResult<Vec<Vec<String>>> {
        let mut records = Vec::new();
        
        self.process(|record, _header| {
            records.push(record.to_vec());
            Ok(())
        })?;
        
        Ok(records)
    }
    
    /// Stream records and apply a transformation function
    pub fn map<F, T>(&mut self, mut mapper: F) -> CsvResult<Vec<T>>
    where
        F: FnMut(&[String], &[String]) -> CsvResult<T>,
    {
        let mut results = Vec::new();
        
        self.process(|record, header| {
            let result = mapper(record, header)?;
            results.push(result);
            Ok(())
        })?;
        
        Ok(results)
    }
    
    /// Stream records and filter them based on a predicate
    pub fn filter<F>(&mut self, mut predicate: F) -> CsvResult<Vec<Vec<String>>>
    where
        F: FnMut(&[String], &[String]) -> CsvResult<bool>,
    {
        let mut filtered_records = Vec::new();
        
        self.process(|record, header| {
            if predicate(record, header)? {
                filtered_records.push(record.to_vec());
            }
            Ok(())
        })?;
        
        Ok(filtered_records)
    }
    
    /// Count records without storing them in memory
    pub fn count(&mut self) -> CsvResult<usize> {
        let mut count = 0;
        
        self.process(|_record, _header| {
            count += 1;
            Ok(())
        })?;
        
        Ok(count)
    }
    
    /// Get statistics about the streaming process
    pub fn statistics(&self) -> StreamingStats {
        StreamingStats {
            records_processed: self.records_processed,
            has_header: self.header.is_some(),
            header_columns: self.header.as_ref().map(|h| h.len()).unwrap_or(0),
            batch_size: self.batch_size,
            error_occurred: self.error.is_some(),
        }
    }
    
    /// Get the header if available
    pub fn header(&self) -> Option<&Vec<String>> {
        self.header.as_ref()
    }
    
    /// Get any error that occurred
    pub fn error(&self) -> Option<&CsvError> {
        self.error.as_ref()
    }
    
    /// Get the number of records processed so far
    pub fn records_processed(&self) -> usize {
        self.records_processed
    }
    
    /// Get the current batch size
    pub fn get_batch_size(&self) -> usize {
        self.batch_size
    }
    
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
    pub records_processed: usize,
    
    /// Whether a header was found
    pub has_header: bool,
    
    /// Number of columns in header
    pub header_columns: usize,
    
    /// Batch size used for processing
    pub batch_size: usize,
    
    /// Whether an error occurred
    pub error_occurred: bool,
}

impl StreamingStats {
    /// Get a summary string of the statistics
    pub fn summary(&self) -> String {
        format!(
            "Processed {} records, {} columns, batch size {}, header: {}, errors: {}",
            self.records_processed,
            self.header_columns,
            self.batch_size,
            self.has_header,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_basic_streaming() {
        let csv_data = "name,age,city\nAlice,30,New York\nBob,25,San Francisco\nCharlie,35,Chicago";
        let cursor = Cursor::new(csv_data);
        let mut streamer = Streamer::new(cursor);
        
        let mut processed_records = Vec::new();
        let count = streamer.process(|record, header| {
            assert_eq!(header, &["name", "age", "city"]);
            processed_records.push(record.to_vec());
            Ok(())
        }).unwrap();
        
        assert_eq!(count, 3); // 3 data records (header not included by default)
        assert_eq!(processed_records.len(), 3);
        assert_eq!(processed_records[0], vec!["Alice", "30", "New York"]);
        assert_eq!(processed_records[1], vec!["Bob", "25", "San Francisco"]);
        assert_eq!(processed_records[2], vec!["Charlie", "35", "Chicago"]);
    }

    #[test]
    fn test_include_header() {
        let csv_data = "name,age\nAlice,30\nBob,25";
        let cursor = Cursor::new(csv_data);
        let mut streamer = Streamer::new(cursor).include_header(true);
        
        let mut all_records = Vec::new();
        let count = streamer.process(|record, _header| {
            all_records.push(record.to_vec());
            Ok(())
        }).unwrap();
        
        assert_eq!(count, 3); // header + 2 data records
        assert_eq!(all_records[0], vec!["name", "age"]); // Header included
        assert_eq!(all_records[1], vec!["Alice", "30"]);
        assert_eq!(all_records[2], vec!["Bob", "25"]);
    }

    #[test]
    fn test_batched_processing() {
        let csv_data = "name,age\nAlice,30\nBob,25\nCharlie,35\nDave,40\nEve,28";
        let cursor = Cursor::new(csv_data);
        let mut streamer = Streamer::new(cursor).batch_size(2);
        
        let mut batch_sizes = Vec::new();
        let count = streamer.process_batched(|batch, header| {
            assert_eq!(header, &["name", "age"]);
            batch_sizes.push(batch.len());
            Ok(())
        }).unwrap();
        
        assert_eq!(count, 5); // 5 data records
        assert_eq!(batch_sizes, vec![2, 2, 1]); // 2 full batches + 1 partial batch
    }

    #[test]
    fn test_collect() {
        let csv_data = "name,age\nAlice,30\nBob,25";
        let cursor = Cursor::new(csv_data);
        let mut streamer = Streamer::new(cursor);
        
        let records = streamer.collect().unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0], vec!["Alice", "30"]);
        assert_eq!(records[1], vec!["Bob", "25"]);
    }

    #[test]
    fn test_map_transformation() {
        let csv_data = "name,age\nAlice,30\nBob,25";
        let cursor = Cursor::new(csv_data);
        let mut streamer = Streamer::new(cursor);
        
        let names: Vec<String> = streamer.map(|record, _header| {
            Ok(record[0].clone()) // Extract just the name
        }).unwrap();
        
        assert_eq!(names, vec!["Alice", "Bob"]);
    }

    #[test]
    fn test_filter() {
        let csv_data = "name,age\nAlice,30\nBob,25\nCharlie,35";
        let cursor = Cursor::new(csv_data);
        let mut streamer = Streamer::new(cursor);
        
        let filtered = streamer.filter(|record, _header| {
            let age: i32 = record[1].parse().unwrap_or(0);
            Ok(age >= 30) // Only people 30 or older
        }).unwrap();
        
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0], vec!["Alice", "30"]);
        assert_eq!(filtered[1], vec!["Charlie", "35"]);
    }

    #[test]
    fn test_count() {
        let csv_data = "name,age\nAlice,30\nBob,25\nCharlie,35";
        let cursor = Cursor::new(csv_data);
        let mut streamer = Streamer::new(cursor);
        
        let count = streamer.count().unwrap();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_statistics() {
        let csv_data = "name,age,city\nAlice,30,New York\nBob,25,San Francisco";
        let cursor = Cursor::new(csv_data);
        let mut streamer = Streamer::new(cursor).batch_size(10);
        
        streamer.collect().unwrap();
        
        let stats = streamer.statistics();
        assert_eq!(stats.records_processed, 2);
        assert_eq!(stats.has_header, true);
        assert_eq!(stats.header_columns, 3);
        assert_eq!(stats.batch_size, 10);
        assert_eq!(stats.error_occurred, false);
        
        let summary = stats.summary();
        assert!(summary.contains("Processed 2 records"));
        assert!(summary.contains("3 columns"));
    }

    #[test]
    fn test_custom_delimiter() {
        let tsv_data = "name\tage\nAlice\t30\nBob\t25";
        let cursor = Cursor::new(tsv_data);
        let mut streamer = Streamer::new(cursor).comma('\t');
        
        let records = streamer.collect().unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0], vec!["Alice", "30"]);
        assert_eq!(records[1], vec!["Bob", "25"]);
    }

    #[test]
    fn test_error_handling() {
        let csv_data = "name,age\nAlice,30\nBob,25";
        let cursor = Cursor::new(csv_data);
        let mut streamer = Streamer::new(cursor);
        
        // Processor that fails on second record
        let result = streamer.process(|record, _header| {
            if record[0] == "Bob" {
                return Err(CsvError::General("test error".to_string()));
            }
            Ok(())
        });
        
        assert!(result.is_err());
        assert!(streamer.error().is_some());
        assert_eq!(streamer.records_processed(), 1); // Only Alice was processed
    }

    #[test]
    fn test_empty_file() {
        let cursor = Cursor::new("");
        let mut streamer = Streamer::new(cursor);
        
        let count = streamer.process(|_record, _header| {
            panic!("Should not be called for empty file");
        }).unwrap();
        
        assert_eq!(count, 0);
        assert!(streamer.header().is_none());
    }

    #[test]
    fn test_header_only_file() {
        let csv_data = "name,age,city";
        let cursor = Cursor::new(csv_data);
        let mut streamer = Streamer::new(cursor);
        
        let count = streamer.process(|_record, _header| {
            panic!("Should not be called for header-only file");
        }).unwrap();
        
        assert_eq!(count, 0);
        assert!(streamer.header().is_some());
        assert_eq!(streamer.header().unwrap(), &["name", "age", "city"]);
    }

    #[test]
    fn test_configuration_methods() {
        let cursor = Cursor::new("a,b\n1,2");
        let streamer = Streamer::new(cursor)
            .batch_size(500)
            .include_header(true)
            .comma(';')
            .trim_leading_space(true);
        
        assert_eq!(streamer.batch_size, 500);
        assert_eq!(streamer.include_header, true);
    }
}
