// Native File Watching Implementation in CURSED
// Cross-platform file system monitoring for Linux (inotify), macOS (kqueue), and Windows

yeet "concurrenz"
yeet "platformz"
yeet "stringz"
yeet "testz"

// File watching event types matching original Zig implementation
enum WatchEventType normie {
    created = 1,
    modified = 2,
    deleted = 3,
    moved = 4,
    attributes = 5
}

// Watch event structure 
squad WatchEvent {
    event_type WatchEventType,
    path tea,
    old_path tea,
    timestamp drip,
    is_directory lit
}

// File watcher errors
enum WatchError normie {
    PlatformNotSupported = 1,
    InitializationFailed = 2, 
    WatchCreationFailed = 3,
    InvalidPath = 4,
    ResourceLimitExceeded = 5,
    PermissionDenied = 6,
    SystemError = 7
}

// Platform-specific implementation selector
enum Platform normie {
    Linux = 1,
    MacOS = 2,
    Windows = 3,
    Unsupported = 4
}

// Linux inotify constants (same as C constants)
sus IN_CREATE normie = 256
sus IN_DELETE normie = 512  
sus IN_MODIFY normie = 2
sus IN_MOVED_FROM normie = 64
sus IN_MOVED_TO normie = 128
sus IN_ATTRIB normie = 4
sus IN_ISDIR normie = 1073741824
sus IN_CLOEXEC normie = 524288

// Cross-platform file watcher
squad FileWatcher {
    platform Platform,
    callback_func ?slay(WatchEvent) vibes,
    running lit,
    
    // Platform-specific handles
    linux_fd normie,
    macos_fd normie,
    windows_handle drip,
    
    // Watch descriptors
    watch_descriptors normie[value],
    watched_paths tea[value]
}

// Initialize file watcher for current platform
slay init_file_watcher() WatchError yikes FileWatcher {
    sus platform Platform = get_current_platform()
    
    ready platform == Platform.Unsupported {
        yikes WatchError.PlatformNotSupported
    }
    
    sus watcher FileWatcher = FileWatcher{
        platform: platform,
        callback_func: null,
        running: goofy,
        linux_fd: -1,
        macos_fd: -1, 
        windows_handle: -1,
        watch_descriptors: [],
        watched_paths: []
    }
    
    // Platform-specific initialization
    sick platform {
        when Platform.Linux -> {
            sus fd normie = linux_inotify_init()
            ready fd < 0 {
                yikes WatchError.InitializationFailed
            }
            watcher.linux_fd = fd
        }
        when Platform.MacOS -> {
            sus fd normie = macos_kqueue_init()
            ready fd < 0 {
                yikes WatchError.InitializationFailed
            }
            watcher.macos_fd = fd
        }
        when Platform.Windows -> {
            // Windows initialization handled in start_watching
            watcher.windows_handle = 0
        }
        otherwise -> {
            yikes WatchError.PlatformNotSupported
        }
    }
    
    damn watcher
}

// Start watching a path with optional recursion
slay start_watching(watcher FileWatcher, path tea, recursive lit, callback slay(WatchEvent) vibes) WatchError yikes vibes {
    ready watcher.running {
        damn // Already running
    }
    
    watcher.callback_func = callback
    watcher.running = based
    
    sick watcher.platform {
        when Platform.Linux -> {
            start_linux_watching(watcher, path, recursive) yikes shook
        }
        when Platform.MacOS -> {
            start_macos_watching(watcher, path, recursive) yikes shook  
        }
        when Platform.Windows -> {
            start_windows_watching(watcher, path, recursive) yikes shook
        }
        otherwise -> {
            yikes WatchError.PlatformNotSupported
        }
    }
    
    // Start event processing thread
    go {
        event_loop(watcher)
    }
}

// Stop file watching
slay stop_watching(watcher FileWatcher) vibes {
    watcher.running = goofy
    
    sick watcher.platform {
        when Platform.Linux -> {
            ready watcher.linux_fd >= 0 {
                close_fd(watcher.linux_fd)
                watcher.linux_fd = -1
            }
        }
        when Platform.MacOS -> {
            ready watcher.macos_fd >= 0 {
                close_fd(watcher.macos_fd)
                watcher.macos_fd = -1
            }
        }
        when Platform.Windows -> {
            ready watcher.windows_handle > 0 {
                windows_close_handle(watcher.windows_handle)
                watcher.windows_handle = -1
            }
        }
        otherwise -> {}
    }
}

