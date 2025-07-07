# Comprehensive CURSED Standard Library Test
# Tests Priority 1 & 2 implementations: Time module and I/O functions

fam "time"
fam "io"
fam "testz"

slay main() {
    # Initialize testing framework
    testz.reset_test_state()
    
    # Test Time Module Functions (Priority 1)
    test_time_module()
    
    # Test I/O Functions (Priority 2)  
    test_io_functions()
    
    # Print comprehensive test results
    testz.print_test_summary()
    
    # Return exit code based on test results
    damn testz.run_all_tests()
}

slay test_time_module() {
    testz.test_start("Time Module - Current Time")
    
    # Test time_now functions
    sus current_time normie = time.now()
    testz.assert_true(current_time > 0)
    
    sus millis normie = time.now_millis()
    testz.assert_true(millis > current_time)
    
    testz.test_start("Time Module - Time Creation")
    
    # Test time creation
    sus created_time normie = time.create(2025, 1, 7, 12, 30, 45)
    testz.assert_true(created_time > 0)
    
    # Test time components
    sus year normie = time.year(created_time)
    testz.assert_eq_int(year, 2025)
    
    sus month normie = time.month(created_time)
    testz.assert_eq_int(month, 1)
    
    sus day normie = time.day(created_time)
    testz.assert_eq_int(day, 7)
    
    testz.test_start("Time Module - Time Arithmetic")
    
    # Test time arithmetic
    sus future_time normie = time.add_hours(created_time, 2)
    testz.assert_true(future_time > created_time)
    
    sus duration normie = time.subtract(future_time, created_time)
    testz.assert_true(duration > 0)
    
    testz.test_start("Time Module - Duration Operations")
    
    # Test duration functions
    sus duration_secs normie = time.duration_from_seconds(3600)
    testz.assert_eq_int(time.duration_to_seconds(duration_secs), 3600)
    
    sus duration_millis normie = time.duration_from_millis(1000)
    testz.assert_eq_int(time.duration_to_millis(duration_millis), 1000)
}

slay test_io_functions() {
    testz.test_start("I/O - File Operations")
    
    # Test file writing and reading
    sus test_file tea = "test_output.txt"
    sus test_content tea = "Hello CURSED stdlib!"
    
    # Test file writing
    sus write_result lit = io.write_file(test_file, test_content)
    testz.assert_true(write_result)
    
    # Test file existence
    testz.assert_true(io.file_exists(test_file))
    
    # Test file reading
    sus read_content tea = io.read_file(test_file)
    testz.assert_eq_string(read_content, test_content)
    
    # Test file size
    sus file_size normie = io.file_size(test_file)
    testz.assert_true(file_size > 0)
    
    testz.test_start("I/O - Directory Operations")
    
    # Test directory creation
    sus test_dir tea = "test_directory"
    testz.assert_true(io.create_directory(test_dir))
    testz.assert_true(io.is_directory(test_dir))
    
    # Test current directory
    sus current_dir tea = io.current_directory()
    testz.assert_true(io.path_exists(current_dir))
    
    testz.test_start("I/O - Path Operations") 
    
    # Test path operations
    sus joined_path tea = io.path_join(["home", "user", "documents"])
    testz.assert_true(joined_path.length() > 0)
    
    sus dirname tea = io.path_dirname("/home/user/file.txt")
    testz.assert_eq_string(dirname, "/home/user")
    
    sus basename tea = io.path_basename("/home/user/file.txt")
    testz.assert_eq_string(basename, "file.txt")
    
    # Cleanup test files
    io.delete_file(test_file)
    io.remove_directory(test_dir)
}
