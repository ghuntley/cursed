yeet "testz"
yeet "csv"

# Test basic CSV parsing
slay test_basic_csv_parsing() {
    test_start("Basic CSV parsing")
    
    sus csv_data tea = "name,age,city\nJohn,25,NYC\nJane,30,LA"
    sus result [[tea]] = csv.parse(csv_data)
    
    assert_eq_int(len(result), 3)
    assert_eq_string(result[0][0], "name")
    assert_eq_string(result[0][1], "age")
    assert_eq_string(result[0][2], "city")
    assert_eq_string(result[1][0], "John")
    assert_eq_string(result[1][1], "25")
    assert_eq_string(result[1][2], "NYC")
}

# Test CSV with different delimiters
slay test_delimiter_detection() {
    test_start("Delimiter detection")
    
    sus semicolon_csv tea = "name;age;city\nJohn;25;NYC"
    sus tab_csv tea = "name\tage\tcity\nJohn\t25\tNYC"
    
    assert_eq_string(csv.detect_delimiter(semicolon_csv), ";")
    assert_eq_string(csv.detect_delimiter(tab_csv), "\t")
    
    sus semicolon_result [[tea]] = csv.parse(semicolon_csv)
    assert_eq_string(semicolon_result[0][0], "name")
    assert_eq_string(semicolon_result[1][1], "25")
}

# Test CSV with quotes and special characters
slay test_quoted_fields() {
    test_start("Quoted fields")
    
    sus csv_data tea = "name,description\n\"John Doe\",\"A person with, comma\"\n\"Jane\",\"Has \"\"quotes\"\" inside\""
    sus result [[tea]] = csv.parse(csv_data)
    
    assert_eq_string(result[1][0], "John Doe")
    assert_eq_string(result[1][1], "A person with, comma")
    assert_eq_string(result[2][1], "Has \"quotes\" inside")
}

# Test escape and unescape functions
slay test_escape_unescape() {
    test_start("Escape/unescape functions")
    
    sus field_with_comma tea = "value, with comma"
    sus escaped tea = csv.escape_field(field_with_comma)
    sus unescaped tea = csv.unescape_field(escaped)
    
    assert_eq_string(escaped, "\"value, with comma\"")
    assert_eq_string(unescaped, "value, with comma")
    
    sus field_with_quotes tea = "value with \"quotes\""
    sus escaped_quotes tea = csv.escape_field(field_with_quotes)
    sus unescaped_quotes tea = csv.unescape_field(escaped_quotes)
    
    assert_eq_string(unescaped_quotes, "value with \"quotes\"")
}

# Test empty fields and rows
slay test_empty_fields() {
    test_start("Empty fields and rows")
    
    sus csv_data tea = "name,age,city\n,25,\nJohn,,NYC"
    sus result [[tea]] = csv.parse(csv_data)
    
    assert_eq_int(len(result), 3)
    assert_eq_string(result[1][0], "")
    assert_eq_string(result[1][2], "")
    assert_eq_string(result[2][1], "")
}

# Test CSV with headers
slay test_parse_with_headers() {
    test_start("Parse with headers")
    
    sus csv_data tea = "name,age,city\nJohn,25,NYC\nJane,30,LA"
    sus result [[tea]] = csv.parse_with_headers(csv_data)
    
    assert_eq_int(len(result), 2)
    # First record should be John's data as key-value pairs
    sus first_record [tea] = result[0]
    assert_eq_string(first_record[0], "name:John")
    assert_eq_string(first_record[1], "age:25")
}

# Test stringify function
slay test_stringify() {
    test_start("Stringify function")
    
    sus data [[tea]] = [["name", "age"], ["John", "25"], ["Jane", "30"]]
    sus result tea = csv.stringify(data)
    
    assert_eq_string(result, "name,age\nJohn,25\nJane,30")
}

# Test stringify with headers
slay test_stringify_with_headers() {
    test_start("Stringify with headers")
    
    sus data [[tea]] = [["John", "25"], ["Jane", "30"]]
    sus headers [tea] = ["name", "age"]
    sus result tea = csv.stringify_with_headers(data, headers)
    
    assert_eq_string(result, "name,age\nJohn,25\nJane,30")
}

# Test round-trip: parse then stringify
slay test_round_trip() {
    test_start("Round-trip consistency")
    
    sus original tea = "name,age,city\nJohn,25,NYC\nJane,30,LA"
    sus parsed [[tea]] = csv.parse(original)
    sus stringified tea = csv.stringify(parsed)
    
    assert_eq_string(original, stringified)
}

# Test row and column counting
slay test_count_functions() {
    test_start("Count functions")
    
    sus csv_data tea = "name,age,city\nJohn,25,NYC\nJane,30,LA"
    
    assert_eq_int(csv.count_rows(csv_data), 3)
    assert_eq_int(csv.count_columns(csv_data), 3)
}

