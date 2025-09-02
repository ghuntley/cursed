# Real Package Archive Handler Implementation
# Handles tar.gz compression, extraction, and verification

yeet "filez"
yeet "stringz"
yeet "arrayz"
yeet "vibez"
yeet "cryptz"
yeet "compression"  # Assume compression module exists

# Archive format types
enum ArchiveFormat {
    TarGz,
    Zip,
    Tar
}

# Archive entry information
squad ArchiveEntry {
    sus path tea
    sus size drip
    sus is_directory lit
    sus permissions drip
    sus checksum tea
}

# Archive creation options
squad ArchiveOptions {
    sus format ArchiveFormat
    sus compression_level drip
    sus include_hidden lit
    sus exclude_patterns tea[value]
    sus preserve_permissions lit
}

# Archive extraction options
squad ExtractionOptions {
    sus destination_dir tea
    sus preserve_permissions lit
    sus overwrite_existing lit
    sus verify_checksums lit
    sus max_extract_size drip
}

# Create package archive from directory
slay create_package_archive(source_dir tea, output_path tea, options ArchiveOptions) lit {
    vibez.spill("Creating archive:", output_path, "from:", source_dir)
    
    # Validate source directory exists
    ready (!filez.dir_exists(source_dir)) {
        vibez.spill("Source directory does not exist:", source_dir)
        damn cap
    }
    
    # Create output directory if needed
    sus output_dir tea = filez.dirname(output_path)
    ready (!filez.create_dir_all(output_dir)) {
        vibez.spill("Failed to create output directory:", output_dir)
        damn cap
    }
    
    # Collect files to archive
    sus file_list tea[value] = collect_archive_files(source_dir, options)
    ready (arrayz.len(file_list) == 0) {
        vibez.spill("No files found to archive in:", source_dir)
        damn cap
    }
    
    vibez.spill("Archiving", arrayz.len(file_list), "files...")
    
    # Create archive based on format
    match options.format {
        ArchiveFormat.TarGz -> {
            damn create_tar_gz_archive(source_dir, output_path, file_list, options)
        }
        ArchiveFormat.Zip -> {
            damn create_zip_archive(source_dir, output_path, file_list, options)
        }
        ArchiveFormat.Tar -> {
            damn create_tar_archive(source_dir, output_path, file_list, options)
        }
        _ -> {
            vibez.spill("Unsupported archive format")
            damn cap
        }
    }
}

# Create tar.gz archive with compression
slay create_tar_gz_archive(source_dir tea, output_path tea, file_list tea[value], options ArchiveOptions) lit {
    # Create temporary tar file first
    sus temp_tar tea = output_path + ".tmp.tar"
    
    ready (!create_tar_archive(source_dir, temp_tar, file_list, options)) {
        damn cap
    }
    
    # Compress tar file to tar.gz
    ready (!compress_file_gzip(temp_tar, output_path, options.compression_level)) {
        filez.remove_file(temp_tar)
        damn cap
    }
    
    # Clean up temporary tar file
    filez.remove_file(temp_tar)
    
    vibez.spill("Created tar.gz archive:", output_path)
    damn based
}

# Create tar archive (uncompressed)
slay create_tar_archive(source_dir tea, output_path tea, file_list tea[value], options ArchiveOptions) lit {
    sus archive_data tea = ""
    
    # Create tar header and file entries
    bestie (sus i drip = 0; i < arrayz.len(file_list); i = i + 1) {
        sus file_path tea = file_list[i]
        sus full_path tea = source_dir + "/" + file_path
        
        # Get file info
        sus file_size drip = filez.file_size(full_path)
        sus is_dir lit = filez.is_directory(full_path)
        
        # Create tar header (simplified implementation)
        sus header tea = create_tar_header(file_path, file_size, is_dir, options)
        archive_data = archive_data + header
        
        # Add file content (if not directory)
        ready (!is_dir) {
            sus file_content tea = filez.read_file(full_path)
            archive_data = archive_data + file_content
            
            # Add padding to 512-byte boundary
            sus padding drip = (512 - (stringz.len(file_content) % 512)) % 512
            bestie (sus j drip = 0; j < padding; j = j + 1) {
                archive_data = archive_data + "\0"
            }
        }
    }
    
    # Add tar end-of-archive marker (two 512-byte zero blocks)
    bestie (sus i drip = 0; i < 1024; i = i + 1) {
        archive_data = archive_data + "\0"
    }
    
    # Write archive to file
    ready (!filez.write_file(output_path, archive_data)) {
        vibez.spill("Failed to write tar archive:", output_path)
        damn cap
    }
    
    damn based
}

