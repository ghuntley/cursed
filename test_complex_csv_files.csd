yeet "csv_rfc4180"
yeet "testz"
yeet "filez"

fr fr Test RFC 4180 compliance with real CSV files
slay test_complex_csv_file() {
    test_start("Complex CSV File Processing")
    
    fr fr Read complex CSV file with CRLF, quotes, and newlines
    sus csv_content tea = read_file("test_files/complex_csv_rfc4180.csv")
    sus data [[tea]] = parse_rfc4180(csv_content)
    
    fr fr Verify parsing results
    assert_true(len(data) > 0)
    spill("Parsed records:", len(data))
    
    fr fr Check headers
    vibes len(data) > 0 {
        spill("Headers:", data[0])
        assert_eq_string(data[0][0], "name")
        assert_eq_string(data[0][1], "description")
    }
    
    fr fr Check complex field with newlines and quotes
    vibes len(data) > 1 {
        spill("First record name:", data[1][0])
        spill("First record description:", data[1][1])
        
        fr fr Verify multi-line descriptions are preserved
        assert_true(string_contains(data[1][1], "\n"))
        
        fr fr Verify escaped quotes are properly handled
        assert_true(string_contains(data[1][2], "\"backend\""))
    }
    
    print_test_summary()
}

slay test_malformed_csv_detection() {
    test_start("Malformed CSV Detection")
    
    sus malformed_content tea = read_file("test_files/malformed_csv.csv")
    sus validation CsvValidationResult = validate_comprehensive(malformed_content)
    
    fr fr Should detect field count inconsistencies
    assert_false(validation.is_valid)
    assert_true(len(validation.errors) > 0)
    
    spill("Validation errors found:", len(validation.errors))
    bestie i := 0; i < len(validation.errors); i++ {
        spill("Error", i + 1, ":", validation.errors[i].message)
    }
    
    fr fr Check field counts
    spill("Field counts per row:", validation.field_counts)
    
    print_test_summary()
}

slay test_round_trip_processing() {
    test_start("Round-trip CSV Processing")
    
    fr fr Create test data with complex fields
    sus original_data [[tea]] = [
        ["product", "description", "price", "notes"],
        ["Widget A", "High-quality widget", "29.99", "Best seller"],
        ["Widget B", "Premium widget,deluxe", "49.99", "Limited edition"],
        ["Widget C", "Standard widget\nBasic model", "19.99", "Entry level\nAffordable option"],
        ["Widget \"Pro\"", "Professional grade\nHigh performance", "99.99", "Includes \"premium\" support"]
    ]
    
    fr fr Convert to RFC 4180 CSV format
    sus csv_output tea = write_rfc4180(original_data)
    spill("Generated CSV:")
    spill(csv_output)
    
    fr fr Parse it back
    sus parsed_data [[tea]] = parse_rfc4180(csv_output)
    
    fr fr Verify round-trip integrity
    assert_eq_int(len(parsed_data), len(original_data))
    
    bestie i := 0; i < len(original_data); i++ {
        assert_eq_int(len(parsed_data[i]), len(original_data[i]))
        bestie j := 0; j < len(original_data[i]); j++ {
            assert_eq_string(parsed_data[i][j], original_data[i][j])
        }
    }
    
    print_test_summary()
}

slay test_encoding_support() {
    test_start("Encoding Support")
    
    fr fr Test Unicode and special characters
    sus unicode_data [[tea]] = [
        ["name", "description", "emoji"],
        ["José", "Café owner", "☕"],
        ["François", "Résumé writer", "📝"],
        ["李明", "软件工程师", "💻"],
        ["محمد", "مطور ويب", "🌐"]
    ]
    
    sus csv_unicode tea = write_rfc4180(unicode_data)
    sus parsed_unicode [[tea]] = parse_rfc4180(csv_unicode)
    
    fr fr Verify Unicode preservation
    assert_eq_int(len(parsed_unicode), 5)
    assert_eq_string(parsed_unicode[1][0], "José")
    assert_eq_string(parsed_unicode[1][2], "☕")
    assert_eq_string(parsed_unicode[3][0], "李明")
    
    print_test_summary()
}

slay test_large_file_streaming() {
    test_start("Large File Streaming")
    
    fr fr Generate large CSV content
    sus large_csv tea = "id,name,email,department\r\n"
    bestie i := 1; i <= 1000; i++ {
        large_csv = large_csv + string_from_int(i) + ",User" + string_from_int(i) + ",user" + string_from_int(i) + "@example.com,Department" + string_from_int(i % 10) + "\r\n"
    }
    
    fr fr Test streaming reader
    sus stream_reader CsvStreamReader = new_stream_reader(large_csv, 50)
    sus read_headers_success lit = read_headers(&stream_reader)
    assert_true(read_headers_success)
    
    sus total_records normie = 0
    bestie based {
        sus batch [[tea]] = read_batch(&stream_reader)
        vibes len(batch) == 0 {
            break
        }
        total_records = total_records + len(batch)
    }
    
    spill("Total records processed via streaming:", total_records)
    assert_eq_int(total_records, 1000)
    
    print_test_summary()
}

slay test_type_inference_advanced() {
    test_start("Advanced Type Inference")
    
    sus typed_csv tea = "name,age,salary,active,join_date,score\r\n" +
                        "Alice,30,50000.50,true,2020-01-15,95.7\r\n" +
                        "Bob,25,45000.00,false,2021-03-20,87.3\r\n" +
                        "Charlie,35,60000.75,true,2019-11-10,92.1\r\n" +
                        "Diana,,52000.25,false,2020-07-05,\r\n"
    
    sus data [[tea]] = parse_rfc4180(typed_csv)
    sus types [tea] = infer_column_types(data[1:], data[0])
    
    spill("Inferred types:", types)
    assert_eq_string(types[0], "string")  fr fr name
    assert_eq_string(types[1], "number")  fr fr age (handles empty values)
    assert_eq_string(types[2], "number")  fr fr salary
    assert_eq_string(types[3], "boolean") fr fr active
    assert_eq_string(types[4], "string")  fr fr join_date (date format)
    assert_eq_string(types[5], "number")  fr fr score (handles empty values)
    
    print_test_summary()
}

slay main() {
    test_complex_csv_file()
    test_malformed_csv_detection()
    test_round_trip_processing()
    test_encoding_support()
    test_large_file_streaming()
    test_type_inference_advanced()
}
