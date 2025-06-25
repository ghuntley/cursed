/// Comprehensive integration tests for the CSV mood module
use std::io::Cursor;
use cursed::stdlib::csv::*;

#[test]
fn test_basic_csv_operations() {
    // Test basic reading and writing
    let csv_data = "name,age,city\nAlice,30,New York\nBob,25,San Francisco";
    
    // Reading test
    let records = read_all_from_string(csv_data).unwrap();
    assert_eq!(records.len(), 3);
    assert_eq!(records[0], vec!["name", "age", "city"]);
    assert_eq!(records[1], vec!["Alice", "30", "New York"]);
    assert_eq!(records[2], vec!["Bob", "25", "San Francisco"]);
    
    // Writing test
    let output = write_all_to_string(&records).unwrap();
    assert!(output.contains("name,age,city"));
    assert!(output.contains("Alice,30,New York"));
    assert!(output.contains("Bob,25,San Francisco"));
}

#[test]
fn test_reader_configuration() {
    // Test custom delimiter
    let tsv_data = "name\tage\tcity\nAlice\t30\tNew York\nBob\t25\tSan Francisco";
    let cursor = Cursor::new(tsv_data);
    let mut reader = new_reader(cursor).comma('\t');
    
    let records = reader.read_all().unwrap();
    assert_eq!(records.len(), 3);
    assert_eq!(records[1], vec!["Alice", "30", "New York"]);
    
    // Test comments
    let csv_with_comments = "# This is a comment\nname,age\n# Another comment\nAlice,30\nBob,25";
    let cursor = Cursor::new(csv_with_comments);
    let mut reader = new_reader(cursor).comment('#');
    
    let records = reader.read_all().unwrap();
    assert_eq!(records.len(), 3); // header + 2 data rows, comments ignored
    
    // Test trimming
    let csv_with_spaces = "name, age,  city\nAlice,  30,   New York";
    let cursor = Cursor::new(csv_with_spaces);
    let mut reader = new_reader(cursor).trim_leading_space(true);
    
    let records = reader.read_all().unwrap();
    assert_eq!(records[0], vec!["name", "age", "city"]);
    assert_eq!(records[1], vec!["Alice", "30", "New York"]);
}

#[test]
fn test_writer_configuration() {
    let records = vec![
        vec!["name".to_string(), "description".to_string()],
        vec!["Smith, John".to_string(), "Senior Developer".to_string()],
    ];
    
    // Test custom delimiter
    let mut buf = Vec::new();
    {
        let mut writer = new_writer(&mut buf).comma('|');
        writer.write_all(&records).unwrap();
        writer.flush().unwrap();
    }
    let output = String::from_utf8(buf).unwrap();
    assert!(output.contains("name|description"));
    assert!(output.contains("\"Smith, John\"|\"Senior Developer\""));
    
    // Test CRLF line endings
    let mut buf = Vec::new();
    {
        let mut writer = new_writer(&mut buf).use_crlf(true);
        writer.write_all(&records).unwrap();
        writer.flush().unwrap();
    }
    let output = String::from_utf8(buf).unwrap();
    assert!(output.contains("\r\n"));
    
    // Test always quote
    let simple_records = vec![
        vec!["name".to_string(), "age".to_string()],
        vec!["Alice".to_string(), "30".to_string()],
    ];
    let mut buf = Vec::new();
    {
        let mut writer = new_writer(&mut buf).always_quote(true);
        writer.write_all(&simple_records).unwrap();
        writer.flush().unwrap();
    }
    let output = String::from_utf8(buf).unwrap();
    assert!(output.contains("\"name\",\"age\""));
    assert!(output.contains("\"Alice\",\"30\""));
}

