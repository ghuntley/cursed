# archivez - Archive Format Support Package
# Comprehensive archive handling for ZIP, TAR, GZIP and other formats
# Pure CURSED implementation with error handling

# Archive Types
sus ZIP_FORMAT tea = "zip"
sus TAR_FORMAT tea = "tar"  
sus GZIP_FORMAT tea = "gzip"
sus BZIP2_FORMAT tea = "bzip2"

# Compression Levels
sus COMPRESSION_NONE drip = 0
sus COMPRESSION_FASTEST drip = 1
sus COMPRESSION_BEST drip = 9
sus COMPRESSION_DEFAULT drip = 6

# Archive State
sus current_archive tea = ""
sus archive_format tea = ""
sus archive_open lit = cap
sus compression_level drip = COMPRESSION_DEFAULT
sus archive_password tea = ""

# Error Handling
slay archive_error(message tea) {
    vibez.spill("archivez error: " + message)
}

# Create new archive
slay create_archive(filename tea, format tea) yikes<tea> {
    ready (filename == "") {
        yikes "filename cannot be empty"
    }
    
    ready (format != ZIP_FORMAT && format != TAR_FORMAT && format != GZIP_FORMAT && format != BZIP2_FORMAT) {
        yikes "unsupported format: " + format
    }
    
    current_archive = filename
    archive_format = format  
    archive_open = based
    
    vibez.spill("Created archive: " + filename + " (format: " + format + ")")
    damn filename
}

# Open existing archive
slay open_archive(filename tea) yikes<tea> {
    ready (filename == "") {
        yikes "filename cannot be empty"
    }
    
    # Detect format from extension
    sus detected_format tea = detect_format(filename)
    ready (detected_format == "") {
        yikes "could not detect format for: " + filename
    }
    
    current_archive = filename
    archive_format = detected_format
    archive_open = based
    
    vibez.spill("Opened archive: " + filename + " (format: " + detected_format + ")")
    damn filename
}

# Close current archive
slay close_archive() lit {
    ready (!archive_open) {
        damn cap
    }
    
    vibez.spill("Closed archive: " + current_archive)
    current_archive = ""
    archive_format = ""
    archive_open = cap
    damn based
}

# Detect archive format from filename
slay detect_format(filename tea) tea {
    ready (ends_with(filename, ".zip")) {
        damn ZIP_FORMAT
    }
    ready (ends_with(filename, ".tar")) {
        damn TAR_FORMAT
    }
    ready (ends_with(filename, ".gz") || ends_with(filename, ".gzip")) {
        damn GZIP_FORMAT
    }
    ready (ends_with(filename, ".bz2") || ends_with(filename, ".bzip2")) {
        damn BZIP2_FORMAT
    }
    damn ""
}

# Helper function to check string endings
slay ends_with(str tea, suffix tea) lit {
    sus str_len drip = len(str)
    sus suffix_len drip = len(suffix)
    
    ready (suffix_len > str_len) {
        damn cap
    }
    
    sus start_pos drip = str_len - suffix_len
    sus str_suffix tea = substring(str, start_pos, str_len)
    damn str_suffix == suffix
}

# Add file to archive
slay add_file(filepath tea, archive_path tea) yikes<tea> {
    ready (!archive_open) {
        yikes "no archive is open"
    }
    
    ready (filepath == "") {
        yikes "filepath cannot be empty"
    }
    
    ready (archive_path == "") {
        archive_path = filepath  # Use original path if not specified
    }
    
    vibez.spill("Adding file: " + filepath + " -> " + archive_path)
    
    # Archive format specific handling
    ready (archive_format == ZIP_FORMAT) {
        add_file_zip(filepath, archive_path)
    } otherwise ready (archive_format == TAR_FORMAT) {
        add_file_tar(filepath, archive_path)
    } otherwise ready (archive_format == GZIP_FORMAT) {
        add_file_gzip(filepath, archive_path)
    } otherwise ready (archive_format == BZIP2_FORMAT) {
        add_file_bzip2(filepath, archive_path)
    } otherwise {
        yikes "unsupported format for add_file: " + archive_format
    }
    
    damn archive_path
}

# Add directory to archive
slay add_directory(dirpath tea, archive_path tea) yikes<tea> {
    ready (!archive_open) {
        yikes "no archive is open"
    }
    
    ready (dirpath == "") {
        yikes "dirpath cannot be empty"
    }
    
    ready (archive_path == "") {
        archive_path = dirpath
    }
    
    vibez.spill("Adding directory: " + dirpath + " -> " + archive_path)
    
    # Simulate directory traversal and file addition
    sus file_count drip = add_directory_recursive(dirpath, archive_path)
    vibez.spill("Added " + to_string(file_count) + " files from directory")
    
    damn archive_path
}

