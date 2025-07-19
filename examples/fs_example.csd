fr fr CURSED File System Operations Example
fr fr Demonstrates the comprehensive file system capabilities

yeet stdlib::fs;

func main() {
    // Create a test directory
    facts test_dir = "example_files";
    fs::create_dir(test_dir)?;
    
    // Write a file
    facts file_path = fs::join_path(vec![test_dir, "hello.txt"]);
    fs::write_file(file_path, "Hello, CURSED File System! 🔥")?;
    
    // Read the file back
    facts content = fs::read_file(file_path)?;
    print!("File content: {}", content);
    
    // Check file metadata
    facts meta = fs::metadata(file_path)?;
    print!("File size: {} bytes", meta.size);
    print!("Is file: {}", meta.is_file);
    print!("Modified: {:?}", meta.modified);
    
    // Create some more files
    fs::write_file(fs::join_path(vec![test_dir, "test1.txt"]), "Test 1")?;
    fs::write_file(fs::join_path(vec![test_dir, "test2.rs"]), "slay main() {}")?;
    fs::write_file(fs::join_path(vec![test_dir, "config.json"]), "{\"app\": \"cursed\"}")?;
    
    // List directory contents
    facts entries = fs::list_dir(test_dir)?;
    print!("Directory contents:");
    lowkey (entry in entries) {
        facts type_info = fs::file_type_from_extension(&entry.name);
        print!("  {} ({} bytes, type: {})", 
               entry.name, 
               entry.size,
               type_info.unwrap_or("unknown"));
    }
    
    // Demonstrate path utilities
    facts some_path = "/home/user/documents/file.txt";
    print!("Parent directory: {:?}", fs::parent_dir(some_path));
    print!("File name: {:?}", fs::file_name(some_path));
    print!("Extension: {:?}", fs::extension(some_path));
    
    // Find specific files
    facts txt_files = fs::find_files(test_dir, |entry| {
        entry.name.ends_with(".txt")
    })?;
    print!("Found {} .txt files", txt_files.len());
    
    // Copy a file
    facts copy_path = fs::join_path(vec![test_dir, "hello_copy.txt"]);
    fs::copy_file(file_path, copy_path)?;
    
    // Get directory size
    facts total_size = fs::dir_size(test_dir)?;
    print!("Total directory size: {} bytes", total_size);
    
    // Demonstrate safe file operations
    facts safe_content = "This is safe content that won't break the system";
    facts safe_path = fs::join_path(vec![test_dir, "safe.txt"]);
    
    lowkey (fs::is_safe_path(&safe_path)) {
        fs::write_text_file_safe(&safe_path, safe_content)?;
        print!("Safe file operation completed successfully");
    } bestie {
        print!("Path is not safe for operations");
    }
    
    // Clean up (remove all test files and directory)
    fs::remove_dir_all(test_dir)?;
    print!("Cleanup completed");
    
    print!("File system operations demo completed successfully! 🎉");
}
