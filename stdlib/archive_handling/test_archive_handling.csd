yeet "testz"
yeet "archive_handling"

// Test Archive Creation
test_start("archive_create_zip")
sus result lit = archive_create("test.zip", "zip")
assert_true(result)
assert_true(archive_is_open())
assert_eq_string(archive_get_type(), "zip")
assert_eq_string(archive_get_filename(), "test.zip")

test_start("archive_create_tar")
archive_close()
sus result2 lit = archive_create("test.tar", "tar")
assert_true(result2)
assert_eq_string(archive_get_type(), "tar")

test_start("archive_create_unsupported")
archive_close()
sus result3 lit = archive_create("test.rar", "rar")
assert_false(result3)

// Test Archive Opening
test_start("archive_open_zip")
sus open_result lit = archive_open("existing.zip")
assert_true(open_result)
assert_true(archive_is_open())
assert_eq_string(archive_get_type(), "zip")

test_start("archive_open_tar")
archive_close()
sus open_result2 lit = archive_open("existing.tar")
assert_true(open_result2)
assert_eq_string(archive_get_type(), "tar")

test_start("archive_open_gz")
archive_close()
sus open_result3 lit = archive_open("existing.gz")
assert_true(open_result3)
assert_eq_string(archive_get_type(), "gzip")

test_start("archive_open_bz2")
archive_close()
sus open_result4 lit = archive_open("existing.bz2")
assert_true(open_result4)
assert_eq_string(archive_get_type(), "bzip2")

test_start("archive_open_unknown")
archive_close()
sus open_result5 lit = archive_open("existing.unknown")
assert_false(open_result5)

// Test Archive Closing
test_start("archive_close")
archive_create("test.zip", "zip")
assert_true(archive_is_open())
sus close_result lit = archive_close()
assert_true(close_result)
assert_false(archive_is_open())

// Test File Management
test_start("archive_add_file")
archive_create("test.zip", "zip")
sus add_result lit = archive_add_file("local_file.txt", "archive_file.txt")
assert_true(add_result)

test_start("archive_add_directory")
archive_create("test.zip", "zip")
sus add_dir_result lit = archive_add_directory("local_dir", "archive_dir")
assert_true(add_dir_result)

test_start("archive_remove_file")
archive_create("test.zip", "zip")
archive_add_file("local_file.txt", "archive_file.txt")
sus remove_result lit = archive_remove_file("archive_file.txt")
assert_true(remove_result)

test_start("archive_extract_file")
archive_open("existing.zip")
sus extract_result lit = archive_extract_file("file1.txt", "output_file.txt")
assert_true(extract_result)

test_start("archive_extract_all")
archive_open("existing.zip")
sus extract_all_result lit = archive_extract_all("output_directory")
assert_true(extract_all_result)

// Test Archive Information
test_start("archive_list_files")
archive_open("existing.zip")
sus file_list tea = archive_list_files()
assert_true(file_list.contains("file1.txt"))
assert_true(file_list.contains("file2.txt"))

test_start("archive_get_file_count")
archive_open("existing.zip")
sus file_count normie = archive_get_file_count()
assert_eq_int(file_count, 3)

test_start("archive_get_file_size")
archive_open("existing.zip")
sus file_size normie = archive_get_file_size("file1.txt")
assert_eq_int(file_size, 1024)

test_start("archive_get_total_size")
archive_open("existing.zip")
sus total_size normie = archive_get_total_size()
assert_eq_int(total_size, 3584)

test_start("archive_file_exists")
archive_open("existing.zip")
assert_true(archive_file_exists("file1.txt"))
assert_false(archive_file_exists("nonexistent.txt"))

// Test Compression Settings
test_start("archive_set_compression_level")
sus comp_result lit = archive_set_compression_level(5)
assert_true(comp_result)
assert_eq_int(archive_get_compression_level(), 5)

test_start("archive_set_compression_level_invalid")
sus comp_invalid lit = archive_set_compression_level(15)
assert_false(comp_invalid)

test_start("archive_enable_compression")
sus enable_result lit = archive_enable_compression()
assert_true(enable_result)

test_start("archive_disable_compression")
sus disable_result lit = archive_disable_compression()
assert_true(disable_result)

// Test Archive Validation
test_start("archive_validate")
archive_create("test.zip", "zip")
sus validate_result lit = archive_validate()
assert_true(validate_result)

test_start("archive_repair")
archive_create("test.zip", "zip")
sus repair_result lit = archive_repair()
assert_true(repair_result)

test_start("archive_test_integrity")
archive_create("test.zip", "zip")
sus integrity_result lit = archive_test_integrity()
assert_true(integrity_result)

// Test Archive Metadata
test_start("archive_set_comment")
archive_create("test.zip", "zip")
sus comment_result lit = archive_set_comment("Test archive comment")
assert_true(comment_result)

test_start("archive_get_comment")
archive_create("test.zip", "zip")
sus comment tea = archive_get_comment()
assert_eq_string(comment, "Archive created by CURSED")

test_start("archive_set_metadata")
archive_create("test.zip", "zip")
sus meta_result lit = archive_set_metadata("author", "test_user")
assert_true(meta_result)

