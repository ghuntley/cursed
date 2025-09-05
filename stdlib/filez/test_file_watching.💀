fr fr Comprehensive Test Suite for CURSED File Watching System
fr fr Tests all functionality including edge cases and error conditions

yeet "filez/file_watching_system"
yeet "filez"
yeet "vibez"
yeet "testz"

fr fr ===== TEST CONFIGURATION =====

sus TEST_DIR tea = "./test_watch_dir"
sus TEST_FILE tea = "./test_watch_dir/test.txt"
sus TEST_SUBDIR tea = "./test_watch_dir/subdir"
sus TEST_EVENTS_TIMEOUT drip = 5000

fr fr ===== MAIN TEST RUNNER =====

slay run_all_tests() {
    vibez.spill("🧪 Running CURSED File Watching System Tests")
    vibez.spill("=============================================")
    
    test_start("File Watching System")
    
    fr fr Setup test environment
    setup_test_environment()
    
    fr fr Core functionality tests
    test_basic_file_watching()
    test_directory_watching()
    test_recursive_watching()
    test_watcher_lifecycle()
    test_event_filtering()
    test_multiple_watchers()
    
    fr fr Error handling tests
    test_error_conditions()
    test_invalid_inputs()
    test_resource_management()
    
    fr fr Platform-specific tests
    test_platform_integration()
    test_performance_characteristics()
    
    fr fr Edge case tests
    test_edge_cases()
    test_concurrent_operations()
    test_memory_safety()
    
    fr fr Cleanup
    cleanup_test_environment()
    
    fr fr Print results
    print_test_summary()
}

fr fr ===== CORE FUNCTIONALITY TESTS =====

slay test_basic_file_watching() {
    vibez.spill("\n📄 Testing Basic File Watching")
    
    sus events_received drip = 0
    sus last_event_type drip = 0
    
    sus callback slay(WatchEvent) lit = slay(event WatchEvent) lit {
        events_received = events_received + 1
        last_event_type = event.event_type
        vibez.spill("Event received: " + event_type_to_string(event.event_type) + " on " + event.path)
        damn based
    }
    
    fr fr Test starting watcher
    (watch_id, start_err) := start_file_watcher(TEST_FILE, callback)
    assert_eq_str(start_err, "", "File watcher should start successfully")
    assert_gt_int(watch_id, 0, "Watch ID should be positive")
    
    fr fr Test file modifications
    write_file(TEST_FILE, "Test content 1")
    sleep_and_process_events(1000)
    
    write_file(TEST_FILE, "Test content 2")  
    sleep_and_process_events(1000)
    
    append_file(TEST_FILE, "\nAppended content")
    sleep_and_process_events(1000)
    
    fr fr Test watcher status
    (is_active, status_err) := get_watcher_status(watch_id)
    assert_eq_str(status_err, "", "Status check should succeed")
    assert_eq_bool(is_active, based, "Watcher should be active")
    
    fr fr Stop watcher
    sus stop_err tea = stop_file_watcher(watch_id)
    assert_eq_str(stop_err, "", "Watcher should stop successfully")
    
    fr fr Verify watcher is stopped
    (is_stopped, _) := get_watcher_status(watch_id)
    assert_eq_bool(is_stopped, cringe, "Watcher should be inactive after stop")
    
    vibez.spill("✅ Basic file watching test completed")
}

slay test_directory_watching() {
    vibez.spill("\n📁 Testing Directory Watching")
    
    sus dir_events drip = 0
    sus file_events drip = 0
    
    sus dir_callback slay(WatchEvent) lit = slay(event WatchEvent) lit {
        ready (event.is_directory) {
            dir_events = dir_events + 1
        } otherwise {
            file_events = file_events + 1
        }
        vibez.spill("Directory event: " + event_type_to_string(event.event_type) + " -> " + event.path)
        damn based
    }
    
    fr fr Start directory watcher (non-recursive)
    (watch_id, start_err) := start_directory_watcher(TEST_DIR, cringe, dir_callback)
    assert_eq_str(start_err, "", "Directory watcher should start")
    
    fr fr Create files in directory
    write_file(TEST_DIR + "/file1.txt", "Content 1")
    sleep_and_process_events(500)
    
    write_file(TEST_DIR + "/file2.log", "Content 2")
    sleep_and_process_events(500)
    
    fr fr Create subdirectory
    create_directory(TEST_DIR + "/newsubdir")
    sleep_and_process_events(500)
    
    fr fr Delete file
    delete_file(TEST_DIR + "/file1.txt")
    sleep_and_process_events(500)
    
    stop_file_watcher(watch_id)
    
    vibez.spill("Directory events: " + int_to_string(dir_events))
    vibez.spill("File events: " + int_to_string(file_events))
    vibez.spill("✅ Directory watching test completed")
}

