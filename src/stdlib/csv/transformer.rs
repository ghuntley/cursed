/// CSV data transformation with column mapping and computed fields
use std::io;
use std::collections::HashMap;
use crate::stdlib::csv::reader::Reader;
use crate::stdlib::csv::error::{CsvError, CsvResult};

/// Function type for column transformation
pub type ColumnTransformFn = Box<dyn Fn(&str) -> CsvResult<String>>;

/// Function type for computed column generation
pub type ComputedColumnFn = Box<dyn Fn(&HashMap<String, String>) -> CsvResult<String>>;

/// Column transformation definition
#[derive(Debug)]
pub enum ColumnTransform {
    /// Transform an existing column's values
    Map {
        column: String,
        transform: ColumnTransformFn,
    },
    
    /// Add a new computed column
    Add {
        column: String,
        compute: ComputedColumnFn,
    },
    
    /// Remove a column
    Remove {
        column: String,
    },
    
    /// Rename a column
    Rename {
        from: String,
        to: String,
    },
    
    /// Reorder columns
    Reorder {
        new_order: Vec<String>,
    },
    
    /// Filter rows based on a condition
    FilterRows {
        condition: Box<dyn Fn(&HashMap<String, String>) -> CsvResult<bool>>,
    },
}

/// CSV data transformer
pub struct Transformer<R: io::Read> {
    /// Underlying CSV reader
    reader: Reader<R>,
    
    /// List of transformations to apply
    transforms: Vec<ColumnTransform>,
    
    /// Whether header has been read
    header_read: bool,
    
    /// Original header
    original_header: Vec<String>,
    
    /// Transformed header
    transformed_header: Vec<String>,
    
    /// Column name to index mapping for original data
    column_map: HashMap<String, usize>,
}

impl<R: io::Read> Transformer<R> {
    /// Create a new transformer
    pub fn new(reader: R) -> Self {
        Self {
            reader: Reader::new(reader),
            transforms: Vec::new(),
            header_read: false,
            original_header: Vec::new(),
            transformed_header: Vec::new(),
            column_map: HashMap::new(),
        }
    }
    
    /// Create a new transformer with custom CSV reader
    pub fn with_reader(reader: Reader<R>) -> Self {
        Self {
            reader,
            transforms: Vec::new(),
            header_read: false,
            original_header: Vec::new(),
            transformed_header: Vec::new(),
            column_map: HashMap::new(),
        }
    }
    
    /// Map an existing column's values using a transformation function
    pub fn map_column<F>(&mut self, column: &str, transform: F) -> &mut Self
    where
        F: Fn(&str) -> CsvResult<String> + 'static,
    {
        self.transforms.push(ColumnTransform::Map {
            column: column.to_string(),
            transform: Box::new(transform),
        });
        self
    }
    
    /// Add a new computed column
    pub fn add_column<F>(&mut self, column: &str, compute: F) -> &mut Self
    where
        F: Fn(&HashMap<String, String>) -> CsvResult<String> + 'static,
    {
        self.transforms.push(ColumnTransform::Add {
            column: column.to_string(),
            compute: Box::new(compute),
        });
        self
    }
    
    /// Remove a column
    pub fn remove_column(&mut self, column: &str) -> &mut Self {
        self.transforms.push(ColumnTransform::Remove {
            column: column.to_string(),
        });
        self
    }
    
    /// Rename a column
    pub fn rename_column(&mut self, from: &str, to: &str) -> &mut Self {
        self.transforms.push(ColumnTransform::Rename {
            from: from.to_string(),
            to: to.to_string(),
        });
        self
    }
    
    /// Reorder columns
    pub fn reorder_columns(&mut self, new_order: Vec<String>) -> &mut Self {
        self.transforms.push(ColumnTransform::Reorder {
            new_order,
        });
        self
    }
    
