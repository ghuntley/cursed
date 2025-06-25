use crate::error::CursedError;
/// CSV data transformation with column mapping and computed fields
use std::io;
use std::collections::HashMap;
// use crate::stdlib::csv::reader::Reader;
// use crate::stdlib::csv::error::{CsvError, CsvResult};

/// Function type for column transformation
pub type ColumnTransformFn = Box<dyn Fn(&str) -> CsvResult<String>>;

/// Function type for computed column generation
pub type ComputedColumnFn = Box<dyn Fn(&HashMap<String, String>) -> CsvResult<String>>;

/// Column transformation definition
#[derive(Debug)]
pub enum ColumnTransform {
    /// Transform an existing column's values
    Map {
    
    /// Add a new computed column
    Add {
    
    /// Remove a column
    Remove {
    
    /// Rename a column
    Rename {
    
    /// Reorder columns
    Reorder {
    
    /// Filter rows based on a condition
    FilterRows {
/// CSV data transformer
pub struct Transformer<R: io::Read> {
    /// Underlying CSV reader
    
    /// List of transformations to apply
    
    /// Whether header has been read
    
    /// Original header
    
    /// Transformed header
    
    /// Column name to index mapping for original data
impl<R: io::Read> Transformer<R> {
    /// Create a new transformer
    pub fn new(reader: R) -> Self {
        Self {
        }
    }
    
    /// Create a new transformer with custom CSV reader
    pub fn with_reader(reader: Reader<R>) -> Self {
        Self {
        }
    }
    
    /// Map an existing column's values using a transformation function
    pub fn map_column<F>(&mut self, column: &str, transform: F) -> &mut Self
    where
    {
        self.transforms.push(ColumnTransform::Map {
        });
        self
    /// Add a new computed column
    pub fn add_column<F>(&mut self, column: &str, compute: F) -> &mut Self
    where
    {
        self.transforms.push(ColumnTransform::Add {
        });
        self
    /// Remove a column
    pub fn remove_column(&mut self, column: &str) -> &mut Self {
        self.transforms.push(ColumnTransform::Remove {
        });
        self
    /// Rename a column
    pub fn rename_column(&mut self, from: &str, to: &str) -> &mut Self {
        self.transforms.push(ColumnTransform::Rename {
        });
        self
    /// Reorder columns
    pub fn reorder_columns(&mut self, new_order: Vec<String>) -> &mut Self {
        self.transforms.push(ColumnTransform::Reorder {
        });
        self
    /// Filter rows based on a condition
    pub fn filter_rows<F>(&mut self, condition: F) -> &mut Self
    where
    {
        self.transforms.push(ColumnTransform::FilterRows {
        });
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
    /// Apply all transformations and return the transformed data
    pub fn transform(&mut self) -> CsvResult<Vec<Vec<String>>> {
        // Read header
        if !self.header_read {
            self.read_header()?;
        let mut results = Vec::new();
        results.push(self.transformed_header.clone());
        
        // Process each record
        while let Some(record) = self.reader.read()? {
            if let Some(transformed_record) = self.transform_record(&record)? {
                results.push(transformed_record);
            }
        }
        
        Ok(results)
    /// Transform data and write to a new CSV format
    pub fn transform_to_string(&mut self) -> CsvResult<String> {
//         use crate::stdlib::csv::writer::write_to_string;
        let transformed = self.transform()?;
        write_to_string(&transformed)
    /// Get the list of transformations
    pub fn transforms(&self) -> &[ColumnTransform] {
        &self.transforms
    /// Get the original header
    pub fn original_header(&self) -> &[String] {
        &self.original_header
    /// Get the transformed header
    pub fn transformed_header(&self) -> &[String] {
        &self.transformed_header
    /// Read and process the header
    fn read_header(&mut self) -> CsvResult<()> {
        if let Some(header) = self.reader.read()? {
            self.original_header = header.clone();
            self.transformed_header = header.clone();
            
            // Build column map
            for (index, column_name) in self.original_header.iter().enumerate() {
                self.column_map.insert(column_name.clone(), index);
            // Apply header transformations
            self.apply_header_transforms()?;
            self.header_read = true;
        } else {
            return Err(CsvError::General("No header found in CSV".to_string()));
        Ok(())
    /// Apply transformations that affect the header
    fn apply_header_transforms(&mut self) -> CsvResult<()> {
        for transform in &self.transforms {
            match transform {
                ColumnTransform::Add { column, .. } => {
                    // Add new column to header
                    if !self.transformed_header.contains(column) {
                        self.transformed_header.push(column.clone());
                    }
                ColumnTransform::Remove { column } => {
                    // Remove column from header
                    self.transformed_header.retain(|c| c != column);
                ColumnTransform::Rename { from, to } => {
                    // Rename column in header
                    for col in &mut self.transformed_header {
                        if col == from {
                            *col = to.clone();
                            break;
                        }
                    }
                ColumnTransform::Reorder { new_order } => {
                    // Reorder header columns
                    let mut new_header = Vec::new();
                    for col_name in new_order {
                        if self.transformed_header.contains(col_name) {
                            new_header.push(col_name.clone());
                        }
                    }
                    // Add any columns not in the new order
                    for col in &self.transformed_header {
                        if !new_header.contains(col) {
                            new_header.push(col.clone());
                        }
                    }
                    self.transformed_header = new_header;
                _ => {} // Other transforms don't affect header
            }
        }
        
        Ok(())
    /// Transform a single record
    fn transform_record(&self, record: &[String]) -> CsvResult<Option<Vec<String>>> {
        // Build record map
        let mut record_map = HashMap::new();
        for (column_name, &index) in &self.column_map {
            let value = if index < record.len() {
                record[index].clone()
            } else {
                String::new()
            record_map.insert(column_name.clone(), value);
        // Apply transformations
        let mut should_include_row = true;
        
        for transform in &self.transforms {
            match transform {
                ColumnTransform::Map { column, transform } => {
                    if let Some(value) = record_map.get(column) {
                        let transformed_value = transform(value)?;
                        record_map.insert(column.clone(), transformed_value);
                    }
                ColumnTransform::Add { column, compute } => {
                    let computed_value = compute(&record_map)?;
                    record_map.insert(column.clone(), computed_value);
                ColumnTransform::Remove { column } => {
                    record_map.remove(column);
                ColumnTransform::Rename { from, to } => {
                    if let Some(value) = record_map.remove(from) {
                        record_map.insert(to.clone(), value);
                    }
                ColumnTransform::FilterRows { condition } => {
                    if !condition(&record_map)? {
                        should_include_row = false;
                    }
                ColumnTransform::Reorder { .. } => {
                    // Reordering is handled when building the output record
            }
        }
        
        if !should_include_row {
            return Ok(None);
        // Build output record based on transformed header
        let mut output_record = Vec::new();
        for column_name in &self.transformed_header {
            let value = record_map.get(column_name).cloned().unwrap_or_else(String::new);
            output_record.push(value);
        Ok(Some(output_record))
    }
}

/// Result of transformation
#[derive(Debug, Clone)]
pub struct TransformResult {
    /// Transformed records (including header)
    
    /// Number of input records processed
    
    /// Number of output records generated
    
    /// Number of columns in input
    
    /// Number of columns in output
impl TransformResult {
    /// Get a summary of the transformation
    pub fn summary(&self) -> String {
        format!(
            self.output_columns
        )
    }
}

/// Common transformation functions
pub mod transforms {
    use super::*;
    
    /// Convert string to uppercase
    pub fn to_uppercase(value: &str) -> CsvResult<String> {
        Ok(value.to_uppercase())
    /// Convert string to lowercase
    pub fn to_lowercase(value: &str) -> CsvResult<String> {
        Ok(value.to_lowercase())
    /// Trim whitespace
    pub fn trim(value: &str) -> CsvResult<String> {
        Ok(value.trim().to_string())
    /// Replace characters
    pub fn replace<'a>(from: &'a str, to: &'a str) -> impl Fn(&str) -> CsvResult<String> + 'a {
        move |value: &str| Ok(value.replace(from, to))
    /// Parse and format numbers
    pub fn format_number(decimals: usize) -> impl Fn(&str) -> CsvResult<String> {
        move |value: &str| {
            if let Ok(num) = value.parse::<f64>() {
                Ok(format!("{:.1$}", num, decimals))
            } else {
                Ok(value.to_string())
            }
        }
    /// Concatenate columns
    pub fn concat_columns(columns: Vec<String>, separator: String) -> impl Fn(&HashMap<String, String>) -> CsvResult<String> {
        move |record: &HashMap<String, String>| {
            let values: Vec<String> = columns.iter()
                .map(|col| record.get(col).cloned().unwrap_or_default())
                .collect();
            Ok(values.join(&separator))
        }
    }
    
    /// Conditional value
    pub fn conditional(condition_column: String, true_value: String, false_value: String) -> impl Fn(&HashMap<String, String>) -> CsvResult<String> {
        move |record: &HashMap<String, String>| {
            let condition_value = record.get(&condition_column).cloned().unwrap_or_default();
            let result = match condition_value.to_lowercase().as_str() {
            Ok(result)
        }
    }
    
    /// Calculate age from birth year
    pub fn age_from_birth_year(current_year: i32) -> impl Fn(&str) -> CsvResult<String> {
        move |value: &str| {
            if let Ok(birth_year) = value.parse::<i32>() {
                let age = current_year - birth_year;
                Ok(age.to_string())
            } else {
                Ok("0".to_string())
            }
        }
    }
}

