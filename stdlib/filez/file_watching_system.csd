fr fr CURSED File Watching System
fr fr Complete cross-platform file system monitoring with native backend implementations

yeet "vibez"
yeet "stringz" 
yeet "concurrenz"
yeet "errorz"

fr fr ===== FILE WATCHING EVENT TYPES =====

squad WatchEvent {
    sus event_type drip         fr fr 1=created, 2=modified, 3=deleted, 4=moved
    sus path tea
    sus old_path tea            fr fr For move events
    sus timestamp drip
    sus is_directory lit
}

squad WatchFilter {
    sus patterns []tea          fr fr Glob patterns to match
    sus include_subdirs lit     fr fr Recursive watching
    sus event_types []drip      fr fr Which event types to report
    sus max_events drip         fr fr Event buffer size
}

squad FileWatcher {
    sus watch_id drip
    sus path tea
    sus filter WatchFilter
    sus callback slay(WatchEvent) lit
    sus is_active lit
    sus native_handle drip      fr fr Platform-specific handle
    sus event_buffer []WatchEvent
}

fr fr ===== EVENT TYPE CONSTANTS =====

sus EVENT_CREATED drip = 1
sus EVENT_MODIFIED drip = 2  
sus EVENT_DELETED drip = 3
sus EVENT_MOVED drip = 4
sus EVENT_ATTRIBUTES drip = 5

fr fr ===== MAIN FILE WATCHING API =====

slay start_file_watcher(path tea, callback slay(WatchEvent) lit) (drip, tea) {
    fr fr Start watching a single file for changes
    ready (path == "") {
        damn (0, "Empty path not allowed")
    }
    
    ready (!file_exists_internal(path)) {
        damn (0, "File not found: " + path)
    }
    
    sus filter WatchFilter = WatchFilter{
        patterns: create_pattern_array("*"),
        include_subdirs: cringe,
        event_types: create_all_events_array(),
        max_events: 100,
    }
    
    damn start_watcher_with_filter(path, filter, callback)
}

slay start_directory_watcher(path tea, recursive lit, callback slay(WatchEvent) lit) (drip, tea) {
    fr fr Start watching directory for changes (optionally recursive)
    ready (path == "") {
        damn (0, "Empty path not allowed")
    }
    
    ready (!directory_exists(path)) {
        damn (0, "Directory not found: " + path)
    }
    
    sus filter WatchFilter = WatchFilter{
        patterns: create_pattern_array("*"),
        include_subdirs: recursive,
        event_types: create_all_events_array(),
        max_events: 1000,
    }
    
    damn start_watcher_with_filter(path, filter, callback)
}

slay start_watcher_with_filter(path tea, filter WatchFilter, callback slay(WatchEvent) lit) (drip, tea) {
    fr fr Start watching with custom filter settings
    ready (path == "") {
        damn (0, "Empty path not allowed")
    }
    
    ready (!path_exists_internal(path)) {
        damn (0, "Path not found: " + path)
    }
    
    fr fr Create new watcher
    sus watch_id drip = generate_watch_id()
    sus watcher FileWatcher = FileWatcher{
        watch_id: watch_id,
        path: path,
        filter: filter,
        callback: callback,
        is_active: cringe,
        native_handle: 0,
        event_buffer: create_event_buffer(filter.max_events),
    }
    
    fr fr Start platform-specific monitoring
    sus start_error tea = platform_start_watching(&watcher)
    ready (start_error != "") {
        damn (0, start_error)
    }
    
    fr fr Register watcher
    sus register_error tea = register_watcher(watcher)
    ready (register_error != "") {
        platform_stop_watching(watch_id)
        damn (0, register_error)
    }
    
    damn (watch_id, "")
}