#[test]
fn test_column_reader_functionality() {
    let csv_data = "name,age,email,registered\nAlice,30,alice@example.com,true\nBob,25,bob@example.com,false";
    let cursor = Cursor::new(csv_data);
    let mut reader = new_column_reader(cursor);
    
    reader.read_header().unwrap();
    assert_eq!(reader.columns(), &["name", "age", "email", "registered"]);
    assert!(reader.has_column("name"));
    assert!(reader.has_column("age"));
    assert!(!reader.has_column("nonexistent"));
    
    // Test first record
    assert!(reader.next());
    assert_eq!(reader.get("name").unwrap(), "Alice");
    assert_eq!(reader.get("age").unwrap(), "30");
    assert_eq!(reader.get("email").unwrap(), "alice@example.com");
    assert_eq!(reader.get("registered").unwrap(), "true");
    
    // Test type conversions
    assert_eq!(reader.get_int("age").unwrap(), 30);
    assert_eq!(reader.get_bool("registered").unwrap(), true);
    
    // Test second record
    assert!(reader.next());
    assert_eq!(reader.get("name").unwrap(), "Bob");
    assert_eq!(reader.get_int("age").unwrap(), 25);
    assert_eq!(reader.get_bool("registered").unwrap(), false);
    
    // Test get_all
    let all_fields = reader.get_all().unwrap();
    assert_eq!(all_fields.get("name"), Some(&"Bob".to_string()));
    assert_eq!(all_fields.get("age"), Some(&"25".to_string()));
    
    // No more records
    assert!(!reader.next());
}

#[test]
fn test_streaming_functionality() {
    let csv_data = "name,age,city\nAlice,30,New York\nBob,25,San Francisco\nCharlie,35,Chicago";
    let cursor = Cursor::new(csv_data);
    let mut streamer = new_streamer(cursor);
    
    let mut processed_records = Vec::new();
    let count = streamer.process(|record, header| {
        assert_eq!(header, &["name", "age", "city"]);
        processed_records.push(record.to_vec());
        Ok(())
    }).unwrap();
    
    assert_eq!(count, 3); // 3 data records
    assert_eq!(processed_records.len(), 3);
    assert_eq!(processed_records[0], vec!["Alice", "30", "New York"]);
    assert_eq!(processed_records[1], vec!["Bob", "25", "San Francisco"]);
    assert_eq!(processed_records[2], vec!["Charlie", "35", "Chicago"]);
    
    // Test statistics
    let stats = streamer.statistics();
    assert_eq!(stats.records_processed, 3);
    assert_eq!(stats.has_header, true);
    assert_eq!(stats.header_columns, 3);
}

#[test]
fn test_streaming_batched_processing() {
    let csv_data = "name,age\nAlice,30\nBob,25\nCharlie,35\nDave,40\nEve,28";
    let cursor = Cursor::new(csv_data);
    let mut streamer = new_streamer(cursor).batch_size(2);
    
    let mut batch_counts = Vec::new();
    let count = streamer.process_batched(|batch, header| {
        assert_eq!(header, &["name", "age"]);
        batch_counts.push(batch.len());
        Ok(())
    }).unwrap();
    
    assert_eq!(count, 5); // 5 data records
    assert_eq!(batch_counts, vec![2, 2, 1]); // 2 full batches + 1 partial
}

#[test]
fn test_streaming_utilities() {
    let csv_data = "name,age\nAlice,30\nBob,25\nCharlie,35";
    
    // Test collect
    let cursor = Cursor::new(csv_data);
    let mut streamer = new_streamer(cursor);
    let records = streamer.collect().unwrap();
    assert_eq!(records.len(), 3);
    
    // Test count
    let cursor = Cursor::new(csv_data);
    let mut streamer = new_streamer(cursor);
    let count = streamer.count().unwrap();
    assert_eq!(count, 3);
    
    // Test map
    let cursor = Cursor::new(csv_data);
    let mut streamer = new_streamer(cursor);
    let names: Vec<String> = streamer.map(|record, _header| {
        Ok(record[0].clone())
    }).unwrap();
    assert_eq!(names, vec!["Alice", "Bob", "Charlie"]);
    
    // Test filter
    let cursor = Cursor::new(csv_data);
    let mut streamer = new_streamer(cursor);
    let filtered = streamer.filter(|record, _header| {
        let age: i32 = record[1].parse().unwrap_or(0);
        Ok(age >= 30)
    }).unwrap();
    assert_eq!(filtered.len(), 2); // Alice and Charlie
}

