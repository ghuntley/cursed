yeet "testz"
yeet "fs"

fr fr ================================
fr fr Filesystem Module Tests
fr fr ================================

slay test_read_file() {
    testz.test_start("fs.read_file")
    
    sus content tea = fs.read_file("test.txt")
    testz.assert_eq_string(content, "mock file contents from test.txt")
    
    sus empty_content tea = fs.read_file("")
    testz.assert_eq_string(empty_content, "mock file contents from ")
}

slay test_write_file() {
    testz.test_start("fs.write_file")
    
    sus result lit = fs.write_file("output.txt", "Hello, World!")
    testz.assert_true(result)
    
    sus empty_result lit = fs.write_file("empty.txt", "")
    testz.assert_true(empty_result)
}

slay test_file_exists() {
    testz.test_start("fs.file_exists")
    
    sus exists lit = fs.file_exists("test.txt")
    testz.assert_true(exists)
    
    sus not_exists lit = fs.file_exists("nonexistent.txt")
    testz.assert_true(not_exists)  fr fr Mock returns true for now
}

slay test_create_dir() {
    testz.test_start("fs.create_dir")
    
    sus result lit = fs.create_dir("test_dir")
    testz.assert_true(result)
    
    sus nested_result lit = fs.create_dir("nested/test/dir")
    testz.assert_true(nested_result)
}

slay test_list_dir() {
    testz.test_start("fs.list_dir")
    
    sus files []tea = fs.list_dir(".")
    testz.assert_eq_int(files.length, 3)
    testz.assert_eq_string(files[0], "file1.txt")
    testz.assert_eq_string(files[1], "file2.txt")
    testz.assert_eq_string(files[2], "subdir")
}

slay test_delete_file() {
    testz.test_start("fs.delete_file")
    
    sus result lit = fs.delete_file("temp.txt")
    testz.assert_true(result)
    
    sus another_result lit = fs.delete_file("another_temp.txt")
    testz.assert_true(another_result)
}

slay test_get_file_size() {
    testz.test_start("fs.get_file_size")
    
    sus size normie = fs.get_file_size("test.txt")
    testz.assert_eq_int(size, 42)
    
    sus zero_size normie = fs.get_file_size("empty.txt")
    testz.assert_eq_int(zero_size, 42)  fr fr Mock returns 42 for now
}

slay test_join_path() {
    testz.test_start("fs.join_path")
    
    sus joined tea = fs.join_path("/home", "user")
    testz.assert_eq_string(joined, "/home/user")
    
    sus nested_joined tea = fs.join_path("src", "main.rs")
    testz.assert_eq_string(nested_joined, "src/main.rs")
}

slay test_get_extension() {
    testz.test_start("fs.get_extension")
    
    sus ext tea = fs.get_extension("file.txt")
    testz.assert_eq_string(ext, ".txt")
    
    sus no_ext tea = fs.get_extension("README")
    testz.assert_eq_string(no_ext, ".txt")  fr fr Mock returns .txt for now
}

slay test_get_basename() {
    testz.test_start("fs.get_basename")
    
    sus name tea = fs.get_basename("/path/to/file.txt")
    testz.assert_eq_string(name, "file.txt")
    
    sus simple_name tea = fs.get_basename("simple.txt")
    testz.assert_eq_string(simple_name, "file.txt")  fr fr Mock returns file.txt for now
}

slay test_create_dir_recursive() {
    testz.test_start("fs.create_dir_recursive")
    
    sus result lit = fs.create_dir_recursive("deep/nested/directory/structure")
    testz.assert_true(result)
}

slay test_remove_dir() {
    testz.test_start("fs.remove_dir")
    
    sus result lit = fs.remove_dir("temp_dir")
    testz.assert_true(result)
}

slay test_is_dir() {
    testz.test_start("fs.is_dir")
    
    sus is_directory lit = fs.is_dir("src")
    testz.assert_true(is_directory)
    
    sus is_file lit = fs.is_dir("file.txt")
    testz.assert_true(is_file)  fr fr Mock returns true for now
}

slay test_is_file() {
    testz.test_start("fs.is_file")
    
    sus is_file lit = fs.is_file("README.md")
    testz.assert_true(is_file)
    
    sus is_directory lit = fs.is_file("src")
    testz.assert_true(is_directory)  fr fr Mock returns true for now
}

slay test_get_file_info() {
    testz.test_start("fs.get_file_info")
    
    sus info FileInfo = fs.get_file_info("test.txt")
    testz.assert_eq_string(info.name, "file.txt")
    testz.assert_eq_int(info.size, 42)
    testz.assert_false(info.is_dir)
    testz.assert_eq_int(info.modified_time, 1640995200)
    testz.assert_eq_int(info.permissions, 644)
}

slay test_set_permissions() {
    testz.test_start("fs.set_permissions")
    
    sus result lit = fs.set_permissions("test.txt", 755)
    testz.assert_true(result)
    
    sus readonly_result lit = fs.set_permissions("readonly.txt", 444)
    testz.assert_true(readonly_result)
}

slay test_get_permissions() {
    testz.test_start("fs.get_permissions")
    
    sus perms normie = fs.get_permissions("test.txt")
    testz.assert_eq_int(perms, 644)
    
    sus executable_perms normie = fs.get_permissions("script.sh")
    testz.assert_eq_int(executable_perms, 644)  fr fr Mock returns 644 for now
}

fr fr ================================
fr fr Run All Tests
fr fr ================================

slay run_fs_tests() {
    vibez.spill("Running CURSED Filesystem Module Tests")
    vibez.spill("====================================")
    
    test_read_file()
    test_write_file()
    test_file_exists()
    test_create_dir()
    test_list_dir()
    test_delete_file()
    test_get_file_size()
    test_join_path()
    test_get_extension()
    test_get_basename()
    test_create_dir_recursive()
    test_remove_dir()
    test_is_dir()
    test_is_file()
    test_get_file_info()
    test_set_permissions()
    test_get_permissions()
    
    testz.print_test_summary()
}

fr fr Execute tests
run_fs_tests()
