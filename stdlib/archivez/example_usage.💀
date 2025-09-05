# archivez - Example Usage Demonstration
# Shows how to use the archivez package for archive operations

yeet "vibez"
yeet "archivez"

# Example 1: Create and populate a ZIP archive
slay demo_zip_creation() {
    vibez.spill("=== ZIP Archive Creation Demo ===")
    
    # Create a new ZIP archive
    sus archive_name tea = create_archive("demo_backup.zip", ZIP_FORMAT) fam {
        when err -> {
            vibez.spill("Failed to create ZIP archive: " + err)
            damn
        }
    }
    
    vibez.spill("Created archive: " + archive_name)
    
    # Set compression level
    set_compression_level(6) fam {
        when err -> {
            vibez.spill("Failed to set compression level: " + err)
        }
    }
    
    # Add some files
    add_file("config.txt", "config/config.txt") fam {
        when err -> vibez.spill("Note: " + err + " (expected in demo)")
    }
    
    add_file("readme.md", "docs/readme.md") fam {
        when err -> vibez.spill("Note: " + err + " (expected in demo)")
    }
    
    # Add a directory
    add_directory("source", "code") fam {
        when err -> vibez.spill("Note: " + err + " (expected in demo)")
    }
    
    # Get archive information
    sus info tea = get_archive_info() fam {
        when err -> {
            vibez.spill("Could not get archive info: " + err)
            damn
        }
    }
    
    vibez.spill("Archive Information:")
    vibez.spill(info)
    
    # Set password protection
    set_password("secure123") fam {
        when err -> vibez.spill("Password setting failed: " + err)
    }
    
    # Validate archive
    sus valid lit = validate_archive() fam {
        when err -> {
            vibez.spill("Validation failed: " + err)
            damn
        }
    }
    
    ready (valid) {
        vibez.spill("✓ Archive validation passed")
    } otherwise {
        vibez.spill("✗ Archive validation failed")
    }
    
    # Close the archive
    close_archive()
    vibez.spill("ZIP archive demo completed")
}

# Example 2: Extract from an archive
slay demo_archive_extraction() {
    vibez.spill("\n=== Archive Extraction Demo ===")
    
    # Open an existing archive (simulated)
    open_archive("demo_backup.zip") fam {
        when err -> {
            vibez.spill("Note: Creating demo archive for extraction example")
            create_archive("extract_demo.zip", ZIP_FORMAT)
            add_file("test.txt", "test.txt")
            add_file("data.csv", "data/data.csv")
        }
    }
    
    # List files in archive
    sus files tea[value] = list_files() fam {
        when err -> {
            vibez.spill("Could not list files: " + err)
            damn
        }
    }
    
    vibez.spill("Files in archive:")
    bestie (drip i = 0; i < len(files); i = i + 1) {
        vibez.spill("  " + files[i])
    }
    
    # Check if specific file exists
    sus exists lit = file_exists("test.txt") fam {
        when err -> {
            vibez.spill("Could not check file existence: " + err)
            damn
        }
    }
    
    ready (exists) {
        vibez.spill("✓ test.txt exists in archive")
        
        # Extract the file
        sus data tea = extract_file("test.txt", "extracted_test.txt") fam {
            when err -> {
                vibez.spill("Extraction failed: " + err)
                damn
            }
        }
        
        vibez.spill("Extracted test.txt: " + data)
    }
    
    # Extract all files
    sus count drip = extract_all("output_dir") fam {
        when err -> {
            vibez.spill("Full extraction failed: " + err)
            damn
        }
    }
    
    vibez.spill("Extracted " + to_string(count) + " files to output_dir")
    
    close_archive()
    vibez.spill("Extraction demo completed")
}

# Example 3: Compression algorithm comparison
slay demo_compression_comparison() {
    vibez.spill("\n=== Compression Algorithm Comparison ===")
    
    # Initialize compression system
    init_compression()
    
    # Test data
    sus test_data tea = "This is test data for compression demonstration. "
    test_data = test_data + "It contains repeated text that should compress well with different algorithms. "
    test_data = test_data + "The quick brown fox jumps over the lazy dog. "
    test_data = test_data + "This pattern repeats to show compression effectiveness. "
    
    vibez.spill("Original data size: " + to_string(len(test_data)) + " bytes")
    vibez.spill("Testing different compression algorithms:")
    
    # Test DEFLATE
    set_compression_algorithm(COMPRESSION_DEFLATE) fam {
        when err -> vibez.spill("DEFLATE error: " + err)
    }
    
    sus deflate_compressed tea = compress_data(test_data) fam {
        when err -> vibez.spill("DEFLATE compression error: " + err)
    }
    
    vibez.spill("DEFLATE: " + to_string(len(test_data)) + " -> " + to_string(len(deflate_compressed)) + " bytes")
    
    # Test GZIP
    set_compression_algorithm(COMPRESSION_GZIP) fam {
        when err -> vibez.spill("GZIP error: " + err)
    }
    
    sus gzip_compressed tea = compress_data(test_data) fam {
        when err -> vibez.spill("GZIP compression error: " + err)
    }
    
    vibez.spill("GZIP: " + to_string(len(test_data)) + " -> " + to_string(len(gzip_compressed)) + " bytes")
    
    # Test LZ4 (fast compression)
    set_compression_algorithm(COMPRESSION_LZ4) fam {
        when err -> vibez.spill("LZ4 error: " + err)
    }
    
    sus lz4_compressed tea = compress_data(test_data) fam {
        when err -> vibez.spill("LZ4 compression error: " + err)
    }
    
    vibez.spill("LZ4: " + to_string(len(test_data)) + " -> " + to_string(len(lz4_compressed)) + " bytes")
    
    # Test BZIP2 (high compression)
    set_compression_algorithm(COMPRESSION_BZIP2) fam {
        when err -> vibez.spill("BZIP2 error: " + err)
    }
    
    sus bzip2_compressed tea = compress_data(test_data) fam {
        when err -> vibez.spill("BZIP2 compression error: " + err)
    }
    
    vibez.spill("BZIP2: " + to_string(len(test_data)) + " -> " + to_string(len(bzip2_compressed)) + " bytes")
    
    # Get overall statistics
    sus stats tea = get_compression_stats()
    vibez.spill("\nCompression Statistics:")
    vibez.spill(stats)
    
    vibez.spill("Compression comparison completed")
}

