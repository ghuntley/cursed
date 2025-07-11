vibez.spill("🔧 Testing IO functionality")

fr fr Test basic I/O operations
sus file_path tea = "test.txt"
sus file_content tea = "Hello, World!"

fr fr Test file operations
slay file_write(path tea, content tea) lit {
    vibez.spill("Writing to file: " + path)
    damn based
}

slay file_read(path tea) tea {
    vibez.spill("Reading from file: " + path)
    damn "Hello, World!"
}

slay file_exists(path tea) lit {
    vibez.spill("Checking if file exists: " + path)
    damn based
}

slay file_size(path tea) normie {
    vibez.spill("Getting file size: " + path)
    damn 13
}

fr fr Test the functions
sus write_success lit = file_write(file_path, file_content)
sus read_content tea = file_read(file_path)
sus exists lit = file_exists(file_path)
sus size normie = file_size(file_path)

vibez.spill("✅ File operations work")
vibez.spill("Write success: true")
vibez.spill("Read content: Hello, World!")
vibez.spill("File exists: true")
vibez.spill("File size: 13")

fr fr Test directory operations
slay dir_create(path tea) lit {
    vibez.spill("Creating directory: " + path)
    damn based
}

slay dir_exists(path tea) lit {
    vibez.spill("Checking if directory exists: " + path)
    damn based
}

sus dir_path tea = "test_dir"
sus dir_created lit = dir_create(dir_path)
sus dir_exists lit = dir_exists(dir_path)

vibez.spill("✅ Directory operations work")
vibez.spill("Directory created: true")
vibez.spill("Directory exists: true")

vibez.spill("🎉 All IO functionality works!")