#[test]
fn test_schema_validation() {
    let mut schema = new_schema();
    schema.require_column("name").non_empty();
    schema.require_column_with_type("age", ColumnType::Integer).with_range(18.0, 65.0);
    schema.require_column_with_type("email", ColumnType::Email);
    schema.require_column_with_type("active", ColumnType::Boolean);
    
    // Valid data
    let valid_csv = "name,age,email,active\nAlice,30,alice@example.com,true\nBob,25,bob@example.com,false";
    let cursor = Cursor::new(valid_csv);
    let result = schema.validate(cursor);
    assert!(result.is_valid());
    assert_eq!(result.valid_records, 2);
    assert_eq!(result.total_records, 2);
    assert_eq!(result.success_rate(), 1.0);
    
    // Invalid data
    let invalid_csv = "name,age,email,active\n,120,invalid-email,maybe\nBob,25,bob@example.com,true";
    let cursor = Cursor::new(invalid_csv);
    let result = schema.validate(cursor);
    assert!(!result.is_valid());
    assert!(result.error_count() > 0);
    assert_eq!(result.valid_records, 1);
    assert_eq!(result.total_records, 2);
}

#[test]
fn test_schema_validation_types() {
    let mut schema = new_schema();
    schema.require_column_with_type("integer_field", ColumnType::Integer);
    schema.require_column_with_type("float_field", ColumnType::Float);
    schema.require_column_with_type("boolean_field", ColumnType::Boolean);
    schema.require_column_with_type("email_field", ColumnType::Email);
    schema.require_column_with_type("url_field", ColumnType::Url);
    
    let csv_data = "integer_field,float_field,boolean_field,email_field,url_field\n42,3.14,true,test@example.com,https://example.com\nabc,def,maybe,invalid,not-url";
    let cursor = Cursor::new(csv_data);
    let result = schema.validate(cursor);
    
    assert!(!result.is_valid());
    assert_eq!(result.valid_records, 1);
    assert_eq!(result.total_records, 2);
    assert_eq!(result.error_count(), 5); // All fields in second record are invalid
}

#[test]
fn test_schema_constraints() {
    let mut schema = new_schema();
    schema.require_column("code").with_length_range(3, 5);
    schema.require_column("age").with_range(0.0, 120.0);
    schema.require_column("status").with_allowed_values(vec![
        "active".to_string(),
        "inactive".to_string(),
        "pending".to_string(),
    ]);
    
    let csv_data = "code,age,status\nABC,30,active\nA,150,invalid\nABCDEF,25,pending";
    let cursor = Cursor::new(csv_data);
    let result = schema.validate(cursor);
    
    assert!(!result.is_valid());
    assert_eq!(result.valid_records, 2); // First and third records
    assert_eq!(result.total_records, 3);
    assert_eq!(result.error_count(), 3); // Short code, high age, invalid status
}

#[test]
fn test_transformation_functionality() {
    let csv_data = "first_name,last_name,age\nalice,smith,30\nbob,jones,25";
    let cursor = Cursor::new(csv_data);
    let mut transformer = new_transformer(cursor);
    
    transformer
        .map_column("first_name", |value| Ok(value.to_uppercase()))
        .map_column("last_name", |value| Ok(value.to_uppercase()))
        .add_column("full_name", |record| {
            let first = record.get("first_name").cloned().unwrap_or_default();
            let last = record.get("last_name").cloned().unwrap_or_default();
            Ok(format!("{} {}", first, last))
        })
        .add_column("status", |record| {
            let age: i32 = record.get("age").unwrap_or(&"0".to_string()).parse().unwrap_or(0);
            Ok(if age >= 30 { "SENIOR".to_string() } else { "JUNIOR".to_string() })
        });
    
    let result = transformer.transform().unwrap();
    assert_eq!(result.len(), 3); // header + 2 records
    assert_eq!(result[0], vec!["first_name", "last_name", "age", "full_name", "status"]);
    assert_eq!(result[1], vec!["ALICE", "SMITH", "30", "ALICE SMITH", "SENIOR"]);
    assert_eq!(result[2], vec!["BOB", "JONES", "25", "BOB JONES", "JUNIOR"]);
}

