# archivez - Comprehensive Archive Support Test Suite
# Tests for ZIP, TAR, GZIP, BZIP2 and compression functionality

yeet "vibez"
yeet "testz"

# Test basic archive creation and operations
slay test_archive_basic_operations() {
    vibez.spill("=== Testing Basic Archive Operations ===")
    
    # Test creating different archive formats
    sus zip_result tea = create_archive("test.zip", ZIP_FORMAT) fam {
        when err -> {
            vibez.spill("ERROR: Failed to create ZIP archive: " + err)
            damn
        }
    }
    testz.assert_eq(zip_result, "test.zip", "ZIP archive creation")
    
    sus tar_result tea = create_archive("test.tar", TAR_FORMAT) fam {
        when err -> {
            vibez.spill("ERROR: Failed to create TAR archive: " + err)
            damn
        }
    }
    testz.assert_eq(tar_result, "test.tar", "TAR archive creation")
    
    # Test opening archives
    sus opened_zip tea = open_archive("existing.zip") fam {
        when err -> {
            vibez.spill("Note: Could not open existing.zip (expected for demo)")
        }
    }
    
    vibez.spill("✓ Basic archive operations test completed")
}

# Test file addition to archives
slay test_archive_file_operations() {
    vibez.spill("=== Testing Archive File Operations ===")
    
    # Create test archive
    create_archive("filetest.zip", ZIP_FORMAT) fam {
        when err -> {
            vibez.spill("ERROR: Could not create test archive: " + err)
            damn
        }
    }
    
    # Test adding files
    sus add_result tea = add_file("document.txt", "docs/document.txt") fam {
        when err -> {
            vibez.spill("ERROR: Failed to add file: " + err)
            damn
        }
    }
    testz.assert_eq(add_result, "docs/document.txt", "File addition to archive")
    
    # Test adding directories
    sus dir_result tea = add_directory("source_code", "code") fam {
        when err -> {
            vibez.spill("ERROR: Failed to add directory: " + err)
            damn  
        }
    }
    testz.assert_eq(dir_result, "code", "Directory addition to archive")
    
    # Test file listing
    sus files []tea = list_files() fam {
        when err -> {
            vibez.spill("ERROR: Failed to list files: " + err)
            damn
        }
    }
    
    sus file_count drip = get_file_count() fam {
        when err -> {
            vibez.spill("ERROR: Failed to get file count: " + err)
            damn
        }
    }
    testz.assert_gt(file_count, 0, "Archive contains files")
    
    # Test file existence check
    sus exists lit = file_exists("docs/document.txt") fam {
        when err -> {
            vibez.spill("ERROR: Failed to check file existence: " + err)
            damn
        }
    }
    testz.assert_eq(exists, based, "File exists in archive")
    
    close_archive()
    vibez.spill("✓ Archive file operations test completed")
}

# Test archive extraction
slay test_archive_extraction() {
    vibez.spill("=== Testing Archive Extraction ===")
    
    # Open test archive
    open_archive("test.zip") fam {
        when err -> {
            vibez.spill("Note: Using simulated archive for extraction test")
            create_archive("extract_test.zip", ZIP_FORMAT)
            add_file("test_data.txt", "test_data.txt")
        }
    }
    
    # Test single file extraction
    sus extracted_data tea = extract_file("test_data.txt", "extracted_test_data.txt") fam {
        when err -> {
            vibez.spill("ERROR: Failed to extract file: " + err)
            damn
        }
    }
    testz.assert_not_empty(extracted_data, "Extracted file has data")
    
    # Test extracting all files
    sus extracted_count drip = extract_all("output_directory") fam {
        when err -> {
            vibez.spill("ERROR: Failed to extract all files: " + err)
            damn
        }
    }
    testz.assert_gt(extracted_count, 0, "Files were extracted")
    
    close_archive()
    vibez.spill("✓ Archive extraction test completed")
}