slay test_recursive_watching() {
    vibez.spill("\n🔄 Testing Recursive Directory Watching")
    
    sus total_events drip = 0
    sus subdir_events drip = 0
    
    sus recursive_callback slay(WatchEvent) lit = slay(event WatchEvent) lit {
        total_events = total_events + 1
        ready (contains_substring(event.path, "subdir") || contains_substring(event.path, "deep")) {
            subdir_events = subdir_events + 1
        }
        vibez.spill("Recursive event: " + event_type_to_string(event.event_type) + " -> " + event.path)
        damn based
    }
    
    fr fr Start recursive watcher
    (watch_id, start_err) := start_directory_watcher(TEST_DIR, based, recursive_callback)
    assert_eq_str(start_err, "", "Recursive watcher should start")
    
    fr fr Create nested directory structure
    create_directory(TEST_DIR + "/level1")
    sleep_and_process_events(500)
    
    create_directory(TEST_DIR + "/level1/level2")
    sleep_and_process_events(500)
    
    fr fr Create files at different levels
    write_file(TEST_DIR + "/root_file.txt", "Root content")
    sleep_and_process_events(500)
    
    write_file(TEST_DIR + "/level1/level1_file.txt", "Level 1 content")
    sleep_and_process_events(500)
    
    write_file(TEST_DIR + "/level1/level2/deep_file.txt", "Deep content")
    sleep_and_process_events(500)
    
    stop_file_watcher(watch_id)
    
    vibez.spill("Total recursive events: " + int_to_string(total_events))
    vibez.spill("Subdirectory events: " + int_to_string(subdir_events))
    vibez.spill("✅ Recursive watching test completed")
}

slay test_watcher_lifecycle() {
    vibez.spill("\n♻️  Testing Watcher Lifecycle Management")
    
    sus dummy_callback slay(WatchEvent) lit = slay(event WatchEvent) lit { damn based }
    
    fr fr Test creating multiple watchers
    sus watcher_ids drip[value] = []
    sus count drip = 0
    
    fr fr Create 5 watchers
    sus i drip = 0
    bestie (i < 5) {
        sus test_file tea = TEST_DIR + "/lifecycle_test_" + int_to_string(i) + ".txt"
        write_file(test_file, "content")
        
        (watch_id, err) := start_file_watcher(test_file, dummy_callback)
        ready (err == "") {
            watcher_ids[count] = watch_id
            count = count + 1
        }
        i = i + 1
    }
    
    assert_eq_int(count, 5, "Should create 5 watchers")
    
    fr fr List active watchers
    (active_ids, list_err) := list_active_watchers()
    assert_eq_str(list_err, "", "Should list active watchers")
    assert_gte_int(array_length(active_ids), 5, "Should have at least 5 active watchers")
    
    fr fr Stop half the watchers
    sus stopped drip = 0
    i = 0
    bestie (i < count && i < 3) {
        sus err tea = stop_file_watcher(watcher_ids[i])
        ready (err == "") {
            stopped = stopped + 1
        }
        i = i + 1
    }
    
    assert_eq_int(stopped, 3, "Should stop 3 watchers")
    
    fr fr Stop remaining watchers
    i = 3
    bestie (i < count) {
        stop_file_watcher(watcher_ids[i])
        i = i + 1
    }
    
    vibez.spill("✅ Watcher lifecycle test completed")
}