#[test]
fn test_transformation_operations() {
    let csv_data = "name,age,secret,city\nAlice,30,password,NYC\nBob,17,123456,SF\nCharlie,25,secret,LA";
    let cursor = Cursor::new(csv_data);
    let mut transformer = new_transformer(cursor);
    
    transformer
        .remove_column("secret") // Remove sensitive data
        .rename_column("name", "full_name") // Rename column
        .reorder_columns(vec!["age".to_string(), "full_name".to_string(), "city".to_string()]) // Reorder
        .filter_rows(|record| { // Filter adults only
            let age: i32 = record.get("age").unwrap_or(&"0".to_string()).parse().unwrap_or(0);
            Ok(age >= 18)
        });
    
    let result = transformer.transform().unwrap();
    assert_eq!(result.len(), 3); // header + 2 adult records
    assert_eq!(result[0], vec!["age", "full_name", "city"]);
    assert_eq!(result[1], vec!["30", "Alice", "NYC"]);
    assert_eq!(result[2], vec!["25", "Charlie", "LA"]);
    // Bob (age 17) filtered out
}

#[test]
fn test_error_handling() {
    // Test malformed CSV
    let malformed_csv = "name,age\nAlice,30\n\"Bob,25"; // Unterminated quote
    let cursor = Cursor::new(malformed_csv);
    let mut reader = new_reader(cursor);
    
    let result = reader.read_all();
    assert!(result.is_err());
    if let Err(CsvError::Parse(parse_error)) = result {
        assert!(parse_error.message.contains("unterminated"));
    }
    
    // Test field count mismatch
    let mismatched_csv = "name,age\nAlice,30,extra";
    let cursor = Cursor::new(mismatched_csv);
    let mut reader = new_reader(cursor);
    
    let result = reader.read_all();
    assert!(result.is_err());
    if let Err(CsvError::FieldCountMismatch { expected, actual, line }) = result {
        assert_eq!(expected, 2);
        assert_eq!(actual, 3);
        assert_eq!(line, 2);
    }
    
    // Test column not found
    let csv_data = "name,age\nAlice,30";
    let cursor = Cursor::new(csv_data);
    let mut reader = new_column_reader(cursor);
    reader.read_header().unwrap();
    reader.next();
    
    let result = reader.get("nonexistent");
    assert!(result.is_err());
    if let Err(CsvError::ColumnNotFound(name)) = result {
        assert_eq!(name, "nonexistent");
    }
    
    // Test type conversion error
    let result = reader.get_int("name"); // "Alice" is not an integer
    assert!(result.is_err());
    if let Err(CsvError::TypeConversion { field, expected_type, value, .. }) = result {
        assert_eq!(field, "name");
        assert_eq!(expected_type, "integer");
        assert_eq!(value, "Alice");
    }
}

