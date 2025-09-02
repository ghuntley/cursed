// CURSED Native File Watching Implementation
// Migrated from Zig to pure CURSED with minimal FFI
// Cross-platform file system monitoring

yeet "platformz"
yeet "concurrenz"
yeet "timez"

// File watching events
sus WatchEventType tea = ready {
    | "created" -> 1
    | "modified" -> 2 
    | "deleted" -> 3
    | "moved" -> 4
    | "attributes" -> 5
}

// Watch event structure
squad WatchEvent {
    sus event_type drip
    sus path tea
    sus old_path tea  // For move events
    sus timestamp drip
    sus is_directory lit
}

// Error handling
sus WatchError tea = ready {
    | "platform_not_supported" -> "Platform not supported"
    | "initialization_failed" -> "Watcher initialization failed"
    | "watch_creation_failed" -> "Watch creation failed"
    | "invalid_path" -> "Invalid file path"
    | "resource_limit" -> "Resource limit exceeded"
    | "permission_denied" -> "Permission denied"
    | "system_error" -> "System error"
}

// Cross-platform file watcher
squad FileWatcher {
    sus platform_impl tea
    sus callback tea
    sus running lit
    sus watch_thread tea
}

// Platform-specific implementations using minimal FFI
slay get_platform() tea {
    sus os_name tea = platformz.get_os_name()
    ready (os_name == "linux") {
        damn "linux"
    } otherwise ready (os_name == "darwin") {
        damn "macos"
    } otherwise ready (os_name == "windows") {
        damn "windows"  
    } otherwise {
        damn "unsupported"
    }
}

// Linux implementation using inotify via FFI
slay linux_watcher_create(path tea, recursive lit) drip yikes<tea> {
    // Minimal FFI call to inotify_init1
    sus fd drip = platformz.syscall("inotify_init1", 0)
    ready (fd < 0) {
        yikes "Failed to create inotify instance"
    }
    
    // Add watch path
    sus flags drip = 0x00000001 | 0x00000002 | 0x00000008  // IN_CREATE | IN_MODIFY | IN_DELETE
    ready (recursive) {
        flags = flags | 0x01000000  // IN_ONLYDIR for directories
    }
    
    sus wd drip = platformz.syscall("inotify_add_watch", fd, path, flags)
    ready (wd < 0) {
        platformz.syscall("close", fd)
        yikes "Failed to add watch path"
    }
    
    damn fd
}

// macOS implementation using kqueue via FFI  
slay macos_watcher_create(path tea, recursive lit) drip yikes<tea> {
    // Minimal FFI call to kqueue
    sus kq drip = platformz.syscall("kqueue")
    ready (kq < 0) {
        yikes "Failed to create kqueue instance"
    }
    
    // Open file for monitoring
    sus fd drip = platformz.syscall("open", path, 0)  // O_RDONLY
    ready (fd < 0) {
        platformz.syscall("close", kq)
        yikes "Failed to open file for monitoring"
    }
    
    // Add kevent filter
    sus filter drip = -4  // EVFILT_VNODE
    sus flags drip = 0x0001 | 0x0020  // EV_ADD | EV_CLEAR
    sus fflags drip = 0x0002 | 0x0008 | 0x0001  // NOTE_WRITE | NOTE_DELETE | NOTE_EXTEND
    
    sus result drip = platformz.syscall("kevent_add", kq, fd, filter, flags, fflags)
    ready (result < 0) {
        platformz.syscall("close", fd)
        platformz.syscall("close", kq)
        yikes "Failed to add kevent filter"
    }
    
    damn kq
}

// Windows implementation using ReadDirectoryChangesW via FFI
slay windows_watcher_create(path tea, recursive lit) drip yikes<tea> {
    // Minimal FFI calls to Windows API
    sus handle drip = platformz.win32_call("CreateFileW", path, 0x00000001, 0x00000007, 0, 3, 0x02000000, 0)
    ready (handle == -1) {
        yikes "Failed to open directory handle"
    }
    
    // Create completion port for async I/O
    sus completion_port drip = platformz.win32_call("CreateIoCompletionPort", handle, 0, 0, 0)
    ready (completion_port == 0) {
        platformz.win32_call("CloseHandle", handle)
        yikes "Failed to create completion port"
    }
    
    damn handle
}

// Main file watcher interface
slay file_watcher_create() WatchError yikes<tea> {
    sus platform tea = get_platform()
    ready (platform == "unsupported") {
        yikes "Platform not supported"
    }
    
    sus watcher WatchError = {
        platform_impl: platform,
        callback: "",
        running: nah,
        watch_thread: ""
    }
    
    damn watcher
}

