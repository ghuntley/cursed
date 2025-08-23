yeet "testz"
yeet "json_tea"
yeet "vibez"

fr fr ==========================================
fr fr JSON File Operations Demo
fr fr Demonstrates real file I/O capabilities
fr fr ==========================================

slay demo_basic_file_operations() {
    vibez.spill("📄 Demo: Basic JSON File Operations")
    vibez.spill("=====================================")
    
    fr fr Test reading from a JSON file
    vibez.spill("1. Reading JSON from file...")
    sus config_result tea = json_tea.parse_json_file("test.json")
    vibez.spill("   Result: " + config_result)
    
    fr fr Test writing JSON to file
    vibez.spill("2. Writing JSON to file...")
    sus sample_data tea = "{\"timestamp\": \"2025-08-23\", \"status\": \"active\"}"
    sus write_result tea = json_tea.write_json_file("output.json", sample_data)
    vibez.spill("   Result: " + write_result)
    
    fr fr Test formatted writing
    vibez.spill("3. Writing formatted JSON...")
    sus formatted_write tea = json_tea.write_json_file_formatted("formatted_output.json", sample_data, "  ")
    vibez.spill("   Result: " + formatted_write)
}

slay demo_streaming_operations() {
    vibez.spill("")
    vibez.spill("🔄 Demo: JSON Streaming Operations")
    vibez.spill("==================================")
    
    fr fr Test streaming large files
    vibez.spill("1. Streaming large JSON file...")
    sus stream_result tea = json_tea.parse_json_stream("large.json", 1024)
    vibez.spill("   Result: " + stream_result)
    
    fr fr Test different chunk sizes
    vibez.spill("2. Testing different chunk sizes...")
    sus small_chunk tea = json_tea.parse_json_stream("large.json", 100)
    vibez.spill("   Small chunks: " + small_chunk)
    
    sus large_chunk tea = json_tea.parse_json_stream("large.json", 5000)
    vibez.spill("   Large chunks: " + large_chunk)
}

slay demo_path_handling() {
    vibez.spill("")
    vibez.spill("🛤️  Demo: Cross-Platform Path Handling")
    vibez.spill("=====================================")
    
    fr fr Test various path formats
    sus unix_path tea = json_tea.normalize_file_path("data/config/settings.json")
    vibez.spill("Unix path: " + unix_path)
    
    sus windows_path tea = json_tea.normalize_file_path("data\\config\\settings.json")
    vibez.spill("Windows path normalized: " + windows_path)
    
    sus relative_path tea = json_tea.normalize_file_path("./configs/../settings.json")
    vibez.spill("Relative path: " + relative_path)
    
    fr fr Test path security
    vibez.spill("Testing security (should fail)...")
    sus unsafe_path tea = json_tea.read_file_safe("../../../etc/passwd")
    vibez.spill("   Security test: " + unsafe_path)
}

slay demo_error_handling() {
    vibez.spill("")
    vibez.spill("⚠️  Demo: Error Handling")
    vibez.spill("========================")
    
    fr fr Test various error conditions
    vibez.spill("1. Missing file...")
    sus missing_result tea = json_tea.parse_json_file("nonexistent.json")
    vibez.spill("   Result: " + missing_result)
    
    vibez.spill("2. Invalid JSON file...")
    sus invalid_result tea = json_tea.parse_json_file("invalid.json")
    vibez.spill("   Result: " + invalid_result)
    
    vibez.spill("3. Empty filename...")
    sus empty_name tea = json_tea.read_file_safe("")
    vibez.spill("   Result: " + empty_name)
    
    vibez.spill("4. Invalid streaming chunk size...")
    sus invalid_chunk tea = json_tea.parse_json_stream("test.json", 0)
    vibez.spill("   Result: " + invalid_chunk)
}

slay demo_complex_data_handling() {
    vibez.spill("")
    vibez.spill("🧩 Demo: Complex Data Structures")
    vibez.spill("================================")
    
    fr fr Create complex nested JSON
    sus complex_data tea = "{\"users\": [{\"id\": 1, \"profile\": {\"name\": \"Alice\", \"settings\": [\"dark_mode\", \"notifications\"]}}, {\"id\": 2, \"profile\": {\"name\": \"Bob\", \"settings\": [\"light_mode\"]}}]}"
    
    vibez.spill("1. Writing complex nested structure...")
    sus complex_write tea = json_tea.write_json_file_formatted("complex.json", complex_data, "    ")
    vibez.spill("   Result: " + complex_write)
    
    fr fr Test JSON validation on complex data
    vibez.spill("2. Validating complex structure...")
    sus is_valid lit = json_tea.IsValidJSON(complex_data)
    ready is_valid == based {
        vibez.spill("   Complex JSON is valid ✓")
    } otherwise {
        vibez.spill("   Complex JSON is invalid ✗")
    }
    
    fr fr Test type detection on complex data
    vibez.spill("3. Detecting JSON type...")
    sus json_type tea = json_tea.get_json_type(complex_data)
    vibez.spill("   Detected type: " + json_type)
}

