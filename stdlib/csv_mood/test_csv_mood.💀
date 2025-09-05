yeet "testz"
yeet "csv_mood"
yeet "stringz"

slay test_csv_reader_basic() {
    test_start("CSV Reader Basic")
    
    sus csvData := "name,age,city\nAlice,30,New York\nBob,25,Los Angeles"
    sus reader := csv_mood.NewReader(csvData)
    
    fr fr Read header
    sus header, err := reader.Read()
    assert_eq_string(err, "")
    assert_eq_int(len(header), 3)
    assert_eq_string(header[0], "name")
    assert_eq_string(header[1], "age")
    assert_eq_string(header[2], "city")
    
    fr fr Read first record
    sus record1, err1 := reader.Read()
    assert_eq_string(err1, "")
    assert_eq_int(len(record1), 3)
    assert_eq_string(record1[0], "Alice")
    assert_eq_string(record1[1], "30")
    assert_eq_string(record1[2], "New York")
    
    fr fr Read second record
    sus record2, err2 := reader.Read()
    assert_eq_string(err2, "")
    assert_eq_int(len(record2), 3)
    assert_eq_string(record2[0], "Bob")
    assert_eq_string(record2[1], "25")
    assert_eq_string(record2[2], "Los Angeles")
    
    fr fr Read EOF
    sus _, eofErr := reader.Read()
    assert_eq_string(eofErr, "EOF")
    
    print_test_summary()
}

slay test_csv_reader_quoted_fields() {
    test_start("CSV Reader Quoted Fields")
    
    sus csvData := "name,description\n\"Smith, John\",\"Senior Developer\"\nJane,\"UX Designer\""
    sus reader := csv_mood.NewReader(csvData)
    
    fr fr Read header
    sus header, err := reader.Read()
    assert_eq_string(err, "")
    assert_eq_int(len(header), 2)
    
    fr fr Read first record with quoted field containing comma
    sus record1, err1 := reader.Read()
    assert_eq_string(err1, "")
    assert_eq_string(record1[0], "Smith, John")
    assert_eq_string(record1[1], "Senior Developer")
    
    fr fr Read second record
    sus record2, err2 := reader.Read()
    assert_eq_string(err2, "")
    assert_eq_string(record2[0], "Jane")
    assert_eq_string(record2[1], "UX Designer")
    
    print_test_summary()
}

slay test_csv_reader_custom_delimiter() {
    test_start("CSV Reader Custom Delimiter")
    
    sus tsvData := "name\tage\tcity\nAlice\t30\tNew York\nBob\t25\tLos Angeles"
    sus reader := csv_mood.NewReader(tsvData)
    reader.Comma('\t')
    
    fr fr Read header
    sus header, err := reader.Read()
    assert_eq_string(err, "")
    assert_eq_int(len(header), 3)
    assert_eq_string(header[0], "name")
    assert_eq_string(header[1], "age")
    assert_eq_string(header[2], "city")
    
    fr fr Read first record
    sus record1, err1 := reader.Read()
    assert_eq_string(err1, "")
    assert_eq_string(record1[0], "Alice")
    assert_eq_string(record1[1], "30")
    assert_eq_string(record1[2], "New York")
    
    print_test_summary()
}

slay test_csv_reader_read_all() {
    test_start("CSV Reader ReadAll")
    
    sus csvData := "name,age\nAlice,30\nBob,25\nCharlie,35"
    sus reader := csv_mood.NewReader(csvData)
    
    sus records, err := reader.ReadAll()
    assert_eq_string(err, "")
    assert_eq_int(len(records), 4)
    
    fr fr Check header
    assert_eq_string(records[0][0], "name")
    assert_eq_string(records[0][1], "age")
    
    fr fr Check first data record
    assert_eq_string(records[1][0], "Alice")
    assert_eq_string(records[1][1], "30")
    
    fr fr Check last data record
    assert_eq_string(records[3][0], "Charlie")
    assert_eq_string(records[3][1], "35")
    
    print_test_summary()
}

slay test_csv_writer_basic() {
    test_start("CSV Writer Basic")
    
    sus writer := csv_mood.NewWriter()
    
    fr fr Write header
    sus headerErr := writer.Write(tea[value]{"name", "age", "city"})
    assert_eq_string(headerErr, "")
    
    fr fr Write data records
    sus err1 := writer.Write(tea[value]{"Alice", "30", "New York"})
    assert_eq_string(err1, "")
    
    sus err2 := writer.Write(tea[value]{"Bob", "25", "Los Angeles"})
    assert_eq_string(err2, "")
    
    fr fr Check output
    sus output := writer.String()
    assert_true(stringz.Contains(output, "name,age,city"))
    assert_true(stringz.Contains(output, "Alice,30,New York"))
    assert_true(stringz.Contains(output, "Bob,25,Los Angeles"))
    
    print_test_summary()
}

