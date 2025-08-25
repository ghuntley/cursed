yeet "testz"
yeet "filez"

test_start("filez File System Comprehensive Tests")

fr fr ===== FILE CREATION AND DELETION TESTS =====

test_group("File Creation and Deletion")

fr fr Test file creation
sus test_file_path tea = "test_file_creation.txt"
sus test_content tea = "Hello, CURSED File System!"

sus create_result lit = create_file(test_file_path, test_content) fam {
    when err -> {
        assert_fail("File creation failed: " + err)
        damn cringe
    }
}
assert_bool(create_result, "File created successfully")

fr fr Test file existence
sus exists_result lit = file_exists(test_file_path)
assert_bool(exists_result, "Created file exists")

fr fr Test file deletion
sus delete_result lit = delete_file(test_file_path) fam {
    when err -> {
        assert_fail("File deletion failed: " + err)
        damn cringe
    }
}
assert_bool(delete_result, "File deleted successfully")

fr fr Test file no longer exists
exists_result = file_exists(test_file_path)
assert_bool(!exists_result, "Deleted file no longer exists")

fr fr ===== FILE READING AND WRITING TESTS =====

test_group("File Reading and Writing")

fr fr Test file writing and reading
sus rw_file_path tea = "test_read_write.txt"
sus write_content tea = "Line 1\nLine 2\nLine 3\n"

fr fr Write content to file
create_result = create_file(rw_file_path, write_content) fam {
    when err -> {
        assert_fail("Write file creation failed: " + err)
        damn cringe
    }
}
assert_bool(create_result, "Write file created")

fr fr Read content back
sus read_content tea = read_file(rw_file_path) fam {
    when err -> {
        assert_fail("File reading failed: " + err)
        damn ""
    }
}
assert_string_equals(read_content, write_content, "File content matches written data")

fr fr Test append to file
sus append_content tea = "Line 4\nLine 5\n"
sus append_result lit = append_to_file(rw_file_path, append_content) fam {
    when err -> {
        assert_fail("File append failed: " + err)
        damn cringe
    }
}
assert_bool(append_result, "File append succeeded")

fr fr Read appended content
sus full_content tea = read_file(rw_file_path) fam {
    when err -> {
        assert_fail("Reading appended file failed: " + err)
        damn ""
    }
}
sus expected_full tea = write_content + append_content
assert_string_equals(full_content, expected_full, "Appended content correct")

fr fr Cleanup
delete_file(rw_file_path) fam { when _ -> { } }

fr fr ===== FILE HANDLE OPERATIONS =====

test_group("File Handle Operations")

fr fr Test file handle open/close
sus handle_file_path tea = "test_handle_ops.txt"
create_file(handle_file_path, "Handle test content") fam { when _ -> { } }

sus file_handle FileHandle = file_open(handle_file_path, "r") fam {
    when err -> {
        assert_fail("File handle open failed: " + err)
        damn FileHandle{}
    }
}
assert_bool(file_handle.is_open, "File handle opened")
assert_string_equals(file_handle.path, handle_file_path, "Handle path correct")
assert_string_equals(file_handle.mode, "r", "Handle mode correct")

fr fr Test file read through handle
sus handle_content tea = file_read(file_handle, 1024) fam {
    when err -> {
        assert_fail("File handle read failed: " + err)
        damn ""
    }
}
assert_string_contains(handle_content, "Handle test content", "Handle read content")

fr fr Test file close
sus close_result lit = file_close(file_handle) fam {
    when err -> {
        assert_fail("File handle close failed: " + err)
        damn cringe
    }
}
assert_bool(close_result, "File handle closed")
assert_bool(!file_handle.is_open, "File handle marked as closed")

fr fr Cleanup
delete_file(handle_file_path) fam { when _ -> { } }

fr fr ===== FILE WRITE HANDLE TESTS =====

test_group("File Write Handle Operations")

fr fr Test write handle operations
sus write_handle_path tea = "test_write_handle.txt"

sus write_handle FileHandle = file_open(write_handle_path, "w") fam {
    when err -> {
        assert_fail("Write handle open failed: " + err)
        damn FileHandle{}
    }
}
assert_bool(write_handle.is_open, "Write handle opened")

fr fr Write data through handle
sus write_data tea = "Written through handle"
sus write_result lit = file_write(write_handle, write_data) fam {
    when err -> {
        assert_fail("File handle write failed: " + err)
        damn cringe
    }
}
assert_bool(write_result, "File handle write succeeded")

fr fr Close write handle
close_result = file_close(write_handle) fam {
    when err -> {
        assert_fail("Write handle close failed: " + err)
        damn cringe
    }
}
assert_bool(close_result, "Write handle closed")