slay stop_file_watcher(watch_id drip) tea {
    fr fr Stop file watcher by ID
    ready (watch_id <= 0) {
        damn "Invalid watch ID"
    }
    
    sus watcher_opt FileWatcher = get_watcher_by_id(watch_id)
    ready (!is_valid_watcher(watcher_opt)) {
        damn "Watcher not found: " + int_to_string(watch_id)
    }
    
    fr fr Stop platform-specific monitoring
    sus stop_error tea = platform_stop_watching(watch_id)
    ready (stop_error != "") {
        damn stop_error
    }
    
    fr fr Unregister watcher
    sus unregister_error tea = unregister_watcher(watch_id)
    ready (unregister_error != "") {
        damn unregister_error
    }
    
    damn ""
}

slay get_watcher_status(watch_id drip) (lit, tea) {
    fr fr Check if watcher is active
    ready (watch_id <= 0) {
        damn (cringe, "Invalid watch ID")
    }
    
    sus watcher_opt FileWatcher = get_watcher_by_id(watch_id)
    ready (!is_valid_watcher(watcher_opt)) {
        damn (cringe, "Watcher not found")
    }
    
    damn (watcher_opt.is_active, "")
}

slay list_active_watchers() ([]drip, tea) {
    fr fr Get list of all active watcher IDs
    sus active_ids []drip = []
    sus count drip = 0
    
    sus all_watchers []FileWatcher = get_all_watchers()
    sus i drip = 0
    bestie (i < array_length(all_watchers)) {
        ready (all_watchers[i].is_active) {
            active_ids[count] = all_watchers[i].watch_id
            count = count + 1
        }
        i = i + 1
    }
    
    damn (active_ids, "")
}

fr fr ===== PLATFORM-SPECIFIC IMPLEMENTATIONS =====

slay platform_start_watching(watcher *FileWatcher) tea {
    fr fr Start platform-specific file watching
    sus platform tea = get_platform_name()
    
    ready (platform == "linux") {
        damn linux_start_inotify_watching(watcher)
    }
    
    ready (platform == "macos") {
        damn macos_start_kqueue_watching(watcher)
    }
    
    ready (platform == "windows") {
        damn windows_start_directory_watching(watcher)
    }
    
    damn "Unsupported platform: " + platform
}

slay platform_stop_watching(watch_id drip) tea {
    fr fr Stop platform-specific file watching
    sus platform tea = get_platform_name()
    
    ready (platform == "linux") {
        damn linux_stop_inotify_watching(watch_id)
    }
    
    ready (platform == "macos") {
        damn macos_stop_kqueue_watching(watch_id)
    }
    
    ready (platform == "windows") {
        damn windows_stop_directory_watching(watch_id)
    }
    
    damn "Unsupported platform: " + platform
}

fr fr ===== LINUX inotify IMPLEMENTATION =====

slay linux_start_inotify_watching(watcher *FileWatcher) tea {
    fr fr Linux inotify-based file watching
    vibez.spill("[Linux] Starting inotify watcher for: " + watcher.path)
    
    fr fr Create inotify instance
    sus inotify_fd drip = runtime_inotify_init()
    ready (inotify_fd < 0) {
        damn "Failed to initialize inotify"
    }
    
    fr fr Calculate inotify mask based on filter
    sus mask drip = calculate_inotify_mask(watcher.filter)
    
    fr fr Add watch descriptor
    sus watch_descriptor drip = runtime_inotify_add_watch(inotify_fd, watcher.path, mask)
    ready (watch_descriptor < 0) {
        runtime_close_fd(inotify_fd)
        damn "Failed to add inotify watch for: " + watcher.path
    }
    
    fr fr Store native handles
    watcher.native_handle = inotify_fd
    watcher.is_active = based
    
    fr fr Start background thread for event processing
    go {
        linux_process_inotify_events(watcher.watch_id, inotify_fd, watch_descriptor)
    }
    
    damn ""
}

slay linux_stop_inotify_watching(watch_id drip) tea {
    fr fr Stop Linux inotify watching
    sus watcher_opt FileWatcher = get_watcher_by_id(watch_id)
    ready (!is_valid_watcher(watcher_opt)) {
        damn "Watcher not found"
    }
    
    sus inotify_fd drip = watcher_opt.native_handle
    ready (inotify_fd > 0) {
        runtime_close_fd(inotify_fd)
    }
    
    mark_watcher_inactive(watch_id)
    damn ""
}