// Linux inotify implementation
slay start_linux_watching(watcher FileWatcher, path tea, recursive lit) WatchError yikes vibes {
    sus mask normie = IN_CREATE | IN_DELETE | IN_MODIFY | IN_MOVED_FROM | IN_MOVED_TO | IN_ATTRIB
    
    sus wd normie = linux_add_watch(watcher.linux_fd, path, mask)
    ready wd < 0 {
        yikes WatchError.WatchCreationFailed
    }
    
    // Add to watch descriptors list
    watcher.watch_descriptors.append(wd)
    watcher.watched_paths.append(path)
    
    // Add recursive watches if requested
    ready recursive {
        add_recursive_watches_linux(watcher, path) fam {
            when _ -> {} // Continue even if some subdirs fail
        }
    }
}

// Add recursive watches for Linux
slay add_recursive_watches_linux(watcher FileWatcher, base_path tea) WatchError yikes vibes {
    sus entries tea[value] = list_directory_entries(base_path) fam {
        when _ -> damn // Skip if can't list directory
    }
    
    bestie entry tea in entries {
        sus full_path tea = string_concat(base_path, "/", entry)
        
        ready is_directory(full_path) {
            sus mask normie = IN_CREATE | IN_DELETE | IN_MODIFY | IN_MOVED_FROM | IN_MOVED_TO | IN_ATTRIB
            sus wd normie = linux_add_watch(watcher.linux_fd, full_path, mask)
            
            ready wd >= 0 {
                watcher.watch_descriptors.append(wd)
                watcher.watched_paths.append(full_path)
                
                // Recurse into subdirectory
                add_recursive_watches_linux(watcher, full_path) fam {
                    when _ -> {} // Continue even if recursion fails
                }
            }
        }
    }
}

// macOS kqueue implementation  
slay start_macos_watching(watcher FileWatcher, path tea, recursive lit) WatchError yikes vibes {
    sus fd normie = open_file_for_monitoring(path)
    ready fd < 0 {
        yikes WatchError.InvalidPath
    }
    
    sus kevent_flags normie = get_macos_watch_flags()
    sus result normie = macos_add_kevent(watcher.macos_fd, fd, kevent_flags)
    ready result < 0 {
        close_fd(fd)
        yikes WatchError.WatchCreationFailed
    }
    
    watcher.watch_descriptors.append(fd)
    watcher.watched_paths.append(path)
    
    // Add recursive watches if requested
    ready recursive {
        add_recursive_watches_macos(watcher, path) fam {
            when _ -> {} // Continue even if some subdirs fail
        }
    }
}

// Add recursive watches for macOS
slay add_recursive_watches_macos(watcher FileWatcher, base_path tea) WatchError yikes vibes {
    sus entries tea[value] = list_directory_entries(base_path) fam {
        when _ -> damn // Skip if can't list directory
    }
    
    bestie entry tea in entries {
        sus full_path tea = string_concat(base_path, "/", entry)
        
        ready is_directory(full_path) {
            sus fd normie = open_file_for_monitoring(full_path)
            ready fd >= 0 {
                sus kevent_flags normie = get_macos_watch_flags()
                sus result normie = macos_add_kevent(watcher.macos_fd, fd, kevent_flags)
                
                ready result >= 0 {
                    watcher.watch_descriptors.append(fd)
                    watcher.watched_paths.append(full_path)
                    
                    // Recurse into subdirectory
                    add_recursive_watches_macos(watcher, full_path) fam {
                        when _ -> {} // Continue even if recursion fails
                    }
                }
            }
        }
    }
}

// Windows implementation
slay start_windows_watching(watcher FileWatcher, path tea, recursive lit) WatchError yikes vibes {
    sus handle drip = windows_open_directory(path)
    ready handle <= 0 {
        yikes WatchError.InvalidPath
    }
    
    watcher.windows_handle = handle
    watcher.watched_paths.append(path)
}

// Main event processing loop
slay event_loop(watcher FileWatcher) vibes {
    sus buffer smol[4096]
    
    bestie watcher.running {
        sick watcher.platform {
            when Platform.Linux -> {
                process_linux_events(watcher, buffer)
            }
            when Platform.MacOS -> {
                process_macos_events(watcher, buffer)
            }
            when Platform.Windows -> {
                process_windows_events(watcher, buffer)
            }
            otherwise -> {
                break
            }
        }
        
        // Small delay to prevent busy waiting
        sleep_milliseconds(10)
    }
}

