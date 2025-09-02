yeet "testz"
yeet "dropz"

fr fr Test dropz core I/O module functionality

slay test_byte_reader() {
    test_start("ByteReader functionality")
    
    sus reader *dropz.ByteReader = dropz.new_byte_reader("Hello, World!")
    sus buffer byte[5]
    
    sus (n, err) = reader.read(buffer[:])
    assert_eq_int(n, 5)
    assert_eq_string(err, "")
    assert_eq_string(string(buffer[:n]), "Hello")
    
    sus (n2, err2) = reader.read(buffer[:])
    assert_eq_int(n2, 5)
    assert_eq_string(err2, "")
    assert_eq_string(string(buffer[:n2]), ", Wor") fr fr Test EOF
    sus (n3, err3) = reader.read(buffer[:])
    assert_eq_int(n3, 3)
    assert_eq_string(err3, "")
    assert_eq_string(string(buffer[:n3]), "ld!") fr fr Read past EOF
    sus (n4, err4) = reader.read(buffer[:])
    assert_eq_int(n4, 0)
    assert_eq_string(err4, dropz.EOF)
}

slay test_byte_writer() {
    test_start("ByteWriter functionality")
    
    sus writer *dropz.ByteWriter = dropz.new_byte_writer()
    
    sus data1 byte[value] = byte[value]("Hello")
    sus (n1, err1) = writer.write(data1)
    assert_eq_int(n1, 5)
    assert_eq_string(err1, "")
    
    sus data2 byte[value] = byte[value](", World!")
    sus (n2, err2) = writer.write(data2)
    assert_eq_int(n2, 8)
    assert_eq_string(err2, "")
    
    sus result tea = writer.get_string()
    assert_eq_string(result, "Hello, World!") fr fr Test close
    sus closeErr tea = writer.close()
    assert_eq_string(closeErr, "") fr fr Test write after close
    sus (n3, err3) = writer.write(byte[value]("test"))
    assert_eq_int(n3, 0)
    assert_eq_string(err3, dropz.ErrClosed)
}

slay test_buffer() {
    test_start("Buffer functionality")
    
    sus buffer *dropz.Buffer = dropz.new_buffer() fr fr Test write
    sus writeData byte[value] = byte[value]("Buffer test data")
    sus (written, writeErr) = buffer.write(writeData)
    assert_eq_int(written, len(writeData))
    assert_eq_string(writeErr, "") fr fr Test read
    sus readBuf byte[6]
    sus (n, readErr) = buffer.read(readBuf[:])
    assert_eq_int(n, 6)
    assert_eq_string(readErr, "")
    assert_eq_string(string(readBuf[:n]), "Buffer") fr fr Test remaining read
    sus readBuf2 byte[20]
    sus (n2, readErr2) = buffer.read(readBuf2[:])
    assert_eq_int(n2, 10)
    assert_eq_string(readErr2, "")
    assert_eq_string(string(readBuf2[:n2]), " test data") fr fr Test EOF
    sus (n3, readErr3) = buffer.read(readBuf[:])
    assert_eq_int(n3, 0)
    assert_eq_string(readErr3, dropz.EOF) fr fr Test reset
    buffer.reset()
    sus content tea = buffer.get_string()
    assert_eq_string(content, "")
}

slay test_file_operations() {
    test_start("File operations") fr fr Test read_text_file
    sus (content, err) = dropz.read_text_file("main.csd")
    assert_eq_string(err, "")
    assert_true(len(content) > 0) fr fr Test write_text_file
    sus writeErr tea = dropz.write_text_file("test_output.csd", "Test content", dropz.MODE_REGULAR)
    assert_eq_string(writeErr, "") fr fr Verify write by reading back
    sus (readContent, readErr) = dropz.read_text_file("test_output.csd")
    assert_eq_string(readErr, "")
    assert_eq_string(readContent, "Test content") fr fr Test copy_file
    sus (copied, copyErr) = dropz.copy_file("test_output.csd", "test_copy.csd")
    assert_true(copied > 0)
    assert_eq_string(copyErr, "") fr fr Verify copy
    sus (copyContent, copyReadErr) = dropz.read_text_file("test_copy.csd")
    assert_eq_string(copyReadErr, "")
    assert_eq_string(copyContent, "Test content")
}

slay test_file_handles() {
    test_start("File handle operations") fr fr Test create file
    sus (file, createErr) = dropz.create("handle_test.csd")
    assert_eq_string(createErr, "")
    assert_true(file != cringe) fr fr Test write to file
    sus writeData byte[value] = byte[value]("File handle test")
    sus (written, writeErr) = file.write(writeData)
    assert_eq_int(written, len(writeData))
    assert_eq_string(writeErr, "") fr fr Test close
    sus closeErr tea = file.close()
    assert_eq_string(closeErr, "") fr fr Test open file
    sus (readFile, openErr) = dropz.open("handle_test.csd")
    assert_eq_string(openErr, "")
    assert_true(readFile != cringe) fr fr Test read from file
    sus readBuf byte[20]
    sus (n, readErr) = readFile.read(readBuf[:])
    assert_eq_int(n, 16)
    assert_eq_string(readErr, "")
    assert_eq_string(string(readBuf[:n]), "File handle test") fr fr Test seek
    sus (pos, seekErr) = readFile.seek(5, dropz.SEEK_START)
    assert_eq_int(normie(pos), 5)
    assert_eq_string(seekErr, "") fr fr Read after seek
    sus readBuf2 byte[6]
    sus (n2, readErr2) = readFile.read(readBuf2[:])
    assert_eq_int(n2, 6)
    assert_eq_string(readErr2, "")
    assert_eq_string(string(readBuf2[:n2]), "handle")
    
    sus closeErr2 tea = readFile.close()
    assert_eq_string(closeErr2, "")
}

