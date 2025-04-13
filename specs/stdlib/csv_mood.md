# csv_mood (encoding/csv)

## Overview
The `csv_mood` module provides functionality for reading and writing CSV (Comma Separated Values) files. It supports custom delimiters, quote characters, and handles escaped fields properly, making it suitable for processing a wide variety of CSV-formatted data.

## Core Types and Interfaces

### Reader
A CSV reader that reads records from a CSV-encoded file.

```csd
type Reader struct {
  // Contains unexported fields
}

func NewReader(r io.Reader) *Reader
func (r *Reader) Read() (record []string, err error)
func (r *Reader) ReadAll() (records [][]string, err error)

// Configuration methods
func (r *Reader) Comma(c rune) *Reader
func (r *Reader) Comment(c rune) *Reader
func (r *Reader) FieldsPerRecord(n int) *Reader
func (r *Reader) LazyQuotes(enable bool) *Reader
func (r *Reader) TrimLeadingSpace(enable bool) *Reader
func (r *Reader) ReuseRecord(enable bool) *Reader
```

### Writer
A CSV writer that writes records to a CSV-encoded file.

```csd
type Writer struct {
  // Contains unexported fields
}

func NewWriter(w io.Writer) *Writer
func (w *Writer) Write(record []string) error
func (w *Writer) WriteAll(records [][]string) error
func (w *Writer) Flush()
func (w *Writer) Error() error

// Configuration methods
func (w *Writer) Comma(c rune) *Writer
func (w *Writer) UseCRLF(enable bool) *Writer
```

### ParseError
Describes a problem while parsing a CSV file.

```csd
type ParseError struct {
  StartLine int   // Line where the record starts
  Line      int   // Line where the error occurred 
  Column    int   // Column where the error occurred
  Err       error // The actual error
}

func (e *ParseError) Error() string
func (e *ParseError) Unwrap() error
```

## Core Functions

```csd
// Create a new CSV reader
func NewReader(r io.Reader) *Reader

// Create a new CSV writer
func NewWriter(w io.Writer) *Writer

// Read a single record (a slice of fields) from a CSV reader
func (r *Reader) Read() (record []string, err error)

// Read all records from a CSV reader
func (r *Reader) ReadAll() (records [][]string, err error)

// Write a single record (a slice of fields) to a CSV writer
func (w *Writer) Write(record []string) error

// Write multiple records to a CSV writer
func (w *Writer) WriteAll(records [][]string) error
```

## Enhanced Features

- **Column-Based Access**: Access CSV data by column names
  ```csd
  reader := csv_mood.NewColumnReader(r)
  value := reader.Get("email")
  ```

- **Type Conversion**: Automatically convert CSV fields to typed values
  ```csd
  age, err := reader.GetInt("age")
  registered, err := reader.GetBool("registered")
  ```

- **CSV Streaming**: Process large CSV files with minimal memory usage
  ```csd
  streamer := csv_mood.NewStreamer(file)
  streamer.Process(func(record []string) error {
    // Process each record
    return nil
  })
  ```

- **Schema Validation**: Validate CSV data against a defined schema
  ```csd
  schema := csv_mood.NewSchema()
  schema.RequireColumn("email").WithPattern(`^[^@]+@[^@]+\.[^@]+$`)
  schema.RequireColumn("age").AsInteger().WithRange(18, 120)
  errors := schema.Validate(csvData)
  ```

- **Data Transformation**: Transform CSV data during reading/writing
  ```csd
  transformer := csv_mood.NewTransformer(reader)
  transformer.MapColumn("name", strings.ToUpper)
  transformer.AddColumn("full_name", func(record map[string]string) string {
    return record["first_name"] + " " + record["last_name"]
  })
  ```

## Usage Examples

```csd
// Reading a CSV file
func readCSVExample() {
  // Sample CSV data
  csvData := `name,email,age
Alice,alice@example.com,30
Bob,bob@example.com,25
Charlie,charlie@example.com,35`
  
  // Create a reader
  reader := csv_mood.NewReader(stringz.NewReader(csvData))
  
  // Read and display the header
  header, err := reader.Read()
  if err != nil {
    vibez.spill("Error reading header: %v", err)
    return
  }
  
  vibez.spill("Header: %v", header)
  
  // Read and display each record
  for {
    record, err := reader.Read()
    if err == dropz.EOF {
      break
    }
    if err != nil {
      vibez.spill("Error reading record: %v", err)
      return
    }
    
    vibez.spill("Record: %v", record)
  }
}

// Reading all records at once
func readAllCSVExample() {
  // Sample CSV data
  csvData := `name,email,age