slay test_event_filtering() {
    vibez.spill("\n🔍 Testing Event Filtering")
    
    sus txt_events drip = 0
    sus log_events drip = 0
    sus modify_events drip = 0
    sus other_events drip = 0
    
    sus filter_callback slay(WatchEvent) lit = slay(event WatchEvent) lit {
        ready (ends_with(event.path, ".txt")) {
            txt_events = txt_events + 1
        }
        ready (ends_with(event.path, ".log")) {
            log_events = log_events + 1
        }
        ready (event.event_type == EVENT_MODIFIED) {
            modify_events = modify_events + 1
        } otherwise {
            other_events = other_events + 1
        }
        vibez.spill("Filtered event: " + event.path)
        damn based
    }
    
    fr fr Create filter for .txt files and modify events only
    sus filter WatchFilter = WatchFilter{
        patterns: create_txt_patterns(),
        include_subdirs: based,
        event_types: create_modify_events_only(),
        max_events: 100,
    }
    
    (watch_id, start_err) := start_watcher_with_filter(TEST_DIR, filter, filter_callback)
    assert_eq_str(start_err, "", "Filtered watcher should start")
    
    fr fr Test different file types and operations
    write_file(TEST_DIR + "/test.txt", "Should trigger")
    sleep_and_process_events(500)
    
    write_file(TEST_DIR + "/test.log", "Should NOT trigger")
    sleep_and_process_events(500)
    
    write_file(TEST_DIR + "/another.txt", "Should trigger")
    sleep_and_process_events(500)
    
    delete_file(TEST_DIR + "/test.txt")  fr fr Delete should NOT trigger (wrong event type)
    sleep_and_process_events(500)
    
    stop_file_watcher(watch_id)
    
    vibez.spill("TXT events: " + int_to_string(txt_events))
    vibez.spill("LOG events: " + int_to_string(log_events))
    vibez.spill("Modify events: " + int_to_string(modify_events))
    vibez.spill("Other events: " + int_to_string(other_events))
    vibez.spill("✅ Event filtering test completed")
}

slay test_multiple_watchers() {
    vibez.spill("\n👥 Testing Multiple Concurrent Watchers")
    
    sus watcher1_events drip = 0
    sus watcher2_events drip = 0
    sus watcher3_events drip = 0
    
    sus callback1 slay(WatchEvent) lit = slay(event WatchEvent) lit {
        watcher1_events = watcher1_events + 1
        damn based
    }
    
    sus callback2 slay(WatchEvent) lit = slay(event WatchEvent) lit {
        watcher2_events = watcher2_events + 1
        damn based
    }
    
    sus callback3 slay(WatchEvent) lit = slay(event WatchEvent) lit {
        watcher3_events = watcher3_events + 1
        damn based
    }
    
    fr fr Start multiple watchers on overlapping paths
    (id1, err1) := start_file_watcher(TEST_FILE, callback1)
    (id2, err2) := start_directory_watcher(TEST_DIR, cringe, callback2)
    (id3, err3) := start_directory_watcher(TEST_DIR, based, callback3)
    
    assert_eq_str(err1, "", "Watcher 1 should start")
    assert_eq_str(err2, "", "Watcher 2 should start")
    assert_eq_str(err3, "", "Watcher 3 should start")
    
    fr fr Perform operations that should trigger multiple watchers
    write_file(TEST_FILE, "Multi-watcher test content")
    sleep_and_process_events(1000)
    
    write_file(TEST_DIR + "/multi_test.txt", "Directory test")
    sleep_and_process_events(1000)
    
    write_file(TEST_SUBDIR + "/nested_multi.txt", "Nested test")
    sleep_and_process_events(1000)
    
    fr fr Stop all watchers
    stop_file_watcher(id1)
    stop_file_watcher(id2)
    stop_file_watcher(id3)
    
    vibez.spill("Watcher 1 events: " + int_to_string(watcher1_events))
    vibez.spill("Watcher 2 events: " + int_to_string(watcher2_events))
    vibez.spill("Watcher 3 events: " + int_to_string(watcher3_events))
    vibez.spill("✅ Multiple watchers test completed")
}

fr fr ===== ERROR HANDLING TESTS =====