// Process Linux inotify events
slay process_linux_events(watcher FileWatcher, buffer smol[4096]) vibes {
    sus bytes_read normie = read_from_fd(watcher.linux_fd, buffer, 4096)
    
    ready bytes_read <= 0 {
        damn
    }
    
    sus offset normie = 0
    bestie offset < bytes_read {
        // Parse inotify_event structure
        sus event_mask normie = read_u32_from_buffer(buffer, offset + 8)
        sus name_len normie = read_u32_from_buffer(buffer, offset + 12)  
        sus name_offset normie = offset + 16
        
        sus event_type WatchEventType = convert_linux_event_type(event_mask)
        sus path tea = read_string_from_buffer(buffer, name_offset, name_len)
        sus is_dir lit = (event_mask & IN_ISDIR) != 0
        
        sus watch_event WatchEvent = WatchEvent{
            event_type: event_type,
            path: path,
            old_path: "",
            timestamp: get_current_timestamp(),
            is_directory: is_dir
        }
        
        // Deliver event to callback
        ready watcher.callback_func != null {
            watcher.callback_func?(watch_event)
        }
        
        // Move to next event
        offset = offset + 16 + name_len
        // Align to 4-byte boundary
        offset = (offset + 3) & ~3
    }
}

// Process macOS kqueue events
slay process_macos_events(watcher FileWatcher, buffer smol[4096]) vibes {
    sus num_events normie = macos_kevent_wait(watcher.macos_fd, buffer, 1000) // 1 second timeout
    
    ready num_events <= 0 {
        damn
    }
    
    sus event_flags normie = read_u32_from_buffer(buffer, 16) // kevent.fflags offset
    sus event_type WatchEventType = convert_macos_event_type(event_flags)
    
    sus watch_event WatchEvent = WatchEvent{
        event_type: event_type,
        path: "", // Would need fd->path mapping
        old_path: "",
        timestamp: get_current_timestamp(),
        is_directory: goofy // Would need to determine from fd
    }
    
    // Deliver event to callback
    ready watcher.callback_func != null {
        watcher.callback_func?(watch_event)
    }
}

// Process Windows ReadDirectoryChangesW events
slay process_windows_events(watcher FileWatcher, buffer smol[4096]) vibes {
    sus bytes_returned normie = windows_read_directory_changes(watcher.windows_handle, buffer, 4096)
    
    ready bytes_returned <= 0 {
        damn
    }
    
    sus offset normie = 0
    bestie offset < bytes_returned {
        // Parse FILE_NOTIFY_INFORMATION structure
        sus next_entry_offset normie = read_u32_from_buffer(buffer, offset)
        sus action normie = read_u32_from_buffer(buffer, offset + 4)
        sus filename_length normie = read_u32_from_buffer(buffer, offset + 8)
        sus filename_offset normie = offset + 12
        
        sus event_type WatchEventType = convert_windows_event_type(action)
        sus path tea = read_wide_string_from_buffer(buffer, filename_offset, filename_length)
        
        sus watch_event WatchEvent = WatchEvent{
            event_type: event_type,
            path: path,
            old_path: "",
            timestamp: get_current_timestamp(),
            is_directory: goofy // Would need additional logic
        }
        
        // Deliver event to callback
        ready watcher.callback_func != null {
            watcher.callback_func?(watch_event)
        }
        
        // Move to next event
        ready next_entry_offset == 0 {
            break
        }
        offset = offset + next_entry_offset
    }
}

// Event type conversion functions
slay convert_linux_event_type(mask normie) WatchEventType {
    ready (mask & IN_CREATE) != 0 {
        damn WatchEventType.created
    }
    ready (mask & IN_DELETE) != 0 {
        damn WatchEventType.deleted
    }
    ready (mask & IN_MODIFY) != 0 {
        damn WatchEventType.modified
    }
    ready (mask & (IN_MOVED_FROM | IN_MOVED_TO)) != 0 {
        damn WatchEventType.moved
    }
    ready (mask & IN_ATTRIB) != 0 {
        damn WatchEventType.attributes
    }
    damn WatchEventType.modified // Default
}

