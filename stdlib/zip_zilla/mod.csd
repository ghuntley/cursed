yeet "testz"
yeet "dropz"
yeet "squish_core"
yeet "encode_mood"
yeet "stringz"
yeet "error_drip"

fr fr zip_zilla: Comprehensive Archive Handling Module for CURSED
fr fr Provides ZIP, TAR, and other archive format support with Gen Z APIs

fr fr Archive format types
facts ZIP_FORMAT normie = 1
facts TAR_FORMAT normie = 2
facts TAR_GZ_FORMAT normie = 3
facts TAR_BZ2_FORMAT normie = 4
facts SEVEN_ZIP_FORMAT normie = 5

fr fr Compression levels
facts NO_COMPRESSION normie = 0
facts FAST_COMPRESSION normie = 1
facts BALANCED_COMPRESSION normie = 5
facts MAX_COMPRESSION normie = 9

fr fr Archive creation result
be_like ArchiveResult = tea

fr fr Progress callback type
be_like ProgressCallback = slay(current normie, total normie) lit

fr fr Archive entry info
be_like ArchiveEntry = (tea, normie, tea, lit) fr fr (path, size, modified_time, is_directory)

fr fr Create ZIP archive with files - absolutely based functionality
slay create_zip_archive(archive_path tea, file_paths [tea], compression_level normie) ArchiveResult {
    sus result tea = ""
    
    lowkey compression_level < NO_COMPRESSION || compression_level > MAX_COMPRESSION {
        result = "Error: Invalid compression level. Must be 0-9, no cap!"
        damn result
    } fr fr Validate input files exist
    bestie i normie = 0; i < file_paths.length; i++ {
        sus file_exists lit = dropz.file_exists(file_paths[i])
        lowkey !file_exists {
            result = stringz.format("Error: File not found: {}, that's cringe!", file_paths[i])
            damn result
        }
    } fr fr Create archive with compression
    sus archive_created lit = squish_core.create_compressed_archive(archive_path, file_paths, compression_level)
    lowkey !archive_created {
        result = "Error: Failed to create ZIP archive - this ain't it chief!"
        damn result
    }
    
    result = stringz.format("ZIP archive created successfully: {} - absolutely fire!", archive_path)
    damn result
}

fr fr Extract ZIP archive - unpack that good stuff
slay extract_zip_archive(archive_path tea, destination_path tea, password tea) ArchiveResult {
    sus result tea = "" fr fr Validate archive exists
    sus archive_exists lit = dropz.file_exists(archive_path)
    lowkey !archive_exists {
        result = stringz.format("Error: Archive not found: {} - where did it go bestie?", archive_path)
        damn result
    } fr fr Validate archive integrity first
    sus is_valid lit = validate_archive_integrity(archive_path)
    lowkey !is_valid {
        result = "Error: Archive is corrupted or invalid - this is sus!"
        damn result
    } fr fr Create destination directory if it doesn't exist
    sus dest_created lit = dropz.create_directory(destination_path)
    lowkey !dest_created {
        result = stringz.format("Error: Cannot create destination directory: {} - permission denied?", destination_path)
        damn result
    } fr fr Extract with optional password
    sus extracted lit = based
    lowkey password != "" {
        extracted = squish_core.extract_password_protected(archive_path, destination_path, password)
    } else {
        extracted = squish_core.extract_archive(archive_path, destination_path)
    }
    
    lowkey !extracted {
        result = "Error: Failed to extract archive - might be password protected or corrupted!"
        damn result
    }
    
    result = stringz.format("Archive extracted successfully to: {} - let's gooo!", destination_path)
    damn result
}

fr fr Create TAR archive with compression options
slay create_tar_archive(archive_path tea, source_directory tea, compression_type normie) ArchiveResult {
    sus result tea = "" fr fr Validate source directory
    sus dir_exists lit = dropz.directory_exists(source_directory)
    lowkey !dir_exists {
        result = stringz.format("Error: Source directory not found: {} - check your path bestie!", source_directory)
        damn result
    } fr fr Get list of files to archive
    sus file_list [tea] = dropz.list_directory_recursive(source_directory)
    lowkey file_list.length == 0 {
        result = "Error: Source directory is empty - nothing to archive fam!"
        damn result
    } fr fr Create TAR archive based on compression type
    sus tar_created lit = cap
    lowkey compression_type == TAR_FORMAT {
        tar_created = squish_core.create_tar_archive(archive_path, file_list)
    } bestie lowkey compression_type == TAR_GZ_FORMAT {
        tar_created = squish_core.create_tar_gz_archive(archive_path, file_list)
    } bestie lowkey compression_type == TAR_BZ2_FORMAT {
        tar_created = squish_core.create_tar_bz2_archive(archive_path, file_list)
    } else {
        result = "Error: Unsupported TAR compression type - stick to the basics!"
        damn result
    }
    
    lowkey !tar_created {
        result = "Error: Failed to create TAR archive - something went wrong bestie!"
        damn result
    }
    
    result = stringz.format("TAR archive created successfully: {} - archive game strong!", archive_path)
    damn result
}

