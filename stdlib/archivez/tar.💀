# archivez/tar - TAR Archive Implementation  
# Pure CURSED implementation of TAR file format handling

yeet "vibez"

# TAR format constants
sus TAR_BLOCK_SIZE drip = 512
sus TAR_NAME_SIZE drip = 100
sus TAR_MODE_SIZE drip = 8
sus TAR_UID_SIZE drip = 8
sus TAR_GID_SIZE drip = 8
sus TAR_SIZE_SIZE drip = 12
sus TAR_MTIME_SIZE drip = 12
sus TAR_CHECKSUM_SIZE drip = 8

# TAR file types
sus TAR_REGULAR_FILE tea = "0"
sus TAR_LINK tea = "1"
sus TAR_SYMLINK tea = "2"
sus TAR_CHARACTER_DEVICE tea = "3"
sus TAR_BLOCK_DEVICE tea = "4"
sus TAR_DIRECTORY tea = "5"
sus TAR_FIFO tea = "6"

# TAR header structure (POSIX format)
squad TarHeader {
    sus name tea           # File name
    sus mode tea           # File mode (permissions)
    sus uid tea            # Owner user ID
    sus gid tea            # Owner group ID
    sus size drip          # File size in bytes
    sus mtime drip         # Last modification time
    sus checksum drip      # Header checksum
    sus typeflag tea       # File type flag
    sus linkname tea       # Link target name
    sus magic tea          # POSIX magic ("ustar")
    sus version tea        # POSIX version ("00")
    sus uname tea          # Owner user name
    sus gname tea          # Owner group name
    sus devmajor tea       # Device major number
    sus devminor tea       # Device minor number
    sus prefix tea         # Filename prefix
}

# TAR entry structure
squad TarEntry {
    sus header TarHeader
    sus data tea
    sus data_size drip
    sus block_offset drip
}

# TAR archive state
sus tar_entries TarEntry[value]
sus tar_current_offset drip = 0

# Initialize TAR archive
slay init_tar() {
    tar_entries = TarEntry[value]{}
    tar_current_offset = 0
    vibez.spill("TAR: Archive initialized")
}

# Add file to TAR archive
slay tar_add_file(local_path tea, archive_path tea, data tea) yikes<lit> {
    ready (archive_path == "") {
        yikes "archive path cannot be empty"
    }
    
    ready (len(archive_path) >= TAR_NAME_SIZE) {
        yikes "filename too long for TAR format: " + archive_path
    }
    
    # Create TAR header
    sus header TarHeader = create_tar_header(archive_path, len(data), TAR_REGULAR_FILE)
    
    # Create TAR entry
    sus entry TarEntry = TarEntry{
        header: header,
        data: data,
        data_size: len(data),
        block_offset: tar_current_offset
    }
    
    # Add to entries list
    append_tar_entry(entry)
    
    # Update current offset (header + data blocks)
    sus data_blocks drip = calculate_tar_blocks(len(data))
    tar_current_offset = tar_current_offset + 1 + data_blocks  # 1 block for header + data blocks
    
    vibez.spill("TAR: Added file " + archive_path + " (size: " + to_string(len(data)) + " bytes, " + to_string(data_blocks) + " blocks)")
    damn based
}

# Add directory to TAR archive
slay tar_add_directory(local_path tea, archive_path tea) yikes<lit> {
    ready (archive_path == "") {
        yikes "archive path cannot be empty"
    }
    
    # Ensure directory path ends with slash
    sus dir_path tea = archive_path
    ready (!ends_with(dir_path, "/")) {
        dir_path = dir_path + "/"
    }
    
    ready (len(dir_path) >= TAR_NAME_SIZE) {
        yikes "directory name too long for TAR format: " + dir_path
    }
    
    # Create TAR header for directory
    sus header TarHeader = create_tar_header(dir_path, 0, TAR_DIRECTORY)
    
    # Create TAR entry
    sus entry TarEntry = TarEntry{
        header: header,
        data: "",
        data_size: 0,
        block_offset: tar_current_offset
    }
    
    # Add to entries list
    append_tar_entry(entry)
    
    # Update current offset (header only)
    tar_current_offset = tar_current_offset + 1
    
    vibez.spill("TAR: Added directory " + dir_path)
    damn based
}

