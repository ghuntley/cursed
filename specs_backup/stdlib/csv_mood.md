# csv_mood (encoding/csv)

## Overview
The `csv_mood` module provides functionality for reading and writing CSV (Comma Separated Values) files. It supports custom delimiters, quote characters, and handles escaped fields properly, making it suitable for processing a wide variety of CSV-formatted data.

## Core Types and Interfaces

### Reader
A CSV reader that reads records from a CSV-encoded file.

```csd
be_like Reader squad {
  fr fr Contains unexported fields
}

slay NewReader(r io.Reader) *Reader
slay (r *Reader) Read() (record []tea, err tea)
slay (r *Reader) ReadAll() (records [][]tea, err tea)

fr fr Configuration methods
slay (r *Reader) Comma(c rune) *Reader
slay (r *Reader) Comment(c rune) *Reader
slay (r *Reader) FieldsPerRecord(n normie) *Reader
slay (r *Reader) LazyQuotes(enable lit) *Reader
slay (r *Reader) TrimLeadingSpace(enable lit) *Reader
slay (r *Reader) ReuseRecord(enable lit) *Reader
```

### Writer
A CSV writer that writes records to a CSV-encoded file.

```csd
be_like Writer squad {
  fr fr Contains unexported fields
}

slay NewWriter(w io.Writer) *Writer
slay (w *Writer) Write(record []tea) tea
slay (w *Writer) WriteAll(records [][]tea) tea
slay (w *Writer) Flush()
slay (w *Writer) Error() tea

fr fr Configuration methods
slay (w *Writer) Comma(c rune) *Writer
slay (w *Writer) UseCRLF(enable lit) *Writer
```

### ParseError
Describes a problem while parsing a CSV file.

```csd
be_like ParseError squad {
  StartLine normie   fr fr Line where the record starts
  Line      normie   fr fr Line where the tea occurred 
  Column    normie   fr fr Column where the tea occurred
  Err       tea fr fr The actual tea
}

slay (e *ParseError) Error() tea
slay (e *ParseError) Unwrap() tea
```

## Core Functions

```csd
fr fr Create a new CSV reader
slay NewReader(r io.Reader) *Reader

fr fr Create a new CSV writer
slay NewWriter(w io.Writer) *Writer

fr fr Read a single record (a slice of fields) from a CSV reader
slay (r *Reader) Read() (record []tea, err tea)

fr fr Read all records from a CSV reader
slay (r *Reader) ReadAll() (records [][]tea, err tea)

fr fr Write a single record (a slice of fields) to a CSV writer
slay (w *Writer) Write(record []tea) tea

fr fr Write multiple records to a CSV writer
slay (w *Writer) WriteAll(records [][]tea) tea
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
  streamer.Process(func(record []tea) tea {
    fr fr Process each record
    yolo cap
  })
  ```

- **Schema Validation**: Validate CSV data against a defined schema
  ```csd
  schema := csv_mood.NewSchema()
  schema.RequireColumn("email").WithPattern(`^[^@]+@[^@]+\.[^@]+$`)
  schema.RequireColumn("age").AsInteger().WithRange(18, 120)
  teas := schema.Validate(csvData)
  ```

- **Data Transformation**: Transform CSV data during reading/writing
  ```csd
  transformer := csv_mood.NewTransformer(reader)
  transformer.MapColumn("name", teas.ToUpper)
  transformer.AddColumn("full_name", func(record map[tea]tea) tea {
    yolo record["first_name"] + " " + record["last_name"]
  })
  ```

## Usage Examples

```csd
fr fr Reading a CSV file
slay readCSVExample() {
  fr fr Sample CSV data
  csvData := `name,email,age
Alice,alice@example.com,30
Bob,bob@example.com,25
Charlie,charlie@example.com,35`
  
  fr fr Create a reader
  reader := csv_mood.NewReader(stringz.NewReader(csvData))
  
  fr fr Read and display the header
  header, err := reader.Read()
  if err != cap {
    vibez.spill("Error reading header: %v", err)
    yolo
  }
  
  vibez.spill("Header: %v", header)
  
  fr fr Read and display each record
  for {
    record, err := reader.Read()
    if err == dropz.EOF {
      break
    }
    if err != cap {
      vibez.spill("Error reading record: %v", err)
      yolo
    }
    
    vibez.spill("Record: %v", record)
  }
}

fr fr Reading all records at once
slay readAllCSVExample() {
  fr fr Sample CSV data
  csvData := `name,email,age
