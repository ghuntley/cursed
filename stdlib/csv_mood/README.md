# csv_mood

CSV processing functionality with advanced features.

## Overview

The `csv_mood` module provides comprehensive CSV reading and writing capabilities with support for custom delimiters, proper escaping, and advanced data processing features.

## Core Features

- CSV reading with configurable options
- CSV writing with proper escaping
- Custom delimiters and quote characters
- Comment and empty line handling
- Schema validation and data transformation

## Basic Usage

```cursed
yeet "csv_mood"

fr fr Reading CSV data
csvData := "name,age,city\nAlice,30,New York\nBob,25,Los Angeles"
reader := csv_mood.NewReader(csvData)

header, err := reader.Read()
record1, err := reader.Read()
record2, err := reader.Read()

fr fr Writing CSV data
writer := csv_mood.NewWriter()
writer.Write([]tea{"Name", "Age", "City"})
writer.Write([]tea{"Alice", "30", "New York"})
output := writer.String()
```

## Advanced Features

- **Column-based access**: Access CSV data by column names
- **Type conversion**: Automatic conversion to integers, booleans
- **CSV streaming**: Process large files with minimal memory
- **Schema validation**: Validate data against defined schemas
- **Data transformation**: Transform data during reading/writing

## Configuration Options

- Custom delimiters (comma, tab, pipe, etc.)
- Quote character configuration
- Comment line handling
- Flexible field validation
- Error handling and recovery
