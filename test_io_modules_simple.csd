fr fr Simple I/O Module Test - Direct Function Calls
yeet "io_basic"
yeet "mathz"

fr fr Test basic I/O functions directly
sus test_result lit = io_basic.file_exists("test.txt")
lowkey test_result {
    sus content tea = io_basic.read_file_content("test.txt")
    lowkey content == "Test file content from CURSED I/O" {
        sus number drip = mathz.add_two(20, 22)
        lowkey number == 42 {
            sus success lit = io_basic.write_file_content("output.txt", "Success!")
        }
    }
}

fr fr Test directory operations
sus dir_result lit = io_basic.create_directory("temp")
lowkey dir_result {
    sus file_count drip = io_basic.list_files("temp")
    sus max_count drip = mathz.max_normie(file_count, 5)
}

fr fr Test path utilities
sus joined tea = io_basic.join_path("temp", "test.txt")
sus extension tea = io_basic.get_extension("test.txt")
sus is_text lit = io_basic.is_text_file("test.txt")

fr fr Test buffer operations
sus buffer drip = io_basic.create_buffer(1024)
sus written drip = io_basic.buffer_write(buffer, "test data")
sus data tea = io_basic.buffer_read(buffer, 10)
sus flushed lit = io_basic.buffer_flush(buffer)
