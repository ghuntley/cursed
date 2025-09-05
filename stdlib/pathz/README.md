# pathz - Path Operations Module

## Overview

The `pathz` module provides cross-platform file system path manipulation and validation for CURSED programs. **Why dedicated path handling?** Because path operations are surprisingly complex with platform differences, encoding issues, security vulnerabilities, and edge cases that cause bugs in production. This module exists to make path handling bulletproof while maintaining clean, intuitive APIs.

**Design Philosophy**: Security-first with path traversal protection, Unicode normalization, and automatic platform detection with zero-allocation path operations where possible.

## Quick Start

```cursed
yeet "pathz"

// Basic path operations
sus home_dir tea = pathz.home_dir()
sus config_path pathz.Path = pathz.join(home_dir, ".config", "myapp", "settings.json")

// Path validation and normalization
ready (pathz.exists(config_path)) {
    sus normalized pathz.Path = pathz.normalize(config_path)
    sus absolute pathz.Path = pathz.absolute(normalized)
    vibez.spill("Config file:", absolute.to_string())
} otherwise {
    // Create directory structure
    sus parent_dir pathz.Path = config_path.parent()
    pathz.create_dir_all(parent_dir) fam {
        when "permission_denied" -> {
            vibez.spill_error("Cannot create config directory")
            damn
        }
    }
    
    pathz.create_file(config_path, "{}") fam {
        when _ -> vibez.spill_error("Failed to create config file")
    }
}

// Safe path manipulation
sus user_input tea = "../../etc/passwd"  // Potentially malicious input
sus safe_path pathz.Path = pathz.join("/app/data", user_input) fam {
    when "path_traversal" -> {
        vibez.spill_error("Path traversal attempt detected")
        damn pathz.Path.from("/app/data/default.txt")
    }
}

vibez.spill("Safe path:", safe_path.to_string())
```

## Why This Design?

### Security-First Path Validation
**Problem**: Path traversal attacks (`../../../etc/passwd`) are common security vulnerabilities that allow attackers to access files outside intended directories.

**Solution**: Built-in path traversal detection with automatic normalization and sandboxing capabilities that reject dangerous path patterns before they can cause harm.

### Unicode Normalization
**Problem**: File names can use different Unicode normalization forms, causing `café.txt` and `café.txt` to appear identical but be treated as different files.

**Solution**: Automatic Unicode normalization (NFC) ensures consistent path handling across all platforms and file systems, preventing subtle bugs.

### Zero-Allocation Fast Path
**Problem**: Path operations in hot loops (file scanning, directory traversal) create excessive allocations that impact performance.

**Solution**: Stack-allocated path buffers and string interning for common paths eliminate most allocations in typical usage patterns.

## API Reference

### Core Types

#### `pathz.Path`
Main path type with platform-aware operations and security features.

```cursed
squad pathz.Path {
    components []tea
    is_absolute lit
    platform pathz.Platform
    
    slay to_string() tea
    slay to_native_string() tea  // Platform-specific separators
    slay parent() pathz.Path
    slay file_name() tea
    slay extension() tea
    slay stem() tea  // File name without extension
    
    slay join(segments ...tea) yikes<pathz.Path>
    slay with_extension(ext tea) pathz.Path
    slay with_file_name(name tea) pathz.Path
    
    slay exists() lit
    slay is_file() lit
    slay is_dir() lit
    slay is_symlink() lit
    
    slay canonicalize() yikes<pathz.Path>
    slay normalize() pathz.Path
    slay relative_to(base pathz.Path) yikes<pathz.Path>
}
```

#### `pathz.PathPattern`
Pattern matching for path operations with glob support.

```cursed
squad pathz.PathPattern {
    pattern tea
    case_sensitive lit
    
    slay matches(path pathz.Path) lit
    slay find_matches(base_dir pathz.Path) yikes<[]pathz.Path>
    slay replace_in_path(path pathz.Path, replacement tea) pathz.Path
}
```

### Path Construction Functions

#### `join(components ...tea) yikes<pathz.Path>`
Safely joins path components with traversal protection.

**Why traversal protection?** Joining user input with base paths is a common source of directory traversal vulnerabilities.