slay file_watcher_start(watcher WatchError, path tea, recursive lit, callback tea) yikes<tea> {
    ready (watcher.running) {
        damn  // Already running
    }
    
    watcher.callback = callback
    watcher.running = based
    
    // Create platform-specific watcher in separate goroutine
    go {
        sus handle drip = ready (watcher.platform_impl == "linux") {
            linux_watcher_create(path, recursive) fam {
                when _ -> {
                    watcher.running = nah
                    damn
                }
            }
        } otherwise ready (watcher.platform_impl == "macos") {
            macos_watcher_create(path, recursive) fam {
                when _ -> {
                    watcher.running = nah
                    damn
                }
            }
        } otherwise ready (watcher.platform_impl == "windows") {
            windows_watcher_create(path, recursive) fam {
                when _ -> {
                    watcher.running = nah
                    damn
                }
            }
        } otherwise {
            watcher.running = nah
            damn
        }
        
        // Event processing loop
        bestie (watcher.running) {
            sus event WatchEvent = ready (watcher.platform_impl == "linux") {
                process_linux_events(handle)
            } otherwise ready (watcher.platform_impl == "macos") {
                process_macos_events(handle)
            } otherwise ready (watcher.platform_impl == "windows") {
                process_windows_events(handle)
            } otherwise {
                sleep(100)  // Fallback polling
                continue
            }
            
            // Invoke callback with event
            ready (watcher.callback != "") {
                // Call user callback function
                invoke_callback(watcher.callback, event)
            }
        }
        
        // Cleanup
        ready (watcher.platform_impl == "linux") {
            platformz.syscall("close", handle)
        } otherwise ready (watcher.platform_impl == "macos") {
            platformz.syscall("close", handle)
        } otherwise ready (watcher.platform_impl == "windows") {
            platformz.win32_call("CloseHandle", handle)
        }
    }
}

slay file_watcher_stop(watcher WatchError) {
    watcher.running = nah
}

// Platform-specific event processing
slay process_linux_events(fd drip) WatchEvent {
    // Read inotify events via minimal FFI
    sus buffer drip[4096]
    sus bytes_read drip = platformz.syscall("read", fd, buffer, 4096)
    
    ready (bytes_read <= 0) {
        damn {event_type: 0, path: "", old_path: "", timestamp: 0, is_directory: nah}
    }
    
    // Parse inotify event structure
    sus mask drip = buffer[8]  // Event mask
    sus name_len drip = buffer[12]  // Name length
    sus name tea = extract_string(buffer, 16, name_len)
    
    sus event_type drip = ready (mask & 0x00000100) {  // IN_CREATE
        1  // created
    } otherwise ready (mask & 0x00000002) {  // IN_MODIFY
        2  // modified
    } otherwise ready (mask & 0x00000200) {  // IN_DELETE
        3  // deleted
    } otherwise ready (mask & 0x00000040) {  // IN_MOVED_FROM
        4  // moved
    } otherwise {
        5  // attributes
    }
    
    damn {
        event_type: event_type,
        path: name,
        old_path: "",
        timestamp: timez.get_timestamp(),
        is_directory: (mask & 0x40000000) != 0  // IN_ISDIR
    }
}

slay process_macos_events(kq drip) WatchEvent {
    // Wait for kevent via minimal FFI
    sus event drip[6]  // struct kevent
    sus result drip = platformz.syscall("kevent_wait", kq, event, 1, 1000)  // 1 second timeout
    
    ready (result <= 0) {
        damn {event_type: 0, path: "", old_path: "", timestamp: 0, is_directory: nah}
    }
    
    sus fflags drip = event[3]  // Event flags
    sus event_type drip = ready (fflags & 0x0002) {  // NOTE_WRITE
        2  // modified
    } otherwise ready (fflags & 0x0008) {  // NOTE_DELETE
        3  // deleted
    } otherwise ready (fflags & 0x0001) {  // NOTE_EXTEND
        1  // created (file extended)
    } otherwise {
        5  // attributes
    }
    
    damn {
        event_type: event_type,
        path: "monitored_file",  // kqueue monitors specific files
        old_path: "",
        timestamp: timez.get_timestamp(),
        is_directory: nah
    }
}