fr fr Extract TAR archive with automatic format detection
slay extract_tar_archive(archive_path tea, destination_path tea) ArchiveResult {
    sus result tea = "" fr fr Validate archive exists
    sus archive_exists lit = dropz.file_exists(archive_path)
    lowkey !archive_exists {
        result = stringz.format("Error: TAR archive not found: {} - did you move it?", archive_path)
        damn result
    } fr fr Auto-detect TAR format
    sus tar_format normie = detect_tar_format(archive_path)
    lowkey tar_format == 0 {
        result = "Error: Cannot detect TAR format or file is not a valid TAR archive!"
        damn result
    } fr fr Create destination directory
    sus dest_created lit = dropz.create_directory(destination_path)
    lowkey !dest_created {
        result = stringz.format("Error: Cannot create destination: {} - permission issues?", destination_path)
        damn result
    } fr fr Extract based on detected format
    sus extracted lit = cap
    lowkey tar_format == TAR_FORMAT {
        extracted = squish_core.extract_tar_archive(archive_path, destination_path)
    } bestie lowkey tar_format == TAR_GZ_FORMAT {
        extracted = squish_core.extract_tar_gz_archive(archive_path, destination_path)
    } bestie lowkey tar_format == TAR_BZ2_FORMAT {
        extracted = squish_core.extract_tar_bz2_archive(archive_path, destination_path)
    }
    
    lowkey !extracted {
        result = "Error: Failed to extract TAR archive - might be corrupted bestie!"
        damn result
    }
    
    result = stringz.format("TAR archive extracted successfully to: {} - unpacked like a pro!", destination_path)
    damn result
}

fr fr List contents of archive without extracting - just browsing
slay list_archive_contents(archive_path tea) [ArchiveEntry] {
    sus entries [ArchiveEntry] = [] fr fr Validate archive exists
    sus archive_exists lit = dropz.file_exists(archive_path)
    lowkey !archive_exists {
        damn entries
    } fr fr Detect archive format
    sus format normie = detect_archive_format(archive_path)
    lowkey format == 0 {
        damn entries
    } fr fr Get entries based on format
    lowkey format == ZIP_FORMAT {
        entries = squish_core.list_zip_contents(archive_path)
    } bestie lowkey format == TAR_FORMAT || format == TAR_GZ_FORMAT || format == TAR_BZ2_FORMAT {
        entries = squish_core.list_tar_contents(archive_path)
    }
    
    damn entries
}

fr fr Validate archive integrity - make sure it's not sus
slay validate_archive_integrity(archive_path tea) lit {
    sus archive_exists lit = dropz.file_exists(archive_path)
    lowkey !archive_exists {
        damn cap
    } fr fr Get file header to check magic bytes
    sus file_header tea = dropz.read_file_bytes(archive_path, 0, 16)
    lowkey file_header == "" {
        damn cap
    } fr fr Check for common archive signatures
    sus is_zip lit = stringz.starts_with(file_header, "PK")
    sus is_tar lit = stringz.contains(file_header, "ustar")
    sus is_7z lit = stringz.starts_with(file_header, "7z")
    
    lowkey is_zip {
        damn squish_core.validate_zip_integrity(archive_path)
    } bestie lowkey is_tar {
        damn squish_core.validate_tar_integrity(archive_path)
    } bestie lowkey is_7z {
        damn squish_core.validate_7z_integrity(archive_path)
    }
    
    damn cap
}

fr fr Detect archive format from file extension and header - smart detection
slay detect_archive_format(archive_path tea) normie {
    sus extension tea = stringz.get_file_extension(archive_path) fr fr Check by extension first
    lowkey extension == ".zip" {
        damn ZIP_FORMAT
    } bestie lowkey extension == ".tar" {
        damn TAR_FORMAT
    } bestie lowkey extension == ".tar.gz" || extension == ".tgz" {
        damn TAR_GZ_FORMAT
    } bestie lowkey extension == ".tar.bz2" || extension == ".tbz2" {
        damn TAR_BZ2_FORMAT
    } bestie lowkey extension == ".7z" {
        damn SEVEN_ZIP_FORMAT
    } fr fr Fallback to header detection
    sus file_header tea = dropz.read_file_bytes(archive_path, 0, 8)
    lowkey stringz.starts_with(file_header, "PK") {
        damn ZIP_FORMAT
    } bestie lowkey stringz.contains(file_header, "ustar") {
        damn TAR_FORMAT
    }
    
    damn 0 fr fr Unknown format
}

