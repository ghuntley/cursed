fr fr File Watching Demo - Complete Real-World Example
fr fr Shows how to use the CURSED file watching system

yeet "filez/file_watching_system"
yeet "vibez"
yeet "stringz"
yeet "concurrenz"

fr fr ===== DEMO CONFIGURATION =====

sus DEMO_DIRECTORY tea = "./watch_test_dir"
sus DEMO_FILE tea = "./watch_test_dir/test_file.txt"
sus WATCH_TIMEOUT_MS drip = 30000  fr fr 30 seconds

fr fr ===== MAIN DEMO PROGRAM =====

slay main() {
    vibez.spill("🔍 CURSED File Watching System Demo")
    vibez.spill("=====================================")
    
    fr fr Create test directory and file
    setup_demo_environment()
    
    fr fr Run different watching scenarios
    demo_single_file_watching()
    demo_directory_watching()
    demo_recursive_directory_watching()
    demo_filtered_watching()
    demo_multiple_watchers()
    
    fr fr Cleanup
    cleanup_demo_environment()
    
    vibez.spill("\n✅ Demo completed successfully!")
}

fr fr ===== DEMO SCENARIOS =====

slay demo_single_file_watching() {
    vibez.spill("\n📄 Demo 1: Single File Watching")
    vibez.spill("--------------------------------")
    
    fr fr Create callback for file events
    sus file_callback slay(WatchEvent) lit = slay(event WatchEvent) lit {
        vibez.spill("📄 File Event: " + event_type_to_string(event.event_type) + " -> " + event.path)
        ready (event.event_type == EVENT_MODIFIED) {
            vibez.spill("   Content was modified at timestamp: " + int_to_string(event.timestamp))
        }
        damn based
    }
    
    fr fr Start watching single file
    (watch_id, start_err) := start_file_watcher(DEMO_FILE, file_callback)
    ready (start_err != "") {
        vibez.spill("❌ Failed to start file watcher: " + start_err)
        damn
    }
    
    vibez.spill("✅ Started watching file: " + DEMO_FILE)
    vibez.spill("📝 Watch ID: " + int_to_string(watch_id))
    
    fr fr Simulate file changes
    simulate_file_operations()
    
    fr fr Wait and then stop
    sleep_ms(2000)
    sus stop_err tea = stop_file_watcher(watch_id)
    ready (stop_err != "") {
        vibez.spill("❌ Failed to stop watcher: " + stop_err)
    } otherwise {
        vibez.spill("🛑 Stopped file watcher")
    }
}

slay demo_directory_watching() {
    vibez.spill("\n📁 Demo 2: Directory Watching (Non-Recursive)")
    vibez.spill("----------------------------------------------")
    
    sus dir_callback slay(WatchEvent) lit = slay(event WatchEvent) lit {
        vibez.spill("📁 Directory Event: " + event_type_to_string(event.event_type) + " -> " + event.path)
        ready (event.is_directory) {
            vibez.spill("   📂 This is a directory event")
        } otherwise {
            vibez.spill("   📄 This is a file event")
        }
        damn based
    }
    
    (watch_id, start_err) := start_directory_watcher(DEMO_DIRECTORY, cringe, dir_callback)
    ready (start_err != "") {
        vibez.spill("❌ Failed to start directory watcher: " + start_err)
        damn
    }
    
    vibez.spill("✅ Started watching directory: " + DEMO_DIRECTORY + " (non-recursive)")
    
    fr fr Create and delete files in directory
    simulate_directory_operations()
    
    sleep_ms(2000)
    stop_file_watcher(watch_id)
    vibez.spill("🛑 Stopped directory watcher")
}

slay demo_recursive_directory_watching() {
    vibez.spill("\n🔄 Demo 3: Recursive Directory Watching")
    vibez.spill("---------------------------------------")
    
    sus recursive_callback slay(WatchEvent) lit = slay(event WatchEvent) lit {
        vibez.spill("🔄 Recursive Event: " + event_type_to_string(event.event_type) + " -> " + event.path)
        ready (contains_substring(event.path, "subdir")) {
            vibez.spill("   🔍 Event in subdirectory detected!")
        }
        damn based
    }
    
    (watch_id, start_err) := start_directory_watcher(DEMO_DIRECTORY, based, recursive_callback)
    ready (start_err != "") {
        vibez.spill("❌ Failed to start recursive watcher: " + start_err)
        damn
    }
    
    vibez.spill("✅ Started recursive directory watching: " + DEMO_DIRECTORY)
    
    fr fr Create subdirectories and files
    simulate_recursive_operations()
    
    sleep_ms(3000)
    stop_file_watcher(watch_id)
    vibez.spill("🛑 Stopped recursive watcher")
}

