// Test CURSED standard library I/O functionality

slay main() -> void {
    // Test basic print functionality
    print("Hello from CURSED stdlib!");
    println("This is a test message");
    
    // Test file operations
    write_file("test_output.txt", "Hello, World from CURSED!");
    
    // Test file existence check
    facts file_exists = file_exists("test_output.txt");
    if (file_exists) {
        println("File created successfully!");
        
        // Read the file back
        facts content = read_file("test_output.txt");
        print("File content: ");
        println(content);
    } else {
        println("File creation failed!");
    }
    
    // Test directory operations
    create_directory("test_dir");
    facts dir_exists = file_exists("test_dir");
    if (dir_exists) {
        println("Directory created successfully!");
    }
    
    println("Standard library I/O test completed!");
}
