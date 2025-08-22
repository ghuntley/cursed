yeet "csv_rfc4180"
yeet "testz"

fr fr Comprehensive RFC 4180 compliance testing
slay test_rfc4180_crlf_support() {
    test_start("RFC 4180 CRLF Line Endings")
    
    fr fr Test CRLF line endings (RFC 4180 standard)
    sus csv_crlf tea = "name,age,city\r\nAlice,30,New York\r\nBob,25,Los Angeles\r\n"
    sus data [[tea]] = parse_rfc4180(csv_crlf)
    
    assert_eq_int(len(data), 3)
    assert_eq_string(data[0][0], "name")
    assert_eq_string(data[1][0], "Alice")
    assert_eq_string(data[2][0], "Bob")
    
    fr fr Test mixed line endings (should handle gracefully)
    sus csv_mixed tea = "name,age\r\nAlice,30\nBob,25\r\n"
    sus data_mixed [[tea]] = parse_rfc4180(csv_mixed)
    assert_eq_int(len(data_mixed), 3)
    
    print_test_summary()
}

slay test_rfc4180_quoted_fields() {
    test_start("RFC 4180 Quoted Fields with Newlines")
    
    fr fr Test newlines inside quoted fields
    sus csv_newlines tea = "name,description\r\n\"Alice\",\"Line 1\r\nLine 2\r\nLine 3\"\r\n\"Bob\",\"Simple description\"\r\n"
    sus data [[tea]] = parse_rfc4180(csv_newlines)
    
    assert_eq_int(len(data), 3)
    assert_eq_string(data[0][0], "name")
    assert_eq_string(data[1][0], "Alice")
    assert_true(string_contains(data[1][1], "\r\n"))
    assert_eq_string(data[2][0], "Bob")
    
    print_test_summary()
}

slay test_rfc4180_escaped_quotes() {
    test_start("RFC 4180 Escaped Quotes")
    
    fr fr Test proper quote escaping
    sus csv_quotes tea = "name,quote\r\n\"Alice\",\"She said \"\"Hello\"\" to me\"\r\n\"Bob\",\"He replied \"\"Hi\"\" back\"\r\n"
    sus data [[tea]] = parse_rfc4180(csv_quotes)
    
    assert_eq_int(len(data), 3)
    assert_eq_string(data[1][1], "She said \"Hello\" to me")
    assert_eq_string(data[2][1], "He replied \"Hi\" back")
    
    print_test_summary()
}

slay test_rfc4180_field_consistency() {
    test_start("RFC 4180 Field Count Consistency")
    
    fr fr Test valid consistent fields
    sus csv_valid tea = "name,age,city\r\nAlice,30,NYC\r\nBob,25,LA\r\n"
    sus valid1 lit = validate_rfc4180(csv_valid)
    assert_true(valid1)
    
    fr fr Test invalid inconsistent fields
    sus csv_invalid tea = "name,age,city\r\nAlice,30\r\nBob,25,LA,Extra\r\n"
    sus valid2 lit = validate_rfc4180(csv_invalid)
    assert_false(valid2)
    
    print_test_summary()
}

slay test_rfc4180_space_preservation() {
    test_start("RFC 4180 Space Preservation")
    
    fr fr RFC 4180: Spaces are significant and must be preserved
    sus csv_spaces tea = "name, age ,city\r\n Alice , 30 , New York \r\n Bob , 25 , Los Angeles \r\n"
    sus data [[tea]] = parse_rfc4180(csv_spaces)
    
    assert_eq_int(len(data), 3)
    assert_eq_string(data[0][1], " age ") fr fr Spaces preserved in header
    assert_eq_string(data[1][0], " Alice ") fr fr Spaces preserved in data
    assert_eq_string(data[1][2], " New York ") fr fr Trailing/leading spaces preserved
    
    print_test_summary()
}

slay test_rfc4180_writing() {
    test_start("RFC 4180 Compliant Writing")
    
    sus data [[tea]] = [
        ["name", "description", "notes"],
        ["Alice", "Software Engineer", "Works on backend"],
        ["Bob", "Designer,Creative", "Specializes in \"UI/UX\""],
        ["Charlie", "Manager\r\nTeam Lead", "Multi-line\r\nrole description"]
    ]
    
    sus csv_output tea = write_rfc4180(data)
    
    fr fr Verify CRLF line endings
    assert_true(string_contains(csv_output, "\r\n"))
    
    fr fr Verify proper quoting of fields with commas
    assert_true(string_contains(csv_output, "\"Designer,Creative\""))
    
    fr fr Verify proper quote escaping
    assert_true(string_contains(csv_output, "\"Specializes in \"\"UI/UX\"\"\""))
    
    fr fr Test round-trip parsing
    sus parsed_back [[tea]] = parse_rfc4180(csv_output)
    assert_eq_int(len(parsed_back), 4)
    assert_eq_string(parsed_back[2][1], "Designer,Creative")
    assert_eq_string(parsed_back[2][2], "Specializes in \"UI/UX\"")
    
    print_test_summary()
}