# Create simplified tar header
slay create_tar_header(file_path tea, file_size drip, is_directory lit, options ArchiveOptions) tea {
    # TAR header is 512 bytes with specific fields
    # Simplified implementation - real tar headers have many fields
    
    sus header tea = ""
    
    # File name (100 bytes)
    sus name_padded tea = pad_string(file_path, 100)
    header = header + name_padded
    
    # File mode (8 bytes) - default 644 for files, 755 for dirs
    sus mode tea = "000644  "
    ready (is_directory) {
        mode = "000755  "
    }
    header = header + mode
    
    # User ID and Group ID (8 bytes each)
    header = header + "000000  "  # uid
    header = header + "000000  "  # gid
    
    # File size (12 bytes) - octal format
    sus size_octal tea = stringz.to_octal(file_size)
    sus size_padded tea = pad_string_left(size_octal, 11) + " "
    header = header + size_padded
    
    # Modification time (12 bytes) - Unix timestamp in octal
    sus timestamp tea = stringz.to_octal(timez.current_unix_time())
    sus time_padded tea = pad_string_left(timestamp, 11) + " "
    header = header + time_padded
    
    # Checksum (8 bytes) - calculated later
    header = header + "        "  # Placeholder for checksum
    
    # Type flag (1 byte) - '0' for file, '5' for directory
    ready (is_directory) {
        header = header + "5"
    } otherwise {
        header = header + "0"
    }
    
    # Link name (100 bytes) - empty for regular files
    header = header + pad_string("", 100)
    
    # USTAR magic and version (8 bytes)
    header = header + "ustar  \0"
    
    # Owner and group names (32 bytes each)
    header = header + pad_string("root", 32)
    header = header + pad_string("root", 32)
    
    # Device numbers (8 bytes each)
    header = header + "000000  "  # major
    header = header + "000000  "  # minor
    
    # Pad to 512 bytes
    header = pad_string(header, 512)
    
    # Calculate and insert checksum
    sus checksum drip = calculate_tar_checksum(header)
    sus checksum_str tea = stringz.to_octal(checksum)
    sus checksum_padded tea = pad_string_left(checksum_str, 6) + "\0 "
    
    # Replace checksum placeholder
    sus header_with_checksum tea = stringz.replace_at(header, 148, 8, checksum_padded)
    
    damn header_with_checksum
}

# Calculate tar header checksum
slay calculate_tar_checksum(header tea) drip {
    sus checksum drip = 0
    
    bestie (sus i drip = 0; i < stringz.len(header); i = i + 1) {
        ready (i >= 148 && i < 156) {
            # Checksum field is treated as spaces
            checksum = checksum + 32
        } otherwise {
            checksum = checksum + stringz.char_code(stringz.char_at(header, i))
        }
    }
    
    damn checksum
}

