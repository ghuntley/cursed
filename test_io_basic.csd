fr fr Test CURSED Basic I/O Module
yeet "io_basic"

lowkey io_basic.file_exists("test.txt") {
    sus content tea = io_basic.read_file_content("test.txt")
    io_basic.print_line("File content: " + content)
    
    io_basic.print_line("Testing integer: ")
    io_basic.print_int(42)
    
    io_basic.print_line("Testing float: ")
    io_basic.print_float(3.14)
    
    sus user_input tea = io_basic.read_line()
    io_basic.print_line("You entered: " + user_input)
    
    sus number drip = io_basic.read_int()
    io_basic.print_line("Number read: ")
    io_basic.print_int(number)
    
    sus char tea = io_basic.read_char()
    io_basic.print_line("Character read: " + char)
    
    lowkey io_basic.write_file_content("output.txt", "Hello from CURSED I/O!") {
        io_basic.print_line("File written successfully!")
    }
    
    lowkey io_basic.create_directory("temp") {
        io_basic.print_line("Directory created!")
        
        sus file_count drip = io_basic.list_files("temp")
        io_basic.print_line("Files in temp: ")
        io_basic.print_int(file_count)
    }
    
    sus joined_path tea = io_basic.join_path("temp", "test.txt")
    io_basic.print_line("Joined path: " + joined_path)
    
    sus extension tea = io_basic.get_extension("test.txt")
    io_basic.print_line("Extension: " + extension)
    
    sus basename tea = io_basic.get_basename("/path/to/test.txt")
    io_basic.print_line("Basename: " + basename)
    
    lowkey io_basic.is_text_file("test.txt") {
        io_basic.print_line("test.txt is a text file")
    }
    
    sus buffer drip = io_basic.create_buffer(1024)
    io_basic.print_line("Created buffer of size: ")
    io_basic.print_int(buffer)
    
    sus written drip = io_basic.buffer_write(buffer, "Hello buffer!")
    io_basic.print_line("Bytes written to buffer: ")
    io_basic.print_int(written)
    
    sus buffer_data tea = io_basic.buffer_read(buffer, 10)
    io_basic.print_line("Buffer data: " + buffer_data)
    
    lowkey io_basic.buffer_flush(buffer) {
        io_basic.print_line("Buffer flushed successfully")
    }
    
    io_basic.flush()
    io_basic.print_line("Basic I/O test completed successfully!")
}