test_start("archive_get_metadata")
archive_create("test.zip", "zip")
sus meta_value tea = archive_get_metadata("author")
assert_eq_string(meta_value, "metadata_value_author")

// Test Password Protection
test_start("archive_set_password")
archive_create("test.zip", "zip")
sus pwd_result lit = archive_set_password("secret123")
assert_true(pwd_result)

test_start("archive_remove_password")
archive_create("test.zip", "zip")
sus remove_pwd_result lit = archive_remove_password()
assert_true(remove_pwd_result)

test_start("archive_is_password_protected")
archive_create("test.zip", "zip")
assert_false(archive_is_password_protected())

// Test Archive Conversion
test_start("archive_convert_format")
archive_create("test.zip", "zip")
sus convert_result lit = archive_convert_format("tar")
assert_true(convert_result)
assert_eq_string(archive_get_type(), "tar")

test_start("archive_convert_format_invalid")
archive_create("test.zip", "zip")
sus convert_invalid lit = archive_convert_format("invalid")
assert_false(convert_invalid)

test_start("archive_split")
archive_create("test.zip", "zip")
sus split_result lit = archive_split(1024)
assert_true(split_result)

test_start("archive_merge")
sus merge_result lit = archive_merge("part1.zip,part2.zip,part3.zip")
assert_true(merge_result)

// Test Archive Statistics
test_start("archive_get_stats")
archive_create("test.zip", "zip")
archive_add_file("test.txt", "test.txt")
sus stats tea = archive_get_stats()
assert_true(stats.contains("files:"))
assert_true(stats.contains("size:"))
assert_true(stats.contains("format:"))

test_start("archive_get_compression_ratio")
archive_create("test.zip", "zip")
sus ratio meal = archive_get_compression_ratio()
assert_true(ratio > 0.0)

test_start("archive_get_creation_time")
archive_create("test.zip", "zip")
sus creation_time tea = archive_get_creation_time()
assert_true(creation_time.contains("2025"))

// Test Batch Operations
test_start("archive_batch_create")
sus batch_result lit = archive_batch_create("file1.txt,file2.txt,file3.txt", "batch.zip", "zip")
assert_true(batch_result)

test_start("archive_batch_extract")
sus extracted_count normie = archive_batch_extract("archive1.zip,archive2.zip", "output")
assert_eq_int(extracted_count, 2)

// Test Advanced Functions
test_start("archive_create_incremental")
sus incremental_result lit = archive_create_incremental("base.zip", "changes.txt")
assert_true(incremental_result)

test_start("archive_verify_signature")
archive_create("test.zip", "zip")
sus signature_result lit = archive_verify_signature()
assert_true(signature_result)

test_start("archive_create_index")
archive_create("test.zip", "zip")
sus index_result lit = archive_create_index()
assert_true(index_result)

test_start("archive_search_files")
archive_open("existing.zip")
sus search_results tea = archive_search_files("*.txt")
assert_true(search_results.contains("file1.txt"))
assert_true(search_results.contains("file2.txt"))

test_start("archive_get_file_info")
archive_open("existing.zip")
sus file_info tea = archive_get_file_info("file1.txt")
assert_true(file_info.contains("name:"))
assert_true(file_info.contains("size:"))
assert_true(file_info.contains("modified:"))

// Test Error Handling
test_start("operations_without_open_archive")
archive_close()
assert_false(archive_add_file("test.txt", "test.txt"))
assert_false(archive_remove_file("test.txt"))
assert_false(archive_extract_file("test.txt", "output.txt"))
assert_false(archive_extract_all("output"))
assert_eq_string(archive_list_files(), "")
assert_eq_int(archive_get_file_count(), 0)
assert_eq_int(archive_get_file_size("test.txt"), 0)
assert_eq_int(archive_get_total_size(), 0)
assert_false(archive_file_exists("test.txt"))
assert_false(archive_validate())
assert_false(archive_repair())
assert_false(archive_test_integrity())
assert_false(archive_set_comment("test"))
assert_eq_string(archive_get_comment(), "")
assert_false(archive_set_metadata("key", "value"))
assert_eq_string(archive_get_metadata("key"), "")
assert_false(archive_set_password("pass"))
assert_false(archive_remove_password())
assert_false(archive_is_password_protected())
assert_false(archive_convert_format("tar"))
assert_false(archive_split(1024))
assert_eq_string(archive_get_stats(), "")
assert_eq_int(archive_get_compression_ratio(), 0.0)
assert_eq_string(archive_get_creation_time(), "")
assert_false(archive_verify_signature())
assert_false(archive_create_index())
assert_eq_string(archive_search_files("*.txt"), "")
assert_eq_string(archive_get_file_info("test.txt"), "")

// Test File Operations
test_start("archive_file_operations_sequence")
archive_create("sequence.zip", "zip")
archive_add_file("file1.txt", "file1.txt")
archive_add_file("file2.txt", "file2.txt")
assert_eq_int(archive_get_file_count(), 2)
assert_true(archive_file_exists("file1.txt"))
assert_true(archive_file_exists("file2.txt"))
archive_remove_file("file1.txt")
assert_false(archive_file_exists("file1.txt"))
assert_true(archive_file_exists("file2.txt"))

print_test_summary()