# Test header extraction
slay test_get_headers() {
    test_start("Get headers")
    
    sus csv_data tea = "name,age,city\nJohn,25,NYC"
    sus headers [tea] = csv.get_headers(csv_data)
    
    assert_eq_int(len(headers), 3)
    assert_eq_string(headers[0], "name")
    assert_eq_string(headers[1], "age")
    assert_eq_string(headers[2], "city")
}

# Test CSV validation
slay test_validate() {
    test_start("CSV validation")
    
    sus valid_csv tea = "name,age\nJohn,25\nJane,30"
    sus invalid_csv tea = "name,age\nJohn,25,NYC\nJane,30"
    
    assert_true(csv.validate(valid_csv))
    assert_false(csv.validate(invalid_csv))
}

# Test filtering rows
slay test_filter_rows() {
    test_start("Filter rows")
    
    sus data [[tea]] = [["name", "age"], ["John", "25"], ["Jane", "30"], ["John", "35"]]
    sus filtered [[tea]] = csv.filter_rows(data, 0, "John")
    
    assert_eq_int(len(filtered), 2)
    assert_eq_string(filtered[0][0], "John")
    assert_eq_string(filtered[1][0], "John")
}

# Test sorting by column
slay test_sort_by_column() {
    test_start("Sort by column")
    
    sus data [[tea]] = [["name", "age"], ["John", "30"], ["Jane", "25"], ["Bob", "35"]]
    sus sorted [[tea]] = csv.sort_by_column(data, 1)
    
    assert_eq_string(sorted[0][1], "25")
    assert_eq_string(sorted[1][1], "30")
    assert_eq_string(sorted[2][1], "35")
}

# Test get column
slay test_get_column() {
    test_start("Get column")
    
    sus data [[tea]] = [["name", "age"], ["John", "25"], ["Jane", "30"]]
    sus names [tea] = csv.get_column(data, 0)
    sus ages [tea] = csv.get_column(data, 1)
    
    assert_eq_string(names[0], "name")
    assert_eq_string(names[1], "John")
    assert_eq_string(ages[1], "25")
}

# Test remove column
slay test_remove_column() {
    test_start("Remove column")
    
    sus data [[tea]] = [["name", "age", "city"], ["John", "25", "NYC"], ["Jane", "30", "LA"]]
    sus result [[tea]] = csv.remove_column(data, 1)
    
    assert_eq_int(len(result[0]), 2)
    assert_eq_string(result[0][0], "name")
    assert_eq_string(result[0][1], "city")
    assert_eq_string(result[1][0], "John")
    assert_eq_string(result[1][1], "NYC")
}

# Test transpose
slay test_transpose() {
    test_start("Transpose")
    
    sus data [[tea]] = [["name", "age"], ["John", "25"], ["Jane", "30"]]
    sus transposed [[tea]] = csv.transpose(data)
    
    assert_eq_string(transposed[0][0], "name")
    assert_eq_string(transposed[0][1], "John")
    assert_eq_string(transposed[0][2], "Jane")
    assert_eq_string(transposed[1][0], "age")
    assert_eq_string(transposed[1][1], "25")
    assert_eq_string(transposed[1][2], "30")
}

# Test single row CSV
slay test_single_row() {
    test_start("Single row CSV")
    
    sus csv_data tea = "name,age,city"
    sus result [[tea]] = csv.parse(csv_data)
    
    assert_eq_int(len(result), 1)
    assert_eq_int(len(result[0]), 3)
    assert_eq_string(result[0][0], "name")
}

# Test empty CSV
slay test_empty_csv() {
    test_start("Empty CSV")
    
    sus csv_data tea = ""
    sus result [[tea]] = csv.parse(csv_data)
    
    assert_eq_int(len(result), 0)
}

# Test single column CSV
slay test_single_column() {
    test_start("Single column CSV")
    
    sus csv_data tea = "name\nJohn\nJane"
    sus result [[tea]] = csv.parse(csv_data)
    
    assert_eq_int(len(result), 3)
    assert_eq_int(len(result[0]), 1)
    assert_eq_string(result[0][0], "name")
    assert_eq_string(result[1][0], "John")
}

# Test CSV with newlines in fields
slay test_newlines_in_fields() {
    test_start("Newlines in fields")
    
    sus csv_data tea = "name,description\n\"John\",\"Line 1\nLine 2\""
    sus result [[tea]] = csv.parse(csv_data)
    
    assert_eq_string(result[1][0], "John")
    assert_eq_string(result[1][1], "Line 1\nLine 2")
}

# Run all tests
slay main() {
    test_basic_csv_parsing()
    test_delimiter_detection()
    test_quoted_fields()
    test_escape_unescape()
    test_empty_fields()
    test_parse_with_headers()
    test_stringify()
    test_stringify_with_headers()
    test_round_trip()
    test_count_functions()
    test_get_headers()
    test_validate()
    test_filter_rows()
    test_sort_by_column()
    test_get_column()
    test_remove_column()
    test_transpose()
    test_single_row()
    test_empty_csv()
    test_single_column()
    test_newlines_in_fields()
    
    print_test_summary()
}

main()