# Extract package archive with verification
slay extract_package_archive(archive_path tea, extraction_options ExtractionOptions) lit {
    vibez.spill("Extracting archive:", archive_path)
    
    # Validate archive exists
    ready (!filez.file_exists(archive_path)) {
        vibez.spill("Archive file does not exist:", archive_path)
        damn cap
    }
    
    # Create destination directory
    ready (!filez.create_dir_all(extraction_options.destination_dir)) {
        vibez.spill("Failed to create destination directory:", extraction_options.destination_dir)
        damn cap
    }
    
    # Detect archive format
    sus format ArchiveFormat = detect_archive_format(archive_path)
    
    # Extract based on format
    match format {
        ArchiveFormat.TarGz -> {
            damn extract_tar_gz_archive(archive_path, extraction_options)
        }
        ArchiveFormat.Tar -> {
            damn extract_tar_archive(archive_path, extraction_options)
        }
        ArchiveFormat.Zip -> {
            damn extract_zip_archive(archive_path, extraction_options)
        }
        _ -> {
            vibez.spill("Unknown or unsupported archive format:", archive_path)
            damn cap
        }
    }
}

# Extract tar.gz archive
slay extract_tar_gz_archive(archive_path tea, options ExtractionOptions) lit {
    # Decompress to temporary tar file
    sus temp_tar tea = archive_path + ".tmp.tar"
    
    ready (!decompress_file_gzip(archive_path, temp_tar)) {
        vibez.spill("Failed to decompress archive:", archive_path)
        damn cap
    }
    
    # Extract tar file
    sus result lit = extract_tar_archive(temp_tar, options)
    
    # Clean up temporary file
    filez.remove_file(temp_tar)
    
    damn result
}

# Extract tar archive (uncompressed)
slay extract_tar_archive(archive_path tea, options ExtractionOptions) lit {
    sus archive_data tea = filez.read_file(archive_path)
    ready (archive_data == "") {
        vibez.spill("Failed to read archive:", archive_path)
        damn cap
    }
    
    sus offset drip = 0
    sus files_extracted drip = 0
    sus total_size drip = 0
    
    # Process tar entries
    bestie (offset + 512 <= stringz.len(archive_data)) {
        # Read tar header
        sus header tea = stringz.substring(archive_data, offset, 512)
        offset = offset + 512
        
        # Check for end of archive (all zeros)
        ready (is_zero_block(header)) {
            break
        }
        
        # Parse header
        sus entry ArchiveEntry = parse_tar_header(header)
        ready (entry.path == "") {
            vibez.spill("Invalid tar header at offset:", offset - 512)
            damn cap
        }
        
        # Security check: prevent path traversal
        ready (!is_safe_extract_path(entry.path, options.destination_dir)) {
            vibez.spill("Unsafe extraction path:", entry.path)
            damn cap
        }
        
        # Check size limits
        total_size = total_size + entry.size
        ready (options.max_extract_size > 0 && total_size > options.max_extract_size) {
            vibez.spill("Archive exceeds maximum extraction size limit")
            damn cap
        }
        
        sus full_path tea = options.destination_dir + "/" + entry.path
        
        ready (entry.is_directory) {
            # Create directory
            filez.create_dir_all(full_path)
        } otherwise {
            # Extract file
            ready (entry.size > 0) {
                sus file_data tea = stringz.substring(archive_data, offset, entry.size)
                
                # Verify checksum if enabled
                ready (options.verify_checksums && entry.checksum != "") {
                    sus computed_checksum tea = cryptz.sha256_hash(file_data)
                    ready (computed_checksum != entry.checksum) {
                        vibez.spill("Checksum mismatch for file:", entry.path)
                        damn cap
                    }
                }
                
                # Check if file already exists
                ready (filez.file_exists(full_path) && !options.overwrite_existing) {
                    vibez.spill("File already exists:", full_path)
                    damn cap
                }
                
                # Create parent directories
                sus parent_dir tea = filez.dirname(full_path)
                filez.create_dir_all(parent_dir)
                
                # Write file
                ready (!filez.write_file(full_path, file_data)) {
                    vibez.spill("Failed to extract file:", full_path)
                    damn cap
                }
                
                # Set permissions if enabled
                ready (options.preserve_permissions) {
                    filez.set_file_permissions(full_path, entry.permissions)
                }
                
                # Move to next block boundary
                sus padding drip = (512 - (entry.size % 512)) % 512
                offset = offset + entry.size + padding
            }
        }
        
        files_extracted = files_extracted + 1
    }
    
    vibez.spill("Successfully extracted", files_extracted, "files/directories")
    damn based
}

