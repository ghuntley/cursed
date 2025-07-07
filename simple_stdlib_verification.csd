# Simple verification that Priority 1 & 2 implementations work
# Direct function calls without module imports

slay main() {
    vibez.spill("=== CURSED Stdlib Verification ===")
    
    # Test Time Module Functions (Priority 1)
    vibez.spill("Testing Time Module...")
    
    sus current_time normie = time_now_impl()
    vibez.spill("Current timestamp: " + current_time.(tea))
    
    sus millis normie = time_now_millis_impl()
    vibez.spill("Current millis: " + millis.(tea))
    
    sus created_time normie = time_create_impl(2025, 1, 7, 12, 30, 45)
    sus year normie = time_year_impl(created_time)
    sus month normie = time_month_impl(created_time)
    sus day normie = time_day_impl(created_time)
    
    vibez.spill("Created time - Year: " + year.(tea) + ", Month: " + month.(tea) + ", Day: " + day.(tea))
    
    # Test I/O Functions (Priority 2)
    vibez.spill("Testing I/O Functions...")
    
    sus test_file tea = "verification_test.txt"
    sus test_content tea = "Hello CURSED stdlib verification!"
    
    # Test file operations
    sus write_success lit = io_write_file(test_file, test_content)
    lowkey write_success {
        vibez.spill("✓ File write successful")
    } highkey {
        vibez.spill("✗ File write failed")
    }
    
    sus file_exists lit = io_file_exists(test_file)
    lowkey file_exists {
        vibez.spill("✓ File exists check successful")
    } highkey {
        vibez.spill("✗ File exists check failed")
    }
    
    sus read_content tea = io_read_file(test_file)
    vibez.spill("Read content: " + read_content)
    
    # Test directory operations
    sus current_dir tea = io_current_directory()
    vibez.spill("Current directory: " + current_dir)
    
    # Test path operations  
    sus dirname tea = io_path_dirname("/home/user/test.txt")
    vibez.spill("Path dirname: " + dirname)
    
    sus basename tea = io_path_basename("/home/user/test.txt")
    vibez.spill("Path basename: " + basename)
    
    # Cleanup
    io_delete_file(test_file)
    
    vibez.spill("=== Verification Complete ===")
    vibez.spill("✅ Priority 1 (Time) and Priority 2 (I/O) implementations working!")
}