slay demo_real_world_workflow() {
    vibez.spill("")
    vibez.spill("🌍 Demo: Real-World Workflow")
    vibez.spill("============================")
    
    fr fr Simulate reading configuration, modifying it, and saving
    vibez.spill("1. Loading application configuration...")
    sus config tea = json_tea.parse_json_file("test.json")
    ready !json_tea.string_starts_with(config, "ERROR") {
        vibez.spill("   Configuration loaded successfully")
        
        fr fr Simulate configuration modification
        vibez.spill("2. Modifying configuration...")
        sus modified_config tea = json_tea.set_value(config, "version", "2.0.0")
        vibez.spill("   Configuration modified")
        
        fr fr Save modified configuration
        vibez.spill("3. Saving updated configuration...")
        sus save_result tea = json_tea.write_json_file_formatted("updated_config.json", modified_config, "  ")
        vibez.spill("   " + save_result)
        
        fr fr Create backup with timestamp
        vibez.spill("4. Creating backup...")
        sus backup_result tea = json_tea.write_json_file("config_backup_20250823.json", config)
        vibez.spill("   " + backup_result)
    } otherwise {
        vibez.spill("   Failed to load configuration: " + config)
    }
}

slay demo_performance_considerations() {
    vibez.spill("")
    vibez.spill("⚡ Demo: Performance Considerations")
    vibez.spill("==================================")
    
    fr fr Demonstrate streaming for large files
    vibez.spill("1. Processing large JSON file with streaming...")
    sus large_file_stream tea = json_tea.parse_json_stream("large.json", 512)
    vibez.spill("   " + large_file_stream)
    
    fr fr Demonstrate chunk size optimization
    vibez.spill("2. Testing optimal chunk sizes...")
    sus small_chunks tea = json_tea.parse_json_stream("large.json", 128)
    sus medium_chunks tea = json_tea.parse_json_stream("large.json", 1024)
    sus large_chunks tea = json_tea.parse_json_stream("large.json", 4096)
    
    vibez.spill("   Small chunks (128): " + small_chunks)
    vibez.spill("   Medium chunks (1024): " + medium_chunks)
    vibez.spill("   Large chunks (4096): " + large_chunks)
    
    fr fr Memory usage considerations
    vibez.spill("3. Memory-efficient processing...")
    vibez.spill("   Using streaming to process large files without loading entire content into memory")
    vibez.spill("   Chunk-based processing allows handling files larger than available RAM")
}

slay demo_security_features() {
    vibez.spill("")
    vibez.spill("🔒 Demo: Security Features")
    vibez.spill("=========================")
    
    fr fr Path traversal prevention
    vibez.spill("1. Testing path traversal prevention...")
    sus traversal_test1 tea = json_tea.read_file_safe("../sensitive_data.json")
    sus traversal_test2 tea = json_tea.read_file_safe("../../etc/passwd")
    sus traversal_test3 tea = json_tea.read_file_safe("..\\..\\windows\\system32\\config")
    
    vibez.spill("   Path traversal test 1: " + traversal_test1)
    vibez.spill("   Path traversal test 2: " + traversal_test2)
    vibez.spill("   Path traversal test 3: " + traversal_test3)
    
    fr fr Input validation
    vibez.spill("2. Testing input validation...")
    sus empty_filename tea = json_tea.read_file_safe("")
    sus null_content tea = json_tea.write_file_safe("test.json", "")
    
    vibez.spill("   Empty filename: " + empty_filename)
    vibez.spill("   Empty content: " + null_content)
    
    fr fr JSON validation before file operations
    vibez.spill("3. JSON validation integration...")
    sus invalid_json tea = "{malformed: json, missing: quotes}"
    sus write_invalid tea = json_tea.write_json_file("bad_file.json", invalid_json)
    vibez.spill("   Invalid JSON write attempt: " + write_invalid)
}

slay main() {
    vibez.spill("🍵 CURSED JSON Tea - File Operations Demo")
    vibez.spill("==========================================")
    vibez.spill("Demonstrating comprehensive JSON file I/O capabilities")
    vibez.spill("")
    
    fr fr Run all demonstrations
    demo_basic_file_operations()
    demo_streaming_operations()
    demo_path_handling()
    demo_error_handling()
    demo_complex_data_handling()
    demo_real_world_workflow()
    demo_performance_considerations()
    demo_security_features()
    
    vibez.spill("")
    vibez.spill("✅ JSON File Operations Demo Complete")
    vibez.spill("=====================================")
    vibez.spill("All JSON file I/O capabilities demonstrated successfully!")
    vibez.spill("")
    vibez.spill("Key Features Implemented:")
    vibez.spill("• Real JSON file reading and writing")
    vibez.spill("• Streaming support for large files")
    vibez.spill("• Cross-platform file path handling")
    vibez.spill("• Comprehensive error handling")
    vibez.spill("• Security features (path traversal prevention)")
    vibez.spill("• JSON validation integration")
    vibez.spill("• Performance optimizations")
}

fr fr Run the demo
main()
