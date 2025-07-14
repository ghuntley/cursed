yeet "testz"
yeet "zip_zilla"
yeet "dropz"
yeet "stringz"

# Comprehensive test suite for zip_zilla archive handling module
# Tests all major archive operations with proper error handling

# Test data setup
slay setup_test_files() lit {
    # Create test directory structure
    sus test_dir_created lit = dropz.create_directory("test_archive_data")
    lowkey !test_dir_created {
        damn cap
    }
    
    # Create sample files for testing
    sus file1_created lit = dropz.write_file("test_archive_data/file1.txt", "This is test file 1 - absolutely fire content!")
    sus file2_created lit = dropz.write_file("test_archive_data/file2.txt", "Test file 2 content - more good stuff here bestie!")
    sus file3_created lit = dropz.write_file("test_archive_data/subdir/nested.txt", "Nested file content - going deep!")
    
    damn file1_created && file2_created && file3_created
}

# Clean up test files
slay cleanup_test_files() lit {
    dropz.remove_directory_recursive("test_archive_data")
    dropz.remove_file("test_output.zip")
    dropz.remove_file("test_output.tar")
    dropz.remove_file("test_output.tar.gz")
    dropz.remove_file("test_protected.zip")
    dropz.remove_directory_recursive("test_extract")
    damn based
}

# Test ZIP archive creation
slay test_zip_creation() lit {
    test_start("ZIP Archive Creation")
    
    # Setup test files
    sus setup_ok lit = setup_test_files()
    assert_true(setup_ok, "Failed to setup test files")
    
    # Create ZIP archive
    sus file_list [tea] = ["test_archive_data/file1.txt", "test_archive_data/file2.txt"]
    sus result tea = zip_zilla.create_zip_archive("test_output.zip", file_list, zip_zilla.BALANCED_COMPRESSION)
    
    # Verify result
    assert_true(stringz.contains(result, "successfully"), "ZIP creation should succeed")
    assert_true(dropz.file_exists("test_output.zip"), "ZIP file should exist")
    
    # Check file size is reasonable
    sus file_size normie = dropz.get_file_size("test_output.zip")
    assert_true(file_size > 0, "ZIP file should have content")
    
    vibez.spill("✅ ZIP creation test passed - archive created successfully!")
    damn based
}

# Test ZIP extraction
slay test_zip_extraction() lit {
    test_start("ZIP Archive Extraction")
    
    # Extract the ZIP we created
    sus result tea = zip_zilla.extract_zip_archive("test_output.zip", "test_extract", "")
    
    # Verify extraction result
    assert_true(stringz.contains(result, "successfully"), "ZIP extraction should succeed")
    assert_true(dropz.directory_exists("test_extract"), "Extract directory should exist")
    assert_true(dropz.file_exists("test_extract/test_archive_data/file1.txt"), "Extracted file1 should exist")
    assert_true(dropz.file_exists("test_extract/test_archive_data/file2.txt"), "Extracted file2 should exist")
    
    # Verify extracted content
    sus content1 tea = dropz.read_file("test_extract/test_archive_data/file1.txt")
    assert_true(stringz.contains(content1, "test file 1"), "Extracted content should match")
    
    vibez.spill("✅ ZIP extraction test passed - files extracted correctly!")
    damn based
}

# Test TAR archive creation
slay test_tar_creation() lit {
    test_start("TAR Archive Creation")
    
    # Create TAR archive from directory
    sus result tea = zip_zilla.create_tar_archive("test_output.tar", "test_archive_data", zip_zilla.TAR_FORMAT)
    
    # Verify TAR creation
    assert_true(stringz.contains(result, "successfully"), "TAR creation should succeed")
    assert_true(dropz.file_exists("test_output.tar"), "TAR file should exist")
    
    # Check file size
    sus file_size normie = dropz.get_file_size("test_output.tar")
    assert_true(file_size > 0, "TAR file should have content")
    
    vibez.spill("✅ TAR creation test passed - TAR archive created!")
    damn based
}