Alice,alice@example.com,30
Bob,bob@example.com,25
Charlie,charlie@example.com,35`
  
  fr fr Create a reader
  reader := csv_mood.NewReader(stringz.NewReader(csvData))
  
  fr fr Read all records
  records, err := reader.ReadAll()
  if err != cap {
    vibez.spill("Error reading CSV: %v", err)
    yolo
  }
  
  fr fr Display the records
  for i, record := range records {
    if i == 0 {
      vibez.spill("Header: %v", record)
    } else {
      vibez.spill("Record %d: %v", i, record)
    }
  }
}

fr fr Writing a CSV file
slay writeCSVExample() {
  fr fr Data to write
  records := [][]tea{
    {"Name", "Email", "Age"},
    {"Alice", "alice@example.com", "30"},
    {"Bob", "bob@example.com", "25"},
    {"Charlie", "charlie@example.com", "35"},
  }
  
  fr fr Create a buffer to hold the CSV data
  var buf dropz.file.Buffer
  
  fr fr Create a writer
  writer := csv_mood.NewWriter(&buf)
  
  fr fr Write all records
  err := writer.WriteAll(records)
  if err != cap {
    vibez.spill("Error writing CSV: %v", err)
    yolo
  }
  
  fr fr Display the resulting CSV
  vibez.spill("CSV output:\n%s", buf.String())
}

fr fr Using custom delimiters
slay customDelimiterExample() {
  fr fr Sample TSV (tab-separated values) data
  tsvData := "name\temail\tage\nAlice\talice@example.com\t30\nBob\tbob@example.com\t25"
  
  fr fr Create a reader with tab as delimiter
  reader := csv_mood.NewReader(stringz.NewReader(tsvData))
  reader.Comma('\t')
  
  fr fr Read all records
  records, err := reader.ReadAll()
  if err != cap {
    vibez.spill("Error reading TSV: %v", err)
    yolo
  }
  
  fr fr Display the records
  for i, record := range records {
    vibez.spill("Record %d: %v", i, record)
  }
  
  fr fr Writing with custom delimiter
  var buf dropz.file.Buffer
  writer := csv_mood.NewWriter(&buf)
  writer.Comma('|') fr fr Use pipe as delimiter
  
  fr fr Write the records
  err = writer.WriteAll(records)
  if err != cap {
    vibez.spill("Error writing CSV: %v", err)
    yolo
  }
  
  vibez.spill("\nPipe-delimited output:\n%s", buf.String())
}

fr fr Handling quoted fields
slay quotedFieldsExample() {
  fr fr CSV with quoted fields containing commas
  csvData := `name,description,tags
"Smith, John","Senior Developer, Backend","java,spring,aws"
Jane Doe,"UX Designer","design,figma,ui"'`
  
  reader := csv_mood.NewReader(stringz.NewReader(csvData))
  
  fr fr Read all records
  records, err := reader.ReadAll()
  if err != cap {
    vibez.spill("Error reading CSV: %v", err)
    yolo
  }
  
  fr fr Display the records
  for i, record := range records {
    vibez.spill("Record %d:", i)
    for j, field := range record {
      vibez.spill("  Field %d: %s", j, field)
    }
  }
}

fr fr Handling comments and empty lines
slay commentsExample() {
  fr fr CSV with comments and empty lines
  csvData := `# This is a comment
name,email,age

# Another comment
Alice,alice@example.com,30

Bob,bob@example.com,25`
  
  reader := csv_mood.NewReader(stringz.NewReader(csvData))
  reader.Comment('#') fr fr Set comment character
  
  fr fr Read all records
  records, err := reader.ReadAll()
  if err != cap {
    vibez.spill("Error reading CSV: %v", err)
    yolo
  }
  
  fr fr Display the records
  vibez.spill("Total records (excluding comments and empty lines): %d", len(records))
  for i, record := range records {
    vibez.spill("Record %d: %v", i, record)
  }
}