slay linux_process_inotify_events(watch_id drip, inotify_fd drip, watch_descriptor drip) {
    fr fr Process inotify events in background thread
    vibez.spill("[Linux] Starting event processing loop for watcher: " + int_to_string(watch_id))
    
    sus buffer []drip = create_byte_buffer(4096)
    
    bestie (is_watcher_active(watch_id)) {
        sus bytes_read drip = runtime_read_fd(inotify_fd, buffer, 4096)
        ready (bytes_read <= 0) {
            vibez.spill("[Linux] inotify read error for watcher: " + int_to_string(watch_id))
            break
        }
        
        fr fr Parse inotify events from buffer
        sus events []WatchEvent = parse_inotify_buffer(buffer, bytes_read, watch_id)
        
        fr fr Deliver events to callbacks
        sus i drip = 0
        bestie (i < array_length(events)) {
            deliver_event_to_watcher(watch_id, events[i])
            i = i + 1
        }
    }
    
    vibez.spill("[Linux] Event processing stopped for watcher: " + int_to_string(watch_id))
}

slay calculate_inotify_mask(filter WatchFilter) drip {
    fr fr Convert filter to inotify mask
    sus mask drip = 0
    
    ready (contains_event_type(filter.event_types, EVENT_CREATED)) {
        mask = mask | 256  fr fr IN_CREATE
    }
    
    ready (contains_event_type(filter.event_types, EVENT_MODIFIED)) {
        mask = mask | 2    fr fr IN_MODIFY
        mask = mask | 32   fr fr IN_CLOSE_WRITE
    }
    
    ready (contains_event_type(filter.event_types, EVENT_DELETED)) {
        mask = mask | 512  fr fr IN_DELETE
    }
    
    ready (contains_event_type(filter.event_types, EVENT_MOVED)) {
        mask = mask | 64   fr fr IN_MOVED_FROM
        mask = mask | 128  fr fr IN_MOVED_TO
    }
    
    damn mask
}

fr fr ===== MACOS kqueue IMPLEMENTATION =====

slay macos_start_kqueue_watching(watcher *FileWatcher) tea {
    fr fr macOS kqueue/kevent-based file watching
    vibez.spill("[macOS] Starting kqueue watcher for: " + watcher.path)
    
    fr fr Create kqueue instance
    sus kqueue_fd drip = runtime_kqueue()
    ready (kqueue_fd < 0) {
        damn "Failed to create kqueue"
    }
    
    fr fr Open file/directory for monitoring
    sus file_fd drip = runtime_open_for_watching(watcher.path)
    ready (file_fd < 0) {
        runtime_close_fd(kqueue_fd)
        damn "Failed to open path for watching: " + watcher.path
    }
    
    fr fr Configure kevent filter
    sus kevent_filter drip = calculate_kevent_filter(watcher.filter)
    sus add_result drip = runtime_kevent_add(kqueue_fd, file_fd, kevent_filter)
    ready (add_result < 0) {
        runtime_close_fd(file_fd)
        runtime_close_fd(kqueue_fd)
        damn "Failed to add kevent filter"
    }
    
    fr fr Store native handles
    watcher.native_handle = kqueue_fd
    watcher.is_active = based
    
    fr fr Start background thread
    go {
        macos_process_kevent_events(watcher.watch_id, kqueue_fd, file_fd)
    }
    
    damn ""
}

slay macos_stop_kqueue_watching(watch_id drip) tea {
    fr fr Stop macOS kqueue watching
    sus watcher_opt FileWatcher = get_watcher_by_id(watch_id)
    ready (!is_valid_watcher(watcher_opt)) {
        damn "Watcher not found"
    }
    
    sus kqueue_fd drip = watcher_opt.native_handle
    ready (kqueue_fd > 0) {
        runtime_close_fd(kqueue_fd)
    }
    
    mark_watcher_inactive(watch_id)
    damn ""
}