**Example:**
```cursed
// Safe joining with automatic traversal detection
sus base_path tea = "/app/user-data"
sus user_file tea = "documents/report.pdf"  // Safe
sus malicious_file tea = "../../../etc/passwd"  // Malicious

sus safe_path pathz.Path = pathz.join(base_path, user_file)  // Works
// Result: "/app/user-data/documents/report.pdf"

sus blocked_path pathz.Path = pathz.join(base_path, malicious_file) fam {
    when "path_traversal" -> {
        vibez.spill_error("Blocked path traversal attempt:", malicious_file)
        damn pathz.join(base_path, "default.txt")  // Safe fallback
    }
}
```

#### `from_string(path_str tea) yikes<pathz.Path>`
Creates Path from string with validation and normalization.

**Example:**
```cursed
// Handle various path formats
sus windows_path tea = `C:\Users\Alice\Documents\file.txt`
sus unix_path tea = "/home/alice/documents/file.txt"
sus relative_path tea = "./docs/../README.md"

sus win_path pathz.Path = pathz.from_string(windows_path) fam {
    when "invalid_path" -> handle_invalid_path(windows_path)
}

sus unix_path_obj pathz.Path = pathz.from_string(unix_path) fam {
    when "invalid_characters" -> handle_invalid_characters(unix_path)  
}

sus normalized pathz.Path = pathz.from_string(relative_path) fam {
    when _ -> pathz.Path.from("README.md")  // Fallback
}
```

#### `current_dir() yikes<pathz.Path>`
Gets current working directory with error handling.

**Example:**
```cursed
sus cwd pathz.Path = pathz.current_dir() fam {
    when "permission_denied" -> {
        vibez.spill_error("Cannot access current directory")
        damn pathz.home_dir()  // Fallback to home
    }
    when "path_too_long" -> {
        vibez.spill_error("Current directory path too long")
        damn pathz.Path.from("/")  // Fallback to root
    }
}

vibez.spill("Working in:", cwd.to_string())
```

### Path Query Operations

#### Path Information
```cursed
sus file_path pathz.Path = pathz.from_string("/home/user/documents/report.pdf")

vibez.spill("Parent:", file_path.parent().to_string())      // "/home/user/documents"
vibez.spill("File name:", file_path.file_name())            // "report.pdf"
vibez.spill("Extension:", file_path.extension())            // ".pdf"
vibez.spill("Stem:", file_path.stem())                      // "report"
vibez.spill("Is absolute:", file_path.is_absolute)          // true

// Check file system properties
ready (file_path.exists()) {
    ready (file_path.is_file()) {
        vibez.spill("File size:", pathz.file_size(file_path))
        vibez.spill("Modified:", pathz.modified_time(file_path))
    } otherwise ready (file_path.is_dir()) {
        vibez.spill("Directory contains:", pathz.list_dir(file_path).length, "items")
    }
}
```

#### Path Relationships
```cursed
sus base_dir pathz.Path = pathz.from_string("/home/user/projects")
sus project_file pathz.Path = pathz.from_string("/home/user/projects/myapp/src/main.💀")

// Get relative path
sus relative pathz.Path = project_file.relative_to(base_dir) fam {
    when "not_relative" -> {
        vibez.spill_error("File is not under base directory")
        damn pathz.Path.from(".")
    }
}
vibez.spill("Relative path:", relative.to_string())  // "myapp/src/main.💀"

// Check path containment (security)
sus is_contained lit = pathz.is_contained_within(project_file, base_dir)
ready (is_contained) {
    vibez.spill("File is safely contained within project directory")
} otherwise {
    vibez.spill_error("File is outside project directory - potential security issue")
}
```

### File System Operations

#### Directory Operations
```cursed
// Create directory with parents
sus config_dir pathz.Path = pathz.join(pathz.home_dir(), ".config", "myapp")
pathz.create_dir_all(config_dir) fam {
    when "permission_denied" -> {
        vibez.spill_error("Cannot create config directory")
        yikes "config_setup_failed"
    }
    when "disk_full" -> {
        vibez.spill_error("Insufficient disk space")
        yikes "insufficient_space"
    }
}

// List directory contents with filtering
sus project_dir pathz.Path = pathz.from_string("./src")
sus source_files []pathz.Path = pathz.list_dir(project_dir) fam {
    when "not_found" -> damn []pathz.Path{}
    when "permission_denied" -> damn []pathz.Path{}
}

// Filter for CURSED source files
sus cursed_files []pathz.Path = []pathz.Path{}
bestie (sus file pathz.Path : source_files) {
    ready (file.extension() == ".💀") {
        cursed_files.push(file)
    }
}

vibez.spill("Found", cursed_files.length, "CURSED source files")
```