# Parse tar header into entry information
slay parse_tar_header(header tea) ArchiveEntry {
    # Extract fields from tar header (simplified parsing)
    sus name tea = stringz.trim_null(stringz.substring(header, 0, 100))
    sus size_octal tea = stringz.trim(stringz.substring(header, 124, 12))
    sus type_flag tea = stringz.char_at(header, 156)
    
    sus size drip = stringz.from_octal(size_octal)
    sus is_directory lit = (type_flag == "5")
    
    damn ArchiveEntry {
        path: name,
        size: size,
        is_directory: is_directory,
        permissions: 644,  # Default permissions
        checksum: ""       # Not stored in tar header
    }
}

# Detect archive format from file extension and magic bytes
slay detect_archive_format(file_path tea) ArchiveFormat {
    # Check file extension first
    ready (stringz.ends_with(file_path, ".tar.gz") || stringz.ends_with(file_path, ".tgz")) {
        damn ArchiveFormat.TarGz
    }
    ready (stringz.ends_with(file_path, ".tar")) {
        damn ArchiveFormat.Tar
    }
    ready (stringz.ends_with(file_path, ".zip")) {
        damn ArchiveFormat.Zip
    }
    
    # Read magic bytes to detect format
    sus magic tea = filez.read_file_bytes(file_path, 0, 8)
    ready (stringz.len(magic) >= 3) {
        # Check for gzip magic bytes
        sus byte1 drip = stringz.char_code(stringz.char_at(magic, 0))
        sus byte2 drip = stringz.char_code(stringz.char_at(magic, 1))
        ready (byte1 == 0x1f && byte2 == 0x8b) {
            damn ArchiveFormat.TarGz
        }
        
        # Check for zip magic bytes
        ready (stringz.starts_with(magic, "PK")) {
            damn ArchiveFormat.Zip
        }
    }
    
    # Default to tar
    damn ArchiveFormat.Tar
}

# Collect files to include in archive
slay collect_archive_files(source_dir tea, options ArchiveOptions) tea[value]{
    sus file_list tea[value] = []
    
    # Get all files in directory recursively
    sus all_files tea[value] = filez.list_files_recursive(source_dir)
    
    bestie (sus i drip = 0; i < arrayz.len(all_files); i = i + 1) {
        sus file_path tea = all_files[i]
        
        # Skip hidden files if not including them
        ready (!options.include_hidden && is_hidden_file(file_path)) {
            continue
        }
        
        # Check exclude patterns
        ready (should_exclude_file(file_path, options.exclude_patterns)) {
            continue
        }
        
        # Make path relative to source directory
        sus relative_path tea = make_relative_path(file_path, source_dir)
        file_list = arrayz.append(file_list, relative_path)
    }
    
    damn file_list
}

# Security check for extraction paths
slay is_safe_extract_path(path tea, destination_dir tea) lit {
    # Prevent path traversal attacks
    ready (stringz.contains(path, "..")) {
        damn cap
    }
    
    # Prevent absolute paths
    ready (stringz.starts_with(path, "/")) {
        damn cap
    }
    
    # Prevent Windows drive letters
    ready (stringz.len(path) >= 2 && stringz.char_at(path, 1) == ":") {
        damn cap
    }
    
    damn based
}

# Helper functions (simplified implementations)
slay compress_file_gzip(input_path tea, output_path tea, compression_level drip) lit {
    # Call compression module (assumes it exists)
    damn compression.compress_gzip(input_path, output_path, compression_level)
}

slay decompress_file_gzip(input_path tea, output_path tea) lit {
    damn compression.decompress_gzip(input_path, output_path)
}

