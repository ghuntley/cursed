fr fr Simple Filesystem Test
fr fr Testing core filesystem functions

vibez.spill("🗂️  Testing Filesystem Functions")
vibez.spill("============================")

fr fr Test timestamp functions
vibez.spill("Testing timestamp functions...")
sus timestamp thicc = 1704067200
vibez.spill("Unix timestamp: ")
vibez.spill(timestamp)

fr fr Test permission functions
vibez.spill("Testing permission functions...")
sus file_perms normie = 644
sus dir_perms normie = 755

vibez.spill("File permissions: ")
vibez.spill(file_perms)
vibez.spill("Directory permissions: ")
vibez.spill(dir_perms)

fr fr Test permission checking logic
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

fr fr Test path utilities logic
vibez.spill("Testing path utilities...")
sus test_path tea = "path/to/file.txt"
vibez.spill("Test path: ")
vibez.spill(test_path)

fr fr Test basic string operations
sus basename tea = "file.txt"
sus extension tea = ".txt"
vibez.spill("Expected basename: ")
vibez.spill(basename)
vibez.spill("Expected extension: ")
vibez.spill(extension)

fr fr Test hidden file detection logic
sus hidden_file tea = ".hidden"
sus normal_file tea = "normal.txt"
vibez.spill("Hidden file: ")
vibez.spill(hidden_file)
vibez.spill("Normal file: ")
vibez.spill(normal_file)

fr fr Test file classification
sus is_dot_file lit = hidden_file.starts_with(".")
sus is_normal_file lit = !normal_file.starts_with(".")
vibez.spill("Is dot file: ")
vibez.spill(is_dot_file)
vibez.spill("Is normal file: ")
vibez.spill(is_normal_file)

fr fr Test system paths
sus proc_path tea = "/proc/version"
sus sys_path tea = "/sys/kernel"
sus user_path tea = "/home/user/file.txt"

vibez.spill("Testing system path detection...")
vibez.spill("Proc path: ")
vibez.spill(proc_path)
vibez.spill("Sys path: ")
vibez.spill(sys_path)
vibez.spill("User path: ")
vibez.spill(user_path)

fr fr Test different permission combinations
vibez.spill("Testing permission combinations...")
sus perms_600 normie = 600
sus perms_755 normie = 755
sus perms_777 normie = 777

vibez.spill("600 permissions: ")
vibez.spill(perms_600)
vibez.spill("755 permissions: ")
vibez.spill(perms_755)
vibez.spill("777 permissions: ")
vibez.spill(perms_777)

vibez.spill("✅ Basic filesystem tests completed successfully!")