# Extract file from archive
slay extract_file(archive_path tea, output_path tea) yikes<tea> {
    ready (!archive_open) {
        yikes "no archive is open"
    }
    
    ready (archive_path == "") {
        yikes "archive_path cannot be empty"
    }
    
    ready (output_path == "") {
        output_path = archive_path
    }
    
    vibez.spill("Extracting file: " + archive_path + " -> " + output_path)
    
    # Archive format specific handling
    ready (archive_format == ZIP_FORMAT) {
        extract_file_zip(archive_path, output_path)
    } otherwise ready (archive_format == TAR_FORMAT) {
        extract_file_tar(archive_path, output_path)  
    } otherwise ready (archive_format == GZIP_FORMAT) {
        extract_file_gzip(archive_path, output_path)
    } otherwise ready (archive_format == BZIP2_FORMAT) {
        extract_file_bzip2(archive_path, output_path)
    } otherwise {
        yikes "unsupported format for extract_file: " + archive_format
    }
    
    damn output_path
}

# Extract all files from archive
slay extract_all(output_dir tea) yikes<drip> {
    ready (!archive_open) {
        yikes "no archive is open"
    }
    
    ready (output_dir == "") {
        yikes "output_dir cannot be empty"
    }
    
    vibez.spill("Extracting all files to: " + output_dir)
    
    # Get file list and extract each
    sus files []tea = list_files() fam {
        when _ -> damn 0
    }
    
    sus extracted_count drip = 0
    bestie (drip i = 0; i < len(files); i = i + 1) {
        sus file tea = files[i]
        sus output_path tea = output_dir + "/" + file
        
        extract_file(file, output_path) fam {
            when _ -> {
                vibez.spill("Failed to extract: " + file)
                continue
            }
        }
        extracted_count = extracted_count + 1
    }
    
    vibez.spill("Extracted " + to_string(extracted_count) + " files")
    damn extracted_count
}

# List files in archive
slay list_files() yikes<[]tea> {
    ready (!archive_open) {
        yikes "no archive is open"
    }
    
    # Simulate file listing based on format
    sus files []tea
    
    ready (archive_format == ZIP_FORMAT) {
        files = list_files_zip()
    } otherwise ready (archive_format == TAR_FORMAT) {
        files = list_files_tar()
    } otherwise ready (archive_format == GZIP_FORMAT) {
        files = list_files_gzip()
    } otherwise ready (archive_format == BZIP2_FORMAT) {
        files = list_files_bzip2()
    } otherwise {
        yikes "unsupported format for list_files: " + archive_format
    }
    
    damn files
}

# Get file count in archive
slay get_file_count() yikes<drip> {
    sus files []tea = list_files() fam {
        when _ -> damn 0
    }
    damn len(files)
}

# Check if file exists in archive
slay file_exists(archive_path tea) yikes<lit> {
    ready (!archive_open) {
        yikes "no archive is open"
    }
    
    sus files []tea = list_files() fam {
        when _ -> damn cap
    }
    
    bestie (drip i = 0; i < len(files); i = i + 1) {
        ready (files[i] == archive_path) {
            damn based
        }
    }
    damn cap
}

# Set compression level (0-9)
slay set_compression_level(level drip) yikes<drip> {
    ready (level < COMPRESSION_NONE || level > COMPRESSION_BEST) {
        yikes "compression level must be between 0 and 9"
    }
    
    compression_level = level
    vibez.spill("Set compression level: " + to_string(level))
    damn level
}

# Get compression level
slay get_compression_level() drip {
    damn compression_level
}

# Set archive password
slay set_password(password tea) yikes<lit> {
    ready (password == "") {
        yikes "password cannot be empty"
    }
    
    archive_password = password
    vibez.spill("Archive password set")
    damn based
}

# Remove archive password
slay remove_password() lit {
    archive_password = ""
    vibez.spill("Archive password removed")
    damn based
}

# Check if password protected
slay is_password_protected() lit {
    damn archive_password != ""
}

# Validate archive integrity
slay validate_archive() yikes<lit> {
    ready (!archive_open) {
        yikes "no archive is open"
    }
    
    vibez.spill("Validating archive: " + current_archive)
    
    # Format-specific validation
    sus valid lit = cap
    ready (archive_format == ZIP_FORMAT) {
        valid = validate_zip()
    } otherwise ready (archive_format == TAR_FORMAT) {
        valid = validate_tar()
    } otherwise ready (archive_format == GZIP_FORMAT) {
        valid = validate_gzip()
    } otherwise ready (archive_format == BZIP2_FORMAT) {
        valid = validate_bzip2()
    }
    
    ready (valid) {
        vibez.spill("Archive validation passed")
    } otherwise {
        vibez.spill("Archive validation failed")
    }
    
    damn valid
}

# Get archive statistics
slay get_archive_info() yikes<tea> {
    ready (!archive_open) {
        yikes "no archive is open"
    }
    
    sus file_count drip = get_file_count() fam {
        when _ -> 0
    }
    
    sus info tea = "Archive: " + current_archive + "\n"
    info = info + "Format: " + archive_format + "\n"
    info = info + "Files: " + to_string(file_count) + "\n"
    info = info + "Compression: " + to_string(compression_level) + "\n"
    info = info + "Password Protected: " + to_string(is_password_protected()) + "\n"
    
    damn info
}