slay test_error_conditions() {
    vibez.spill("\n❌ Testing Error Conditions")
    
    sus dummy_callback slay(WatchEvent) lit = slay(event WatchEvent) lit { damn based }
    
    fr fr Test nonexistent file
    (id1, err1) := start_file_watcher("/nonexistent/path/file.txt", dummy_callback)
    assert_ne_str(err1, "", "Should fail for nonexistent file")
    assert_eq_int(id1, 0, "Should return invalid ID for failed watcher")
    
    fr fr Test nonexistent directory
    (id2, err2) := start_directory_watcher("/nonexistent/directory", cringe, dummy_callback)
    assert_ne_str(err2, "", "Should fail for nonexistent directory")
    
    fr fr Test invalid watch ID operations
    sus stop_err tea = stop_file_watcher(99999)
    assert_ne_str(stop_err, "", "Should fail to stop invalid watcher")
    
    (is_active, status_err) := get_watcher_status(99999)
    assert_ne_str(status_err, "", "Should fail status check for invalid watcher")
    assert_eq_bool(is_active, cringe, "Invalid watcher should not be active")
    
    vibez.spill("✅ Error conditions test completed")
}

slay test_invalid_inputs() {
    vibez.spill("\n🚫 Testing Invalid Input Handling")
    
    sus dummy_callback slay(WatchEvent) lit = slay(event WatchEvent) lit { damn based }
    
    fr fr Test empty paths
    (id1, err1) := start_file_watcher("", dummy_callback)
    assert_ne_str(err1, "", "Should fail for empty file path")
    
    (id2, err2) := start_directory_watcher("", cringe, dummy_callback)
    assert_ne_str(err2, "", "Should fail for empty directory path")
    
    fr fr Test invalid characters in path
    (id3, err3) := start_file_watcher("invalid\0path", dummy_callback)
    assert_ne_str(err3, "", "Should fail for path with null character")
    
    fr fr Test extremely long path
    sus long_path tea = create_long_path(1000)
    (id4, err4) := start_file_watcher(long_path, dummy_callback)
    fr fr This might succeed or fail depending on platform limits
    
    fr fr Test invalid filter patterns
    sus bad_filter WatchFilter = WatchFilter{
        patterns: create_empty_patterns(),
        include_subdirs: based,
        event_types: create_empty_events(),
        max_events: 0,
    }
    
    (id5, err5) := start_watcher_with_filter(TEST_DIR, bad_filter, dummy_callback)
    fr fr Should handle empty patterns gracefully
    ready (err5 == "") {
        stop_file_watcher(id5)
    }
    
    vibez.spill("✅ Invalid inputs test completed")
}

slay test_resource_management() {
    vibez.spill("\n💾 Testing Resource Management")
    
    sus dummy_callback slay(WatchEvent) lit = slay(event WatchEvent) lit { damn based }
    
    fr fr Test creating many watchers to check resource limits
    sus created_watchers drip[value] = []
    sus created_count drip = 0
    
    sus i drip = 0
    bestie (i < 50 && created_count < 50) {  fr fr Try to create up to 50 watchers
        sus test_file tea = TEST_DIR + "/resource_test_" + int_to_string(i) + ".txt"
        write_file(test_file, "content")
        
        (watch_id, err) := start_file_watcher(test_file, dummy_callback)
        ready (err == "") {
            created_watchers[created_count] = watch_id
            created_count = created_count + 1
        } otherwise {
            vibez.spill("Resource limit reached at watcher " + int_to_string(i) + ": " + err)
            break
        }
        
        i = i + 1
    }
    
    vibez.spill("Successfully created " + int_to_string(created_count) + " watchers")
    assert_gt_int(created_count, 0, "Should create at least some watchers")
    
    fr fr Clean up all created watchers
    i = 0
    bestie (i < created_count) {
        stop_file_watcher(created_watchers[i])
        i = i + 1
    }
    
    fr fr Verify no active watchers remain from this test
    (active_ids, _) := list_active_watchers()
    sus remaining_from_test drip = 0
    i = 0
    bestie (i < array_length(active_ids)) {
        sus j drip = 0
        bestie (j < created_count) {
            ready (active_ids[i] == created_watchers[j]) {
                remaining_from_test = remaining_from_test + 1
                break
            }
            j = j + 1
        }
        i = i + 1
    }
    
    assert_eq_int(remaining_from_test, 0, "All test watchers should be stopped")
    
    vibez.spill("✅ Resource management test completed")
}

fr fr ===== PLATFORM INTEGRATION TESTS =====