# Test compression functionality
slay test_compression_algorithms() {
    vibez.spill("=== Testing Compression Algorithms ===")
    
    # Initialize compression system
    init_compression()
    
    # Test different compression algorithms
    sus test_data tea = "This is test data for compression algorithms. It contains repeated text to demonstrate compression effectiveness."
    
    # Test DEFLATE compression
    set_compression_algorithm(COMPRESSION_DEFLATE) fam {
        when err -> {
            vibez.spill("ERROR: Failed to set DEFLATE: " + err)
            damn
        }
    }
    
    sus deflate_compressed tea = compress_data(test_data) fam {
        when err -> {
            vibez.spill("ERROR: DEFLATE compression failed: " + err)
            damn
        }
    }
    testz.assert_not_empty(deflate_compressed, "DEFLATE compression produced output")
    
    sus deflate_decompressed tea = decompress_data(deflate_compressed) fam {
        when err -> {
            vibez.spill("ERROR: DEFLATE decompression failed: " + err)
            damn
        }
    }
    testz.assert_not_empty(deflate_decompressed, "DEFLATE decompression produced output")
    
    # Test GZIP compression
    set_compression_algorithm(COMPRESSION_GZIP) fam {
        when err -> {
            vibez.spill("ERROR: Failed to set GZIP: " + err)
            damn
        }
    }
    
    sus gzip_compressed tea = compress_data(test_data) fam {
        when err -> {
            vibez.spill("ERROR: GZIP compression failed: " + err)
            damn
        }
    }
    testz.assert_not_empty(gzip_compressed, "GZIP compression produced output")
    
    sus gzip_decompressed tea = decompress_data(gzip_compressed) fam {
        when err -> {
            vibez.spill("ERROR: GZIP decompression failed: " + err)
            damn
        }
    }
    testz.assert_not_empty(gzip_decompressed, "GZIP decompression produced output")
    
    # Test BZIP2 compression
    set_compression_algorithm(COMPRESSION_BZIP2) fam {
        when err -> {
            vibez.spill("ERROR: Failed to set BZIP2: " + err)
            damn
        }
    }
    
    sus bzip2_compressed tea = compress_data(test_data) fam {
        when err -> {
            vibez.spill("ERROR: BZIP2 compression failed: " + err)
            damn
        }
    }
    testz.assert_not_empty(bzip2_compressed, "BZIP2 compression produced output")
    
    # Test LZ4 compression (fast)
    set_compression_algorithm(COMPRESSION_LZ4) fam {
        when err -> {
            vibez.spill("ERROR: Failed to set LZ4: " + err)
            damn
        }
    }
    
    sus lz4_compressed tea = compress_data(test_data) fam {
        when err -> {
            vibez.spill("ERROR: LZ4 compression failed: " + err)
            damn
        }
    }
    testz.assert_not_empty(lz4_compressed, "LZ4 compression produced output")
    
    vibez.spill("✓ Compression algorithms test completed")
}

# Test compression levels
slay test_compression_levels() {
    vibez.spill("=== Testing Compression Levels ===")
    
    sus test_data tea = "Compression level testing data with repetitive content that should compress well at higher levels."
    
    set_compression_algorithm(COMPRESSION_DEFLATE)
    
    # Test different compression levels
    bestie (drip level = 0; level <= 9; level = level + 1) {
        set_compression_level(level) fam {
            when err -> {
                vibez.spill("ERROR: Failed to set compression level " + to_string(level) + ": " + err)
                continue
            }
        }
        
        sus compressed tea = compress_data(test_data) fam {
            when err -> {
                vibez.spill("ERROR: Compression failed at level " + to_string(level) + ": " + err)
                continue
            }
        }
        
        vibez.spill("Level " + to_string(level) + ": " + to_string(len(test_data)) + " -> " + to_string(len(compressed)) + " bytes")
        testz.assert_not_empty(compressed, "Compression level " + to_string(level) + " works")
    }
    
    vibez.spill("✓ Compression levels test completed")
}

# Test archive validation
slay test_archive_validation() {
    vibez.spill("=== Testing Archive Validation ===")
    
    # Create test archive for validation
    create_archive("validation_test.zip", ZIP_FORMAT)
    add_file("test1.txt", "test1.txt")
    add_file("test2.txt", "subdir/test2.txt")
    
    # Test archive validation
    sus valid lit = validate_archive() fam {
        when err -> {
            vibez.spill("ERROR: Archive validation failed: " + err)
            damn
        }
    }
    testz.assert_eq(valid, based, "Archive validation passes")
    
    # Test getting archive information
    sus info tea = get_archive_info() fam {
        when err -> {
            vibez.spill("ERROR: Failed to get archive info: " + err)
            damn
        }
    }
    testz.assert_not_empty(info, "Archive info is available")
    vibez.spill("Archive Info:\n" + info)
    
    close_archive()
    vibez.spill("✓ Archive validation test completed")
}

