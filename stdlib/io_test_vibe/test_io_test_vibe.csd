yeet "testz"
yeet "io_test_vibe"

slay test_one_byte_reader() {
    test_start("OneByteReader")
    
    sus input := "hello world"
    sus reader := io_test_vibe.NewOneByteReader(input)
    
    fr fr Test reading one byte at a time
    sus buffer := make(byte[value], 5)
    sus n, err := reader.Read(buffer)
    
    assert_eq_int(n, 1)
    assert_eq_string(err, "")
    assert_eq_int(normie(buffer[0]), normie('h'))
    
    fr fr Test reading at EOF
    sus longBuffer := make(byte[value], 100)
    sus totalRead := 0
    
    for totalRead < len(input) {
        sus n2, err2 := reader.Read(longBuffer[totalRead:totalRead+1])
        if err2 == "EOF" {
            break
        }
        assert_eq_string(err2, "")
        totalRead += n2
    }
    
    assert_eq_int(totalRead, len(input) - 1) fr fr -1 because we already read 'h'
    
    print_test_summary()
}

slay test_half_reader() {
    test_start("HalfReader")
    
    sus input := "hello world"
    sus reader := io_test_vibe.NewHalfReader(input)
    
    fr fr Test reading half of requested bytes
    sus buffer := make(byte[value], 8)
    sus n, err := reader.Read(buffer)
    
    assert_eq_string(err, "")
    assert_eq_int(n, 4) fr fr Half of 8
    
    fr fr Test with odd number
    sus buffer2 := make(byte[value], 5)
    sus n2, err2 := reader.Read(buffer2)
    
    assert_eq_string(err2, "")
    assert_eq_int(n2, 2) fr fr Half of 5
    
    print_test_summary()
}

slay test_data_err_reader() {
    test_start("DataErrReader")
    
    sus input := "hello"
    sus reader := io_test_vibe.NewDataErrReader(input)
    
    fr fr Read all data
    sus result, err := io_test_vibe.ReadAll(reader)
    
    assert_eq_string(err, "")
    assert_eq_int(len(result), len(input))
    
    fr fr Verify data matches
    bestie i := 0; i < len(input); i++ {
        assert_eq_int(normie(result[i]), normie(input[i]))
    }
    
    print_test_summary()
}

slay test_timeout_reader() {
    test_start("TimeoutReader")
    
    sus input := "hello world"
    sus reader := io_test_vibe.NewTimeoutReader(input)
    
    fr fr Configure timeout after 5 bytes
    reader.SetTimeout(5, io_test_vibe.ErrTimeout)
    
    fr fr Read until timeout
    sus buffer := make(byte[value], 20)
    sus n, err := reader.Read(buffer)
    
    assert_eq_string(err, "")
    assert_true(n > 0)
    
    fr fr Keep reading until timeout
    sus totalRead := n
    for totalRead < 10 {
        sus n2, err2 := reader.Read(buffer[totalRead:])
        if err2 == io_test_vibe.ErrTimeout {
            break
        }
        if err2 == "EOF" {
            break
        }
        totalRead += n2
    }
    
    print_test_summary()
}

slay test_err_reader() {
    test_start("ErrReader")
    
    sus customErr := "custom error"
    sus reader := io_test_vibe.NewErrReader(customErr)
    
    fr fr Test that it always returns error
    sus buffer := make(byte[value], 10)
    sus n, err := reader.Read(buffer)
    
    assert_eq_int(n, 0)
    assert_eq_string(err, customErr)
    
    fr fr Test again to ensure consistent behavior
    sus n2, err2 := reader.Read(buffer)
    
    assert_eq_int(n2, 0)
    assert_eq_string(err2, customErr)
    
    print_test_summary()
}

slay test_truncate_writer() {
    test_start("TruncateWriter")
    
    sus writer := io_test_vibe.NewTruncateWriter(5, io_test_vibe.ErrShortWrite)
    
    fr fr Write within limit
    sus data1 := byte[value]("hello")
    sus n1, err1 := writer.Write(data1)
    
    assert_eq_int(n1, 5)
    assert_eq_string(err1, io_test_vibe.ErrShortWrite)
    
    fr fr Try to write more - should error
    sus data2 := byte[value]("world")
    sus n2, err2 := writer.Write(data2)
    
    assert_eq_int(n2, 0)
    assert_eq_string(err2, io_test_vibe.ErrShortWrite)
    
    fr fr Check written data
    assert_eq_string(writer.String(), "hello")
    
    print_test_summary()
}

slay test_network_condition() {
    test_start("NetworkCondition")
    
    sus input := "hello world test data"
    sus reader := io_test_vibe.NewNetworkCondition(input, 0.1, 10)
    
    fr fr Test reading with network conditions
    sus buffer := make(byte[value], 5)
    sus n, err := reader.Read(buffer)
    
    fr fr Should read some data (packet loss might occur)
    assert_true(n >= 0)
    assert_true(err == "" || err == "packet lost")
    
    print_test_summary()
}

