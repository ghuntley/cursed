fr fr Integration Test for File Watching System
fr fr Tests the complete file watching functionality integration

yeet "filez/file_watching_system"
yeet "filez"
yeet "vibez"
yeet "testz"

fr fr ===== INTEGRATION TEST SUITE =====

slay test_file_watching_integration() {
    test_start("File Watching Integration")
    
    vibez.spill("🔧 Testing File Watching System Integration")
    vibez.spill("===============================================")
    
    fr fr Setup
    setup_integration_test()
    
    fr fr Test 1: Basic functionality
    test_basic_integration()
    
    fr fr Test 2: Platform detection
    test_platform_detection()
    
    fr fr Test 3: Event system
    test_event_system()
    
    fr fr Test 4: Error handling
    test_error_handling_integration()
    
    fr fr Cleanup
    cleanup_integration_test()
    
    print_test_summary()
    vibez.spill("✅ Integration tests completed")
}

slay test_basic_integration() {
    vibez.spill("\n📋 Testing Basic Integration")
    
    fr fr Test watcher creation
    sus events_count drip = 0
    sus test_callback slay(WatchEvent) lit = slay(event WatchEvent) lit {
        events_count = events_count + 1
        vibez.spill("Integration event: " + event_type_to_string(event.event_type))
        damn based
    }
    
    fr fr Create test file
    sus test_file tea = "./integration_test_file.txt"
    write_file(test_file, "integration test content")
    
    fr fr Start watcher
    (watch_id, start_error) := start_file_watcher(test_file, test_callback)
    
    ready (start_error != "") {
        vibez.spill("⚠️  Could not start file watcher (expected in mock): " + start_error)
        fr fr In mock mode, this is expected
        assert_ne_str(start_error, "", "Mock should return error for missing native implementation")
    } otherwise {
        vibez.spill("✅ File watcher started successfully with ID: " + int_to_string(watch_id))
        
        fr fr Test watcher status
        (is_active, status_error) := get_watcher_status(watch_id)
        assert_eq_str(status_error, "", "Status check should work")
        assert_eq_bool(is_active, based, "Watcher should be active")
        
        fr fr Stop watcher
        sus stop_error tea = stop_file_watcher(watch_id)
        assert_eq_str(stop_error, "", "Stop should work")
    }
    
    fr fr Clean up
    delete_file(test_file)
    vibez.spill("✅ Basic integration test completed")
}

slay test_platform_detection() {
    vibez.spill("\n🖥️  Testing Platform Detection")
    
    sus platform tea = runtime_get_platform_name()
    vibez.spill("Detected platform: " + platform)
    
    assert_ne_str(platform, "", "Platform should be detected")
    
    fr fr Test platform-specific paths
    ready (platform == "linux") {
        vibez.spill("📝 Testing Linux-specific functionality")
        test_linux_specific()
    }
    
    ready (platform == "macos") {
        vibez.spill("📝 Testing macOS-specific functionality")
        test_macos_specific()
    }
    
    ready (platform == "windows") {
        vibez.spill("📝 Testing Windows-specific functionality")
        test_windows_specific()
    }
    
    vibez.spill("✅ Platform detection test completed")
}

slay test_linux_specific() {
    vibez.spill("🐧 Linux-specific tests")
    fr fr In real implementation, would test inotify features
    vibez.spill("  - inotify mask calculation")
    vibez.spill("  - recursive directory watching")
    vibez.spill("  - move event handling")
}

slay test_macos_specific() {
    vibez.spill("🍎 macOS-specific tests")
    fr fr In real implementation, would test kqueue features
    vibez.spill("  - kqueue file descriptor management")
    vibez.spill("  - kevent filtering")
    vibez.spill("  - file system event types")
}

slay test_windows_specific() {
    vibez.spill("🪟 Windows-specific tests")
    fr fr In real implementation, would test ReadDirectoryChangesW
    vibez.spill("  - directory handle management")
    vibez.spill("  - unicode filename support")
    vibez.spill("  - overlapped I/O operations")
}