slay test_platform_integration() {
    vibez.spill("\n🖥️  Testing Platform Integration")
    
    sus platform tea = runtime_get_platform_name()
    vibez.spill("Testing on platform: " + platform)
    
    fr fr Test platform-specific features
    ready (platform == "linux") {
        test_linux_inotify_features()
    }
    
    ready (platform == "macos") {
        test_macos_kqueue_features()
    }
    
    ready (platform == "windows") {
        test_windows_readdir_features()
    }
    
    vibez.spill("✅ Platform integration test completed")
}

slay test_linux_inotify_features() {
    vibez.spill("🐧 Testing Linux inotify-specific features")
    
    sus events_with_details drip = 0
    sus move_events drip = 0
    
    sus linux_callback slay(WatchEvent) lit = slay(event WatchEvent) lit {
        events_with_details = events_with_details + 1
        ready (event.event_type == EVENT_MOVED) {
            move_events = move_events + 1
        }
        vibez.spill("Linux event: " + event_type_to_string(event.event_type))
        damn based
    }
    
    (watch_id, err) := start_directory_watcher(TEST_DIR, based, linux_callback)
    ready (err != "") {
        vibez.spill("⚠️  Cannot test Linux features: " + err)
        damn
    }
    
    fr fr Test move operations (specific to inotify)
    write_file(TEST_DIR + "/move_source.txt", "content")
    sleep_and_process_events(500)
    
    move_file(TEST_DIR + "/move_source.txt", TEST_DIR + "/move_dest.txt")
    sleep_and_process_events(1000)
    
    stop_file_watcher(watch_id)
    
    vibez.spill("Linux-specific events: " + int_to_string(events_with_details))
    vibez.spill("Move events: " + int_to_string(move_events))
}

slay test_macos_kqueue_features() {
    vibez.spill("🍎 Testing macOS kqueue-specific features")
    
    fr fr kqueue is good at file descriptor-based watching
    sus fd_events drip = 0
    
    sus macos_callback slay(WatchEvent) lit = slay(event WatchEvent) lit {
        fd_events = fd_events + 1
        vibez.spill("macOS event: " + event_type_to_string(event.event_type))
        damn based
    }
    
    (watch_id, err) := start_file_watcher(TEST_FILE, macos_callback)
    ready (err != "") {
        vibez.spill("⚠️  Cannot test macOS features: " + err)
        damn
    }
    
    fr fr Test rapid file modifications
    sus i drip = 0
    bestie (i < 5) {
        write_file(TEST_FILE, "macOS test content " + int_to_string(i))
        sleep_and_process_events(200)
        i = i + 1
    }
    
    stop_file_watcher(watch_id)
    vibez.spill("macOS fd events: " + int_to_string(fd_events))
}

slay test_windows_readdir_features() {
    vibez.spill("🪟 Testing Windows ReadDirectoryChangesW features")
    
    sus unicode_events drip = 0
    sus long_path_events drip = 0
    
    sus windows_callback slay(WatchEvent) lit = slay(event WatchEvent) lit {
        unicode_events = unicode_events + 1
        ready (string_length(event.path) > 50) {
            long_path_events = long_path_events + 1
        }
        vibez.spill("Windows event: " + event_type_to_string(event.event_type))
        damn based
    }
    
    (watch_id, err) := start_directory_watcher(TEST_DIR, based, windows_callback)
    ready (err != "") {
        vibez.spill("⚠️  Cannot test Windows features: " + err)
        damn
    }
    
    fr fr Test Unicode filename handling (Windows specialty)
    write_file(TEST_DIR + "/unicode_test_файл.txt", "unicode content")
    sleep_and_process_events(500)
    
    fr fr Test long path handling  
    sus long_name tea = create_long_filename(100)
    write_file(TEST_DIR + "/" + long_name, "long path content")
    sleep_and_process_events(500)
    
    stop_file_watcher(watch_id)
    vibez.spill("Unicode events: " + int_to_string(unicode_events))
    vibez.spill("Long path events: " + int_to_string(long_path_events))
}