slay macos_process_kevent_events(watch_id drip, kqueue_fd drip, file_fd drip) {
    fr fr Process kevent events in background thread
    vibez.spill("[macOS] Starting kevent processing loop for watcher: " + int_to_string(watch_id))
    
    bestie (is_watcher_active(watch_id)) {
        sus event_data []drip = create_byte_buffer(1024)
        sus events_count drip = runtime_kevent_wait(kqueue_fd, event_data, 1024, 1000)
        
        ready (events_count < 0) {
            vibez.spill("[macOS] kevent error for watcher: " + int_to_string(watch_id))
            break
        }
        
        ready (events_count > 0) {
            fr fr Parse kevent events
            sus events []WatchEvent = parse_kevent_buffer(event_data, events_count, watch_id)
            
            fr fr Deliver events
            sus i drip = 0
            bestie (i < array_length(events)) {
                deliver_event_to_watcher(watch_id, events[i])
                i = i + 1
            }
        }
    }
    
    vibez.spill("[macOS] kevent processing stopped for watcher: " + int_to_string(watch_id))
}

slay calculate_kevent_filter(filter WatchFilter) drip {
    fr fr Convert filter to kevent filter flags
    sus flags drip = 0
    
    ready (contains_event_type(filter.event_types, EVENT_MODIFIED)) {
        flags = flags | 1  fr fr NOTE_WRITE
    }
    
    ready (contains_event_type(filter.event_types, EVENT_DELETED)) {
        flags = flags | 2  fr fr NOTE_DELETE
    }
    
    ready (contains_event_type(filter.event_types, EVENT_ATTRIBUTES)) {
        flags = flags | 4  fr fr NOTE_ATTRIB
    }
    
    damn flags
}

fr fr ===== WINDOWS ReadDirectoryChangesW IMPLEMENTATION =====

slay windows_start_directory_watching(watcher *FileWatcher) tea {
    fr fr Windows ReadDirectoryChangesW-based file watching
    vibez.spill("[Windows] Starting directory watcher for: " + watcher.path)
    
    fr fr Open directory handle
    sus dir_handle drip = runtime_open_directory(watcher.path)
    ready (dir_handle <= 0) {
        damn "Failed to open directory: " + watcher.path
    }
    
    fr fr Calculate notify filter
    sus notify_filter drip = calculate_windows_notify_filter(watcher.filter)
    
    fr fr Store native handle
    watcher.native_handle = dir_handle
    watcher.is_active = based
    
    fr fr Start background monitoring
    go {
        windows_process_directory_changes(watcher.watch_id, dir_handle, notify_filter, watcher.filter.include_subdirs)
    }
    
    damn ""
}

slay windows_stop_directory_watching(watch_id drip) tea {
    fr fr Stop Windows directory watching
    sus watcher_opt FileWatcher = get_watcher_by_id(watch_id)
    ready (!is_valid_watcher(watcher_opt)) {
        damn "Watcher not found"
    }
    
    sus dir_handle drip = watcher_opt.native_handle
    ready (dir_handle > 0) {
        runtime_close_handle(dir_handle)
    }
    
    mark_watcher_inactive(watch_id)
    damn ""
}