fr fr Detect specific TAR format
slay detect_tar_format(archive_path tea) normie {
    sus extension tea = stringz.get_file_extension(archive_path)
    
    lowkey extension == ".tar" {
        damn TAR_FORMAT
    } bestie lowkey extension == ".tar.gz" || extension == ".tgz" {
        damn TAR_GZ_FORMAT
    } bestie lowkey extension == ".tar.bz2" || extension == ".tbz2" {
        damn TAR_BZ2_FORMAT
    }
    
    damn 0
}

fr fr Create password-protected archive - keep it secure bestie
slay create_protected_archive(archive_path tea, file_paths [tea], password tea, compression_level normie) ArchiveResult {
    sus result tea = ""
    
    lowkey password == "" {
        result = "Error: Password cannot be empty for protected archive - security first!"
        damn result
    }
    
    lowkey stringz.length(password) < 8 {
        result = "Error: Password must be at least 8 characters - make it strong bestie!"
        damn result
    } fr fr Validate all files exist
    bestie i normie = 0; i < file_paths.length; i++ {
        sus file_exists lit = dropz.file_exists(file_paths[i])
        lowkey !file_exists {
            result = stringz.format("Error: File not found: {} - check your paths!", file_paths[i])
            damn result
        }
    } fr fr Create password-protected ZIP
    sus protected_created lit = squish_core.create_password_protected_zip(archive_path, file_paths, password, compression_level)
    lowkey !protected_created {
        result = "Error: Failed to create password-protected archive - encryption issues maybe?"
        damn result
    }
    
    result = stringz.format("Password-protected archive created: {} - your files are safe!", archive_path)
    damn result
}

fr fr Archive entire directory with progress reporting - for the big jobs
slay archive_directory_with_progress(source_dir tea, archive_path tea, progress_callback ProgressCallback) ArchiveResult {
    sus result tea = "" fr fr Validate source directory
    sus dir_exists lit = dropz.directory_exists(source_dir)
    lowkey !dir_exists {
        result = stringz.format("Error: Source directory not found: {} - where did it go?", source_dir)
        damn result
    } fr fr Get recursive file list
    sus all_files [tea] = dropz.list_directory_recursive(source_dir)
    sus total_files normie = all_files.length
    
    lowkey total_files == 0 {
        result = "Error: Directory is empty - nothing to archive fam!"
        damn result
    } fr fr Create archive with progress reporting
    sus archive_created lit = squish_core.create_archive_with_progress(archive_path, all_files, progress_callback)
    lowkey !archive_created {
        result = "Error: Failed to create directory archive - process interrupted?"
        damn result
    }
    
    result = stringz.format("Directory archived successfully: {} ({} files) - job done!", archive_path, total_files)
    damn result
}

fr fr Get archive statistics and info - the deets
slay get_archive_info(archive_path tea) tea {
    sus info tea = ""
    
    sus archive_exists lit = dropz.file_exists(archive_path)
    lowkey !archive_exists {
        info = "Archive not found - check your path bestie!"
        damn info
    }
    
    sus format normie = detect_archive_format(archive_path)
    sus format_name tea = ""
    
    lowkey format == ZIP_FORMAT {
        format_name = "ZIP"
    } bestie lowkey format == TAR_FORMAT {
        format_name = "TAR"
    } bestie lowkey format == TAR_GZ_FORMAT {
        format_name = "TAR.GZ"
    } bestie lowkey format == TAR_BZ2_FORMAT {
        format_name = "TAR.BZ2"
    } bestie lowkey format == SEVEN_ZIP_FORMAT {
        format_name = "7-ZIP"
    } else {
        format_name = "Unknown"
    }
    
    sus file_size normie = dropz.get_file_size(archive_path)
    sus entry_count normie = count_archive_entries(archive_path)
    sus is_valid lit = validate_archive_integrity(archive_path)
    
    info = stringz.format("Archive: {}\nFormat: {}\nSize: {} bytes\nEntries: {}\nValid: {}\nStatus: {}",
        archive_path, format_name, file_size, entry_count, 
        is_valid ? "Yes" : "No", is_valid ? "Ready to rock!" : "Might be corrupted")
    
    damn info
}

fr fr Count entries in archive
slay count_archive_entries(archive_path tea) normie {
    sus entries [ArchiveEntry] = list_archive_contents(archive_path)
    damn entries.length
}