# Test TAR.GZ compressed archive
slay test_tar_gz_creation() lit {
    test_start("TAR.GZ Archive Creation")
    
    # Create compressed TAR archive
    sus result tea = zip_zilla.create_tar_archive("test_output.tar.gz", "test_archive_data", zip_zilla.TAR_GZ_FORMAT)
    
    # Verify compressed TAR creation
    assert_true(stringz.contains(result, "successfully"), "TAR.GZ creation should succeed")
    assert_true(dropz.file_exists("test_output.tar.gz"), "TAR.GZ file should exist")
    
    # Compressed should be smaller than uncompressed
    sus tar_size normie = dropz.get_file_size("test_output.tar")
    sus tar_gz_size normie = dropz.get_file_size("test_output.tar.gz")
    assert_true(tar_gz_size <= tar_size, "Compressed TAR should be smaller or equal")
    
    vibez.spill("✅ TAR.GZ creation test passed - compression working!")
    damn based
}

# Test archive format detection
slay test_format_detection() lit {
    test_start("Archive Format Detection")
    
    # Test ZIP format detection
    sus zip_format normie = zip_zilla.detect_archive_format("test_output.zip")
    assert_eq_int(zip_format, zip_zilla.ZIP_FORMAT, "Should detect ZIP format")
    
    # Test TAR format detection
    sus tar_format normie = zip_zilla.detect_archive_format("test_output.tar")
    assert_eq_int(tar_format, zip_zilla.TAR_FORMAT, "Should detect TAR format")
    
    # Test TAR.GZ format detection
    sus tar_gz_format normie = zip_zilla.detect_archive_format("test_output.tar.gz")
    assert_eq_int(tar_gz_format, zip_zilla.TAR_GZ_FORMAT, "Should detect TAR.GZ format")
    
    # Test unknown format
    sus unknown_format normie = zip_zilla.detect_archive_format("nonexistent.xyz")
    assert_eq_int(unknown_format, 0, "Should return 0 for unknown format")
    
    vibez.spill("✅ Format detection test passed - all formats detected correctly!")
    damn based
}

# Test archive integrity validation
slay test_archive_validation() lit {
    test_start("Archive Integrity Validation")
    
    # Test valid archive
    sus zip_valid lit = zip_zilla.validate_archive_integrity("test_output.zip")
    assert_true(zip_valid, "Valid ZIP should pass integrity check")
    
    sus tar_valid lit = zip_zilla.validate_archive_integrity("test_output.tar")
    assert_true(tar_valid, "Valid TAR should pass integrity check")
    
    # Test invalid/nonexistent archive
    sus invalid_valid lit = zip_zilla.validate_archive_integrity("nonexistent.zip")
    assert_false(invalid_valid, "Nonexistent archive should fail integrity check")
    
    vibez.spill("✅ Archive validation test passed - integrity checks working!")
    damn based
}

# Test listing archive contents
slay test_list_contents() lit {
    test_start("Archive Contents Listing")
    
    # List ZIP contents
    sus zip_contents [zip_zilla.ArchiveEntry] = zip_zilla.list_archive_contents("test_output.zip")
    assert_true(zip_contents.length > 0, "ZIP should have entries")
    
    # Check if our test files are in the list
    sus found_file1 lit = cap
    bestie i normie = 0; i < zip_contents.length; i++ {
        lowkey stringz.contains(zip_contents[i].0, "file1.txt") {
            found_file1 = based
            ghosted
        }
    }
    assert_true(found_file1, "Should find file1.txt in archive contents")
    
    vibez.spill("✅ Contents listing test passed - can browse archive contents!")
    damn based
}

# Test password-protected archives
slay test_password_protection() lit {
    test_start("Password-Protected Archives")
    
    # Create password-protected archive
    sus file_list [tea] = ["test_archive_data/file1.txt", "test_archive_data/file2.txt"]
    sus result tea = zip_zilla.create_protected_archive("test_protected.zip", file_list, "supersecret123", zip_zilla.BALANCED_COMPRESSION)
    
    # Verify protected archive creation
    assert_true(stringz.contains(result, "successfully"), "Protected archive creation should succeed")
    assert_true(dropz.file_exists("test_protected.zip"), "Protected ZIP should exist")
    
    # Test extraction with correct password
    sus extract_result tea = zip_zilla.extract_zip_archive("test_protected.zip", "test_extract_protected", "supersecret123")
    assert_true(stringz.contains(extract_result, "successfully"), "Should extract with correct password")
    
    # Test extraction with wrong password (should fail)
    sus wrong_password_result tea = zip_zilla.extract_zip_archive("test_protected.zip", "test_extract_wrong", "wrongpassword")
    assert_true(stringz.contains(wrong_password_result, "Error"), "Should fail with wrong password")
    
    vibez.spill("✅ Password protection test passed - security working!")
    damn based
}