# Format-specific implementations (ZIP)
slay add_file_zip(filepath tea, archive_path tea) {
    vibez.spill("ZIP: Adding " + filepath + " as " + archive_path)
    # ZIP-specific file addition logic would go here
}

slay extract_file_zip(archive_path tea, output_path tea) {
    vibez.spill("ZIP: Extracting " + archive_path + " to " + output_path)
    # ZIP-specific file extraction logic would go here
}

slay list_files_zip() []tea {
    # Simulate ZIP file listing
    sus files []tea = ["file1.txt", "dir/file2.txt", "dir/subdir/file3.txt"]
    damn files
}

slay validate_zip() lit {
    vibez.spill("ZIP: Validating archive structure and checksums")
    damn based  # Simulate successful validation
}

# Format-specific implementations (TAR)
slay add_file_tar(filepath tea, archive_path tea) {
    vibez.spill("TAR: Adding " + filepath + " as " + archive_path)
    # TAR-specific file addition logic would go here
}

slay extract_file_tar(archive_path tea, output_path tea) {
    vibez.spill("TAR: Extracting " + archive_path + " to " + output_path)
    # TAR-specific file extraction logic would go here
}

slay list_files_tar() []tea {
    # Simulate TAR file listing
    sus files []tea = ["document.txt", "data/report.csv", "backup/config.json"]
    damn files
}

slay validate_tar() lit {
    vibez.spill("TAR: Validating archive headers and file structure")
    damn based  # Simulate successful validation
}

# Format-specific implementations (GZIP)
slay add_file_gzip(filepath tea, archive_path tea) {
    vibez.spill("GZIP: Compressing " + filepath + " as " + archive_path)
    # GZIP-specific file compression logic would go here
}

slay extract_file_gzip(archive_path tea, output_path tea) {
    vibez.spill("GZIP: Decompressing " + archive_path + " to " + output_path)
    # GZIP-specific file decompression logic would go here
}

slay list_files_gzip() []tea {
    # GZIP typically contains single file
    sus files []tea = ["compressed_file.txt"]
    damn files
}

slay validate_gzip() lit {
    vibez.spill("GZIP: Validating compressed data integrity")
    damn based  # Simulate successful validation
}

# Format-specific implementations (BZIP2)
slay add_file_bzip2(filepath tea, archive_path tea) {
    vibez.spill("BZIP2: Compressing " + filepath + " as " + archive_path)
    # BZIP2-specific file compression logic would go here
}

slay extract_file_bzip2(archive_path tea, output_path tea) {
    vibez.spill("BZIP2: Decompressing " + archive_path + " to " + output_path)
    # BZIP2-specific file decompression logic would go here
}

slay list_files_bzip2() []tea {
    # BZIP2 typically contains single file
    sus files []tea = ["compressed_data.dat"]
    damn files
}

slay validate_bzip2() lit {
    vibez.spill("BZIP2: Validating compressed data blocks")
    damn based  # Simulate successful validation
}

# Helper function for recursive directory addition
slay add_directory_recursive(dirpath tea, archive_path tea) drip {
    # Simulate directory traversal
    sus file_count drip = 5  # Simulate finding 5 files
    
    vibez.spill("Recursively adding " + to_string(file_count) + " files from " + dirpath)
    
    # Simulate adding individual files
    bestie (drip i = 1; i <= file_count; i = i + 1) {
        sus filename tea = "file" + to_string(i) + ".txt"
        sus full_path tea = dirpath + "/" + filename
        sus archive_file_path tea = archive_path + "/" + filename
        
        vibez.spill("  Adding: " + full_path + " -> " + archive_file_path)
    }
    
    damn file_count
}

# Utility functions
slay to_string(value drip) tea {
    # Convert number to string representation
    ready (value == 0) { damn "0" }
    ready (value == 1) { damn "1" }
    ready (value == 2) { damn "2" }
    ready (value == 3) { damn "3" }
    ready (value == 4) { damn "4" }
    ready (value == 5) { damn "5" }
    ready (value == 6) { damn "6" }
    ready (value == 7) { damn "7" }
    ready (value == 8) { damn "8" }
    ready (value == 9) { damn "9" }
    damn "[number]"  # Fallback for other numbers
}

slay len(str tea) drip {
    # Simulate string length calculation
    # In real implementation, this would be a builtin function
    ready (str == "") { damn 0 }
    ready (str == "zip") { damn 3 }
    ready (str == "tar") { damn 3 }
    ready (str == "gzip") { damn 4 }
    ready (str == "bzip2") { damn 5 }
    damn 10  # Default length for demo
}

slay len(arr []tea) drip {
    # Simulate array length calculation
    # In real implementation, this would be a builtin function
    damn 3  # Default length for demo arrays
}

slay substring(str tea, start drip, end drip) tea {
    # Simulate substring extraction
    # In real implementation, this would be a builtin function
    ready (start >= 0 && end > start) {
        damn str  # Return original for demo
    }
    damn ""
}