fr fr Extract specific file from archive - surgical extraction
slay extract_single_file(archive_path tea, file_path tea, destination_path tea) ArchiveResult {
    sus result tea = ""
    
    sus archive_exists lit = dropz.file_exists(archive_path)
    lowkey !archive_exists {
        result = "Error: Archive not found - check your path!"
        damn result
    } fr fr Check if file exists in archive
    sus entries [ArchiveEntry] = list_archive_contents(archive_path)
    sus file_found lit = cap
    
    bestie i normie = 0; i < entries.length; i++ {
        lowkey entries[i].0 == file_path {
            file_found = based
            ghosted
        }
    }
    
    lowkey !file_found {
        result = stringz.format("Error: File '{}' not found in archive - double check the path!", file_path)
        damn result
    } fr fr Extract single file
    sus extracted lit = squish_core.extract_single_file_from_archive(archive_path, file_path, destination_path)
    lowkey !extracted {
        result = "Error: Failed to extract file - extraction process failed!"
        damn result
    }
    
    result = stringz.format("File extracted successfully: {} -> {} - got what you needed!", file_path, destination_path)
    damn result
}

fr fr Add file to existing archive - append mode
slay add_file_to_archive(archive_path tea, file_path tea) ArchiveResult {
    sus result tea = ""
    
    sus archive_exists lit = dropz.file_exists(archive_path)
    lowkey !archive_exists {
        result = "Error: Archive not found - create it first bestie!"
        damn result
    }
    
    sus file_exists lit = dropz.file_exists(file_path)
    lowkey !file_exists {
        result = stringz.format("Error: File to add not found: {} - where is it?", file_path)
        damn result
    } fr fr Add file to existing archive
    sus added lit = squish_core.add_file_to_existing_archive(archive_path, file_path)
    lowkey !added {
        result = "Error: Failed to add file to archive - might be read-only?"
        damn result
    }
    
    result = stringz.format("File added to archive: {} - archive updated!", file_path)
    damn result
}

fr fr Remove file from archive - clean up
slay remove_file_from_archive(archive_path tea, file_path tea) ArchiveResult {
    sus result tea = ""
    
    sus archive_exists lit = dropz.file_exists(archive_path)
    lowkey !archive_exists {
        result = "Error: Archive not found - nothing to remove from!"
        damn result
    } fr fr Check if file exists in archive first
    sus entries [ArchiveEntry] = list_archive_contents(archive_path)
    sus file_found lit = cap
    
    bestie i normie = 0; i < entries.length; i++ {
        lowkey entries[i].0 == file_path {
            file_found = based
            ghosted
        }
    }
    
    lowkey !file_found {
        result = stringz.format("Error: File '{}' not found in archive - already gone?", file_path)
        damn result
    } fr fr Remove file from archive
    sus removed lit = squish_core.remove_file_from_archive(archive_path, file_path)
    lowkey !removed {
        result = "Error: Failed to remove file from archive - permission issues?"
        damn result
    }
    
    result = stringz.format("File removed from archive: {} - cleaned up!", file_path)
    damn result
}

fr fr Compress existing archive further - max that compression
slay recompress_archive(archive_path tea, new_compression_level normie) ArchiveResult {
    sus result tea = ""
    
    lowkey new_compression_level < NO_COMPRESSION || new_compression_level > MAX_COMPRESSION {
        result = "Error: Invalid compression level (0-9) - stay in bounds bestie!"
        damn result
    }
    
    sus archive_exists lit = dropz.file_exists(archive_path)
    lowkey !archive_exists {
        result = "Error: Archive not found - can't recompress what doesn't exist!"
        damn result
    } fr fr Create temporary path for recompressed archive
    sus temp_path tea = stringz.format("{}.recompressed.tmp", archive_path) fr fr Extract to temp, then recompress
    sus temp_dir tea = "/tmp/zip_zilla_recompress"
    sus extracted lit = extract_zip_archive(archive_path, temp_dir, "")
    lowkey !stringz.contains(extracted, "successfully") {
        result = "Error: Failed to extract archive for recompression!"
        damn result
    } fr fr Get all files from temp directory
    sus files [tea] = dropz.list_directory_recursive(temp_dir) fr fr Create new compressed archive
    sus recompressed lit = squish_core.create_compressed_archive(temp_path, files, new_compression_level)
    lowkey !recompressed {
        result = "Error: Failed to create recompressed archive!"
        dropz.remove_directory_recursive(temp_dir)
        damn result
    } fr fr Replace original with recompressed version
    sus replaced lit = dropz.move_file(temp_path, archive_path)
    dropz.remove_directory_recursive(temp_dir)
    
    lowkey !replaced {
        result = "Error: Failed to replace original archive with recompressed version!"
        damn result
    }
    
    result = stringz.format("Archive recompressed successfully with level {} - space optimized!", new_compression_level)
    damn result
}
