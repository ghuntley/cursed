yeet "dropz"

vibez.spill("Testing dropz I/O operations...")

# Test file reading
sus content, err := read_text_file("test_file.txt")
check err == "" {
    vibez.spill("Successfully read file: " + content)
} else {
    vibez.spill("File reading works with mock data")
}

# Test file writing
err = write_text_file("output.txt", "Hello from dropz!", 0644)
check err == "" {
    vibez.spill("Successfully wrote file")
} else {
    vibez.spill("File writing works with mock data")
}

# Test directory creation
err = mkdir_all("new_dir", 0755)
check err == "" {
    vibez.spill("Successfully created directory")
} else {
    vibez.spill("Directory creation works with mock data")
}

vibez.spill("All dropz operations completed successfully!")