slay test_rfc4180_custom_delimiters() {
    test_start("RFC 4180 Custom Delimiters")
    
    fr fr Test tab-separated values (TSV)
    sus tsv_data tea = "name\tage\tcity\r\nAlice\t30\tNew York\r\nBob\t25\tLos Angeles\r\n"
    sus data [[tea]] = parse_rfc4180_with_delimiter(tsv_data, "\t")
    
    assert_eq_int(len(data), 3)
    assert_eq_string(data[1][0], "Alice")
    assert_eq_string(data[1][2], "New York")
    
    fr fr Test pipe-separated values
    sus psv_data tea = "name|age|city\r\nAlice|30|New York\r\nBob|25|Los Angeles\r\n"
    sus data_pipe [[tea]] = parse_rfc4180_with_delimiter(psv_data, "|")
    
    assert_eq_int(len(data_pipe), 3)
    assert_eq_string(data_pipe[1][1], "30")
    
    print_test_summary()
}

slay test_rfc4180_type_inference() {
    test_start("RFC 4180 Type Inference")
    
    sus csv_data tea = "name,age,active,score\r\nAlice,30,true,95.5\r\nBob,25,false,87.2\r\nCharlie,35,true,92.8\r\n"
    sus data [[tea]] = parse_rfc4180(csv_data)
    sus types [tea] = infer_column_types(data[1:], data[0])
    
    assert_eq_int(len(types), 4)
    assert_eq_string(types[0], "string")  fr fr name
    assert_eq_string(types[1], "number")  fr fr age  
    assert_eq_string(types[2], "boolean") fr fr active
    assert_eq_string(types[3], "number")  fr fr score
    
    print_test_summary()
}

slay test_rfc4180_comprehensive_validation() {
    test_start("RFC 4180 Comprehensive Validation")
    
    sus valid_csv tea = "name,age,city\r\nAlice,30,NYC\r\nBob,25,LA\r\n"
    sus result CsvValidationResult = validate_comprehensive(valid_csv)
    
    assert_true(result.is_valid)
    assert_eq_int(len(result.errors), 0)
    assert_eq_int(len(result.field_counts), 3)
    
    fr fr Test invalid CSV
    sus invalid_csv tea = "name,age\r\nAlice,30,extra\r\nBob\r\n"
    sus invalid_result CsvValidationResult = validate_comprehensive(invalid_csv)
    
    assert_false(invalid_result.is_valid)
    assert_true(len(invalid_result.errors) > 0)
    
    print_test_summary()
}

slay test_rfc4180_streaming() {
    test_start("RFC 4180 Streaming Reader")
    
    sus large_csv tea = "name,age,city\r\nAlice,30,NYC\r\nBob,25,LA\r\nCharlie,35,SF\r\nDiana,28,Seattle\r\nEve,32,Portland\r\n"
    sus stream_reader CsvStreamReader = new_stream_reader(large_csv, 2)
    
    fr fr Read headers
    sus has_headers lit = read_headers(&stream_reader)
    assert_true(has_headers)
    assert_eq_int(len(stream_reader.headers), 3)
    
    fr fr Read first batch
    sus batch1 [[tea]] = read_batch(&stream_reader)
    assert_eq_int(len(batch1), 2)
    
    fr fr Read second batch  
    sus batch2 [[tea]] = read_batch(&stream_reader)
    assert_eq_int(len(batch2), 2)
    
    fr fr Read final batch
    sus batch3 [[tea]] = read_batch(&stream_reader)
    assert_eq_int(len(batch3), 1)
    
    print_test_summary()
}

slay test_rfc4180_edge_cases() {
    test_start("RFC 4180 Edge Cases")
    
    fr fr Test empty fields
    sus csv_empty tea = "name,age,notes\r\nAlice,,\r\n,25,\r\n\"\",\"\",\"\"\r\n"
    sus data [[tea]] = parse_rfc4180(csv_empty)
    
    assert_eq_int(len(data), 4)
    assert_eq_string(data[1][1], "")  fr fr Empty unquoted field
    assert_eq_string(data[2][0], "")  fr fr Empty unquoted field
    assert_eq_string(data[3][0], "")  fr fr Empty quoted field
    
    fr fr Test single field records
    sus csv_single tea = "value\r\n42\r\n\"hello\"\r\n"
    sus single_data [[tea]] = parse_rfc4180(csv_single)
    assert_eq_int(len(single_data), 3)
    assert_eq_string(single_data[1][0], "42")
    assert_eq_string(single_data[2][0], "hello")
    
    fr fr Test trailing commas (should create empty fields)
    sus csv_trailing tea = "a,b,\r\n1,2,\r\n"
    sus trailing_data [[tea]] = parse_rfc4180(csv_trailing)
    assert_eq_int(len(trailing_data[0]), 3) fr fr Should have 3 fields including empty one
    assert_eq_string(trailing_data[0][2], "")
    
    print_test_summary()
}

slay main() {
    test_rfc4180_crlf_support()
    test_rfc4180_quoted_fields()
    test_rfc4180_escaped_quotes()
    test_rfc4180_field_consistency()
    test_rfc4180_space_preservation()
    test_rfc4180_writing()
    test_rfc4180_custom_delimiters()
    test_rfc4180_type_inference()
    test_rfc4180_comprehensive_validation()
    test_rfc4180_streaming()
    test_rfc4180_edge_cases()
}