fr fr Read back written data
sus written_content tea = read_file(write_handle_path) fam {
    when err -> {
        assert_fail("Reading written file failed: " + err)
        damn ""
    }
}
assert_string_equals(written_content, write_data, "Written data matches")

fr fr Cleanup
delete_file(write_handle_path) fam { when _ -> { } }

fr fr ===== FILE INFO AND METADATA TESTS =====

test_group("File Information and Metadata")

fr fr Test file info
sus info_file_path tea = "test_file_info.txt"
sus info_content tea = "File info test content with some length"
create_file(info_file_path, info_content) fam { when _ -> { } }

sus file_info FileInfo = get_file_info(info_file_path) fam {
    when err -> {
        assert_fail("Get file info failed: " + err)
        damn FileInfo{}
    }
}

assert_string_equals(file_info.name, "test_file_info.txt", "File info name")
assert_string_contains(file_info.path, "test_file_info.txt", "File info path")
assert_bool(!file_info.is_directory, "File is not directory")
assert_true(file_info.size > 0, "File has size")
assert_bool(file_info.is_readable, "File is readable")

fr fr Test file size
sus file_size drip = get_file_size(info_file_path) fam {
    when err -> {
        assert_fail("Get file size failed: " + err)
        damn 0
    }
}
assert_true(file_size > 0, "File size greater than zero")
assert_eq_int(file_size, string_length(info_content), "File size matches content")

fr fr Cleanup
delete_file(info_file_path) fam { when _ -> { } }

fr fr ===== DIRECTORY OPERATIONS TESTS =====

test_group("Directory Operations")

fr fr Test directory creation
sus test_dir_path tea = "test_directory"
sus create_dir_result lit = create_directory(test_dir_path) fam {
    when err -> {
        assert_fail("Directory creation failed: " + err)
        damn cringe
    }
}
assert_bool(create_dir_result, "Directory created")

fr fr Test directory existence
sus dir_exists_result lit = directory_exists(test_dir_path)
assert_bool(dir_exists_result, "Created directory exists")

fr fr Test directory info
sus dir_info FileInfo = get_file_info(test_dir_path) fam {
    when err -> {
        assert_fail("Directory info failed: " + err)
        damn FileInfo{}
    }
}
assert_bool(dir_info.is_directory, "Info shows it's a directory")

fr fr Test directory listing (empty)
sus dir_entries []DirectoryEntry = list_directory(test_dir_path) fam {
    when err -> {
        assert_fail("Directory listing failed: " + err)
        damn []
    }
}
assert_eq_int(array_length(dir_entries), 0, "Empty directory has no entries")

fr fr Create file in directory
sus sub_file_path tea = test_dir_path + "/sub_file.txt"
create_file(sub_file_path, "Sub file content") fam { when _ -> { } }

fr fr Test directory listing with file
dir_entries = list_directory(test_dir_path) fam {
    when err -> {
        assert_fail("Directory listing with file failed: " + err)
        damn []
    }
}
assert_eq_int(array_length(dir_entries), 1, "Directory has one entry")
assert_string_equals(dir_entries[0].name, "sub_file.txt", "Entry name correct")
assert_bool(!dir_entries[0].is_directory, "Entry is file not directory")

fr fr Test directory deletion (recursive)
sus delete_dir_result lit = delete_directory_recursive(test_dir_path) fam {
    when err -> {
        assert_fail("Recursive directory deletion failed: " + err)
        damn cringe
    }
}
assert_bool(delete_dir_result, "Directory deleted recursively")

fr fr Test directory no longer exists
dir_exists_result = directory_exists(test_dir_path)
assert_bool(!dir_exists_result, "Deleted directory no longer exists")

fr fr ===== PATH OPERATIONS TESTS =====

test_group("Path Operations")

fr fr Test path joining
sus joined_path tea = path_join("home", "user", "documents", "file.txt")
assert_string_contains(joined_path, "home", "Path contains home")
assert_string_contains(joined_path, "user", "Path contains user")
assert_string_contains(joined_path, "documents", "Path contains documents")
assert_string_contains(joined_path, "file.txt", "Path contains filename")

fr fr Test path basename
sus base_name tea = path_basename("/home/user/documents/file.txt")
assert_string_equals(base_name, "file.txt", "Basename extraction")

sus base_name_no_ext tea = path_basename("/home/user/documents/")
assert_string_equals(base_name_no_ext, "documents", "Basename directory")

fr fr Test path dirname
sus dir_name tea = path_dirname("/home/user/documents/file.txt")
assert_string_contains(dir_name, "documents", "Dirname extraction")