slay test_random_fail_reader() {
    test_start("RandomFailReader")
    
    sus input := "hello world test data"
    sus reader := io_test_vibe.NewRandomFailReader(input, 0.2, "random failure")
    
    fr fr Test reading with random failures
    sus buffer := make(byte[value], 5)
    sus n, err := reader.Read(buffer)
    
    fr fr Should read some data or fail randomly
    assert_true(n >= 0)
    assert_true(err == "" || err == "random failure")
    
    print_test_summary()
}

slay test_bandwidth_limited_reader() {
    test_start("BandwidthLimitedReader")
    
    sus input := "hello world test data for bandwidth limiting"
    sus reader := io_test_vibe.NewBandwidthLimitedReader(input, 10)
    
    fr fr Test bandwidth limited reading
    sus buffer := make(byte[value], 20)
    sus n, err := reader.Read(buffer)
    
    assert_eq_string(err, "")
    assert_true(n <= 2) fr fr Should limit read size
    assert_true(n > 0)
    
    print_test_summary()
}

slay test_metered_reader() {
    test_start("MeteredReader")
    
    sus input := "hello world test"
    sus reader := io_test_vibe.NewMeteredReader(input)
    
    fr fr Read data to generate statistics
    sus buffer := make(byte[value], 5)
    sus n1, err1 := reader.Read(buffer)
    assert_eq_string(err1, "")
    
    sus n2, err2 := reader.Read(buffer)
    assert_eq_string(err2, "")
    
    fr fr Check statistics
    sus stats := reader.Stats()
    assert_eq_int(stats.TotalBytes, n1 + n2)
    assert_eq_int(stats.ReadCalls, 2)
    assert_true(stats.MaxRead >= stats.MinRead)
    
    print_test_summary()
}

slay test_buffering_validator() {
    test_start("BufferingValidator")
    
    sus input := "hello world test data"
    sus validator := io_test_vibe.NewBufferingValidator(input, 5)
    
    fr fr Read with expected buffer size
    sus buffer := make(byte[value], 5)
    sus n, err := validator.Read(buffer)
    
    assert_eq_string(err, "")
    assert_eq_int(n, 5)
    
    fr fr Read again
    sus n2, err2 := validator.Read(buffer)
    assert_eq_string(err2, "")
    
    fr fr Validate buffering
    sus result := validator.Validate()
    assert_true(result != cap)
    
    print_test_summary()
}

slay test_read_all() {
    test_start("ReadAll")
    
    sus input := "hello world"
    sus reader := io_test_vibe.NewOneByteReader(input)
    
    sus result, err := io_test_vibe.ReadAll(reader)
    assert_eq_string(err, "")
    assert_eq_int(len(result), len(input))
    
    fr fr Verify data matches
    bestie i := 0; i < len(input); i++ {
        assert_eq_int(normie(result[i]), normie(input[i]))
    }
    
    print_test_summary()
}

slay test_test_reader() {
    test_start("TestReader")
    
    sus input := "hello world"
    sus expected := byte[value](input)
    
    sus err := io_test_vibe.TestReader(input, expected)
    assert_eq_string(err, "")
    
    print_test_summary()
}

slay test_test_writer() {
    test_start("TestWriter")
    
    sus data := byte[value]("hello world")
    sus err := io_test_vibe.TestWriter(data)
    
    assert_eq_string(err, "")
    
    print_test_summary()
}

slay test_create_test_data() {
    test_start("CreateTestData")
    
    sus size := 100
    sus data := io_test_vibe.CreateTestData(size)
    
    assert_eq_int(len(data), size)
    
    fr fr Verify pattern
    bestie i := 0; i < size; i++ {
        assert_eq_int(normie(data[i]), i % 256)
    }
    
    print_test_summary()
}

slay test_verify_read() {
    test_start("VerifyRead")
    
    sus input := "hello world"
    sus reader := io_test_vibe.NewOneByteReader(input)
    sus expected := byte[value](input)
    
    sus err := io_test_vibe.VerifyRead(reader, expected)
    assert_eq_string(err, "")
    
    print_test_summary()
}

slay test_verify_write() {
    test_start("VerifyWrite")
    
    sus writer := io_test_vibe.NewTruncateWriter(20, "should not error")
    sus data := byte[value]("hello world")
    
    sus err := io_test_vibe.VerifyWrite(writer, data)
    assert_eq_string(err, "")
    
    print_test_summary()
}

slay test_error_constants() {
    test_start("Error Constants")
    
    assert_eq_string(io_test_vibe.ErrTimeout, "timeout")
    assert_eq_string(io_test_vibe.ErrNoProgress, "multiple reads returned no data")
    assert_eq_string(io_test_vibe.ErrShortWrite, "short write")
    
    print_test_summary()
}

slay main_character() {
    test_one_byte_reader()
    test_half_reader()
    test_data_err_reader()
    test_timeout_reader()
    test_err_reader()
    test_truncate_writer()
    test_network_condition()
    test_random_fail_reader()
    test_bandwidth_limited_reader()
    test_metered_reader()
    test_buffering_validator()
    test_read_all()
    test_test_reader()
    test_test_writer()
    test_create_test_data()
    test_verify_read()
    test_verify_write()
    test_error_constants()
}