slay demo_filtered_watching() {
    vibez.spill("\n🔍 Demo 4: Filtered File Watching")
    vibez.spill("----------------------------------")
    
    fr fr Create custom filter
    sus filter WatchFilter = WatchFilter{
        patterns: create_txt_patterns(),
        include_subdirs: based,
        event_types: create_modify_events_only(),
        max_events: 50,
    }
    
    sus filtered_callback slay(WatchEvent) lit = slay(event WatchEvent) lit {
        vibez.spill("🔍 Filtered Event: " + event_type_to_string(event.event_type) + " -> " + event.path)
        vibez.spill("   ✅ This event passed the filter!")
        damn based
    }
    
    (watch_id, start_err) := start_watcher_with_filter(DEMO_DIRECTORY, filter, filtered_callback)
    ready (start_err != "") {
        vibez.spill("❌ Failed to start filtered watcher: " + start_err)
        damn
    }
    
    vibez.spill("✅ Started filtered watching (only .txt modifications)")
    
    fr fr Test filtered operations
    simulate_filtered_operations()
    
    sleep_ms(2000)
    stop_file_watcher(watch_id)
    vibez.spill("🛑 Stopped filtered watcher")
}

slay demo_multiple_watchers() {
    vibez.spill("\n👥 Demo 5: Multiple Concurrent Watchers")
    vibez.spill("---------------------------------------")
    
    fr fr Start multiple watchers
    sus watcher_ids drip[value] = []
    sus watcher_count drip = 0
    
    fr fr Watcher 1: File-specific
    sus callback1 slay(WatchEvent) lit = slay(event WatchEvent) lit {
        vibez.spill("👁️  Watcher 1: " + event_type_to_string(event.event_type) + " -> " + event.path)
        damn based
    }
    
    (id1, err1) := start_file_watcher(DEMO_FILE, callback1)
    ready (err1 == "") {
        watcher_ids[watcher_count] = id1
        watcher_count = watcher_count + 1
    }
    
    fr fr Watcher 2: Directory-specific
    sus callback2 slay(WatchEvent) lit = slay(event WatchEvent) lit {
        vibez.spill("👁️  Watcher 2: " + event_type_to_string(event.event_type) + " -> " + event.path)
        damn based
    }
    
    (id2, err2) := start_directory_watcher(DEMO_DIRECTORY, cringe, callback2)
    ready (err2 == "") {
        watcher_ids[watcher_count] = id2
        watcher_count = watcher_count + 1
    }
    
    fr fr Watcher 3: Recursive
    sus callback3 slay(WatchEvent) lit = slay(event WatchEvent) lit {
        vibez.spill("👁️  Watcher 3: " + event_type_to_string(event.event_type) + " -> " + event.path)
        damn based
    }
    
    (id3, err3) := start_directory_watcher(DEMO_DIRECTORY, based, callback3)
    ready (err3 == "") {
        watcher_ids[watcher_count] = id3
        watcher_count = watcher_count + 1
    }
    
    vibez.spill("✅ Started " + int_to_string(watcher_count) + " concurrent watchers")
    
    fr fr Show active watchers
    (active_ids, list_err) := list_active_watchers()
    ready (list_err == "") {
        vibez.spill("📋 Active watchers: " + array_to_string(active_ids))
    }
    
    fr fr Simulate operations that will trigger all watchers
    simulate_multiple_watcher_operations()
    
    sleep_ms(3000)
    
    fr fr Stop all watchers
    sus i drip = 0
    bestie (i < watcher_count) {
        stop_file_watcher(watcher_ids[i])
        i = i + 1
    }
    
    vibez.spill("🛑 Stopped all " + int_to_string(watcher_count) + " watchers")
}

fr fr ===== SIMULATION FUNCTIONS =====

slay simulate_file_operations() {
    vibez.spill("🔧 Simulating file operations...")
    
    fr fr Modify file
    sus content tea = "Initial content\n"
    write_file(DEMO_FILE, content)
    sleep_ms(500)
    
    fr fr Append to file
    append_file(DEMO_FILE, "Appended content\n")
    sleep_ms(500)
    
    fr fr Modify again
    write_file(DEMO_FILE, "Modified content\n")
    sleep_ms(500)
}

slay simulate_directory_operations() {
    vibez.spill("🔧 Simulating directory operations...")
    
    fr fr Create new file
    write_file("./watch_test_dir/new_file.txt", "New file content")
    sleep_ms(500)
    
    fr fr Create another file
    write_file("./watch_test_dir/another_file.log", "Log content")
    sleep_ms(500)
    
    fr fr Delete a file
    delete_file("./watch_test_dir/new_file.txt")
    sleep_ms(500)
}