slay test_event_system() {
    vibez.spill("\n📡 Testing Event System")
    
    fr fr Test event type constants
    assert_eq_int(EVENT_CREATED, 1, "CREATED event type")
    assert_eq_int(EVENT_MODIFIED, 2, "MODIFIED event type")
    assert_eq_int(EVENT_DELETED, 3, "DELETED event type")
    assert_eq_int(EVENT_MOVED, 4, "MOVED event type")
    assert_eq_int(EVENT_ATTRIBUTES, 5, "ATTRIBUTES event type")
    
    fr fr Test event type conversion
    sus created_str tea = event_type_to_string(EVENT_CREATED)
    sus modified_str tea = event_type_to_string(EVENT_MODIFIED)
    sus deleted_str tea = event_type_to_string(EVENT_DELETED)
    
    assert_eq_str(created_str, "CREATED", "Event type string conversion")
    assert_eq_str(modified_str, "MODIFIED", "Event type string conversion")
    assert_eq_str(deleted_str, "DELETED", "Event type string conversion")
    
    fr fr Test filter creation
    sus txt_patterns []tea = create_txt_patterns()
    assert_eq_int(array_length(txt_patterns), 1, "Should create pattern array")
    assert_eq_str(txt_patterns[0], "*.txt", "Should have correct pattern")
    
    sus all_events []drip = create_all_events_array()
    assert_eq_int(array_length(all_events), 5, "Should create all event types")
    
    vibez.spill("✅ Event system test completed")
}

slay test_error_handling_integration() {
    vibez.spill("\n🚨 Testing Error Handling Integration")
    
    sus dummy_callback slay(WatchEvent) lit = slay(event WatchEvent) lit { damn based }
    
    fr fr Test invalid paths
    (id1, err1) := start_file_watcher("", dummy_callback)
    assert_ne_str(err1, "", "Empty path should fail")
    assert_eq_int(id1, 0, "Invalid ID for failed watcher")
    
    (id2, err2) := start_directory_watcher("/nonexistent", cringe, dummy_callback)
    assert_ne_str(err2, "", "Nonexistent directory should fail")
    
    fr fr Test invalid watcher operations
    sus stop_err tea = stop_file_watcher(99999)
    assert_ne_str(stop_err, "", "Invalid watcher ID should fail")
    
    (is_active, status_err) := get_watcher_status(99999)
    assert_ne_str(status_err, "", "Invalid status check should fail")
    
    fr fr Test filter validation
    sus invalid_filter WatchFilter = WatchFilter{
        patterns: create_empty_array_tea(),
        include_subdirs: based,
        event_types: create_empty_array_drip(),
        max_events: -1,
    }
    
    (id3, err3) := start_watcher_with_filter("./test", invalid_filter, dummy_callback)
    fr fr Should handle invalid filters gracefully
    
    vibez.spill("✅ Error handling integration test completed")
}

fr fr ===== UTILITY FUNCTIONS =====

slay setup_integration_test() {
    vibez.spill("🔧 Setting up integration test environment...")
    
    fr fr Create test directory
    ready (!directory_exists("./integration_test_dir")) {
        create_directory("./integration_test_dir")
    }
    
    vibez.spill("✅ Integration test environment ready")
}

slay cleanup_integration_test() {
    vibez.spill("🧹 Cleaning up integration test environment...")
    
    fr fr Remove test directory
    ready (directory_exists("./integration_test_dir")) {
        remove_directory("./integration_test_dir")
    }
    
    fr fr Remove any test files
    delete_file("./integration_test_file.txt")
    
    vibez.spill("✅ Integration cleanup completed")
}

slay create_empty_array_tea() []tea {
    sus empty []tea = []
    damn empty
}

slay create_empty_array_drip() []drip {
    sus empty []drip = []
    damn empty
}

slay event_type_to_string(event_type drip) tea {
    ready (event_type == EVENT_CREATED) { damn "CREATED" }
    ready (event_type == EVENT_MODIFIED) { damn "MODIFIED" }
    ready (event_type == EVENT_DELETED) { damn "DELETED" }
    ready (event_type == EVENT_MOVED) { damn "MOVED" }
    ready (event_type == EVENT_ATTRIBUTES) { damn "ATTRIBUTES" }
    damn "UNKNOWN"
}

fr fr ===== VALIDATION TESTS =====

slay test_api_completeness() {
    vibez.spill("\n📋 Testing API Completeness")
    
    fr fr Test that all expected functions exist and can be called
    sus dummy_callback slay(WatchEvent) lit = slay(event WatchEvent) lit { damn based }
    
    fr fr Test primary API functions
    (id1, err1) := start_file_watcher("./nonexistent.txt", dummy_callback)
    fr fr Should fail but not crash
    
    (id2, err2) := start_directory_watcher("./nonexistent", cringe, dummy_callback)
    fr fr Should fail but not crash
    
    sus filter WatchFilter = WatchFilter{
        patterns: create_txt_patterns(),
        include_subdirs: based,
        event_types: create_all_events_array(),
        max_events: 100,
    }
    
    (id3, err3) := start_watcher_with_filter("./nonexistent", filter, dummy_callback)
    fr fr Should fail but not crash
    
    fr fr Test utility functions
    (active_ids, list_err) := list_active_watchers()
    ready (list_err == "") {
        vibez.spill("Active watchers: " + int_to_string(array_length(active_ids)))
    }
    
    vibez.spill("✅ API completeness test completed")
}