# Test archive information retrieval
slay test_archive_info() lit {
    test_start("Archive Information Retrieval")
    
    # Get info for ZIP archive
    sus zip_info tea = zip_zilla.get_archive_info("test_output.zip")
    assert_true(stringz.contains(zip_info, "ZIP"), "Info should mention ZIP format")
    assert_true(stringz.contains(zip_info, "bytes"), "Info should include size")
    assert_true(stringz.contains(zip_info, "Entries"), "Info should include entry count")
    
    # Get info for TAR archive
    sus tar_info tea = zip_zilla.get_archive_info("test_output.tar")
    assert_true(stringz.contains(tar_info, "TAR"), "Info should mention TAR format")
    
    vibez.spill("✅ Archive info test passed - detailed information available!")
    damn based
}

# Test single file extraction
slay test_single_file_extraction() lit {
    test_start("Single File Extraction")
    
    # Extract just one file from archive
    sus result tea = zip_zilla.extract_single_file("test_output.zip", "test_archive_data/file1.txt", "test_single_extract.txt")
    
    # Verify single file extraction
    assert_true(stringz.contains(result, "successfully"), "Single file extraction should succeed")
    assert_true(dropz.file_exists("test_single_extract.txt"), "Extracted single file should exist")
    
    # Verify content
    sus content tea = dropz.read_file("test_single_extract.txt")
    assert_true(stringz.contains(content, "test file 1"), "Single extracted file should have correct content")
    
    # Clean up
    dropz.remove_file("test_single_extract.txt")
    
    vibez.spill("✅ Single file extraction test passed - surgical extraction working!")
    damn based
}

# Test compression levels
slay test_compression_levels() lit {
    test_start("Compression Levels")
    
    sus test_files [tea] = ["test_archive_data/file1.txt", "test_archive_data/file2.txt"]
    
    # Test no compression
    sus no_compress_result tea = zip_zilla.create_zip_archive("test_no_compress.zip", test_files, zip_zilla.NO_COMPRESSION)
    assert_true(stringz.contains(no_compress_result, "successfully"), "No compression should work")
    
    # Test fast compression
    sus fast_compress_result tea = zip_zilla.create_zip_archive("test_fast_compress.zip", test_files, zip_zilla.FAST_COMPRESSION)
    assert_true(stringz.contains(fast_compress_result, "successfully"), "Fast compression should work")
    
    # Test max compression
    sus max_compress_result tea = zip_zilla.create_zip_archive("test_max_compress.zip", test_files, zip_zilla.MAX_COMPRESSION)
    assert_true(stringz.contains(max_compress_result, "successfully"), "Max compression should work")
    
    # Verify file sizes make sense (no compression >= fast >= max)
    sus no_size normie = dropz.get_file_size("test_no_compress.zip")
    sus fast_size normie = dropz.get_file_size("test_fast_compress.zip")
    sus max_size normie = dropz.get_file_size("test_max_compress.zip")
    
    assert_true(no_size >= fast_size, "No compression should be larger or equal to fast")
    assert_true(fast_size >= max_size, "Fast compression should be larger or equal to max")
    
    # Clean up
    dropz.remove_file("test_no_compress.zip")
    dropz.remove_file("test_fast_compress.zip")
    dropz.remove_file("test_max_compress.zip")
    
    vibez.spill("✅ Compression levels test passed - all levels working correctly!")
    damn based
}

# Test error handling
slay test_error_handling() lit {
    test_start("Error Handling")
    
    # Test creating archive with nonexistent files
    sus bad_files [tea] = ["nonexistent1.txt", "nonexistent2.txt"]
    sus error_result tea = zip_zilla.create_zip_archive("should_fail.zip", bad_files, zip_zilla.BALANCED_COMPRESSION)
    assert_true(stringz.contains(error_result, "Error"), "Should error on nonexistent files")
    
    # Test extracting nonexistent archive
    sus extract_error tea = zip_zilla.extract_zip_archive("nonexistent.zip", "test_extract", "")
    assert_true(stringz.contains(extract_error, "Error"), "Should error on nonexistent archive")
    
    # Test invalid compression level
    sus invalid_compression tea = zip_zilla.create_zip_archive("test.zip", ["test_archive_data/file1.txt"], 999)
    assert_true(stringz.contains(invalid_compression, "Error"), "Should error on invalid compression level")
    
    # Test weak password
    sus weak_password tea = zip_zilla.create_protected_archive("test.zip", ["test_archive_data/file1.txt"], "123", zip_zilla.BALANCED_COMPRESSION)
    assert_true(stringz.contains(weak_password, "Error"), "Should error on weak password")
    
    vibez.spill("✅ Error handling test passed - robust error detection!")
    damn based
}