#### File Operations
```cursed
// Safe file operations with atomic writes
slay save_config(config_data tea, config_path pathz.Path) yikes<> {
    // Write to temporary file first
    sus temp_path pathz.Path = config_path.with_extension(".tmp")
    
    pathz.write_file(temp_path, config_data) fam {
        when "permission_denied" -> yikes "cannot_write_config"
        when "disk_full" -> yikes "insufficient_space"
    }
    
    // Atomic rename
    pathz.rename(temp_path, config_path) fam {
        when _ -> {
            // Cleanup temp file on error
            pathz.remove_file(temp_path) fam { when _ -> {} }
            yikes error
        }
    }
}

// Copy with progress and error recovery
slay copy_with_progress(src pathz.Path, dest pathz.Path) yikes<> {
    ready (!src.exists()) {
        yikes "source_not_found"
    }
    
    ready (dest.exists()) {
        ready (!confirm_overwrite(dest)) {
            yikes "user_cancelled"
        }
    }
    
    // Ensure destination directory exists
    pathz.create_dir_all(dest.parent()) fam {
        when _ -> yikes "cannot_create_dest_dir"
    }
    
    sus file_size drip = pathz.file_size(src)
    sus bytes_copied drip = 0
    
    pathz.copy_file_with_callback(src, dest, slay(copied drip) {
        bytes_copied = copied
        sus progress drip = (copied * 100) / file_size
        vibez.spill_no_newline("\rCopying... " + progress.(tea) + "%")
    }) fam {
        when "disk_full" -> {
            pathz.remove_file(dest) fam { when _ -> {} }  // Cleanup partial file
            yikes "insufficient_space"
        }
        when "permission_denied" -> yikes "cannot_write_dest"
    }
    
    vibez.spill("\nCopy completed:", bytes_copied, "bytes")
}
```

## Advanced Features

### Path Patterns and Globbing

**Why globbing?** File selection patterns like `*.💀` or `test/**/*.txt` are essential for build tools, file processing, and directory operations.

```cursed
// Create path pattern
sus source_pattern pathz.PathPattern = pathz.create_pattern("src/**/*.💀") fam {
    when "invalid_pattern" -> handle_pattern_error()
}

// Find all matching files
sus source_files []pathz.Path = source_pattern.find_matches(pathz.current_dir()) fam {
    when "permission_denied" -> damn []pathz.Path{}
}

vibez.spill("Found", source_files.length, "CURSED source files:")
bestie (sus file pathz.Path : source_files) {
    vibez.spill("  ", file.to_string())
}

// Complex pattern matching
sus test_pattern pathz.PathPattern = pathz.create_pattern("test/**/test_*.💀")
sus backup_pattern pathz.PathPattern = pathz.create_pattern("**/*.{bak,tmp,~}")

// Multiple pattern matching
sus patterns []pathz.PathPattern = [source_pattern, test_pattern]
sus all_matches []pathz.Path = pathz.find_matches_multiple(patterns, pathz.current_dir())
```

### Path Sandboxing

**Why sandboxing?** Applications processing user-provided file paths need to restrict access to prevent unauthorized file system access.

```cursed
// Create sandboxed path handler
sus sandbox pathz.Sandbox = pathz.create_sandbox(pathz.join(pathz.home_dir(), "app-data"))

// All path operations are restricted to sandbox
slay process_user_file(user_path_input tea) yikes<tea> {
    sus safe_path pathz.Path = sandbox.resolve_path(user_path_input) fam {
        when "outside_sandbox" -> {
            vibez.spill_error("User attempted to access file outside allowed area")
            yikes "access_denied"
        }
        when "path_traversal" -> {
            vibez.spill_error("Path traversal attempt detected")
            yikes "security_violation"
        }
    }
    
    // Now safe to operate on the path
    sus content tea = pathz.read_file(safe_path) fam {
        when _ -> yikes "read_error"
    }
    
    damn content
}

// Whitelist-based access control
sus restricted_sandbox pathz.Sandbox = pathz.create_restricted_sandbox([
    pathz.join(pathz.home_dir(), "Documents"),
    pathz.join(pathz.home_dir(), "Downloads"),
    "/tmp"  // Allow temp directory access
])

// Check if path is allowed
ready (restricted_sandbox.is_path_allowed(user_path)) {
    // Process file
} otherwise {
    vibez.spill_error("Access to this path is not permitted")
}
```

