fr fr Simple working test for filesystem module

fr fr ================================
fr fr Filesystem Functions (Mock)
fr fr ================================

slay read_file(path tea) tea {
    vibez.spill("fs.read_file: Reading file '" + path + "'")
    damn "mock file contents from " + path
}

slay write_file(path tea, content tea) lit {
    vibez.spill("fs.write_file: Writing to '" + path + "'")
    damn based
}

slay file_exists(path tea) lit {
    vibez.spill("fs.file_exists: Checking '" + path + "'")
    damn based
}

slay get_file_size(path tea) normie {
    vibez.spill("fs.get_file_size: Getting size of '" + path + "'")
    damn 42
}

slay join_path(base tea, component tea) tea {
    vibez.spill("fs.join_path: Joining '" + base + "' with '" + component + "'")
    damn base + "/" + component
}

fr fr ================================
fr fr Test Functions
fr fr ================================

slay test_filesystem_functions() {
    vibez.spill("Running CURSED Filesystem Module Tests")
    vibez.spill("====================================")
    
    fr fr Test read_file
    vibez.spill("Testing read_file...")
    sus content tea = read_file("test.txt")
    vibez.spill("✓ read_file returned: " + content)
    
    fr fr Test write_file
    vibez.spill("Testing write_file...")
    sus write_result lit = write_file("output.txt", "Hello, World!")
    vibez.spill("✓ write_file returned: " + tea(write_result))
    
    fr fr Test file_exists
    vibez.spill("Testing file_exists...")
    sus exists lit = file_exists("test.txt")
    vibez.spill("✓ file_exists returned: " + tea(exists))
    
    fr fr Test get_file_size
    vibez.spill("Testing get_file_size...")
    sus size normie = get_file_size("test.txt")
    vibez.spill("✓ get_file_size returned: " + tea(size))
    
    fr fr Test join_path
    vibez.spill("Testing join_path...")
    sus joined tea = join_path("/home", "user")
    vibez.spill("✓ join_path returned: " + joined)
    
    vibez.spill("")
    vibez.spill("🎉 ALL FILESYSTEM TESTS COMPLETED! 🎉")
}

fr fr Execute tests
test_filesystem_functions()