slay process_windows_events(handle drip) WatchEvent {
    // ReadDirectoryChangesW via minimal FFI
    sus buffer drip[8192]
    sus bytes_returned drip = 0
    
    sus result drip = platformz.win32_call("ReadDirectoryChangesW", 
        handle, buffer, 8192, 1,  // recursive = true
        0x00000001 | 0x00000002 | 0x00000008,  // FILE_NOTIFY_CHANGE flags
        &bytes_returned, 0, 0)
    
    ready (result == 0 | bytes_returned == 0) {
        damn {event_type: 0, path: "", old_path: "", timestamp: 0, is_directory: nah}
    }
    
    // Parse FILE_NOTIFY_INFORMATION structure
    sus action drip = buffer[1]  // Action field
    sus filename_len drip = buffer[2] // FileNameLength
    sus filename tea = extract_wide_string(buffer, 3, filename_len)
    
    sus event_type drip = ready (action == 1) {  // FILE_ACTION_ADDED
        1  // created
    } otherwise ready (action == 3) {  // FILE_ACTION_MODIFIED
        2  // modified
    } otherwise ready (action == 2) {  // FILE_ACTION_REMOVED
        3  // deleted
    } otherwise ready (action == 4 | action == 5) {  // FILE_ACTION_RENAMED
        4  // moved
    } otherwise {
        5  // attributes
    }
    
    damn {
        event_type: event_type,
        path: filename,
        old_path: "",
        timestamp: timez.get_timestamp(),
        is_directory: nah  // Windows provides file attributes separately
    }
}

// Utility functions
slay extract_string(buffer drip[value], offset drip, length drip) tea {
    // Extract null-terminated string from buffer
    sus result tea = ""
    bestie (length > 0) {
        sus char drip = buffer[offset]
        ready (char == 0) {
            break
        }
        result = result + char_to_string(char)
        offset = offset + 1
        length = length - 1
    }
    damn result
}

slay extract_wide_string(buffer drip[value], offset drip, length drip) tea {
    // Extract wide string (Windows WCHAR) from buffer
    sus result tea = ""
    sus pos drip = offset
    bestie (pos < offset + length) {
        sus wchar drip = buffer[pos] + (buffer[pos + 1] * 256)  // Little-endian
        ready (wchar == 0) {
            break
        }
        result = result + wchar_to_string(wchar)
        pos = pos + 2
    }
    damn result
}

slay char_to_string(char drip) tea {
    // Convert ASCII character to string
    ready (char >= 32 & char <= 126) {
        damn string_from_ascii(char)
    } otherwise {
        damn "?"
    }
}

slay wchar_to_string(wchar drip) tea {
    // Convert wide character to UTF-8 string
    ready (wchar <= 127) {
        damn string_from_ascii(wchar)
    } otherwise {
        damn utf16_to_utf8(wchar)
    }
}

slay invoke_callback(callback tea, event WatchEvent) {
    // Dynamic callback invocation
    // This would use CURSED's runtime reflection system
    vibez.spill("File event:", event.event_type, "path:", event.path)
}

// Export functions for CURSED runtime integration
export slay cursed_file_watcher_create() drip {
    sus watcher WatchError = file_watcher_create() fam {
        when _ -> damn -1
    }
    damn allocate_watcher_handle(watcher)
}

export slay cursed_file_watcher_start(handle drip, path_ptr drip, path_len drip, 
                                     recursive drip, callback_ptr drip) drip {
    sus path tea = ptr_to_string(path_ptr, path_len)
    sus watcher WatchError = get_watcher_from_handle(handle)
    
    file_watcher_start(watcher, path, recursive != 0, callback_ptr) fam {
        when _ -> damn -1
    }
    
    damn 0
}

export slay cursed_file_watcher_stop(handle drip) drip {
    sus watcher WatchError = get_watcher_from_handle(handle)
    file_watcher_stop(watcher)
    damn 0
}

export slay cursed_file_watcher_destroy(handle drip) drip {
    sus watcher WatchError = get_watcher_from_handle(handle)
    file_watcher_stop(watcher)
    deallocate_watcher_handle(handle)
    damn 0
}

// Handle management for C FFI compatibility
sus watcher_handles {}drip WatchError
sus next_handle drip = 1

slay allocate_watcher_handle(watcher WatchError) drip {
    watcher_handles[next_handle] = watcher
    sus handle drip = next_handle
    next_handle = next_handle + 1
    damn handle
}

slay get_watcher_from_handle(handle drip) WatchError {
    damn watcher_handles[handle]
}

slay deallocate_watcher_handle(handle drip) {
    delete watcher_handles[handle]
}

slay ptr_to_string(ptr drip, len drip) tea {
    // Convert C string pointer to CURSED string
    // This would use CURSED's FFI bridge
    damn platformz.ptr_to_string(ptr, len)
}