### Path Watching

**Why path watching?** Applications need to react to file system changes (file creation, modification, deletion) for features like auto-reload, sync, and monitoring.

```cursed
// Watch directory for changes
sus watcher pathz.PathWatcher = pathz.create_watcher() fam {
    when "not_supported" -> {
        vibez.spill_error("File watching not supported on this platform")
        yikes "watch_unavailable"
    }
}

// Add paths to watch
watcher.watch(pathz.from_string("./src"), pathz.WatchFlags{
    create: based,
    modify: based, 
    delete: based,
    recursive: based
}) fam {
    when _ -> vibez.spill_error("Failed to watch src directory")
}

// Process file system events
bestie (sus event pathz.WatchEvent : watcher.events()) {
    sick (event.kind) {
        when pathz.EventKind.Create -> {
            vibez.spill("File created:", event.path.to_string())
            ready (event.path.extension() == ".💀") {
                trigger_build()
            }
        }
        when pathz.EventKind.Modify -> {
            vibez.spill("File modified:", event.path.to_string())
            ready (event.path.file_name() == "config.json") {
                reload_configuration()
            }
        }
        when pathz.EventKind.Delete -> {
            vibez.spill("File deleted:", event.path.to_string())
            cleanup_references(event.path)
        }
    }
}
```

### Cross-Platform Path Handling

**Why explicit cross-platform support?** Path separators, case sensitivity, and naming rules differ significantly between Windows, Unix, and other platforms.

```cursed
// Platform-aware path operations
sus current_platform pathz.Platform = pathz.get_platform()

sick (current_platform) {
    when pathz.Platform.Windows -> {
        // Windows-specific path handling
        sus drive_letter tea = pathz.get_drive_letter(pathz.current_dir())
        vibez.spill("Current drive:", drive_letter)
        
        // Handle UNC paths
        sus network_path tea = `\\server\share\file.txt`
        sus unc_path pathz.Path = pathz.from_unc_string(network_path) fam {
            when "invalid_unc" -> handle_invalid_unc()
        }
    }
    when pathz.Platform.Unix -> {
        // Unix-specific operations  
        sus mount_points []pathz.Path = pathz.get_mount_points()
        bestie (sus mount pathz.Path : mount_points) {
            vibez.spill("Mount point:", mount.to_string())
        }
        
        // Handle symlinks properly
        sus link_target pathz.Path = pathz.read_link(symbolic_link) fam {
            when "not_symlink" -> original_path
            when "broken_link" -> handle_broken_symlink()
        }
    }
    when pathz.Platform.MacOS -> {
        // macOS-specific handling
        sus bundle_path pathz.Path = pathz.get_app_bundle_path() fam {
            when _ -> pathz.current_dir()
        }
    }
}

// Convert paths between platforms
sus unix_style_path tea = pathz.to_unix_path(windows_path)
sus windows_style_path tea = pathz.to_windows_path(unix_path)
```

## Performance Characteristics

### Path Operation Performance
- **Path creation**: ~100ns for simple paths, ~1μs for complex normalization
- **Path joining**: ~50ns per component (stack allocated)
- **Path exists check**: ~10μs (system call overhead)
- **Directory listing**: ~100μs + 5μs per file

### Memory Optimization
```cursed
// Stack-allocated path operations (zero heap allocation)
slay fast_path_operations(base_path pathz.Path, files []tea) {
    bestie (sus filename tea : files) {
        // These operations use stack allocation
        sus full_path pathz.Path = base_path.join_fast(filename) // Stack allocated
        ready (full_path.exists_fast()) {  // No string allocation
            process_file_fast(full_path)
        }
    }
}

// Memory pool for path-heavy operations
sus path_pool pathz.PathPool = pathz.create_path_pool(1000)  // Pre-allocate 1000 path slots

bestie (sus entry pathz.DirEntry : large_directory) {
    sus full_path pathz.Path = path_pool.get_path()  // Reuse allocated path
    full_path.set_from_dir_entry(entry)
    
    process_path(full_path)
    
    path_pool.return_path(full_path)  // Return to pool for reuse
}
```