Alice,alice@example.com,30
Bob,bob@example.com,25
Charlie,charlie@example.com,35`
  
  // Create a reader
  reader := csv_mood.NewReader(stringz.NewReader(csvData))
  
  // Read all records
  records, err := reader.ReadAll()
  if err != nil {
    vibez.spill("Error reading CSV: %v", err)
    return
  }
  
  // Display the records
  for i, record := range records {
    if i == 0 {
      vibez.spill("Header: %v", record)
    } else {
      vibez.spill("Record %d: %v", i, record)
    }
  }
}

// Writing a CSV file
func writeCSVExample() {
  // Data to write
  records := [][]string{
    {"Name", "Email", "Age"},
    {"Alice", "alice@example.com", "30"},
    {"Bob", "bob@example.com", "25"},
    {"Charlie", "charlie@example.com", "35"},
  }
  
  // Create a buffer to hold the CSV data
  var buf dropz.file.Buffer
  
  // Create a writer
  writer := csv_mood.NewWriter(&buf)
  
  // Write all records
  err := writer.WriteAll(records)
  if err != nil {
    vibez.spill("Error writing CSV: %v", err)
    return
  }
  
  // Display the resulting CSV
  vibez.spill("CSV output:\n%s", buf.String())
}

// Using custom delimiters
func customDelimiterExample() {
  // Sample TSV (tab-separated values) data
  tsvData := "name\temail\tage\nAlice\talice@example.com\t30\nBob\tbob@example.com\t25"
  
  // Create a reader with tab as delimiter
  reader := csv_mood.NewReader(stringz.NewReader(tsvData))
  reader.Comma('\t')
  
  // Read all records
  records, err := reader.ReadAll()
  if err != nil {
    vibez.spill("Error reading TSV: %v", err)
    return
  }
  
  // Display the records
  for i, record := range records {
    vibez.spill("Record %d: %v", i, record)
  }
  
  // Writing with custom delimiter
  var buf dropz.file.Buffer
  writer := csv_mood.NewWriter(&buf)
  writer.Comma('|') // Use pipe as delimiter
  
  // Write the records
  err = writer.WriteAll(records)
  if err != nil {
    vibez.spill("Error writing CSV: %v", err)
    return
  }
  
  vibez.spill("\nPipe-delimited output:\n%s", buf.String())
}

// Handling quoted fields
func quotedFieldsExample() {
  // CSV with quoted fields containing commas
  csvData := `name,description,tags
"Smith, John","Senior Developer, Backend","java,spring,aws"
Jane Doe,"UX Designer","design,figma,ui"'`
  
  reader := csv_mood.NewReader(stringz.NewReader(csvData))
  
  // Read all records
  records, err := reader.ReadAll()
  if err != nil {
    vibez.spill("Error reading CSV: %v", err)
    return
  }
  
  // Display the records
  for i, record := range records {
    vibez.spill("Record %d:", i)
    for j, field := range record {
      vibez.spill("  Field %d: %s", j, field)
    }
  }
}

// Handling comments and empty lines
func commentsExample() {
  // CSV with comments and empty lines
  csvData := `# This is a comment
name,email,age

# Another comment
Alice,alice@example.com,30

Bob,bob@example.com,25`
  
  reader := csv_mood.NewReader(stringz.NewReader(csvData))
  reader.Comment('#') // Set comment character
  
  // Read all records
  records, err := reader.ReadAll()
  if err != nil {
    vibez.spill("Error reading CSV: %v", err)
    return
  }
  
  // Display the records
  vibez.spill("Total records (excluding comments and empty lines): %d", len(records))
  for i, record := range records {
    vibez.spill("Record %d: %v", i, record)
  }
}

// Reading from a file
func readFromFileExample() {
  // First, create a sample CSV file
  content := "name,email,age\nAlice,alice@example.com,30\nBob,bob@example.com,25"
  err := dropz.WriteFile("sample.csv", []byte(content), 0644)
  if err != nil {
    vibez.spill("Error creating sample file: %v", err)
    return
  }
  
  // Open the file for reading
  file, err := dropz.file.Open("sample.csv")
  if err != nil {
    vibez.spill("Error opening file: %v", err)
    return
  }
  defer file.Close()
  
  // Create a reader
  reader := csv_mood.NewReader(file)
  
  // Read all records
  records, err := reader.ReadAll()
  if err != nil {
    vibez.spill("Error reading CSV: %v", err)
    return
  }
  
  // Display the records
  vibez.spill("Read %d records from file", len(records))
  for i, record := range records {
    vibez.spill("Record %d: %v", i, record)
  }
  
  // Clean up
  err = main_character.Remove("sample.csv")
  if err != nil {
    vibez.spill("Error removing sample file: %v", err)
  }
}