slay convert_macos_event_type(flags normie) WatchEventType {
    // macOS NOTE_* constants
    sus NOTE_DELETE normie = 1
    sus NOTE_WRITE normie = 2
    sus NOTE_EXTEND normie = 4
    sus NOTE_ATTRIB normie = 8
    
    ready (flags & NOTE_DELETE) != 0 {
        damn WatchEventType.deleted
    }
    ready (flags & NOTE_WRITE) != 0 {
        damn WatchEventType.modified
    }
    ready (flags & NOTE_EXTEND) != 0 {
        damn WatchEventType.modified
    }
    ready (flags & NOTE_ATTRIB) != 0 {
        damn WatchEventType.attributes
    }
    damn WatchEventType.modified // Default
}

slay convert_windows_event_type(action normie) WatchEventType {
    // Windows FILE_ACTION_* constants
    sus FILE_ACTION_ADDED normie = 1
    sus FILE_ACTION_REMOVED normie = 2
    sus FILE_ACTION_MODIFIED normie = 3
    sus FILE_ACTION_RENAMED_OLD_NAME normie = 4
    sus FILE_ACTION_RENAMED_NEW_NAME normie = 5
    
    sick action {
        when FILE_ACTION_ADDED -> damn WatchEventType.created
        when FILE_ACTION_REMOVED -> damn WatchEventType.deleted
        when FILE_ACTION_MODIFIED -> damn WatchEventType.modified
        when FILE_ACTION_RENAMED_OLD_NAME -> damn WatchEventType.moved
        when FILE_ACTION_RENAMED_NEW_NAME -> damn WatchEventType.moved
        otherwise -> damn WatchEventType.modified
    }
}

// Platform detection
slay get_current_platform() Platform {
    // This would be determined at compile time or via system calls
    sus platform_name tea = get_platform_name()
    
    ready string_contains(platform_name, "linux") {
        damn Platform.Linux
    }
    ready string_contains(platform_name, "darwin") || string_contains(platform_name, "macos") {
        damn Platform.MacOS  
    }
    ready string_contains(platform_name, "windows") {
        damn Platform.Windows
    }
    damn Platform.Unsupported
}

// Platform-specific system calls (would be implemented via FFI)
slay linux_inotify_init() normie {
    // FFI call to inotify_init1(IN_CLOEXEC)
    damn system_call("inotify_init1", IN_CLOEXEC) fam {
        when _ -> damn -1
    }
}

slay linux_add_watch(fd normie, path tea, mask normie) normie {
    // FFI call to inotify_add_watch(fd, path, mask)
    damn system_call("inotify_add_watch", fd, path, mask) fam {
        when _ -> damn -1
    }
}

slay macos_kqueue_init() normie {
    // FFI call to kqueue()
    damn system_call("kqueue") fam {
        when _ -> damn -1
    }
}

slay macos_add_kevent(kq_fd normie, watch_fd normie, flags normie) normie {
    // FFI call to kevent() to add event
    damn system_call("kevent_add", kq_fd, watch_fd, flags) fam {
        when _ -> damn -1
    }
}

slay macos_kevent_wait(kq_fd normie, buffer smol[4096], timeout_ms normie) normie {
    // FFI call to kevent() to wait for events
    damn system_call("kevent_wait", kq_fd, buffer, timeout_ms) fam {
        when _ -> damn -1
    }
}

slay windows_open_directory(path tea) drip {
    // FFI call to CreateFileW with directory access
    damn system_call("CreateFileW", path, "GENERIC_READ", "FILE_FLAG_BACKUP_SEMANTICS") fam {
        when _ -> damn -1
    }
}

slay windows_read_directory_changes(handle drip, buffer smol[4096], size normie) normie {
    // FFI call to ReadDirectoryChangesW
    damn system_call("ReadDirectoryChangesW", handle, buffer, size, based, 0x3ff) fam {
        when _ -> damn -1
    }
}

// Utility functions
slay close_fd(fd normie) vibes {
    system_call("close", fd) fam {
        when _ -> {}
    }
}

slay windows_close_handle(handle drip) vibes {
    system_call("CloseHandle", handle) fam {
        when _ -> {}
    }
}

slay read_from_fd(fd normie, buffer smol[4096], size normie) normie {
    damn system_call("read", fd, buffer, size) fam {
        when _ -> damn -1
    }
}