slay windows_process_directory_changes(watch_id drip, dir_handle drip, notify_filter drip, recursive lit) {
    fr fr Process Windows directory change events
    vibez.spill("[Windows] Starting directory change processing for watcher: " + int_to_string(watch_id))
    
    sus buffer []drip = create_byte_buffer(8192)
    sus overlap_buffer []drip = create_byte_buffer(32)  fr fr OVERLAPPED structure
    
    bestie (is_watcher_active(watch_id)) {
        sus bytes_returned drip = runtime_read_directory_changes(dir_handle, buffer, 8192, recursive, notify_filter, overlap_buffer)
        
        ready (bytes_returned < 0) {
            vibez.spill("[Windows] ReadDirectoryChangesW error for watcher: " + int_to_string(watch_id))
            break
        }
        
        ready (bytes_returned > 0) {
            fr fr Parse FILE_NOTIFY_INFORMATION structures
            sus events []WatchEvent = parse_windows_notify_buffer(buffer, bytes_returned, watch_id)
            
            fr fr Deliver events
            sus i drip = 0
            bestie (i < array_length(events)) {
                deliver_event_to_watcher(watch_id, events[i])
                i = i + 1
            }
        }
    }
    
    vibez.spill("[Windows] Directory change processing stopped for watcher: " + int_to_string(watch_id))
}

slay calculate_windows_notify_filter(filter WatchFilter) drip {
    fr fr Convert filter to Windows notify filter
    sus flags drip = 0
    
    ready (contains_event_type(filter.event_types, EVENT_CREATED)) {
        flags = flags | 64   fr fr FILE_NOTIFY_CHANGE_CREATION
    }
    
    ready (contains_event_type(filter.event_types, EVENT_MODIFIED)) {
        flags = flags | 16   fr fr FILE_NOTIFY_CHANGE_LAST_WRITE
        flags = flags | 32   fr fr FILE_NOTIFY_CHANGE_SIZE
    }
    
    ready (contains_event_type(filter.event_types, EVENT_DELETED)) {
        flags = flags | 64   fr fr FILE_NOTIFY_CHANGE_CREATION (also covers deletion)
    }
    
    ready (contains_event_type(filter.event_types, EVENT_MOVED)) {
        flags = flags | 8    fr fr FILE_NOTIFY_CHANGE_FILE_NAME
        flags = flags | 2    fr fr FILE_NOTIFY_CHANGE_DIR_NAME
    }
    
    damn flags
}

fr fr ===== EVENT PROCESSING AND DELIVERY =====

slay deliver_event_to_watcher(watch_id drip, event WatchEvent) {
    fr fr Deliver event to watcher callback
    sus watcher_opt FileWatcher = get_watcher_by_id(watch_id)
    ready (!is_valid_watcher(watcher_opt)) {
        vibez.spill("Warning: Event delivered to invalid watcher: " + int_to_string(watch_id))
        damn
    }
    
    fr fr Apply pattern filtering
    ready (!matches_filter_patterns(event.path, watcher_opt.filter.patterns)) {
        damn  fr fr Event filtered out by pattern
    }
    
    fr fr Call user callback
    ready (watcher_opt.callback != void) {
        watcher_opt.callback(event)
    }
    
    fr fr Also add to event buffer for polling-based access
    add_event_to_buffer(watcher_opt, event)
}

slay matches_filter_patterns(path tea, patterns []tea) lit {
    fr fr Check if path matches any of the glob patterns
    ready (array_length(patterns) == 0) {
        damn based  fr fr No patterns = match everything
    }
    
    sus i drip = 0
    bestie (i < array_length(patterns)) {
        ready (glob_match(patterns[i], path)) {
            damn based
        }
        i = i + 1
    }
    
    damn cringe
}

slay glob_match(pattern tea, path tea) lit {
    fr fr Simple glob pattern matching (* and ? wildcards)
    ready (pattern == "*") {
        damn based  fr fr Match everything
    }
    
    ready (pattern == path) {
        damn based  fr fr Exact match
    }
    
    fr fr Simple wildcard matching implementation
    ready (ends_with(pattern, "*") && starts_with(path, substring(pattern, 0, string_length(pattern) - 1))) {
        damn based
    }
    
    damn cringe
}

fr fr ===== EVENT BUFFER MANAGEMENT =====