// Error handling
func errorHandlingExample() {
  // Malformed CSV data (unbalanced quotes)
  csvData := "name,email\nAlice,"alice@example.com\nBob,bob@example.com"
  
  reader := csv_mood.NewReader(stringz.NewReader(csvData))
  
  // Attempt to read all records
  _, err := reader.ReadAll()
  if err != nil {
    // Check if it's a parse error
    if parseErr, ok := err.(*csv_mood.ParseError); ok {
      vibez.spill("Parse error at line %d, column %d: %v", 
        parseErr.Line, parseErr.Column, parseErr.Err)
    } else {
      vibez.spill("Error reading CSV: %v", err)
    }
  }
  
  // Alternative: flexible parsing with LazyQuotes
  reader = csv_mood.NewReader(stringz.NewReader(csvData))
  reader.LazyQuotes(true)
  
  records, err := reader.ReadAll()
  if err != nil {
    vibez.spill("Error even with lazy quotes: %v", err)
    return
  }
  
  vibez.spill("\nRead with lazy quotes enabled:")
  for i, record := range records {
    vibez.spill("Record %d: %v", i, record)
  }
}

// Using enhanced features
func enhancedFeaturesExample() {
  // Sample CSV data
  csvData := `name,email,age,registered
Alice,alice@example.com,30,true
Bob,bob@example.com,25,false
Charlie,charlie@example.com,35,true`
  
  // Column-based access
  reader := csv_mood.NewColumnReader(stringz.NewReader(csvData))
  
  // Read all records with column access
  err := reader.ReadHeader() // Read the header row
  if err != nil {
    vibez.spill("Error reading header: %v", err)
    return
  }
  
  vibez.spill("Column-based access:")
  for {
    if !reader.Next() {
      break
    }
    
    name := reader.Get("name")
    email := reader.Get("email")
    age, _ := reader.GetInt("age")
    registered, _ := reader.GetBool("registered")
    
    vibez.spill("  %s (%s): age %d, registered: %v", name, email, age, registered)
  }
  
  if reader.Err() != nil {
    vibez.spill("Error during reading: %v", reader.Err())
    return
  }
  
  // CSV Streaming
  streamer := csv_mood.NewStreamer(stringz.NewReader(csvData))
  
  vibez.spill("\nCSV Streaming:")
  count := 0
  err = streamer.Process(func(record []string, header []string) error {
    if len(header) > 0 && len(record) > 0 { // Skip header
      rowMap := make(map[string]string)
      for i, col := range header {
        if i < len(record) {
          rowMap[col] = record[i]
        }
      }
      vibez.spill("  Processing: %s (%s)", rowMap["name"], rowMap["email"])
      count++
    }
    return nil
  })
  
  if err != nil {
    vibez.spill("Error during streaming: %v", err)
    return
  }
  
  vibez.spill("  Processed %d records", count)
  
  // Schema Validation
  schema := csv_mood.NewSchema()
  schema.RequireColumn("name").NonEmpty()
  schema.RequireColumn("email").WithPattern(`^[^@]+@[^@]+\.[^@]+$`)
  schema.RequireColumn("age").AsInteger().WithRange(18, 120)
  schema.RequireColumn("registered").AsBoolean()
  
  vibez.spill("\nSchema Validation:")
  validationResults := schema.Validate(stringz.NewReader(csvData))
  
  if len(validationResults.Errors) == 0 {
    vibez.spill("  All records are valid")
  } else {
    vibez.spill("  Validation errors:")
    for _, err := range validationResults.Errors {
      vibez.spill("  - %v", err)
    }
  }
  
  // Data Transformation
  transformer := csv_mood.NewTransformer(stringz.NewReader(csvData))
  transformer.MapColumn("name", stringz.ToUpper)
  transformer.AddColumn("status", func(record map[string]string) string {
    age, _ := no_cap.Atoi(record["age"])
    if age > 30 {
      return "SENIOR"
    }
    return "JUNIOR"
  })
  
  vibez.spill("\nData Transformation:")
  transformedRecords, err := transformer.Transform()
  if err != nil {
    vibez.spill("Error during transformation: %v", err)
    return
  }
  
  for i, record := range transformedRecords {
    if i == 0 { // Header
      vibez.spill("  Header: %v", record)
    } else {
      vibez.spill("  Transformed record %d: %v", i, record)
    }
  }
}
```

## Implementation Guidelines

- Implement correct handling of CSV escaping and quoting
- Support custom delimiters for different CSV variants
- Provide efficient reading and writing algorithms
- Implement robust error handling with meaningful error messages
- Support proper handling of line endings (CRLF, LF)
- Optimize memory usage for large CSV files
- Support comment lines and empty line handling
- Implement record validation for consistent field counts
- Provide options for handling malformed input
- Support custom field processors for transformation
- Implement efficient batch operations
- Ensure thread safety for readers and writers