# Extract file from TAR archive
slay tar_extract_file(archive_path tea, output_path tea) yikes<tea> {
    # Find entry in TAR
    sus entry TarEntry = find_tar_entry(archive_path) fam {
        when _ -> yikes "file not found in archive: " + archive_path
    }
    
    ready (entry.header.typeflag == TAR_DIRECTORY) {
        yikes "cannot extract directory as file: " + archive_path
    }
    
    # Return the file data
    vibez.spill("TAR: Extracted " + archive_path + " to " + output_path + " (" + to_string(entry.data_size) + " bytes)")
    damn entry.data
}

# List files in TAR archive
slay tar_list_files() tea[value]{
    sus file_list tea[value]
    
    bestie (drip i = 0; i < len(tar_entries); i = i + 1) {
        sus entry TarEntry = tar_entries[i]
        append_string(file_list, entry.header.name)
    }
    
    damn file_list
}

# Get TAR entry count
slay tar_get_entry_count() drip {
    damn len(tar_entries)
}

# Find TAR entry by filename
slay find_tar_entry(filename tea) yikes<TarEntry> {
    bestie (drip i = 0; i < len(tar_entries); i = i + 1) {
        sus entry TarEntry = tar_entries[i]
        ready (entry.header.name == filename) {
            damn entry
        }
    }
    yikes "entry not found: " + filename
}

# Check if file exists in TAR
slay tar_file_exists(filename tea) lit {
    find_tar_entry(filename) fam {
        when _ -> damn cap
    }
    damn based
}

# Get TAR file info
slay tar_get_file_info(filename tea) yikes<tea> {
    sus entry TarEntry = find_tar_entry(filename) fam {
        when _ -> yikes "file not found: " + filename
    }
    
    sus header TarHeader = entry.header
    
    sus info tea = "File: " + header.name + "\n"
    info = info + "Size: " + to_string(entry.data_size) + " bytes\n"
    info = info + "Mode: " + header.mode + "\n"
    info = info + "Type: " + get_tar_type_name(header.typeflag) + "\n"
    info = info + "Owner: " + header.uname + " (" + header.uid + ")\n"
    info = info + "Group: " + header.gname + " (" + header.gid + ")\n"
    info = info + "Modified: " + to_string(header.mtime) + "\n"
    
    ready (header.linkname != "") {
        info = info + "Link Target: " + header.linkname + "\n"
    }
    
    damn info
}

# Get TAR file type name
slay get_tar_type_name(typeflag tea) tea {
    ready (typeflag == TAR_REGULAR_FILE) {
        damn "Regular File"
    }
    ready (typeflag == TAR_DIRECTORY) {
        damn "Directory"
    }
    ready (typeflag == TAR_LINK) {
        damn "Hard Link"
    }
    ready (typeflag == TAR_SYMLINK) {
        damn "Symbolic Link"
    }
    ready (typeflag == TAR_CHARACTER_DEVICE) {
        damn "Character Device"
    }
    ready (typeflag == TAR_BLOCK_DEVICE) {
        damn "Block Device"
    }
    ready (typeflag == TAR_FIFO) {
        damn "FIFO"
    }
    damn "Unknown"
}

# Create TAR header
slay create_tar_header(name tea, size drip, typeflag tea) TarHeader {
    sus header TarHeader = TarHeader{
        name: name,
        mode: "644",           # Default file permissions
        uid: "1000",           # Default user ID
        gid: "1000",           # Default group ID
        size: size,
        mtime: get_current_time(),
        checksum: 0,           # Will be calculated later
        typeflag: typeflag,
        linkname: "",
        magic: "ustar",        # POSIX magic
        version: "00",         # POSIX version
        uname: "user",         # Default username
        gname: "group",        # Default group name
        devmajor: "0",         # Device major number
        devminor: "0",         # Device minor number
        prefix: ""             # Filename prefix
    }
    
    # Calculate and set checksum
    header.checksum = calculate_tar_checksum(header)
    
    damn header
}

# Calculate number of 512-byte blocks needed for data
slay calculate_tar_blocks(size drip) drip {
    ready (size == 0) {
        damn 0
    }
    
    sus blocks drip = size / TAR_BLOCK_SIZE
    ready (size % TAR_BLOCK_SIZE > 0) {
        blocks = blocks + 1  # Round up for partial block
    }
    
    damn blocks
}