# Test archive modification (add/remove files)
slay test_archive_modification() lit {
    test_start("Archive Modification")
    
    # Create base archive
    sus file_list [tea] = ["test_archive_data/file1.txt"]
    sus create_result tea = zip_zilla.create_zip_archive("test_modify.zip", file_list, zip_zilla.BALANCED_COMPRESSION)
    assert_true(stringz.contains(create_result, "successfully"), "Base archive should be created")
    
    # Add file to existing archive
    sus add_result tea = zip_zilla.add_file_to_archive("test_modify.zip", "test_archive_data/file2.txt")
    assert_true(stringz.contains(add_result, "successfully"), "Should add file to archive")
    
    # Verify file was added by listing contents
    sus contents [zip_zilla.ArchiveEntry] = zip_zilla.list_archive_contents("test_modify.zip")
    sus found_file2 lit = cap
    bestie i normie = 0; i < contents.length; i++ {
        lowkey stringz.contains(contents[i].0, "file2.txt") {
            found_file2 = based
            ghosted
        }
    }
    assert_true(found_file2, "Should find added file2.txt in archive")
    
    # Remove file from archive
    sus remove_result tea = zip_zilla.remove_file_from_archive("test_modify.zip", "test_archive_data/file2.txt")
    assert_true(stringz.contains(remove_result, "successfully"), "Should remove file from archive")
    
    # Clean up
    dropz.remove_file("test_modify.zip")
    
    vibez.spill("✅ Archive modification test passed - can add/remove files!")
    damn based
}

# Test recompression functionality
slay test_recompression() lit {
    test_start("Archive Recompression")
    
    # Create archive with medium compression
    sus file_list [tea] = ["test_archive_data/file1.txt", "test_archive_data/file2.txt"]
    sus create_result tea = zip_zilla.create_zip_archive("test_recompress.zip", file_list, zip_zilla.FAST_COMPRESSION)
    assert_true(stringz.contains(create_result, "successfully"), "Original archive should be created")
    
    sus original_size normie = dropz.get_file_size("test_recompress.zip")
    
    # Recompress with maximum compression
    sus recompress_result tea = zip_zilla.recompress_archive("test_recompress.zip", zip_zilla.MAX_COMPRESSION)
    assert_true(stringz.contains(recompress_result, "successfully"), "Recompression should succeed")
    
    sus new_size normie = dropz.get_file_size("test_recompress.zip")
    assert_true(new_size <= original_size, "Recompressed archive should be smaller or equal")
    
    # Clean up
    dropz.remove_file("test_recompress.zip")
    
    vibez.spill("✅ Recompression test passed - can optimize existing archives!")
    damn based
}

# Run comprehensive test suite
slay run_zip_zilla_tests() lit {
    vibez.spill("🚀 Starting zip_zilla comprehensive test suite...")
    vibez.spill("Testing all archive operations with Gen Z flair!")
    vibez.spill("")
    
    # Core functionality tests
    test_zip_creation()
    test_zip_extraction()
    test_tar_creation()
    test_tar_gz_creation()
    
    # Advanced functionality tests
    test_format_detection()
    test_archive_validation()
    test_list_contents()
    test_password_protection()
    test_archive_info()
    test_single_file_extraction()
    
    # Quality and edge case tests
    test_compression_levels()
    test_error_handling()
    test_archive_modification()
    test_recompression()
    
    # Clean up all test files
    cleanup_test_files()
    
    vibez.spill("")
    vibez.spill("🎉 zip_zilla test suite completed successfully!")
    vibez.spill("All archive operations working perfectly - absolutely fire! 🔥")
    
    print_test_summary()
    damn based
}

# Run the comprehensive test suite
run_zip_zilla_tests()