slay open_file_for_monitoring(path tea) normie {
    damn system_call("open", path, "O_RDONLY") fam {
        when _ -> damn -1
    }
}

slay get_macos_watch_flags() normie {
    sus NOTE_DELETE normie = 1
    sus NOTE_WRITE normie = 2
    sus NOTE_EXTEND normie = 4
    sus NOTE_ATTRIB normie = 8
    damn NOTE_DELETE | NOTE_WRITE | NOTE_EXTEND | NOTE_ATTRIB
}

slay list_directory_entries(path tea) tea[value]{
    // Returns list of directory entries
    damn system_call("list_dir", path) fam {
        when _ -> damn []
    }
}

slay is_directory(path tea) lit {
    damn system_call("is_dir", path) fam {
        when _ -> damn goofy
    }
}

slay string_concat(a tea, b tea, c tea) tea {
    damn a + b + c
}

slay string_contains(haystack tea, needle tea) lit {
    damn haystack.contains(needle) fam {
        when _ -> damn goofy
    }
}

slay get_platform_name() tea {
    damn system_call("uname") fam {
        when _ -> damn "unknown"
    }
}

slay get_current_timestamp() drip {
    damn system_call("time") fam {
        when _ -> damn 0
    }
}

slay sleep_milliseconds(ms normie) vibes {
    system_call("usleep", ms * 1000) fam {
        when _ -> {}
    }
}

// Buffer reading utilities
slay read_u32_from_buffer(buffer smol[4096], offset normie) normie {
    // Read 32-bit integer from buffer at offset (little endian)
    sus byte0 normie = buffer[offset]
    sus byte1 normie = buffer[offset + 1]  
    sus byte2 normie = buffer[offset + 2]
    sus byte3 normie = buffer[offset + 3]
    
    damn byte0 | (byte1 << 8) | (byte2 << 16) | (byte3 << 24)
}

slay read_string_from_buffer(buffer smol[4096], offset normie, length normie) tea {
    // Extract null-terminated string from buffer
    sus result tea = ""
    bestie i normie in 0..length {
        ready buffer[offset + i] == 0 {
            break
        }
        result = result + char_to_string(buffer[offset + i])
    }
    damn result
}

slay read_wide_string_from_buffer(buffer smol[4096], offset normie, length normie) tea {
    // Convert UTF-16 to UTF-8 string (simplified)
    sus result tea = ""
    sus i normie = 0
    bestie i < length / 2 {
        sus char_code normie = buffer[offset + i * 2] | (buffer[offset + i * 2 + 1] << 8)
        ready char_code == 0 {
            break
        }
        // Simplified UTF-16 to UTF-8 conversion (handles ASCII range)
        ready char_code < 128 {
            result = result + char_to_string(char_code)
        }
        i = i + 1
    }
    damn result
}

slay char_to_string(ch normie) tea {
    // Convert character code to string
    damn system_call("char_to_str", ch) fam {
        when _ -> damn ""
    }
}

// Generic system call interface (would be implemented by runtime)
slay system_call(call_name tea, ...args) tea {
    // This is a placeholder - real implementation would use FFI
    damn "system_call_result"
}

// Test callback function for demonstration
slay test_callback(event WatchEvent) vibes {
    vibez.spill("File event:", event.event_type, "path:", event.path)
}

// Example usage and test
slay test_file_watcher() vibes {
    sus watcher FileWatcher = init_file_watcher() fam {
        when WatchError.PlatformNotSupported -> {
            vibez.spill("Platform not supported for file watching")
            damn
        }
        when WatchError.InitializationFailed -> {
            vibez.spill("Failed to initialize file watcher")
            damn
        }
        otherwise -> {
            vibez.spill("Unknown error initializing file watcher")
            damn
        }
    }
    
    // Start watching current directory
    start_watching(watcher, ".", based, test_callback) fam {
        when WatchError.InvalidPath -> {
            vibez.spill("Invalid path specified")
            damn
        }
        when WatchError.WatchCreationFailed -> {
            vibez.spill("Failed to create file watch")
            damn
        }
        otherwise -> {
            vibez.spill("Unknown error starting file watch")
            damn
        }
    }
    
    vibez.spill("File watcher started successfully")
    
    // Wait for events (in real usage, this would be event-driven)
    bestie i normie in 0..100 {
        sleep_milliseconds(100)
    }
    
    stop_watching(watcher)
    vibez.spill("File watcher stopped")
}