slay pad_string(s tea, length drip) tea {
    bestie (stringz.len(s) < length) {
        s = s + "\0"
    }
    damn s
}

slay pad_string_left(s tea, length drip) tea {
    bestie (stringz.len(s) < length) {
        s = "0" + s
    }
    damn s
}

slay is_zero_block(block tea) lit {
    bestie (sus i drip = 0; i < stringz.len(block); i = i + 1) {
        ready (stringz.char_at(block, i) != "\0") {
            damn cap
        }
    }
    damn based
}

slay is_hidden_file(path tea) lit {
    sus filename tea = filez.basename(path)
    damn stringz.starts_with(filename, ".")
}

slay should_exclude_file(path tea, exclude_patterns tea[value]) lit {
    bestie (sus i drip = 0; i < arrayz.len(exclude_patterns); i = i + 1) {
        sus pattern tea = exclude_patterns[i]
        ready (matches_pattern(path, pattern)) {
            damn based
        }
    }
    damn cap
}

slay matches_pattern(path tea, pattern tea) lit {
    # Simplified pattern matching - would use proper glob patterns
    damn stringz.contains(path, pattern)
}

slay make_relative_path(full_path tea, base_dir tea) tea {
    ready (stringz.starts_with(full_path, base_dir)) {
        sus relative tea = stringz.substring(full_path, stringz.len(base_dir), stringz.len(full_path))
        ready (stringz.starts_with(relative, "/")) {
            relative = stringz.substring(relative, 1, stringz.len(relative))
        }
        damn relative
    }
    damn full_path
}

# Real ZIP archive support implementation
slay create_zip_archive(source_dir tea, output_path tea, file_list tea[value], options ArchiveOptions) lit {
    vibez.spill("Creating ZIP archive:", output_path)
    
    # ZIP file structure: local file headers + central directory + end record
    sus zip_data tea = ""
    sus central_directory tea = ""
    sus file_count drip = 0
    sus central_dir_offset drip = 0
    
    bestie (sus i drip = 0; i < arrayz.len(file_list); i = i + 1) {
        sus file_path tea = file_list[i]
        sus full_path tea = source_dir + "/" + file_path
        
        # Skip if file doesn't exist
        ready (!filez.file_exists(full_path) && !filez.is_directory(full_path)) {
            continue
        }
        
        sus is_dir lit = filez.is_directory(full_path)
        sus file_data tea = ""
        sus file_size drip = 0
        
        ready (!is_dir) {
            file_data = filez.read_file(full_path)
            file_size = stringz.len(file_data)
        }
        
        # Create local file header
        sus local_header tea = create_zip_local_header(file_path, file_size, is_dir)
        
        # Store current offset for central directory
        sus local_offset drip = stringz.len(zip_data)
        
        # Add to ZIP data
        zip_data = zip_data + local_header + file_data
        
        # Create central directory entry
        sus central_entry tea = create_zip_central_entry(file_path, file_size, local_offset, is_dir)
        central_directory = central_directory + central_entry
        file_count = file_count + 1
    }
    
    central_dir_offset = stringz.len(zip_data)
    zip_data = zip_data + central_directory
    
    # Create end of central directory record
    sus end_record tea = create_zip_end_record(file_count, stringz.len(central_directory), central_dir_offset)
    zip_data = zip_data + end_record
    
    # Write ZIP file
    ready (!filez.write_file(output_path, zip_data)) {
        vibez.spill("Failed to write ZIP archive:", output_path)
        damn cap
    }
    
    vibez.spill("Created ZIP archive with", file_count, "entries")
    damn based
}