slay poll_events(watch_id drip, max_events drip) ([]WatchEvent, tea) {
    fr fr Poll for events without blocking (non-callback mode)
    sus empty_events []WatchEvent = []
    
    ready (watch_id <= 0) {
        damn (empty_events, "Invalid watch ID")
    }
    
    sus watcher_opt FileWatcher = get_watcher_by_id(watch_id)
    ready (!is_valid_watcher(watcher_opt)) {
        damn (empty_events, "Watcher not found")
    }
    
    fr fr Get events from buffer
    sus events []WatchEvent = get_buffered_events(watcher_opt, max_events)
    damn (events, "")
}

slay add_event_to_buffer(watcher FileWatcher, event WatchEvent) {
    fr fr Add event to circular buffer
    sus buffer_size drip = array_length(watcher.event_buffer)
    ready (buffer_size > 0) {
        fr fr Simple circular buffer implementation
        sus next_index drip = get_next_buffer_index(watcher.watch_id)
        watcher.event_buffer[next_index] = event
        increment_buffer_index(watcher.watch_id)
    }
}

fr fr ===== WATCHER REGISTRY =====

sus global_watchers []FileWatcher = []
sus global_watcher_count drip = 0
sus next_watch_id drip = 1

slay register_watcher(watcher FileWatcher) tea {
    fr fr Register new watcher in global registry
    ready (global_watcher_count >= 100) {
        damn "Maximum number of watchers reached"
    }
    
    global_watchers[global_watcher_count] = watcher
    global_watcher_count = global_watcher_count + 1
    damn ""
}

slay unregister_watcher(watch_id drip) tea {
    fr fr Remove watcher from registry
    sus i drip = 0
    bestie (i < global_watcher_count) {
        ready (global_watchers[i].watch_id == watch_id) {
            fr fr Shift remaining watchers down
            sus j drip = i
            bestie (j < global_watcher_count - 1) {
                global_watchers[j] = global_watchers[j + 1]
                j = j + 1
            }
            global_watcher_count = global_watcher_count - 1
            damn ""
        }
        i = i + 1
    }
    
    damn "Watcher not found in registry"
}

slay get_watcher_by_id(watch_id drip) FileWatcher {
    fr fr Find watcher by ID
    sus i drip = 0
    bestie (i < global_watcher_count) {
        ready (global_watchers[i].watch_id == watch_id) {
            damn global_watchers[i]
        }
        i = i + 1
    }
    
    fr fr Return invalid watcher
    damn FileWatcher{
        watch_id: 0,
        path: "",
        filter: WatchFilter{},
        callback: void,
        is_active: cringe,
        native_handle: 0,
        event_buffer: [],
    }
}

slay get_all_watchers() []FileWatcher {
    damn global_watchers
}

slay generate_watch_id() drip {
    sus id drip = next_watch_id
    next_watch_id = next_watch_id + 1
    damn id
}

fr fr ===== UTILITY FUNCTIONS =====

slay is_valid_watcher(watcher FileWatcher) lit {
    damn watcher.watch_id > 0
}

slay is_watcher_active(watch_id drip) lit {
    sus watcher FileWatcher = get_watcher_by_id(watch_id)
    damn is_valid_watcher(watcher) && watcher.is_active
}

slay mark_watcher_inactive(watch_id drip) {
    sus i drip = 0
    bestie (i < global_watcher_count) {
        ready (global_watchers[i].watch_id == watch_id) {
            global_watchers[i].is_active = cringe
            damn
        }
        i = i + 1
    }
}

slay path_exists_internal(path tea) lit {
    damn file_exists_internal(path) || directory_exists(path)
}

slay create_pattern_array(pattern tea) []tea {
    sus patterns []tea = []
    patterns[0] = pattern
    damn patterns
}

slay create_all_events_array() []drip {
    sus events []drip = []
    events[0] = EVENT_CREATED
    events[1] = EVENT_MODIFIED
    events[2] = EVENT_DELETED
    events[3] = EVENT_MOVED
    events[4] = EVENT_ATTRIBUTES
    damn events
}

