fr fr Test specific filesystem functions
fr fr This tests the timestamp and permission functions I implemented

vibez.spill("Testing filesystem utility functions...")

fr fr Test path utility functions
slay get_basename_test(path tea) tea {
    sus slash_pos normie = path.last_index_of("/")
    lowkey slash_pos == -1 {
        damn path
    }
    damn path.substring(slash_pos + 1)
}

slay get_extension_test(path tea) tea {
    sus dot_pos normie = path.last_index_of(".")
    sus slash_pos normie = path.last_index_of("/")
    
    lowkey dot_pos == -1 || dot_pos < slash_pos {
        damn ""
    }
    damn path.substring(dot_pos)
}

fr fr Test the functions
sus test_path tea = "path/to/file.txt"
sus basename tea = get_basename_test(test_path)
sus extension tea = get_extension_test(test_path)

vibez.spill("Test path: ")
vibez.spill(test_path)
vibez.spill("Basename: ")
vibez.spill(basename)
vibez.spill("Extension: ")
vibez.spill(extension)

fr fr Test permission checking logic
slay test_permissions(perms normie) lit {
    sus owner_perms normie = (perms / 100) % 10
    sus has_read lit = owner_perms >= 4
    sus has_write lit = (owner_perms == 2 || owner_perms == 3 || owner_perms == 6 || owner_perms == 7)
    sus has_execute lit = (owner_perms == 1 || owner_perms == 3 || owner_perms == 5 || owner_perms == 7)
    
    vibez.spill("Permissions: ")
    vibez.spill(perms)
    vibez.spill("Read: ")
    vibez.spill(has_read)
    vibez.spill("Write: ")
    vibez.spill(has_write)
    vibez.spill("Execute: ")
    vibez.spill(has_execute)
    
    damn based
}

fr fr Test different permission combinations
test_permissions(644)  fr fr rw-r--r--
test_permissions(755)  fr fr rwxr-xr-x
test_permissions(600)  fr fr rw-------
test_permissions(777)  fr fr rwxrwxrwx

fr fr Test timestamp simulation
vibez.spill("Testing timestamp functions...")
sus timestamp thicc = 1704067200
vibez.spill("Unix timestamp: ")
vibez.spill(timestamp)

fr fr Test hidden file detection
slay is_hidden_test(path tea) lit {
    sus basename tea = get_basename_test(path)
    damn basename.starts_with(".")
}

vibez.spill("Hidden file tests:")
vibez.spill(is_hidden_test(".hidden"))
vibez.spill(is_hidden_test("normal.txt"))
vibez.spill(is_hidden_test("path/to/.hidden"))

vibez.spill("Filesystem utility tests completed!")
