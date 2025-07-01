// Test enhanced I/O functions
slay main() -> void {
    // Test basic print functions
    println("Testing enhanced I/O functions...");
    
    // Test file operations
    write_file("test_output.txt", "Hello from CURSED enhanced I/O!");
    
    lowkey file_exists("test_output.txt") {
        println("File created successfully!");
        
        // Test file size
        sus size = file_size("test_output.txt");
        println("File size: " + size);
        
        // Test copy operation
        copy_file("test_output.txt", "test_copy.txt");
        println("File copied successfully!");
        
        // Test append operation
        append_file("test_output.txt", "\nAppended content!");
        
        // Read and display content
        sus content = read_file("test_output.txt");
        println("File content: " + content);
        
        // Clean up
        delete_file("test_output.txt");
        delete_file("test_copy.txt");
        println("Test completed successfully!");
    } highkey {
        println("File creation failed!");
    }
}