### Benchmarking Path Operations
```cursed
slay benchmark_path_operations() {
    sus iterations drip = 100_000
    sus base_path pathz.Path = pathz.from_string("/home/user/projects")
    
    // Benchmark path joining
    sus join_start drip = get_microseconds()
    bestie (sus i drip = 0; i < iterations; i++) {
        sus joined pathz.Path = base_path.join("subdir", "file.txt")
    }
    sus join_time drip = get_microseconds() - join_start
    
    vibez.spill("Path join:", iterations, "operations in", join_time, "μs")
    vibez.spill("Average join time:", join_time / iterations, "μs per operation")
    
    // Benchmark file existence checks
    sus exists_start drip = get_microseconds()
    bestie (sus i drip = 0; i < 1000; i++) {  // Fewer iterations for I/O
        sus test_path pathz.Path = base_path.join("test" + i.(tea) + ".txt")
        sus _ lit = test_path.exists()
    }
    sus exists_time drip = get_microseconds() - exists_start
    
    vibez.spill("Exists check:", 1000, "operations in", exists_time, "μs")
}
```

## Error Handling Patterns

### Robust File Operations
```cursed
slay process_directory_safely(dir_path pathz.Path) yikes<ProcessingResult> {
    // Validate directory exists and is accessible
    ready (!dir_path.exists()) {
        yikes "directory_not_found"
    }
    
    ready (!dir_path.is_dir()) {
        yikes "not_a_directory" 
    }
    
    // Check permissions before proceeding
    ready (!pathz.is_readable(dir_path)) {
        yikes "permission_denied"
    }
    
    sus processed_files drip = 0
    sus failed_files drip = 0
    sus entries []pathz.Path = pathz.list_dir(dir_path) fam {
        when "permission_denied" -> {
            vibez.spill_error("Cannot list directory contents")
            yikes "access_denied"
        }
        when "io_error" -> {
            vibez.spill_error("I/O error reading directory")
            yikes "io_failure"
        }
    }
    
    bestie (sus entry pathz.Path : entries) {
        process_file_entry(entry) fam {
            when "file_corrupted" -> {
                failed_files++
                vibez.spill_error("Corrupted file:", entry.to_string())
                continue
            }
            when "permission_denied" -> {
                failed_files++
                vibez.spill_error("No permission for:", entry.to_string())
                continue
            }
            when _ -> {
                failed_files++
                vibez.spill_error("Error processing:", entry.to_string(), error)
                continue
            }
        }
        processed_files++
    }
    
    ready (failed_files > processed_files / 2) {
        yikes "too_many_failures"  // Fail if >50% of files failed
    }
    
    damn ProcessingResult{
        processed: processed_files,
        failed: failed_files,
        success_rate: (processed_files * 100) / (processed_files + failed_files)
    }
}
```

### Path Traversal Attack Prevention
```cursed
slay secure_file_access(base_dir pathz.Path, user_path tea) yikes<tea> {
    // Normalize user input
    sus requested_path pathz.Path = pathz.from_string(user_path) fam {
        when "invalid_path" -> {
            vibez.spill_error("Invalid path format:", user_path)
            yikes "invalid_input"
        }
    }
    
    // Resolve to absolute path
    sus absolute_path pathz.Path = requested_path.canonicalize() fam {
        when "path_not_found" -> {
            vibez.spill_error("Path does not exist:", user_path)
            yikes "not_found"
        }
        when "too_many_symlinks" -> {
            vibez.spill_error("Symlink loop detected:", user_path)
            yikes "symlink_loop"
        }
    }
    
    // Security check: ensure path is within allowed directory
    ready (!pathz.is_contained_within(absolute_path, base_dir)) {
        vibez.spill_error("Path traversal attack detected:", user_path)
        vibez.spill_error("Requested path:", absolute_path.to_string()) 
        vibez.spill_error("Base directory:", base_dir.to_string())
        yikes "security_violation"
    }
    
    // Additional security: check for suspicious patterns
    sus path_str tea = absolute_path.to_string()
    ready (path_str.contains("..") || path_str.contains("~") || path_str.contains("$")) {
        vibez.spill_error("Suspicious path pattern:", path_str)
        yikes "suspicious_path"
    }
    
    // Safe to read file
    sus content tea = pathz.read_file(absolute_path) fam {
        when _ -> yikes "read_error"
    }
    
    damn content
}
```