    /// Filter rows based on a condition
    pub fn filter_rows<F>(&mut self, condition: F) -> &mut Self
    where
        F: Fn(&HashMap<String, String>) -> CsvResult<bool> + 'static,
    {
        self.transforms.push(ColumnTransform::FilterRows {
            condition: Box::new(condition),
        });
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
    
    /// Apply all transformations and return the transformed data
    pub fn transform(&mut self) -> CsvResult<Vec<Vec<String>>> {
        // Read header
        if !self.header_read {
            self.read_header()?;
        }
        
        let mut results = Vec::new();
        results.push(self.transformed_header.clone());
        
        // Process each record
        while let Some(record) = self.reader.read()? {
            if let Some(transformed_record) = self.transform_record(&record)? {
                results.push(transformed_record);
            }
        }
        
        Ok(results)
    }
    
    /// Transform data and write to a new CSV format
    pub fn transform_to_string(&mut self) -> CsvResult<String> {
        use crate::stdlib::csv::writer::write_to_string;
        let transformed = self.transform()?;
        write_to_string(&transformed)
    }
    
    /// Get the list of transformations
    pub fn transforms(&self) -> &[ColumnTransform] {
        &self.transforms
    }
    
    /// Get the original header
    pub fn original_header(&self) -> &[String] {
        &self.original_header
    }
    
    /// Get the transformed header
    pub fn transformed_header(&self) -> &[String] {
        &self.transformed_header
    }
    
    /// Read and process the header
    fn read_header(&mut self) -> CsvResult<()> {
        if let Some(header) = self.reader.read()? {
            self.original_header = header.clone();
            self.transformed_header = header.clone();
            
            // Build column map
            for (index, column_name) in self.original_header.iter().enumerate() {
                self.column_map.insert(column_name.clone(), index);
            }
            
            // Apply header transformations
            self.apply_header_transforms()?;
            self.header_read = true;
        } else {
            return Err(CsvError::General("No header found in CSV".to_string()));
        }
        
        Ok(())
    }
    
    /// Apply transformations that affect the header
    fn apply_header_transforms(&mut self) -> CsvResult<()> {
        for transform in &self.transforms {
            match transform {
                ColumnTransform::Add { column, .. } => {
                    // Add new column to header
                    if !self.transformed_header.contains(column) {
                        self.transformed_header.push(column.clone());
                    }
                },
                ColumnTransform::Remove { column } => {
                    // Remove column from header
                    self.transformed_header.retain(|c| c != column);
                },
                ColumnTransform::Rename { from, to } => {
                    // Rename column in header
                    for col in &mut self.transformed_header {
                        if col == from {
                            *col = to.clone();
                            break;
                        }
                    }
                },
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
                },
                _ => {} // Other transforms don't affect header
            }
        }
        
        Ok(())
    }
    
    /// Transform a single record
    fn transform_record(&self, record: &[String]) -> CsvResult<Option<Vec<String>>> {
        // Build record map
        let mut record_map = HashMap::new();
        for (column_name, &index) in &self.column_map {
            let value = if index < record.len() {
                record[index].clone()
            } else {
                String::new()
            };
            record_map.insert(column_name.clone(), value);
        }
        
        // Apply transformations
        let mut should_include_row = true;
        
        for transform in &self.transforms {
            match transform {
                ColumnTransform::Map { column, transform } => {
                    if let Some(value) = record_map.get(column) {
                        let transformed_value = transform(value)?;
                        record_map.insert(column.clone(), transformed_value);
                    }
                },
                ColumnTransform::Add { column, compute } => {
                    let computed_value = compute(&record_map)?;
                    record_map.insert(column.clone(), computed_value);
                },
                ColumnTransform::Remove { column } => {
                    record_map.remove(column);
                },
                ColumnTransform::Rename { from, to } => {
                    if let Some(value) = record_map.remove(from) {
                        record_map.insert(to.clone(), value);
                    }
                },
                ColumnTransform::FilterRows { condition } => {
                    if !condition(&record_map)? {
                        should_include_row = false;
                    }
                },
                ColumnTransform::Reorder { .. } => {
                    // Reordering is handled when building the output record
                },
            }
        }
        
        if !should_include_row {
            return Ok(None);
        }
        
        // Build output record based on transformed header
        let mut output_record = Vec::new();
        for column_name in &self.transformed_header {
            let value = record_map.get(column_name).cloned().unwrap_or_else(String::new);
            output_record.push(value);
        }
        
        Ok(Some(output_record))
    }
}

/// Result of transformation
#[derive(Debug, Clone)]
pub struct TransformResult {
    /// Transformed records (including header)
    pub records: Vec<Vec<String>>,
    
    /// Number of input records processed
    pub input_records: usize,
    
    /// Number of output records generated
    pub output_records: usize,
    
    /// Number of columns in input
    pub input_columns: usize,
    
    /// Number of columns in output
    pub output_columns: usize,
}