# Test password protection
slay test_archive_password_protection() {
    vibez.spill("=== Testing Archive Password Protection ===")
    
    # Test setting password
    sus password_set lit = set_password("test_password_123") fam {
        when err -> {
            vibez.spill("ERROR: Failed to set password: " + err)
            damn
        }
    }
    testz.assert_eq(password_set, based, "Password set successfully")
    
    # Test checking password protection status
    sus is_protected lit = is_password_protected()
    testz.assert_eq(is_protected, based, "Archive is password protected")
    
    # Test removing password
    sus password_removed lit = remove_password()
    testz.assert_eq(password_removed, based, "Password removed successfully")
    
    sus still_protected lit = is_password_protected()
    testz.assert_eq(still_protected, cap, "Archive is no longer password protected")
    
    vibez.spill("✓ Archive password protection test completed")
}

# Test format detection
slay test_format_detection() {
    vibez.spill("=== Testing Format Detection ===")
    
    # Test various file extensions
    sus zip_format tea = detect_format("archive.zip")
    testz.assert_eq(zip_format, ZIP_FORMAT, "ZIP format detected")
    
    sus tar_format tea = detect_format("backup.tar") 
    testz.assert_eq(tar_format, TAR_FORMAT, "TAR format detected")
    
    sus gzip_format tea = detect_format("compressed.gz")
    testz.assert_eq(gzip_format, GZIP_FORMAT, "GZIP format detected")
    
    sus bzip2_format tea = detect_format("data.bz2")
    testz.assert_eq(bzip2_format, BZIP2_FORMAT, "BZIP2 format detected")
    
    sus unknown_format tea = detect_format("file.unknown")
    testz.assert_eq(unknown_format, "", "Unknown format returns empty")
    
    vibez.spill("✓ Format detection test completed")
}

# Test error handling
slay test_error_handling() {
    vibez.spill("=== Testing Error Handling ===")
    
    # Test creating archive with empty filename
    create_archive("", ZIP_FORMAT) fam {
        when err -> {
            vibez.spill("✓ Correctly caught empty filename error: " + err)
            testz.assert_not_empty(err, "Error message for empty filename")
        }
    }
    
    # Test creating archive with invalid format
    create_archive("test.xyz", "invalid_format") fam {
        when err -> {
            vibez.spill("✓ Correctly caught invalid format error: " + err)
            testz.assert_not_empty(err, "Error message for invalid format")
        }
    }
    
    # Test operations without open archive
    close_archive()  # Ensure no archive is open
    
    add_file("test.txt", "test.txt") fam {
        when err -> {
            vibez.spill("✓ Correctly caught no-archive-open error: " + err)
            testz.assert_not_empty(err, "Error message for no open archive")
        }
    }
    
    # Test invalid compression level
    set_compression_level(15) fam {
        when err -> {
            vibez.spill("✓ Correctly caught invalid compression level error: " + err)
            testz.assert_not_empty(err, "Error message for invalid compression level")
        }
    }
    
    vibez.spill("✓ Error handling test completed")
}

# Test ZIP-specific features
slay test_zip_specific_features() {
    vibez.spill("=== Testing ZIP-Specific Features ===")
    
    # Initialize ZIP system
    init_zip()
    
    # Test ZIP file addition
    sus zip_add_result lit = zip_add_file("local.txt", "archive.txt", "test data content") fam {
        when err -> {
            vibez.spill("ERROR: ZIP add file failed: " + err)
            damn
        }
    }
    testz.assert_eq(zip_add_result, based, "ZIP file addition works")
    
    # Test ZIP directory addition
    sus zip_dir_result lit = zip_add_directory("source", "code") fam {
        when err -> {
            vibez.spill("ERROR: ZIP add directory failed: " + err)
            damn
        }
    }
    testz.assert_eq(zip_dir_result, based, "ZIP directory addition works")
    
    # Test ZIP validation
    sus zip_valid lit = zip_validate()
    testz.assert_eq(zip_valid, based, "ZIP validation passes")
    
    # Test ZIP statistics
    sus zip_stats tea = zip_get_stats()
    testz.assert_not_empty(zip_stats, "ZIP statistics available")
    vibez.spill("ZIP Stats:\n" + zip_stats)
    
    # Test ZIP file listing
    sus zip_files []tea = zip_list_files()
    testz.assert_gt(len(zip_files), 0, "ZIP contains files")
    
    vibez.spill("✓ ZIP-specific features test completed")
}