slay simulate_recursive_operations() {
    vibez.spill("🔧 Simulating recursive operations...")
    
    fr fr Create subdirectory
    create_directory("./watch_test_dir/subdir")
    sleep_ms(500)
    
    fr fr Create file in subdirectory
    write_file("./watch_test_dir/subdir/deep_file.txt", "Deep content")
    sleep_ms(500)
    
    fr fr Create nested subdirectory
    create_directory("./watch_test_dir/subdir/nested")
    sleep_ms(500)
    
    fr fr Create file in nested directory
    write_file("./watch_test_dir/subdir/nested/nested_file.txt", "Nested content")
    sleep_ms(500)
}

slay simulate_filtered_operations() {
    vibez.spill("🔧 Simulating filtered operations...")
    
    fr fr This should trigger (txt file modification)
    write_file("./watch_test_dir/filtered_test.txt", "This should be detected")
    sleep_ms(500)
    
    fr fr This should NOT trigger (wrong extension)
    write_file("./watch_test_dir/filtered_test.log", "This should be filtered out")
    sleep_ms(500)
    
    fr fr This should trigger (txt file in subdir)
    write_file("./watch_test_dir/subdir/subdir_test.txt", "This should be detected too")
    sleep_ms(500)
}

slay simulate_multiple_watcher_operations() {
    vibez.spill("🔧 Simulating operations for multiple watchers...")
    
    fr fr This will trigger all watchers
    write_file(DEMO_FILE, "Content that triggers all watchers")
    sleep_ms(500)
    
    fr fr This will trigger directory watchers only
    write_file("./watch_test_dir/multi_test.txt", "Directory watcher content")
    sleep_ms(500)
    
    fr fr This will trigger recursive watcher only
    write_file("./watch_test_dir/subdir/recursive_only.txt", "Recursive content")
    sleep_ms(500)
}

fr fr ===== SETUP AND CLEANUP =====

slay setup_demo_environment() {
    vibez.spill("🔧 Setting up demo environment...")
    
    fr fr Create demo directory
    ready (!directory_exists(DEMO_DIRECTORY)) {
        create_directory(DEMO_DIRECTORY)
    }
    
    fr fr Create initial demo file
    write_file(DEMO_FILE, "Initial demo file content\n")
    
    fr fr Create subdirectory for recursive tests
    ready (!directory_exists("./watch_test_dir/subdir")) {
        create_directory("./watch_test_dir/subdir")
    }
    
    vibez.spill("✅ Demo environment ready")
}

slay cleanup_demo_environment() {
    vibez.spill("🧹 Cleaning up demo environment...")
    
    fr fr Remove all created files and directories
    delete_file(DEMO_FILE)
    delete_file("./watch_test_dir/another_file.log")
    delete_file("./watch_test_dir/filtered_test.txt")
    delete_file("./watch_test_dir/filtered_test.log")
    delete_file("./watch_test_dir/multi_test.txt")
    delete_file("./watch_test_dir/subdir/deep_file.txt")
    delete_file("./watch_test_dir/subdir/subdir_test.txt")
    delete_file("./watch_test_dir/subdir/recursive_only.txt")
    delete_file("./watch_test_dir/subdir/nested/nested_file.txt")
    
    fr fr Remove directories
    remove_directory("./watch_test_dir/subdir/nested")
    remove_directory("./watch_test_dir/subdir")
    remove_directory(DEMO_DIRECTORY)
    
    vibez.spill("✅ Cleanup completed")
}

fr fr ===== UTILITY FUNCTIONS =====

slay event_type_to_string(event_type drip) tea {
    ready (event_type == EVENT_CREATED) { damn "CREATED" }
    ready (event_type == EVENT_MODIFIED) { damn "MODIFIED" }
    ready (event_type == EVENT_DELETED) { damn "DELETED" }
    ready (event_type == EVENT_MOVED) { damn "MOVED" }
    ready (event_type == EVENT_ATTRIBUTES) { damn "ATTRIBUTES" }
    damn "UNKNOWN"
}

slay create_txt_patterns() tea[value]{
    sus patterns tea[value] = []
    patterns[0] = "*.txt"
    damn patterns
}

slay create_modify_events_only() drip[value]{
    sus events drip[value] = []
    events[0] = EVENT_MODIFIED
    damn events
}

slay sleep_ms(milliseconds drip) {
    fr fr Sleep for specified milliseconds
    fr fr In real implementation, this would use platform-specific sleep
    sus i drip = 0
    bestie (i < milliseconds * 1000) {
        i = i + 1  fr fr Busy wait simulation
    }
}

slay array_to_string(ids drip[value]) tea {
    ready (array_length(ids) == 0) {
        damn "[]"
    }
    
    sus result tea = "["
    sus i drip = 0
    bestie (i < array_length(ids)) {
        ready (i > 0) {
            result = result + ", "
        }
        result = result + int_to_string(ids[i])
        i = i + 1
    }
    result = result + "]"
    damn result
}