impl TransformResult {
    /// Get a summary of the transformation
    pub fn summary(&self) -> String {
        format!(
            "Transformed {} -> {} records, {} -> {} columns",
            self.input_records,
            self.output_records,
            self.input_columns,
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
    }
    
    /// Convert string to lowercase
    pub fn to_lowercase(value: &str) -> CsvResult<String> {
        Ok(value.to_lowercase())
    }
    
    /// Trim whitespace
    pub fn trim(value: &str) -> CsvResult<String> {
        Ok(value.trim().to_string())
    }
    
    /// Replace characters
    pub fn replace<'a>(from: &'a str, to: &'a str) -> impl Fn(&str) -> CsvResult<String> + 'a {
        move |value: &str| Ok(value.replace(from, to))
    }
    
    /// Parse and format numbers
    pub fn format_number(decimals: usize) -> impl Fn(&str) -> CsvResult<String> {
        move |value: &str| {
            if let Ok(num) = value.parse::<f64>() {
                Ok(format!("{:.1$}", num, decimals))
            } else {
                Ok(value.to_string())
            }
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
                "true" | "yes" | "1" | "on" | "based" => true_value.clone(),
                _ => false_value.clone(),
            };
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_map_column() {
        let csv_data = "name,age\nAlice,30\nBob,25";
        let cursor = Cursor::new(csv_data);
        let mut transformer = Transformer::new(cursor);
        
        transformer.map_column("name", |value| Ok(value.to_uppercase()));
        
        let result = transformer.transform().unwrap();
        assert_eq!(result.len(), 3); // header + 2 records
        assert_eq!(result[0], vec!["name", "age"]); // header unchanged
        assert_eq!(result[1], vec!["ALICE", "30"]);
        assert_eq!(result[2], vec!["BOB", "25"]);
    }

    #[test]
    fn test_add_column() {
        let csv_data = "first_name,last_name\nAlice,Smith\nBob,Jones";
        let cursor = Cursor::new(csv_data);
        let mut transformer = Transformer::new(cursor);
        
        transformer.add_column("full_name", |record| {
            let first = record.get("first_name").cloned().unwrap_or_default();
            let last = record.get("last_name").cloned().unwrap_or_default();
            Ok(format!("{} {}", first, last))
        });
        
        let result = transformer.transform().unwrap();
        assert_eq!(result[0], vec!["first_name", "last_name", "full_name"]);
        assert_eq!(result[1], vec!["Alice", "Smith", "Alice Smith"]);
        assert_eq!(result[2], vec!["Bob", "Jones", "Bob Jones"]);
    }

    #[test]
    fn test_remove_column() {
        let csv_data = "name,age,secret\nAlice,30,password\nBob,25,123456";
        let cursor = Cursor::new(csv_data);
        let mut transformer = Transformer::new(cursor);
        
        transformer.remove_column("secret");
        
        let result = transformer.transform().unwrap();
        assert_eq!(result[0], vec!["name", "age"]);
        assert_eq!(result[1], vec!["Alice", "30"]);
        assert_eq!(result[2], vec!["Bob", "25"]);
    }

    #[test]
    fn test_rename_column() {
        let csv_data = "old_name,age\nAlice,30\nBob,25";
        let cursor = Cursor::new(csv_data);
        let mut transformer = Transformer::new(cursor);
        
        transformer.rename_column("old_name", "new_name");
        
        let result = transformer.transform().unwrap();
        assert_eq!(result[0], vec!["new_name", "age"]);
        assert_eq!(result[1], vec!["Alice", "30"]);
        assert_eq!(result[2], vec!["Bob", "25"]);
    }

    #[test]
    fn test_reorder_columns() {
        let csv_data = "name,age,city\nAlice,30,NYC\nBob,25,SF";
        let cursor = Cursor::new(csv_data);
        let mut transformer = Transformer::new(cursor);
        
        transformer.reorder_columns(vec!["age".to_string(), "name".to_string(), "city".to_string()]);
        
        let result = transformer.transform().unwrap();
        assert_eq!(result[0], vec!["age", "name", "city"]);
        assert_eq!(result[1], vec!["30", "Alice", "NYC"]);
        assert_eq!(result[2], vec!["25", "Bob", "SF"]);
    }

    #[test]
    fn test_filter_rows() {
        let csv_data = "name,age\nAlice,30\nBob,17\nCharlie,25";
        let cursor = Cursor::new(csv_data);
        let mut transformer = Transformer::new(cursor);
        
        transformer.filter_rows(|record| {
            let age: i32 = record.get("age").unwrap_or(&"0".to_string()).parse().unwrap_or(0);
            Ok(age >= 18) // Only adults
        });
        
        let result = transformer.transform().unwrap();
        assert_eq!(result.len(), 3); // header + 2 adult records
        assert_eq!(result[1][0], "Alice");
        assert_eq!(result[2][0], "Charlie");
    }

    #[test]
    fn test_multiple_transformations() {
        let csv_data = "first_name,last_name,age\nalice,smith,30\nbob,jones,25";
        let cursor = Cursor::new(csv_data);
        let mut transformer = Transformer::new(cursor);
        
        transformer
            .map_column("first_name", |value| Ok(value.to_uppercase()))
            .map_column("last_name", |value| Ok(value.to_uppercase()))
            .add_column("full_name", |record| {
                let first = record.get("first_name").cloned().unwrap_or_default();
                let last = record.get("last_name").cloned().unwrap_or_default();
                Ok(format!("{} {}", first, last))
            })
            .filter_rows(|record| {
                let age: i32 = record.get("age").unwrap_or(&"0".to_string()).parse().unwrap_or(0);
                Ok(age >= 25)
            });
        
        let result = transformer.transform().unwrap();
        assert_eq!(result.len(), 3); // header + 2 records (both qualify)
        assert_eq!(result[0], vec!["first_name", "last_name", "age", "full_name"]);
        assert_eq!(result[1], vec!["ALICE", "SMITH", "30", "ALICE SMITH"]);
        assert_eq!(result[2], vec!["BOB", "JONES", "25", "BOB JONES"]);
    }

    #[test]
    fn test_transform_functions() {
        use super::transforms::*;
        
        assert_eq!(to_uppercase("hello").unwrap(), "HELLO");
        assert_eq!(to_lowercase("WORLD").unwrap(), "world");
        assert_eq!(trim("  test  ").unwrap(), "test");
        
        let replace_fn = replace("a", "b");
        assert_eq!(replace_fn("cat").unwrap(), "cbt");
        
        let format_fn = format_number(2);
        assert_eq!(format_fn("3.14159").unwrap(), "3.14");
        
        let age_fn = age_from_birth_year(2023);
        assert_eq!(age_fn("1990").unwrap(), "33");
    }

    #[test]
    fn test_computed_columns() {
        use super::transforms::*;
        
        let mut record = HashMap::new();
        record.insert("first".to_string(), "John".to_string());
        record.insert("last".to_string(), "Doe".to_string());
        record.insert("active".to_string(), "true".to_string());
        
        let concat_fn = concat_columns(vec!["first".to_string(), "last".to_string()], " ".to_string());
        assert_eq!(concat_fn(&record).unwrap(), "John Doe");
        
        let conditional_fn = conditional("active".to_string(), "Yes".to_string(), "No".to_string());
        assert_eq!(conditional_fn(&record).unwrap(), "Yes");
    }

    #[test]
    fn test_custom_delimiter() {
        let tsv_data = "name\tage\nAlice\t30\nBob\t25";
        let cursor = Cursor::new(tsv_data);
        let mut transformer = Transformer::new(cursor).comma('\t');
        
        transformer.map_column("name", |value| Ok(value.to_uppercase()));
        
        let result = transformer.transform().unwrap();
        assert_eq!(result[1], vec!["ALICE", "30"]);
        assert_eq!(result[2], vec!["BOB", "25"]);
    }

    #[test]
    fn test_transform_to_string() {
        let csv_data = "name,age\nAlice,30\nBob,25";
        let cursor = Cursor::new(csv_data);
        let mut transformer = Transformer::new(cursor);
        
        transformer.map_column("name", |value| Ok(value.to_uppercase()));
        
        let result = transformer.transform_to_string().unwrap();
        assert!(result.contains("ALICE,30"));
        assert!(result.contains("BOB,25"));
    }

    #[test]
    fn test_empty_csv() {
        let cursor = Cursor::new("");
        let mut transformer = Transformer::new(cursor);
        
        let result = transformer.transform();
        assert!(result.is_err()); // Should fail because no header
    }

    #[test]
    fn test_header_only_csv() {
        let csv_data = "name,age";
        let cursor = Cursor::new(csv_data);
        let mut transformer = Transformer::new(cursor);
        
        transformer.add_column("status", |_| Ok("new".to_string()));
        
        let result = transformer.transform().unwrap();
        assert_eq!(result.len(), 1); // Only header
        assert_eq!(result[0], vec!["name", "age", "status"]);
    }
}