slay test_performance_characteristics() {
    vibez.spill("\n⚡ Testing Performance Characteristics")
    
    sus start_time drip = get_timestamp()
    sus events_processed drip = 0
    
    sus perf_callback slay(WatchEvent) lit = slay(event WatchEvent) lit {
        events_processed = events_processed + 1
        damn based
    }
    
    (watch_id, err) := start_directory_watcher(TEST_DIR, based, perf_callback)
    assert_eq_str(err, "", "Performance test watcher should start")
    
    fr fr Create many files rapidly
    sus i drip = 0
    bestie (i < 100) {
        write_file(TEST_DIR + "/perf_test_" + int_to_string(i) + ".txt", "content " + int_to_string(i))
        ready (i % 10 == 0) {
            sleep_and_process_events(100)  fr fr Brief pause every 10 files
        }
        i = i + 1
    }
    
    sleep_and_process_events(2000)  fr fr Wait for all events to be processed
    stop_file_watcher(watch_id)
    
    sus end_time drip = get_timestamp()
    sus duration drip = end_time - start_time
    
    vibez.spill("Performance test results:")
    vibez.spill("  Duration: " + int_to_string(duration) + "ms")
    vibez.spill("  Events processed: " + int_to_string(events_processed))
    vibez.spill("  Events per second: " + int_to_string(events_processed * 1000 / duration))
    
    assert_gt_int(events_processed, 0, "Should process some events")
    vibez.spill("✅ Performance characteristics test completed")
}

fr fr ===== EDGE CASE TESTS =====

slay test_edge_cases() {
    vibez.spill("\n🔍 Testing Edge Cases")
    
    sus dummy_callback slay(WatchEvent) lit = slay(event WatchEvent) lit { damn based }
    
    fr fr Test watching a file that gets deleted
    sus temp_file tea = TEST_DIR + "/temp_delete_test.txt"
    write_file(temp_file, "temporary content")
    
    (watch_id, err) := start_file_watcher(temp_file, dummy_callback)
    assert_eq_str(err, "", "Should start watching file")
    
    delete_file(temp_file)  fr fr Delete the watched file
    sleep_and_process_events(1000)
    
    fr fr Watcher should handle file deletion gracefully
    (is_active, _) := get_watcher_status(watch_id)
    fr fr Behavior may vary by platform
    
    stop_file_watcher(watch_id)  fr fr Should not crash
    
    fr fr Test watching a directory that gets deleted
    sus temp_dir tea = TEST_DIR + "/temp_delete_dir"
    create_directory(temp_dir)
    
    (dir_watch_id, dir_err) := start_directory_watcher(temp_dir, cringe, dummy_callback)
    assert_eq_str(dir_err, "", "Should start watching directory")
    
    remove_directory(temp_dir)
    sleep_and_process_events(1000)
    
    stop_file_watcher(dir_watch_id)
    
    fr fr Test watching a file that gets replaced
    sus replace_test tea = TEST_DIR + "/replace_test.txt"
    write_file(replace_test, "original content")
    
    (replace_id, replace_err) := start_file_watcher(replace_test, dummy_callback)
    assert_eq_str(replace_err, "", "Should start watching file")
    
    delete_file(replace_test)
    write_file(replace_test, "replaced content")
    sleep_and_process_events(1000)
    
    stop_file_watcher(replace_id)
    
    vibez.spill("✅ Edge cases test completed")
}

slay test_concurrent_operations() {
    vibez.spill("\n🔄 Testing Concurrent Operations")
    
    sus concurrent_events drip = 0
    sus concurrent_callback slay(WatchEvent) lit = slay(event WatchEvent) lit {
        concurrent_events = concurrent_events + 1
        damn based
    }
    
    fr fr Start multiple watchers concurrently
    (id1, _) := start_directory_watcher(TEST_DIR, based, concurrent_callback)
    (id2, _) := start_directory_watcher(TEST_SUBDIR, cringe, concurrent_callback)
    
    fr fr Simulate concurrent file operations using goroutines
    go {
        sus i drip = 0
        bestie (i < 20) {
            write_file(TEST_DIR + "/concurrent_1_" + int_to_string(i) + ".txt", "content")
            sleep_and_process_events(50)
            i = i + 1
        }
    }
    
    go {
        sus i drip = 0
        bestie (i < 20) {
            write_file(TEST_SUBDIR + "/concurrent_2_" + int_to_string(i) + ".txt", "content")
            sleep_and_process_events(60)
            i = i + 1
        }
    }
    
    fr fr Wait for concurrent operations to complete
    sleep_and_process_events(5000)
    
    stop_file_watcher(id1)
    stop_file_watcher(id2)
    
    vibez.spill("Concurrent events processed: " + int_to_string(concurrent_events))
    assert_gt_int(concurrent_events, 0, "Should process concurrent events")
    vibez.spill("✅ Concurrent operations test completed")
}