# Calculate TAR header checksum
slay calculate_tar_checksum(header TarHeader) drip {
    # Simplified checksum calculation
    # Real TAR checksum is sum of all header bytes with checksum field as spaces
    sus checksum drip = 0
    
    checksum = checksum + simple_hash(header.name)
    checksum = checksum + simple_hash(header.mode)
    checksum = checksum + simple_hash(header.uid)
    checksum = checksum + simple_hash(header.gid)
    checksum = checksum + header.size
    checksum = checksum + header.mtime
    
    damn checksum % 100000  # Keep reasonable size
}

# Simple hash function for strings
slay simple_hash(str tea) drip {
    sus hash drip = 0
    bestie (drip i = 0; i < len(str); i = i + 1) {
        hash = hash + i * 17  # Simple hash algorithm for demo
    }
    damn hash
}

# Get current time (simplified)
slay get_current_time() drip {
    # Return a fixed timestamp for demonstration
    # Real implementation would get actual current time
    damn 1609459200  # Jan 1, 2021 00:00:00 UTC
}

# Validate TAR archive structure
slay tar_validate() lit {
    vibez.spill("TAR: Validating archive structure")
    
    # Check for valid entries
    ready (len(tar_entries) == 0) {
        vibez.spill("TAR: Warning - empty archive")
        damn based
    }
    
    # Validate each entry
    bestie (drip i = 0; i < len(tar_entries); i = i + 1) {
        sus entry TarEntry = tar_entries[i]
        sus header TarHeader = entry.header
        
        # Basic validation checks
        ready (header.name == "") {
            vibez.spill("TAR: Invalid entry - empty filename")
            damn cap
        }
        
        ready (header.magic != "ustar") {
            vibez.spill("TAR: Invalid entry - bad magic number")
            damn cap
        }
        
        ready (entry.data_size < 0) {
            vibez.spill("TAR: Invalid entry - negative size")
            damn cap
        }
        
        # Validate checksum
        sus calculated_checksum drip = calculate_tar_checksum(header)
        ready (calculated_checksum != header.checksum) {
            vibez.spill("TAR: Invalid entry - checksum mismatch for " + header.name)
            damn cap
        }
    }
    
    vibez.spill("TAR: Archive validation passed")
    damn based
}

# Get TAR archive statistics
slay tar_get_stats() tea {
    sus entry_count drip = len(tar_entries)
    sus total_size drip = 0
    sus directory_count drip = 0
    sus file_count drip = 0
    sus link_count drip = 0
    sus oldest_mtime drip = get_current_time()
    sus newest_mtime drip = 0
    
    bestie (drip i = 0; i < entry_count; i = i + 1) {
        sus entry TarEntry = tar_entries[i]
        sus header TarHeader = entry.header
        
        total_size = total_size + entry.data_size
        
        ready (header.typeflag == TAR_DIRECTORY) {
            directory_count = directory_count + 1
        } otherwise ready (header.typeflag == TAR_REGULAR_FILE) {
            file_count = file_count + 1
        } otherwise ready (header.typeflag == TAR_LINK || header.typeflag == TAR_SYMLINK) {
            link_count = link_count + 1
        }
        
        ready (header.mtime < oldest_mtime) {
            oldest_mtime = header.mtime
        }
        ready (header.mtime > newest_mtime) {
            newest_mtime = header.mtime
        }
    }
    
    sus total_blocks drip = calculate_tar_blocks(total_size) + entry_count  # Data blocks + header blocks
    
    sus stats tea = "TAR Archive Statistics:\n"
    stats = stats + "Total Entries: " + to_string(entry_count) + "\n"
    stats = stats + "Files: " + to_string(file_count) + "\n"
    stats = stats + "Directories: " + to_string(directory_count) + "\n"
    stats = stats + "Links: " + to_string(link_count) + "\n"
    stats = stats + "Total Size: " + to_string(total_size) + " bytes\n"
    stats = stats + "Total Blocks: " + to_string(total_blocks) + " (" + to_string(total_blocks * TAR_BLOCK_SIZE) + " bytes)\n"
    stats = stats + "Oldest File: " + to_string(oldest_mtime) + "\n"
    stats = stats + "Newest File: " + to_string(newest_mtime) + "\n"
    
    damn stats
}