#[test]
fn test_complex_csv_features() {
    // Test quoted fields with commas and quotes
    let complex_csv = r#"name,description,tags
"Smith, John","Senior Developer, ""Full Stack""","java,spring,aws"
"Jane Doe","UX Designer","design,figma,ui"
"O'Connor, Mike","Data Scientist","python,""machine learning"",sql""#;
    
    let cursor = Cursor::new(complex_csv);
    let mut reader = new_reader(cursor);
    
    let records = reader.read_all().unwrap();
    assert_eq!(records.len(), 4); // header + 3 records
    
    // Test first record
    assert_eq!(records[1][0], "Smith, John");
    assert_eq!(records[1][1], r#"Senior Developer, "Full Stack""#);
    assert_eq!(records[1][2], "java,spring,aws");
    
    // Test escaped quotes in third record
    assert_eq!(records[3][1], "Data Scientist");
    assert_eq!(records[3][2], r#"python,"machine learning",sql"#);
}

#[test]
fn test_large_file_simulation() {
    // Simulate a larger CSV file
    let mut csv_data = String::from("id,name,email,age,department\n");
    for i in 1..=1000 {
        csv_data.push_str(&format!("{},User{},user{}@example.com,{},Dept{}\n", 
                                  i, i, i, 20 + (i % 50), (i % 5) + 1));
    }
    
    // Test streaming with large file
    let cursor = Cursor::new(csv_data.clone());
    let mut streamer = new_streamer(cursor).batch_size(100);
    
    let count = streamer.count().unwrap();
    assert_eq!(count, 1000);
    
    let stats = streamer.statistics();
    assert_eq!(stats.records_processed, 1000);
    assert_eq!(stats.header_columns, 5);
    
    // Test column reader with large file
    let cursor = Cursor::new(csv_data);
    let mut reader = new_column_reader(cursor);
    reader.read_header().unwrap();
    
    let mut processed = 0;
    while reader.next() {
        processed += 1;
        if processed == 1 {
            assert_eq!(reader.get("name").unwrap(), "User1");
            assert_eq!(reader.get_int("age").unwrap(), 21);
        }
        if processed == 1000 {
            assert_eq!(reader.get("name").unwrap(), "User1000");
            assert_eq!(reader.get_int("age").unwrap(), 20);
        }
    }
    assert_eq!(processed, 1000);
}

#[test]
fn test_unicode_support() {
    let unicode_csv = "名前,年齢,都市\n田中太郎,30,東京\n佐藤花子,25,大阪";
    
    let cursor = Cursor::new(unicode_csv);
    let mut reader = new_reader(cursor);
    
    let records = reader.read_all().unwrap();
    assert_eq!(records.len(), 3);
    assert_eq!(records[0], vec!["名前", "年齢", "都市"]);
    assert_eq!(records[1], vec!["田中太郎", "30", "東京"]);
    assert_eq!(records[2], vec!["佐藤花子", "25", "大阪"]);
    
    // Test with column reader
    let cursor = Cursor::new(unicode_csv);
    let mut reader = new_column_reader(cursor);
    reader.read_header().unwrap();
    reader.next();
    
    assert_eq!(reader.get("名前").unwrap(), "田中太郎");
    assert_eq!(reader.get_int("年齢").unwrap(), 30);
}

#[test]
fn test_end_to_end_workflow() {
    // Complete workflow: Read -> Validate -> Transform -> Write
    let input_csv = "first_name,last_name,age,email,active\nAlice,Smith,30,alice@example.com,true\nBob,Jones,17,bob@invalid,false\nCharlie,Brown,25,charlie@example.com,true";
    
    // Step 1: Validate the input
    let mut schema = new_schema();
    schema.require_column("first_name").non_empty();
    schema.require_column("last_name").non_empty();
    schema.require_column_with_type("age", ColumnType::Integer).with_range(18.0, 65.0);
    schema.require_column_with_type("email", ColumnType::Email);
    schema.require_column_with_type("active", ColumnType::Boolean);
    
    let cursor = Cursor::new(input_csv);
    let validation_result = schema.validate(cursor);
    assert!(!validation_result.is_valid()); // Bob has invalid age and email
    
    // Step 2: Transform the data (filter valid records and add computed fields)
    let cursor = Cursor::new(input_csv);
    let mut transformer = new_transformer(cursor);
    
    transformer
        .add_column("full_name", |record| {
            let first = record.get("first_name").cloned().unwrap_or_default();
            let last = record.get("last_name").cloned().unwrap_or_default();
            Ok(format!("{} {}", first, last))
        })
        .add_column("status", |record| {
            let active = record.get("active").cloned().unwrap_or_default();
            Ok(if active == "true" { "ACTIVE".to_string() } else { "INACTIVE".to_string() })
        })
        .filter_rows(|record| {
            // Filter adults with valid emails
            let age: i32 = record.get("age").unwrap_or(&"0".to_string()).parse().unwrap_or(0);
            let email = record.get("email").unwrap_or(&String::new());
            Ok(age >= 18 && email.contains("@") && email.contains("."))
        })
        .remove_column("active") // Remove original boolean field
        .reorder_columns(vec![
            "full_name".to_string(),
            "age".to_string(),
            "email".to_string(),
            "status".to_string(),
        ]);
    
    // Step 3: Get the transformed result
    let result = transformer.transform().unwrap();
    assert_eq!(result.len(), 3); // header + 2 valid records (Bob filtered out)
    assert_eq!(result[0], vec!["full_name", "age", "email", "status"]);
    assert_eq!(result[1], vec!["Alice Smith", "30", "alice@example.com", "ACTIVE"]);
    assert_eq!(result[2], vec!["Charlie Brown", "25", "charlie@example.com", "ACTIVE"]);
    
    // Step 4: Write the result back to CSV
    let output_csv = write_all_to_string(&result).unwrap();
    assert!(output_csv.contains("full_name,age,email,status"));
    assert!(output_csv.contains("Alice Smith,30,alice@example.com,ACTIVE"));
    assert!(output_csv.contains("Charlie Brown,25,charlie@example.com,ACTIVE"));
    assert!(!output_csv.contains("Bob")); // Filtered out
}