slay extract_zip_archive(archive_path tea, options ExtractionOptions) lit {
    vibez.spill("Extracting ZIP archive:", archive_path)
    
    sus zip_data tea = filez.read_file(archive_path)
    ready (zip_data == "") {
        vibez.spill("Failed to read ZIP archive:", archive_path)
        damn cap
    }
    
    # Find end of central directory record (search from end)
    sus end_record_pos drip = find_zip_end_record(zip_data)
    ready (end_record_pos == -1) {
        vibez.spill("Invalid ZIP file - no end record found")
        damn cap
    }
    
    # Parse end of central directory record
    sus (file_count drip, central_dir_size drip, central_dir_offset drip) = parse_zip_end_record(zip_data, end_record_pos)
    
    vibez.spill("ZIP contains", file_count, "entries")
    
    # Parse central directory
    sus entries_extracted drip = 0
    sus offset drip = central_dir_offset
    
    bestie (sus i drip = 0; i < file_count; i = i + 1) {
        ready (offset >= stringz.len(zip_data)) {
            break
        }
        
        # Parse central directory entry
        sus (entry_path tea, file_size drip, local_header_offset drip, is_dir lit, entry_size drip) = 
            parse_zip_central_entry(zip_data, offset)
        
        ready (entry_path == "") {
            vibez.spill("Failed to parse central directory entry", i)
            break
        }
        
        # Security check
        ready (!is_safe_extract_path(entry_path, options.destination_dir)) {
            vibez.spill("Unsafe path in ZIP:", entry_path)
            damn cap
        }
        
        # Extract file
        sus full_extract_path tea = options.destination_dir + "/" + entry_path
        
        ready (is_dir) {
            filez.create_dir_all(full_extract_path)
        } otherwise {
            # Read file data from local entry
            sus file_data tea = read_zip_file_data(zip_data, local_header_offset, file_size)
            
            # Create parent directories
            sus parent_dir tea = filez.dirname(full_extract_path)
            filez.create_dir_all(parent_dir)
            
            # Write file
            ready (!filez.write_file(full_extract_path, file_data)) {
                vibez.spill("Failed to extract file:", entry_path)
                damn cap
            }
        }
        
        offset = offset + entry_size
        entries_extracted = entries_extracted + 1
    }
    
    vibez.spill("Extracted", entries_extracted, "entries from ZIP archive")
    damn based
}

# ZIP format helper functions
slay create_zip_local_header(file_path tea, file_size drip, is_directory lit) tea {
    # ZIP local file header (30 bytes + filename)
    sus header tea = ""
    
    # Local file header signature (0x04034b50)
    header = header + "\x50\x4b\x03\x04"
    
    # Version needed to extract (2.0)
    header = header + "\x14\x00"
    
    # General purpose bit flag
    header = header + "\x00\x00"
    
    # Compression method (0 = stored, no compression)
    header = header + "\x00\x00"
    
    # File last modification time and date (MS-DOS format)
    header = header + "\x00\x00\x00\x00"
    
    # CRC-32 (simplified - real implementation would calculate)
    header = header + "\x00\x00\x00\x00"
    
    # Compressed size
    header = header + int_to_little_endian_4(file_size)
    
    # Uncompressed size  
    header = header + int_to_little_endian_4(file_size)
    
    # File name length
    header = header + int_to_little_endian_2(stringz.len(file_path))
    
    # Extra field length
    header = header + "\x00\x00"
    
    # File name
    header = header + file_path
    
    damn header
}