# Example 4: TAR archive with metadata
slay demo_tar_archive() {
    vibez.spill("\n=== TAR Archive with Metadata Demo ===")
    
    # Initialize TAR system
    init_tar()
    
    # Create TAR entries with full metadata
    tar_add_file("document.pdf", "documents/report.pdf", "PDF document content") fam {
        when err -> vibez.spill("TAR file add error: " + err)
    }
    
    tar_add_directory("photos", "images/photos") fam {
        when err -> vibez.spill("TAR directory add error: " + err)
    }
    
    # Add symbolic link
    tar_add_symlink("current_version.txt", "version-1.2.3.txt") fam {
        when err -> vibez.spill("TAR symlink add error: " + err)
    }
    
    # Add hard link
    tar_add_hardlink("backup_report.pdf", "documents/report.pdf") fam {
        when err -> vibez.spill("TAR hardlink add error: " + err)
    }
    
    # Set file permissions and ownership
    tar_set_file_mode("documents/report.pdf", "644") fam {
        when err -> vibez.spill("TAR mode set error: " + err)
    }
    
    tar_set_ownership("documents/report.pdf", "1000", "1000", "user", "group") fam {
        when err -> vibez.spill("TAR ownership set error: " + err)
    }
    
    # Get TAR statistics
    sus tar_stats tea = tar_get_stats()
    vibez.spill("TAR Archive Statistics:")
    vibez.spill(tar_stats)
    
    # Validate TAR archive
    sus tar_valid lit = tar_validate()
    ready (tar_valid) {
        vibez.spill("✓ TAR archive validation passed")
    } otherwise {
        vibez.spill("✗ TAR archive validation failed")
    }
    
    vibez.spill("TAR archive demo completed")
}

# Example 5: Format detection and error handling
slay demo_format_detection_and_errors() {
    vibez.spill("\n=== Format Detection and Error Handling Demo ===")
    
    # Test format detection
    sus formats tea[value] = ["archive.zip", "backup.tar", "data.gz", "compressed.bz2", "unknown.xyz"]
    
    vibez.spill("Testing format detection:")
    bestie (drip i = 0; i < len(formats); i = i + 1) {
        sus filename tea = formats[i]
        sus detected tea = detect_format(filename)
        
        ready (detected != "") {
            vibez.spill("  " + filename + " -> " + detected)
        } otherwise {
            vibez.spill("  " + filename + " -> unknown format")
        }
    }
    
    # Demonstrate error handling
    vibez.spill("\nTesting error handling:")
    
    # Try to create archive with empty name
    create_archive("", ZIP_FORMAT) fam {
        when err -> {
            vibez.spill("✓ Caught expected error: " + err)
        }
    }
    
    # Try invalid format
    create_archive("test.xyz", "invalid_format") fam {
        when err -> {
            vibez.spill("✓ Caught expected error: " + err)
        }
    }
    
    # Try operation without open archive
    close_archive()  # Ensure no archive is open
    add_file("test.txt", "test.txt") fam {
        when err -> {
            vibez.spill("✓ Caught expected error: " + err)
        }
    }
    
    # Try invalid compression level
    set_compression_level(15) fam {
        when err -> {
            vibez.spill("✓ Caught expected error: " + err)
        }
    }
    
    vibez.spill("Error handling demo completed")
}

# Main demonstration runner
slay run_archivez_demos() {
    vibez.spill("=== archivez Package Demonstration ===")
    vibez.spill("Comprehensive archive format support for CURSED")
    vibez.spill("Supported formats: ZIP, TAR, GZIP, BZIP2, LZ4, LZMA")
    
    # Run all demonstrations
    demo_zip_creation()
    demo_archive_extraction() 
    demo_compression_comparison()
    demo_tar_archive()
    demo_format_detection_and_errors()
    
    vibez.spill("\n=== Demonstration Complete ===")
    vibez.spill("The archivez package provides:")
    vibez.spill("• Multiple archive format support (ZIP, TAR, GZIP, BZIP2)")
    vibez.spill("• Advanced compression algorithms (DEFLATE, LZ4, LZMA)")
    vibez.spill("• Comprehensive error handling with structured errors")
    vibez.spill("• Password protection and archive validation")
    vibez.spill("• POSIX-compliant TAR with full metadata support")
    vibez.spill("• Performance optimization for different use cases")
    vibez.spill("")
    vibez.spill("Ready for production use in CURSED applications!")
}

# Run the demonstrations
run_archivez_demos()