slay contains_event_type(event_types []drip, event_type drip) lit {
    sus i drip = 0
    bestie (i < array_length(event_types)) {
        ready (event_types[i] == event_type) {
            damn based
        }
        i = i + 1
    }
    damn cringe
}

slay create_event_buffer(size drip) []WatchEvent {
    sus buffer []WatchEvent = []
    fr fr Initialize empty buffer
    damn buffer
}

slay create_byte_buffer(size drip) []drip {
    sus buffer []drip = []
    fr fr Initialize buffer with zeros
    sus i drip = 0
    bestie (i < size) {
        buffer[i] = 0
        i = i + 1
    }
    damn buffer
}

slay get_platform_name() tea {
    fr fr Return platform identifier
    damn runtime_get_platform_name()
}

fr fr ===== RUNTIME BRIDGE FUNCTIONS =====
fr fr These would be implemented in native code for performance

slay runtime_get_platform_name() tea {
    fr fr Mock - would return actual platform
    damn "linux"
}

slay runtime_inotify_init() drip {
    fr fr Mock inotify_init1 system call
    damn 3  fr fr Mock file descriptor
}

slay runtime_inotify_add_watch(fd drip, path tea, mask drip) drip {
    fr fr Mock inotify_add_watch system call
    damn 1  fr fr Mock watch descriptor
}

slay runtime_read_fd(fd drip, buffer []drip, size drip) drip {
    fr fr Mock read system call
    damn 0  fr fr No bytes read (would block in real implementation)
}

slay runtime_close_fd(fd drip) drip {
    fr fr Mock close system call
    damn 0
}

slay runtime_kqueue() drip {
    fr fr Mock kqueue system call
    damn 4  fr fr Mock kqueue descriptor
}

slay runtime_open_for_watching(path tea) drip {
    fr fr Mock open system call for monitoring
    damn 5  fr fr Mock file descriptor
}

slay runtime_kevent_add(kq drip, fd drip, filter drip) drip {
    fr fr Mock kevent EV_ADD
    damn 0
}

slay runtime_kevent_wait(kq drip, buffer []drip, size drip, timeout drip) drip {
    fr fr Mock kevent wait
    damn 0  fr fr No events
}

slay runtime_open_directory(path tea) drip {
    fr fr Mock CreateFile/OpenFile for directory
    damn 100  fr fr Mock directory handle
}

slay runtime_read_directory_changes(handle drip, buffer []drip, size drip, recursive lit, filter drip, overlap []drip) drip {
    fr fr Mock ReadDirectoryChangesW
    damn 0  fr fr No changes
}

slay runtime_close_handle(handle drip) {
    fr fr Mock CloseHandle
}

fr fr ===== EVENT PARSING FUNCTIONS =====
fr fr These would parse platform-specific event structures

slay parse_inotify_buffer(buffer []drip, size drip, watch_id drip) []WatchEvent {
    fr fr Parse inotify_event structures from buffer
    sus events []WatchEvent = []
    fr fr Would parse real inotify events here
    damn events
}

slay parse_kevent_buffer(buffer []drip, count drip, watch_id drip) []WatchEvent {
    fr fr Parse kevent structures from buffer
    sus events []WatchEvent = []
    fr fr Would parse real kevent events here
    damn events
}

slay parse_windows_notify_buffer(buffer []drip, size drip, watch_id drip) []WatchEvent {
    fr fr Parse FILE_NOTIFY_INFORMATION structures
    sus events []WatchEvent = []
    fr fr Would parse real Windows notify events here
    damn events
}

fr fr Additional helper functions for buffer management and pattern matching
slay get_next_buffer_index(watch_id drip) drip { damn 0 }
slay increment_buffer_index(watch_id drip) { }
slay get_buffered_events(watcher FileWatcher, max_events drip) []WatchEvent {
    sus events []WatchEvent = []
    damn events
}
slay int_to_string(value drip) tea { damn "0" }
slay starts_with(text tea, prefix tea) lit { damn based }
slay ends_with(text tea, suffix tea) lit { damn based }