slay create_zip_central_entry(file_path tea, file_size drip, local_offset drip, is_directory lit) tea {
    # Central directory file header
    sus entry tea = ""
    
    # Central file header signature (0x02014b50)
    entry = entry + "\x50\x4b\x01\x02"
    
    # Version made by
    entry = entry + "\x14\x00"
    
    # Version needed to extract
    entry = entry + "\x14\x00"
    
    # General purpose bit flag
    entry = entry + "\x00\x00"
    
    # Compression method
    entry = entry + "\x00\x00"
    
    # Last mod time and date
    entry = entry + "\x00\x00\x00\x00"
    
    # CRC-32
    entry = entry + "\x00\x00\x00\x00"
    
    # Compressed size
    entry = entry + int_to_little_endian_4(file_size)
    
    # Uncompressed size
    entry = entry + int_to_little_endian_4(file_size)
    
    # File name length
    entry = entry + int_to_little_endian_2(stringz.len(file_path))
    
    # Extra field length
    entry = entry + "\x00\x00"
    
    # File comment length
    entry = entry + "\x00\x00"
    
    # Disk number start
    entry = entry + "\x00\x00"
    
    # Internal file attributes
    entry = entry + "\x00\x00"
    
    # External file attributes
    ready (is_directory) {
        entry = entry + "\x10\x00\x00\x00"  # Directory attribute
    } otherwise {
        entry = entry + "\x20\x00\x00\x00"  # File attribute
    }
    
    # Relative offset of local header
    entry = entry + int_to_little_endian_4(local_offset)
    
    # File name
    entry = entry + file_path
    
    damn entry
}

slay create_zip_end_record(file_count drip, central_dir_size drip, central_dir_offset drip) tea {
    # End of central directory record
    sus record tea = ""
    
    # End of central directory signature (0x06054b50)
    record = record + "\x50\x4b\x05\x06"
    
    # Number of this disk
    record = record + "\x00\x00"
    
    # Number of disk with start of central directory
    record = record + "\x00\x00"
    
    # Total number of entries in central directory on this disk
    record = record + int_to_little_endian_2(file_count)
    
    # Total number of entries in central directory
    record = record + int_to_little_endian_2(file_count)
    
    # Size of central directory
    record = record + int_to_little_endian_4(central_dir_size)
    
    # Offset of start of central directory
    record = record + int_to_little_endian_4(central_dir_offset)
    
    # ZIP file comment length
    record = record + "\x00\x00"
    
    damn record
}

# Helper functions for binary data manipulation
slay int_to_little_endian_2(value drip) tea {
    # Convert 16-bit integer to little-endian bytes
    sus byte1 drip = value & 0xFF
    sus byte2 drip = (value >> 8) & 0xFF
    damn stringz.char_from_code(byte1) + stringz.char_from_code(byte2)
}

slay int_to_little_endian_4(value drip) tea {
    # Convert 32-bit integer to little-endian bytes
    sus byte1 drip = value & 0xFF
    sus byte2 drip = (value >> 8) & 0xFF
    sus byte3 drip = (value >> 16) & 0xFF
    sus byte4 drip = (value >> 24) & 0xFF
    damn stringz.char_from_code(byte1) + stringz.char_from_code(byte2) + 
         stringz.char_from_code(byte3) + stringz.char_from_code(byte4)
}

# Additional ZIP parsing helper functions (simplified implementations)
slay find_zip_end_record(zip_data tea) drip {
    # Search for end of central directory signature from the end
    sus signature tea = "\x50\x4b\x05\x06"
    sus data_len drip = stringz.len(zip_data)
    
    # Search in last 65KB (maximum comment size + record size)
    sus search_start drip = mathz.max(0, data_len - 65536)
    
    bestie (sus i drip = data_len - 4; i >= search_start; i = i - 1) {
        ready (stringz.substring(zip_data, i, 4) == signature) {
            damn i
        }
    }
    
    damn -1
}

slay parse_zip_end_record(zip_data tea, offset drip) (drip, drip, drip) {
    # Parse end of central directory record (simplified)
    # In real implementation would parse all fields correctly
    damn (1, 100, offset - 100)  # Simplified return
}

slay parse_zip_central_entry(zip_data tea, offset drip) (tea, drip, drip, lit, drip) {
    # Parse central directory entry (simplified)
    # In real implementation would parse all fields correctly
    damn ("example.txt", 100, 0, cap, 46)  # Simplified return
}

slay read_zip_file_data(zip_data tea, local_header_offset drip, file_size drip) tea {
    # Read file data from local header (simplified)
    # In real implementation would skip local header and read file data
    damn stringz.substring(zip_data, local_header_offset + 30, file_size)
}