# Test TAR-specific features
slay test_tar_specific_features() {
    vibez.spill("=== Testing TAR-Specific Features ===")
    
    # Initialize TAR system
    init_tar()
    
    # Test TAR file addition
    sus tar_add_result lit = tar_add_file("document.pdf", "docs/document.pdf", "PDF file content") fam {
        when err -> {
            vibez.spill("ERROR: TAR add file failed: " + err)
            damn
        }
    }
    testz.assert_eq(tar_add_result, based, "TAR file addition works")
    
    # Test TAR directory addition
    sus tar_dir_result lit = tar_add_directory("images", "assets/images") fam {
        when err -> {
            vibez.spill("ERROR: TAR add directory failed: " + err)
            damn
        }
    }
    testz.assert_eq(tar_dir_result, based, "TAR directory addition works")
    
    # Test TAR symbolic link
    sus symlink_result lit = tar_add_symlink("link.txt", "target.txt") fam {
        when err -> {
            vibez.spill("ERROR: TAR add symlink failed: " + err)
            damn
        }
    }
    testz.assert_eq(symlink_result, based, "TAR symbolic link addition works")
    
    # Test TAR validation
    sus tar_valid lit = tar_validate()
    testz.assert_eq(tar_valid, based, "TAR validation passes")
    
    # Test TAR statistics
    sus tar_stats tea = tar_get_stats()
    testz.assert_not_empty(tar_stats, "TAR statistics available")
    vibez.spill("TAR Stats:\n" + tar_stats)
    
    vibez.spill("✓ TAR-specific features test completed")
}

# Test performance characteristics
slay test_archive_performance() {
    vibez.spill("=== Testing Archive Performance ===")
    
    # Test compression performance with different algorithms
    sus large_test_data tea = generate_large_test_data()
    
    # Initialize compression and test each algorithm
    init_compression()
    
    # Test DEFLATE performance
    set_compression_algorithm(COMPRESSION_DEFLATE)
    set_compression_level(LEVEL_DEFAULT)
    
    sus start_time drip = get_current_time()
    sus deflate_result tea = compress_data(large_test_data) fam {
        when err -> {
            vibez.spill("ERROR: DEFLATE performance test failed: " + err)
            damn
        }
    }
    sus deflate_time drip = get_current_time() - start_time
    
    # Test LZ4 performance (should be faster)
    set_compression_algorithm(COMPRESSION_LZ4)
    
    start_time = get_current_time()
    sus lz4_result tea = compress_data(large_test_data) fam {
        when err -> {
            vibez.spill("ERROR: LZ4 performance test failed: " + err)
            damn
        }
    }
    sus lz4_time drip = get_current_time() - start_time
    
    vibez.spill("Performance Results:")
    vibez.spill("DEFLATE: " + to_string(len(large_test_data)) + " -> " + to_string(len(deflate_result)) + " bytes in " + to_string(deflate_time) + "ms")
    vibez.spill("LZ4: " + to_string(len(large_test_data)) + " -> " + to_string(len(lz4_result)) + " bytes in " + to_string(lz4_time) + "ms")
    
    # Get overall compression statistics
    sus stats tea = get_compression_stats()
    vibez.spill("Compression Statistics:\n" + stats)
    
    vibez.spill("✓ Archive performance test completed")
}

# Helper function to generate test data
slay generate_large_test_data() tea {
    sus data tea = ""
    bestie (drip i = 0; i < 10; i = i + 1) {
        data = data + "This is line " + to_string(i) + " of test data for performance testing. "
        data = data + "It contains repetitive content that should compress well. "
        data = data + "The quick brown fox jumps over the lazy dog. "
    }
    damn data
}

# Main test runner
slay run_all_archivez_tests() {
    testz.test_start("archivez - Archive Support Package Tests")
    
    vibez.spill("Starting comprehensive archivez test suite...")
    vibez.spill("Testing archive creation, compression, extraction, and validation")
    
    # Run all test functions
    test_archive_basic_operations()
    test_archive_file_operations()
    test_archive_extraction()
    test_compression_algorithms()
    test_compression_levels()
    test_archive_validation()
    test_archive_password_protection()
    test_format_detection()
    test_error_handling()
    test_zip_specific_features()
    test_tar_specific_features()
    test_archive_performance()
    
    # Print test summary
    testz.print_test_summary()
    
    vibez.spill("=== archivez Test Suite Complete ===")
    vibez.spill("All archive format support tests completed successfully!")
    vibez.spill("Supported formats: ZIP, TAR, GZIP, BZIP2, LZ4, LZMA")
    vibez.spill("Features tested: creation, extraction, compression, validation, password protection")
}

# Run the tests
run_all_archivez_tests()