## Testing Strategy

### Unit Tests
**Why comprehensive path testing?** Path operations have many platform-specific edge cases and security implications that require thorough testing.

```cursed
// stdlib/pathz/test_pathz.💀
yeet "testz"
yeet "pathz"

slay test_path_construction() {
    sus simple_path pathz.Path = pathz.from_string("/home/user/file.txt") fam {
        when _ -> testz.fail("Simple path should be valid")
    }
    
    testz.assert_eq_string(simple_path.parent().to_string(), "/home/user")
    testz.assert_eq_string(simple_path.file_name(), "file.txt")
    testz.assert_eq_string(simple_path.extension(), ".txt")
    testz.assert_eq_string(simple_path.stem(), "file")
}

slay test_path_joining() {
    sus base pathz.Path = pathz.from_string("/home/user")
    sus joined pathz.Path = pathz.join(base.to_string(), "documents", "report.pdf") fam {
        when _ -> testz.fail("Path joining should succeed")
    }
    
    testz.assert_eq_string(joined.to_string(), "/home/user/documents/report.pdf")
}

slay test_path_traversal_protection() {
    sus base pathz.Path = pathz.from_string("/app/data")
    
    // These should be blocked
    sus malicious_inputs []tea = [
        "../../../etc/passwd",
        "..\\..\\..\\windows\\system32\\config\\sam",
        "./../../root/.ssh/id_rsa",
        "subdir/../../../sensitive.txt"
    ]
    
    bestie (sus malicious tea : malicious_inputs) {
        sus result pathz.Path = pathz.join(base.to_string(), malicious) fam {
            when "path_traversal" -> continue  // Expected error
            when _ -> testz.fail("Should detect path traversal")
        }
        testz.fail("Path traversal should be blocked:", malicious)
    }
}

slay test_unicode_normalization() {
    // These look identical but use different Unicode normalization
    sus nfc tea = "café.txt"  // NFC normalization  
    sus nfd tea = "café.txt"  // NFD normalization (decomposed)
    
    sus path1 pathz.Path = pathz.from_string(nfc)
    sus path2 pathz.Path = pathz.from_string(nfd)
    
    // Should be normalized to same form
    testz.assert_eq_string(path1.to_string(), path2.to_string())
}

slay test_cross_platform_paths() {
    sus windows_path tea = `C:\Users\Alice\Documents\file.txt`
    sus unix_path tea = "/home/alice/documents/file.txt"
    
    sus win_path pathz.Path = pathz.from_string(windows_path) fam {
        when _ -> testz.fail("Windows path should be valid")
    }
    
    sus nix_path pathz.Path = pathz.from_string(unix_path) fam {
        when _ -> testz.fail("Unix path should be valid")
    }
    
    // Convert between formats
    sus win_as_unix tea = pathz.to_unix_path(win_path)
    sus unix_as_win tea = pathz.to_windows_path(nix_path)
    
    testz.assert_true(win_as_unix.contains("/"))
    testz.assert_true(unix_as_win.contains("\\"))
}

slay test_path_patterns() {
    sus pattern pathz.PathPattern = pathz.create_pattern("*.{txt,md,csd}") fam {
        when _ -> testz.fail("Pattern should be valid")
    }
    
    testz.assert_true(pattern.matches(pathz.from_string("README.md")))
    testz.assert_true(pattern.matches(pathz.from_string("main.💀")))
    testz.assert_true(pattern.matches(pathz.from_string("notes.txt")))
    testz.assert_false(pattern.matches(pathz.from_string("binary.exe")))
}

slay test_sandboxing() {
    sus sandbox pathz.Sandbox = pathz.create_sandbox(pathz.from_string("/app/data"))
    
    // Should be allowed
    sus safe_path pathz.Path = sandbox.resolve_path("user/document.txt") fam {
        when _ -> testz.fail("Safe path should be resolved")
    }
    testz.assert_true(safe_path.to_string().starts_with("/app/data/"))
    
    // Should be blocked
    sus blocked pathz.Path = sandbox.resolve_path("../../../etc/passwd") fam {
        when "outside_sandbox" -> damn pathz.Path.from("")  // Expected
        when _ -> testz.fail("Should block path traversal")
    }
}

slay main() {
    testz.start_suite("pathz Tests")
    test_path_construction()
    test_path_joining()
    test_path_traversal_protection()
    test_unicode_normalization()
    test_cross_platform_paths()
    test_path_patterns()
    test_sandboxing()
    testz.print_summary()
}
```