slay test_memory_safety() {
    vibez.spill("\n🛡️  Testing Memory Safety")
    
    fr fr Test callback with event data lifecycle
    sus memory_events drip = 0
    sus memory_callback slay(WatchEvent) lit = slay(event WatchEvent) lit {
        memory_events = memory_events + 1
        fr fr Access all fields to test memory validity
        sus path_len drip = string_length(event.path)
        sus old_path_len drip = string_length(event.old_path)
        sus timestamp drip = event.timestamp
        sus is_dir lit = event.is_directory
        sus type drip = event.event_type
        
        ready (path_len > 0 && timestamp > 0) {
            fr fr Event data appears valid
        }
        damn based
    }
    
    (watch_id, err) := start_directory_watcher(TEST_DIR, based, memory_callback)
    assert_eq_str(err, "", "Memory safety test watcher should start")
    
    fr fr Generate events and immediately access data
    sus i drip = 0
    bestie (i < 10) {
        write_file(TEST_DIR + "/memory_test_" + int_to_string(i) + ".txt", "memory test content")
        sleep_and_process_events(100)
        i = i + 1
    }
    
    stop_file_watcher(watch_id)
    
    assert_gt_int(memory_events, 0, "Should receive memory-safe events")
    vibez.spill("✅ Memory safety test completed")
}

fr fr ===== TEST UTILITIES =====

slay setup_test_environment() {
    vibez.spill("🔧 Setting up test environment...")
    
    ready (!directory_exists(TEST_DIR)) {
        create_directory(TEST_DIR)
    }
    
    ready (!directory_exists(TEST_SUBDIR)) {
        create_directory(TEST_SUBDIR)
    }
    
    write_file(TEST_FILE, "Initial test content")
    
    vibez.spill("✅ Test environment ready")
}

slay cleanup_test_environment() {
    vibez.spill("🧹 Cleaning up test environment...")
    
    fr fr Remove all test files
    delete_file(TEST_FILE)
    
    fr fr Clean up any remaining test files
    (entries, _) := list_directory(TEST_DIR)
    ready (array_length(entries) > 0) {
        sus i drip = 0
        bestie (i < array_length(entries)) {
            sus entry_path tea = TEST_DIR + "/" + entries[i]
            ready (is_file(entry_path)) {
                delete_file(entry_path)
            } otherwise {
                remove_directory(entry_path)
            }
            i = i + 1
        }
    }
    
    remove_directory(TEST_SUBDIR)
    remove_directory(TEST_DIR)
    
    vibez.spill("✅ Test cleanup completed")
}

slay sleep_and_process_events(milliseconds drip) {
    fr fr Sleep and allow event processing
    sus i drip = 0
    bestie (i < milliseconds) {
        sus j drip = 0
        bestie (j < 1000) {
            j = j + 1  fr fr Busy wait
        }
        i = i + 1
    }
}

slay get_timestamp() drip {
    fr fr Return current timestamp in milliseconds
    damn 1640995200000  fr fr Mock timestamp
}

slay create_long_path(length drip) tea {
    sus path tea = TEST_DIR + "/"
    sus i drip = 0
    bestie (i < length) {
        path = path + "a"
        i = i + 1
    }
    path = path + ".txt"
    damn path
}

slay create_long_filename(length drip) tea {
    sus name tea = ""
    sus i drip = 0
    bestie (i < length) {
        name = name + "a"
        i = i + 1
    }
    name = name + ".txt"
    damn name
}

slay create_empty_patterns() tea[value]{
    sus patterns tea[value] = []
    damn patterns
}

slay create_empty_events() drip[value]{
    sus events drip[value] = []
    damn events
}

fr fr Run all tests
run_all_tests()