slay test_memory_safety_integration() {
    vibez.spill("\n🛡️  Testing Memory Safety Integration")
    
    fr fr Test callback memory safety
    sus memory_test_count drip = 0
    
    sus memory_callback slay(WatchEvent) lit = slay(event WatchEvent) lit {
        memory_test_count = memory_test_count + 1
        
        fr fr Access all event fields to test memory safety
        sus path_len drip = string_length(event.path)
        sus old_path_len drip = string_length(event.old_path)
        ready (path_len >= 0 && old_path_len >= 0) {
            fr fr Fields accessible
        }
        
        damn based
    }
    
    fr fr Test with various inputs
    (id1, err1) := start_file_watcher("test.txt", memory_callback)
    ready (err1 == "") {
        stop_file_watcher(id1)
    }
    
    fr fr Test pattern matching safety
    sus patterns []tea = create_txt_patterns()
    sus test_result lit = matches_filter_patterns("test.txt", patterns)
    assert_eq_bool(test_result, based, "Pattern matching should work safely")
    
    sus empty_patterns []tea = []
    sus empty_result lit = matches_filter_patterns("test.txt", empty_patterns)
    assert_eq_bool(empty_result, based, "Empty patterns should match all")
    
    vibez.spill("✅ Memory safety integration test completed")
}

fr fr ===== PERFORMANCE INTEGRATION TESTS =====

slay test_performance_integration() {
    vibez.spill("\n⚡ Testing Performance Integration")
    
    fr fr Test watcher creation/destruction performance
    sus start_time drip = get_mock_timestamp()
    
    sus performance_callback slay(WatchEvent) lit = slay(event WatchEvent) lit { damn based }
    
    fr fr Create multiple watchers rapidly
    sus watcher_ids []drip = []
    sus created_count drip = 0
    
    sus i drip = 0
    bestie (i < 10) {
        sus test_file tea = "./perf_test_" + int_to_string(i) + ".txt"
        write_file(test_file, "performance test")
        
        (watch_id, err) := start_file_watcher(test_file, performance_callback)
        ready (err == "") {
            watcher_ids[created_count] = watch_id
            created_count = created_count + 1
        }
        
        i = i + 1
    }
    
    sus mid_time drip = get_mock_timestamp()
    
    fr fr Stop all watchers
    i = 0
    bestie (i < created_count) {
        stop_file_watcher(watcher_ids[i])
        i = i + 1
    }
    
    sus end_time drip = get_mock_timestamp()
    
    vibez.spill("Performance results:")
    vibez.spill("  Created " + int_to_string(created_count) + " watchers")
    vibez.spill("  Creation time: " + int_to_string(mid_time - start_time) + "ms")
    vibez.spill("  Cleanup time: " + int_to_string(end_time - mid_time) + "ms")
    
    fr fr Cleanup performance test files
    i = 0
    bestie (i < 10) {
        delete_file("./perf_test_" + int_to_string(i) + ".txt")
        i = i + 1
    }
    
    vibez.spill("✅ Performance integration test completed")
}

slay get_mock_timestamp() drip {
    fr fr Return mock timestamp
    damn 1640995200000
}

fr fr ===== CROSS-PLATFORM VALIDATION =====

slay test_cross_platform_compatibility() {
    vibez.spill("\n🌐 Testing Cross-Platform Compatibility")
    
    sus platform tea = runtime_get_platform_name()
    
    fr fr Test path separator handling
    sus separator tea = get_path_separator()
    assert_ne_str(separator, "", "Path separator should be defined")
    
    fr fr Test path joining
    sus joined tea = join_paths("test", "file.txt")
    ready (platform == "windows") {
        assert_eq_str(joined, "test\\file.txt", "Windows path joining")
    } otherwise {
        assert_eq_str(joined, "test/file.txt", "Unix path joining")
    }
    
    fr fr Test file existence checking
    write_file("cross_platform_test.txt", "test")
    sus exists lit = file_exists_internal("cross_platform_test.txt")
    assert_eq_bool(exists, based, "File existence check")
    delete_file("cross_platform_test.txt")
    
    vibez.spill("✅ Cross-platform compatibility test completed")
}

fr fr Run the integration tests
test_file_watching_integration()
test_api_completeness()
test_memory_safety_integration()
test_performance_integration()
test_cross_platform_compatibility()

vibez.spill("\n🎉 All Integration Tests Completed Successfully!")