slay test_directory_operations() {
    test_start("Directory operations") fr fr Test create directory
    sus mkdirErr tea = dropz.mkdir("test_dir", dropz.MODE_DIR)
    assert_eq_string(mkdirErr, "") fr fr Test directory exists
    sus dirExists lit = dropz.exists("test_dir")
    assert_true(dirExists)
    
    sus isDir lit = dropz.is_dir("test_dir")
    assert_true(isDir) fr fr Test read directory
    sus (entries, readDirErr) = dropz.read_dir(".")
    assert_eq_string(readDirErr, "")
    assert_true(len(entries) > 0) fr fr Check first entry
    bestie len(entries) > 0 {
        sus entry dropz.DirEntry = entries[0]
        assert_true(len(entry.name) > 0)
        assert_true(entry.is_file || entry.is_dir)
    }
}

slay test_utility_functions() {
    test_start("Utility functions") fr fr Test copy between Reader and Writer
    sus reader *dropz.ByteReader = dropz.new_byte_reader("Copy test data")
    sus writer *dropz.ByteWriter = dropz.new_byte_writer()
    
    sus (copied, copyErr) = dropz.copy(writer, reader)
    assert_true(copied > 0)
    assert_eq_string(copyErr, "")
    
    sus result tea = writer.get_string()
    assert_eq_string(result, "Copy test data") fr fr Test read_all
    sus reader2 *dropz.ByteReader = dropz.new_byte_reader("Read all test")
    sus (allData, readAllErr) = dropz.read_all(reader2)
    assert_eq_string(readAllErr, "")
    assert_eq_string(string(allData), "Read all test") fr fr Test write_string
    sus writer2 *dropz.ByteWriter = dropz.new_byte_writer()
    sus (strWritten, strWriteErr) = dropz.write_string(writer2, "String write test")
    assert_true(strWritten > 0)
    assert_eq_string(strWriteErr, "")
    
    sus strResult tea = writer2.get_string()
    assert_eq_string(strResult, "String write test")
}

slay test_self_hosting_support() {
    test_start("Self-hosting compiler support") fr fr Test read_source_file
    sus (sourceContent, sourceErr) = dropz.read_source_file("main.csd")
    assert_eq_string(sourceErr, "")
    assert_true(len(sourceContent) > 0) fr fr Test write_compiled_output
    sus outputErr tea = dropz.write_compiled_output("test_output", "Compiled content")
    assert_eq_string(outputErr, "") fr fr Verify output was written
    sus outputExists lit = dropz.exists("output/test_output")
    assert_true(outputExists) fr fr Test temp_file
    sus (tempFile, tempErr) = dropz.temp_file("compiler_temp.ll")
    assert_eq_string(tempErr, "")
    assert_true(tempFile != cringe)
    
    sus tempCloseErr tea = tempFile.close()
    assert_eq_string(tempCloseErr, "")
}

slay test_error_handling() {
    test_start("Error handling") fr fr Test file not found
    sus (content, err) = dropz.read_text_file("nonexistent.csd")
    assert_eq_string(content, "")
    assert_eq_string(err, dropz.ErrNotExist) fr fr Test open nonexistent file
    sus (file, openErr) = dropz.open("nonexistent.csd")
    assert_true(file == cringe)
    assert_eq_string(openErr, dropz.ErrNotExist) fr fr Test write to closed file
    sus (testFile, createErr) = dropz.create("error_test.csd")
    assert_eq_string(createErr, "")
    
    sus closeErr tea = testFile.close()
    assert_eq_string(closeErr, "")
    
    sus writeData byte[value] = byte[value]("test")
    sus (written, writeErr) = testFile.write(writeData)
    assert_eq_int(written, 0)
    assert_eq_string(writeErr, dropz.ErrClosed) fr fr Test read from closed file
    sus readBuf byte[10]
    sus (n, readErr) = testFile.read(readBuf[:])
    assert_eq_int(n, 0)
    assert_eq_string(readErr, dropz.ErrClosed)
}

slay test_constants() {
    test_start("Constants and flags") fr fr Test file flags
    assert_eq_int(dropz.O_RDONLY, 0)
    assert_eq_int(dropz.O_WRONLY, 1)
    assert_eq_int(dropz.O_RDWR, 2) fr fr Test permissions
    assert_eq_int(dropz.MODE_REGULAR, 0644)
    assert_eq_int(dropz.MODE_EXECUTABLE, 0755)
    assert_eq_int(dropz.MODE_DIR, 0755) fr fr Test seek constants
    assert_eq_int(dropz.SEEK_START, 0)
    assert_eq_int(dropz.SEEK_CURRENT, 1)
    assert_eq_int(dropz.SEEK_END, 2) fr fr Test error constants
    assert_eq_string(dropz.EOF, "EOF")
    assert_eq_string(dropz.ErrInvalid, "invalid argument")
    assert_eq_string(dropz.ErrPermission, "permission denied")
    assert_eq_string(dropz.ErrExist, "file already exists")
    assert_eq_string(dropz.ErrNotExist, "file does not exist")
    assert_eq_string(dropz.ErrClosed, "file already closed")
}

fr fr Run all tests
test_start("dropz Core I/O Module Tests")

dropz.init_dropz()

test_byte_reader()
test_byte_writer()
test_buffer()
test_file_operations()
test_file_handles()
test_directory_operations()
test_utility_functions()
test_self_hosting_support()
test_error_handling()
test_constants()

print_test_summary()