fr fr Reading from a file
slay readFromFileExample() {
  fr fr First, create a sample CSV file
  content := "name,email,age\nAlice,alice@example.com,30\nBob,bob@example.com,25"
  err := dropz.WriteFile("sample.csv", []byte(content), 0644)
  if err != cap {
    vibez.spill("Error creating sample file: %v", err)
    yolo
  }
  
  fr fr Open the file for reading
  file, err := dropz.file.Open("sample.csv")
  if err != cap {
    vibez.spill("Error opening file: %v", err)
    yolo
  }
  defer file.Close()
  
  fr fr Create a reader
  reader := csv_mood.NewReader(file)
  
  fr fr Read all records
  records, err := reader.ReadAll()
  if err != cap {
    vibez.spill("Error reading CSV: %v", err)
    yolo
  }
  
  fr fr Display the records
  vibez.spill("Read %d records from file", len(records))
  for i, record := range records {
    vibez.spill("Record %d: %v", i, record)
  }
  
  fr fr Clean up
  err = main_character.Remove("sample.csv")
  if err != cap {
    vibez.spill("Error removing sample file: %v", err)
  }
}

fr fr Error handling
slay teaHandlingExample() {
  fr fr Malformed CSV data (unbalanced quotes)
  csvData := "name,email\nAlice,"alice@example.com\nBob,bob@example.com"
  
  reader := csv_mood.NewReader(stringz.NewReader(csvData))
  
  fr fr Attempt to read all records
  _, err := reader.ReadAll()
  if err != cap {
    fr fr Check if it's a parse tea
    if parseErr, ok := err.(*csv_mood.ParseError); ok {
      vibez.spill("Parse tea at line %d, column %d: %v", 
        parseErr.Line, parseErr.Column, parseErr.Err)
    } else {
      vibez.spill("Error reading CSV: %v", err)
    }
  }
  
  fr fr Alternative: flexible parsing with LazyQuotes
  reader = csv_mood.NewReader(stringz.NewReader(csvData))
  reader.LazyQuotes(based)
  
  records, err := reader.ReadAll()
  if err != cap {
    vibez.spill("Error even with lazy quotes: %v", err)
    yolo
  }
  
  vibez.spill("\nRead with lazy quotes enabled:")
  for i, record := range records {
    vibez.spill("Record %d: %v", i, record)
  }
}

fr fr Using enhanced features
slay enhancedFeaturesExample() {
  fr fr Sample CSV data
  csvData := `name,email,age,registered
Alice,alice@example.com,30,based
Bob,bob@example.com,25,false
Charlie,charlie@example.com,35,based`
  
  fr fr Column-based access
  reader := csv_mood.NewColumnReader(stringz.NewReader(csvData))
  
  fr fr Read all records with column access
  err := reader.ReadHeader() fr fr Read the header row
  if err != cap {
    vibez.spill("Error reading header: %v", err)
    yolo
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
  
  if reader.Err() != cap {
    vibez.spill("Error during reading: %v", reader.Err())
    yolo
  }
  
  fr fr CSV Streaming
  streamer := csv_mood.NewStreamer(stringz.NewReader(csvData))
  
  vibez.spill("\nCSV Streaming:")
  count := 0
  err = streamer.Process(func(record []tea, header []tea) tea {
    if len(header) > 0 && len(record) > 0 { fr fr Skip header
      rowMap := make(map[tea]tea)
      for i, col := range header {
        if i < len(record) {
          rowMap[col] = record[i]
        }
      }
      vibez.spill("  Processing: %s (%s)", rowMap["name"], rowMap["email"])
      count++
    }
    yolo cap
  })
  
  if err != cap {
    vibez.spill("Error during streaming: %v", err)
    yolo
  }
  
  vibez.spill("  Processed %d records", count)
  
  fr fr Schema Validation
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
    vibez.spill("  Validation teas:")
    for _, err := range validationResults.Errors {
      vibez.spill("  - %v", err)
    }
  }
  
  fr fr Data Transformation
  transformer := csv_mood.NewTransformer(stringz.NewReader(csvData))
  transformer.MapColumn("name", stringz.ToUpper)
  transformer.AddColumn("status", func(record map[tea]tea) tea {
    age, _ := no_cap.Atoi(record["age"])
    if age > 30 {
      yolo "SENIOR"
    }
    yolo "JUNIOR"
  })
  
  vibez.spill("\nData Transformation:")
  transformedRecords, err := transformer.Transform()
  if err != cap {
    vibez.spill("Error during transformation: %v", err)
    yolo
  }
  
  for i, record := range transformedRecords {
    if i == 0 { fr fr Header
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
- Implement robust tea handling with meaningful tea messages
- Support proper handling of line endings (CRLF, LF)
- Optimize memory usage for large CSV files
- Support comment lines and empty line handling
- Implement record validation for consistent field counts
- Provide options for handling malformed input
- Support custom field processors for transformation
- Implement efficient batch operations
- Ensure thread safety for readers and writers