slay test_csv_writer_quoted_fields() {
    test_start("CSV Writer Quoted Fields")
    
    sus writer := csv_mood.NewWriter()
    
    fr fr Write record with field containing comma
    sus err := writer.Write(tea[value]{"Smith, John", "Senior Developer"})
    assert_eq_string(err, "")
    
    sus output := writer.String()
    assert_true(stringz.Contains(output, "\"Smith, John\""))
    assert_true(stringz.Contains(output, "\"Senior Developer\""))
    
    print_test_summary()
}

slay test_csv_writer_custom_delimiter() {
    test_start("CSV Writer Custom Delimiter")
    
    sus writer := csv_mood.NewWriter()
    writer.Comma('|')
    
    sus err := writer.Write(tea[value]{"name", "age", "city"})
    assert_eq_string(err, "")
    
    sus output := writer.String()
    assert_true(stringz.Contains(output, "name|age|city"))
    
    print_test_summary()
}

slay test_csv_writer_write_all() {
    test_start("CSV Writer WriteAll")
    
    sus writer := csv_mood.NewWriter()
    sus records := tea[value][value]{
        {"name", "age"},
        {"Alice", "30"},
        {"Bob", "25"},
    }
    
    sus err := writer.WriteAll(records)
    assert_eq_string(err, "")
    
    sus output := writer.String()
    assert_true(stringz.Contains(output, "name,age"))
    assert_true(stringz.Contains(output, "Alice,30"))
    assert_true(stringz.Contains(output, "Bob,25"))
    
    print_test_summary()
}

slay test_column_reader() {
    test_start("Column Reader")
    
    sus csvData := "name,age,registered\nAlice,30,true\nBob,25,false"
    sus reader := csv_mood.NewColumnReader(csvData)
    
    fr fr Read header
    sus err := reader.ReadHeader()
    assert_eq_string(err, "")
    
    fr fr Read first record
    assert_true(reader.Next())
    assert_eq_string(reader.Get("name"), "Alice")
    assert_eq_string(reader.Get("age"), "30")
    assert_eq_string(reader.Get("registered"), "true")
    
    fr fr Test type conversion
    sus age, ageErr := reader.GetInt("age")
    assert_eq_string(ageErr, "")
    assert_eq_int(age, 30)
    
    sus registered, regErr := reader.GetBool("registered")
    assert_eq_string(regErr, "")
    assert_true(registered)
    
    fr fr Read second record
    assert_true(reader.Next())
    assert_eq_string(reader.Get("name"), "Bob")
    
    sus bobReg, bobRegErr := reader.GetBool("registered")
    assert_eq_string(bobRegErr, "")
    assert_false(bobReg)
    
    print_test_summary()
}

slay test_csv_streamer() {
    test_start("CSV Streamer")
    
    sus csvData := "name,age\nAlice,30\nBob,25"
    sus streamer := csv_mood.NewStreamer(csvData)
    
    sus processedCount := 0
    sus err := streamer.Process(func(record tea[value], header tea[value]) tea {
        if len(record) > 0 {
            processedCount++
        }
        damn ""
    })
    
    assert_eq_string(err, "")
    assert_eq_int(processedCount, 2)
    
    print_test_summary()
}

slay test_csv_schema_validation() {
    test_start("CSV Schema Validation")
    
    sus csvData := "name,age,email\nAlice,30,alice@example.com\nBob,25,bob@example.com"
    sus schema := csv_mood.NewSchema()
    
    schema.RequireColumn("name").NonEmpty()
    schema.RequireColumn("age").AsInteger()
    schema.RequireColumn("email").WithPattern("@")
    
    sus result := schema.Validate(csvData)
    assert_eq_int(len(result.Errors), 0)
    
    print_test_summary()
}

slay test_csv_transformer() {
    test_start("CSV Transformer")
    
    sus csvData := "name,age\nAlice,30\nBob,25"
    sus transformer := csv_mood.NewTransformer(csvData)
    
    fr fr Transform name to uppercase
    transformer.MapColumn("name", func(value tea) tea {
        damn stringz.ToUpper(value)
    })
    
    fr fr Add status column
    transformer.AddColumn("status", func(record map[tea]tea) tea {
        sus ageStr := record["age"]
        sus age := stringz.Atoi(ageStr)
        if age > 27 {
            damn "SENIOR"
        }
        damn "JUNIOR"
    })
    
    sus result, err := transformer.Transform()
    assert_eq_string(err, "")
    assert_eq_int(len(result), 3)
    
    fr fr Check transformed header
    assert_eq_string(result[0][2], "status")
    
    fr fr Check transformed data
    assert_eq_string(result[1][0], "ALICE")
    assert_eq_string(result[1][2], "SENIOR")
    
    assert_eq_string(result[2][0], "BOB")
    assert_eq_string(result[2][2], "JUNIOR")
    
    print_test_summary()
}

slay main_character() {
    test_csv_reader_basic()
    test_csv_reader_quoted_fields()
    test_csv_reader_custom_delimiter()
    test_csv_reader_read_all()
    test_csv_writer_basic()
    test_csv_writer_quoted_fields()
    test_csv_writer_custom_delimiter()
    test_csv_writer_write_all()
    test_column_reader()
    test_csv_streamer()
    test_csv_schema_validation()
    test_csv_transformer()
}