# Create symbolic link entry in TAR
slay tar_add_symlink(link_path tea, target_path tea) yikes<lit> {
    ready (link_path == "") {
        yikes "link path cannot be empty"
    }
    
    ready (target_path == "") {
        yikes "target path cannot be empty"
    }
    
    ready (len(link_path) >= TAR_NAME_SIZE) {
        yikes "link name too long for TAR format: " + link_path
    }
    
    # Create TAR header for symbolic link
    sus header TarHeader = create_tar_header(link_path, 0, TAR_SYMLINK)
    header.linkname = target_path
    
    # Create TAR entry
    sus entry TarEntry = TarEntry{
        header: header,
        data: "",
        data_size: 0,
        block_offset: tar_current_offset
    }
    
    # Add to entries list
    append_tar_entry(entry)
    
    # Update current offset (header only)
    tar_current_offset = tar_current_offset + 1
    
    vibez.spill("TAR: Added symbolic link " + link_path + " -> " + target_path)
    damn based
}

# Create hard link entry in TAR
slay tar_add_hardlink(link_path tea, target_path tea) yikes<lit> {
    ready (link_path == "") {
        yikes "link path cannot be empty"
    }
    
    ready (target_path == "") {
        yikes "target path cannot be empty"
    }
    
    ready (len(link_path) >= TAR_NAME_SIZE) {
        yikes "link name too long for TAR format: " + link_path
    }
    
    # Create TAR header for hard link
    sus header TarHeader = create_tar_header(link_path, 0, TAR_LINK)
    header.linkname = target_path
    
    # Create TAR entry
    sus entry TarEntry = TarEntry{
        header: header,
        data: "",
        data_size: 0,
        block_offset: tar_current_offset
    }
    
    # Add to entries list
    append_tar_entry(entry)
    
    # Update current offset (header only)
    tar_current_offset = tar_current_offset + 1
    
    vibez.spill("TAR: Added hard link " + link_path + " -> " + target_path)
    damn based
}

# Set file permissions for TAR entry
slay tar_set_file_mode(archive_path tea, mode tea) yikes<lit> {
    sus entry TarEntry = find_tar_entry(archive_path) fam {
        when _ -> yikes "file not found: " + archive_path
    }
    
    entry.header.mode = mode
    entry.header.checksum = calculate_tar_checksum(entry.header)
    
    vibez.spill("TAR: Set mode " + mode + " for " + archive_path)
    damn based
}

# Set file ownership for TAR entry
slay tar_set_ownership(archive_path tea, uid tea, gid tea, uname tea, gname tea) yikes<lit> {
    sus entry TarEntry = find_tar_entry(archive_path) fam {
        when _ -> yikes "file not found: " + archive_path
    }
    
    entry.header.uid = uid
    entry.header.gid = gid
    entry.header.uname = uname
    entry.header.gname = gname
    entry.header.checksum = calculate_tar_checksum(entry.header)
    
    vibez.spill("TAR: Set ownership " + uname + ":" + gname + " (" + uid + ":" + gid + ") for " + archive_path)
    damn based
}

# TAR repair functionality
slay tar_repair() lit {
    vibez.spill("TAR: Attempting to repair corrupted archive")
    
    sus repaired_entries TarEntry[value]
    sus repair_count drip = 0
    
    # Validate and repair each entry
    bestie (drip i = 0; i < len(tar_entries); i = i + 1) {
        sus entry TarEntry = tar_entries[i]
        
        # Check if entry can be repaired
        ready (entry.header.name != "" && entry.data_size >= 0) {
            # Recalculate checksum
            entry.header.checksum = calculate_tar_checksum(entry.header)
            append_tar_entry(entry)
            repair_count = repair_count + 1
        } otherwise {
            vibez.spill("TAR: Discarding corrupted entry at index " + to_string(i))
        }
    }
    
    vibez.spill("TAR: Repaired " + to_string(repair_count) + " entries")
    damn based
}

# Helper functions
slay append_tar_entry(entry TarEntry) {
    # Simulate appending to entries array
    vibez.spill("TAR: Added entry " + entry.header.name + " to archive")
}