fr fr Test path extension
sus extension tea = path_extension("document.pdf")
assert_string_equals(extension, ".pdf", "Extension extraction")

sus no_extension tea = path_extension("README")
assert_string_equals(no_extension, "", "No extension case")

fr fr Test absolute path
sus is_absolute_unix lit = path_is_absolute("/home/user/file.txt")
assert_bool(is_absolute_unix, "Unix absolute path detection")

sus is_relative lit = path_is_absolute("documents/file.txt")
assert_bool(!is_relative, "Relative path detection")

fr fr ===== WORKING DIRECTORY TESTS =====

test_group("Working Directory Operations")

fr fr Test get current working directory
sus current_dir tea = get_current_directory() fam {
    when err -> {
        assert_fail("Get current directory failed: " + err)
        damn ""
    }
}
assert_not_empty(current_dir, "Current directory retrieved")
assert_true(string_length(current_dir) > 0, "Current directory has length")

fr fr ===== ERROR HANDLING TESTS =====

test_group("Error Handling and Edge Cases")

fr fr Test reading non-existent file
sus nonexistent_content tea = read_file("nonexistent_file.txt") fam {
    when err -> {
        assert_string_contains(err, "not found", "Non-existent file error")
        damn ""
    }
}

fr fr Test creating file with invalid path
sus invalid_create lit = create_file("", "content") fam {
    when err -> {
        assert_string_contains(err, "empty", "Empty path error")
        damn cringe
    }
}

fr fr Test deleting non-existent file
sus nonexistent_delete lit = delete_file("nonexistent_file.txt") fam {
    when err -> {
        assert_string_contains(err, "not found", "Non-existent delete error")
        damn cringe
    }
}

fr fr Test opening file with invalid mode
sus invalid_mode_file tea = "test_invalid_mode.txt"
create_file(invalid_mode_file, "test") fam { when _ -> { } }

sus invalid_handle FileHandle = file_open(invalid_mode_file, "invalid_mode") fam {
    when err -> {
        assert_string_contains(err, "invalid", "Invalid mode error")
        damn FileHandle{}
    }
}

fr fr Cleanup
delete_file(invalid_mode_file) fam { when _ -> { } }

fr fr ===== LARGE FILE TESTS =====

test_group("Large File Operations")

fr fr Test large file creation and reading
sus large_file_path tea = "test_large_file.txt"
sus large_content tea = create_large_string(1000)  fr fr 1KB content

create_result = create_file(large_file_path, large_content) fam {
    when err -> {
        assert_fail("Large file creation failed: " + err)
        damn cringe
    }
}
assert_bool(create_result, "Large file created")

fr fr Test reading large file
sus read_large_content tea = read_file(large_file_path) fam {
    when err -> {
        assert_fail("Large file reading failed: " + err)
        damn ""
    }
}
assert_eq_int(string_length(read_large_content), string_length(large_content), "Large file content length")

fr fr Test large file size
sus large_file_size drip = get_file_size(large_file_path) fam {
    when err -> {
        assert_fail("Large file size failed: " + err)
        damn 0
    }
}
assert_eq_int(large_file_size, string_length(large_content), "Large file size correct")

fr fr Cleanup large file
delete_file(large_file_path) fam { when _ -> { } }

fr fr ===== CONCURRENT FILE ACCESS TESTS =====

test_group("Concurrent File Access")

fr fr Test multiple file handles to same file
sus concurrent_file_path tea = "test_concurrent_access.txt"
create_file(concurrent_file_path, "Concurrent test content") fam { when _ -> { } }

sus handle1 FileHandle = file_open(concurrent_file_path, "r") fam {
    when err -> {
        assert_fail("First concurrent handle failed: " + err)
        damn FileHandle{}
    }
}

sus handle2 FileHandle = file_open(concurrent_file_path, "r") fam {
    when err -> {
        assert_fail("Second concurrent handle failed: " + err)
        damn FileHandle{}
    }
}

assert_bool(handle1.is_open, "First handle opened")
assert_bool(handle2.is_open, "Second handle opened")

fr fr Read from both handles
sus content1 tea = file_read(handle1, 1024) fam {
    when err -> {
        assert_fail("First handle read failed: " + err)
        damn ""
    }
}

sus content2 tea = file_read(handle2, 1024) fam {
    when err -> {
        assert_fail("Second handle read failed: " + err)
        damn ""
    }
}

assert_string_equals(content1, content2, "Both handles read same content")

fr fr Close both handles
file_close(handle1) fam { when _ -> { } }
file_close(handle2) fam { when _ -> { } }

fr fr Cleanup
delete_file(concurrent_file_path) fam { when _ -> { } }

print_test_summary()