### Integration Tests
```bash
# Test with real file system operations
./zig-out/bin/cursed-zig stdlib/pathz/integration_test.💀

# Memory safety testing
valgrind --leak-check=full ./zig-out/bin/cursed-zig stdlib/pathz/memory_test.💀

# Performance benchmarks
./zig-out/bin/cursed-zig stdlib/pathz/benchmark_test.💀

# Cross-platform testing
./zig-out/bin/cursed-zig stdlib/pathz/platform_test.💀
```

## Implementation Choices Explained

### Why Security-First Design?
**Problem**: Path handling is a common source of security vulnerabilities (directory traversal, symlink attacks, Unicode spoofing).

**Solution**: Every path operation includes security checks by default, with explicit opt-out for performance-critical code that has been manually audited.

### Why Unicode Normalization?
**Problem**: Different Unicode normalization forms cause identical-looking file names to be treated as different files, leading to user confusion and bugs.

**Solution**: Automatic normalization to NFC (Canonical Decomposition followed by Canonical Composition) ensures consistent behavior across all platforms.

### Why Stack Allocation Optimization?
**Problem**: Path operations in tight loops (directory traversal, file scanning) can create performance bottlenecks due to excessive heap allocation.

**Solution**: Stack-allocated path buffers for common cases, with automatic fallback to heap allocation for very long paths.

## Security Considerations

### Path Traversal Prevention
```cursed
// Built-in protection mechanisms:
// 1. Automatic detection of .. sequences
// 2. Symlink resolution with loop detection  
// 3. Absolute path normalization
// 4. Sandbox boundary enforcement
// 5. Unicode normalization to prevent spoofing
```

### File System Race Conditions
```cursed
// TOCTOU (Time-of-Check-Time-of-Use) attack prevention
slay secure_file_write(path pathz.Path, content tea) yikes<> {
    // Atomic check-and-write operation
    pathz.create_file_exclusive(path, content) fam {
        when "already_exists" -> {
            // File was created between check and write
            yikes "race_condition_detected"
        }
        when _ -> yikes error
    }
}
```

### Platform-Specific Security
```cursed
// Windows-specific security considerations
ready (pathz.get_platform() == pathz.Platform.Windows) {
    // Check for reserved names (CON, PRN, AUX, NUL, etc.)
    ready (pathz.is_reserved_name(file_name)) {
        yikes "reserved_filename"
    }
    
    // Check for alternate data streams
    ready (file_name.contains(":")) {
        yikes "alternate_data_stream_detected"
    }
}
```

## Migration Guide

### From Other Languages

#### From Node.js (path module)
```javascript
// Node.js
const path = require('path');
const joined = path.join('/home/user', 'documents', 'file.txt');
const ext = path.extname(joined);

// CURSED
yeet "pathz"
sus joined pathz.Path = pathz.join("/home/user", "documents", "file.txt")
sus ext tea = joined.extension()
```

#### From Python (pathlib)
```python
# Python
from pathlib import Path
p = Path("/home/user") / "documents" / "file.txt"
print(p.parent, p.name, p.suffix)

# CURSED
sus path pathz.Path = pathz.join("/home/user", "documents", "file.txt")
vibez.spill(path.parent().to_string(), path.file_name(), path.extension())
```

## Future Enhancements

### Planned Features
- **Async Path Operations**: Non-blocking file system operations
- **Path Compression**: Built-in support for compressed file paths
- **Network Paths**: SMB, NFS, and cloud storage path handling
- **Path Encryption**: Encrypted file name support

### Performance Improvements
- **SIMD Path Parsing**: Vector instruction acceleration for path processing
- **Memory Mapping**: Direct memory-mapped file operations
- **Batch Operations**: Optimize multiple path operations
- **Cache Integration**: LRU cache for path existence checks

---

The `pathz` module provides secure, cross-platform file system path handling with built-in protection against common vulnerabilities. Its design prioritizes security and correctness while maintaining the performance characteristics needed for high-throughput file operations.