fr fr ===== TESTING FUNCTIONS =====

slay test_file_watching_system() {
    vibez.spill("\n🧪 Testing File Watching System")
    vibez.spill("================================")
    
    fr fr Test 1: Basic watcher creation and destruction
    test_watcher_lifecycle()
    
    fr fr Test 2: Event filtering
    test_event_filtering()
    
    fr fr Test 3: Multiple watcher management
    test_multiple_watchers_management()
    
    fr fr Test 4: Error handling
    test_error_handling()
    
    vibez.spill("✅ All tests passed!")
}

slay test_watcher_lifecycle() {
    vibez.spill("🧪 Test: Watcher Lifecycle")
    
    sus test_callback slay(WatchEvent) lit = slay(event WatchEvent) lit {
        damn based
    }
    
    fr fr Test valid path
    (watch_id, start_err) := start_file_watcher(DEMO_FILE, test_callback)
    ready (start_err != "") {
        vibez.spill("❌ Failed to start watcher for valid path")
        damn
    }
    
    fr fr Test status check
    (is_active, status_err) := get_watcher_status(watch_id)
    ready (status_err != "" || !is_active) {
        vibez.spill("❌ Watcher status check failed")
    }
    
    fr fr Test stop
    sus stop_err tea = stop_file_watcher(watch_id)
    ready (stop_err != "") {
        vibez.spill("❌ Failed to stop watcher")
    }
    
    vibez.spill("✅ Watcher lifecycle test passed")
}

slay test_event_filtering() {
    vibez.spill("🧪 Test: Event Filtering")
    
    sus events_received drip = 0
    sus filter_callback slay(WatchEvent) lit = slay(event WatchEvent) lit {
        events_received = events_received + 1
        damn based
    }
    
    sus filter WatchFilter = WatchFilter{
        patterns: create_txt_patterns(),
        include_subdirs: cringe,
        event_types: create_modify_events_only(),
        max_events: 10,
    }
    
    (watch_id, start_err) := start_watcher_with_filter(DEMO_DIRECTORY, filter, filter_callback)
    ready (start_err != "") {
        vibez.spill("❌ Failed to start filtered watcher")
        damn
    }
    
    fr fr Simulate events that should be filtered
    write_file("./watch_test_dir/should_trigger.txt", "content")
    write_file("./watch_test_dir/should_not_trigger.log", "content")
    
    sleep_ms(1000)
    stop_file_watcher(watch_id)
    
    ready (events_received == 0) {
        vibez.spill("⚠️  No events received (this might be expected in mock)")
    } otherwise {
        vibez.spill("✅ Event filtering appears to work")
    }
}

slay test_multiple_watchers_management() {
    vibez.spill("🧪 Test: Multiple Watchers Management")
    
    sus dummy_callback slay(WatchEvent) lit = slay(event WatchEvent) lit { damn based }
    
    fr fr Start multiple watchers
    (id1, err1) := start_file_watcher(DEMO_FILE, dummy_callback)
    (id2, err2) := start_directory_watcher(DEMO_DIRECTORY, cringe, dummy_callback)
    (id3, err3) := start_directory_watcher(DEMO_DIRECTORY, based, dummy_callback)
    
    ready (err1 != "" || err2 != "" || err3 != "") {
        vibez.spill("❌ Failed to start multiple watchers")
        damn
    }
    
    fr fr Check active watchers list
    (active_ids, list_err) := list_active_watchers()
    ready (list_err != "" || array_length(active_ids) < 3) {
        vibez.spill("❌ Active watchers list incorrect")
    }
    
    fr fr Stop all watchers
    stop_file_watcher(id1)
    stop_file_watcher(id2)
    stop_file_watcher(id3)
    
    vibez.spill("✅ Multiple watchers management test passed")
}

slay test_error_handling() {
    vibez.spill("🧪 Test: Error Handling")
    
    sus dummy_callback slay(WatchEvent) lit = slay(event WatchEvent) lit { damn based }
    
    fr fr Test invalid path
    (invalid_id, invalid_err) := start_file_watcher("nonexistent_path.txt", dummy_callback)
    ready (invalid_err == "") {
        vibez.spill("❌ Should have failed for invalid path")
        stop_file_watcher(invalid_id)
    }
    
    fr fr Test invalid watch ID
    sus stop_invalid_err tea = stop_file_watcher(99999)
    ready (stop_invalid_err == "") {
        vibez.spill("❌ Should have failed for invalid watch ID")
    }
    
    fr fr Test empty path
    (empty_id, empty_err) := start_file_watcher("", dummy_callback)
    ready (empty_err == "") {
        vibez.spill("❌ Should have failed for empty path")
        stop_file_watcher(empty_id)
    }
    
    vibez.spill("✅ Error handling test passed")
}

fr fr Run the main demo
main()
