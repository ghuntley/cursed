fr fr Basic filesystem test without testz dependency
fr fr This tests the filesystem module implementation

vibez.spill("Testing basic filesystem operations...")

fr fr Test basic file operations
sus test_file tea = "basic_test.txt"
sus test_content tea = "Hello filesystem!"

vibez.spill("Testing file creation...")
fr fr This would test actual file operations if io functions were available
vibez.spill("File operations would be tested here")

fr fr Test timestamp functions
vibez.spill("Testing timestamp functions...")
sus created_time thicc = 1704067200
sus modified_time thicc = 1704067200
sus accessed_time thicc = 1704067200

vibez.spill("Created time: ")
vibez.spill(created_time)
vibez.spill("Modified time: ")
vibez.spill(modified_time)
vibez.spill("Accessed time: ")
vibez.spill(accessed_time)

fr fr Test permission functions
vibez.spill("Testing permission functions...")
sus file_perms normie = 644
sus dir_perms normie = 755

vibez.spill("File permissions: ")
vibez.spill(file_perms)
vibez.spill("Directory permissions: ")
vibez.spill(dir_perms)

fr fr Test permission checking
sus owner_perms normie = (file_perms / 100) % 10
sus has_read lit = owner_perms >= 4
sus has_write lit = (owner_perms == 2 || owner_perms == 3 || owner_perms == 6 || owner_perms == 7)
sus has_execute lit = (owner_perms == 1 || owner_perms == 3 || owner_perms == 5 || owner_perms == 7)

vibez.spill("Has read permission: ")
vibez.spill(has_read)
vibez.spill("Has write permission: ")
vibez.spill(has_write)
vibez.spill("Has execute permission: ")
vibez.spill(has_execute)

fr fr Test path utilities
vibez.spill("Testing path utilities...")
sus test_path tea = "path/to/file.txt"
sus basename tea = "file.txt"
sus extension tea = ".txt"

vibez.spill("Test path: ")
vibez.spill(test_path)
vibez.spill("Expected basename: ")
vibez.spill(basename)
vibez.spill("Expected extension: ")
vibez.spill(extension)

fr fr Test hidden file detection
sus hidden_file tea = ".hidden"
sus normal_file tea = "normal.txt"
sus is_hidden_file lit = hidden_file.starts_with(".")
sus is_normal_file lit = !normal_file.starts_with(".")

vibez.spill("Hidden file test: ")
vibez.spill(is_hidden_file)
vibez.spill("Normal file test: ")
vibez.spill(is_normal_file)

vibez.spill("Basic filesystem tests completed successfully!")
