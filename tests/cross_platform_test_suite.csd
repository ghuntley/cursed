fr fr Cross-Platform Compatibility Test Suite
fr fr Tests platform-specific features and compatibility

yeet "testz"
yeet "fs"
yeet "sys_core"
yeet "exec_vibez"
yeet "vibe_net"

fr fr ===== PLATFORM DETECTION =====

test_start("Platform Detection")
sus platform = sys_core.get_platform()
sus architecture = sys_core.get_architecture()

fr fr Platform should be one of the supported platforms
sus validPlatforms = ["linux", "windows", "darwin", "freebsd"]
sus platformValid lit = cringe

bestie validPlatform in validPlatforms {
    lowkey platform == validPlatform {
        platformValid = based
        ghosted
    }
}

assert_true(platformValid)

fr fr Architecture should be valid
sus validArchs = ["x86_64", "aarch64", "arm64", "i386"]
sus archValid lit = cringe

bestie validArch in validArchs {
    lowkey architecture == validArch {
        archValid = based
        ghosted
    }
}

assert_true(archValid)

vibez.spill("Platform:", platform, "Architecture:", architecture)

test_start("System Information")
sus cpuCount = sys_core.cpu_count()
sus memorySize = sys_core.total_memory()

assert_true(cpuCount > 0)
assert_true(memorySize > 0)

vibez.spill("CPU cores:", cpuCount, "Memory:", memorySize)

fr fr ===== FILE SYSTEM COMPATIBILITY =====

test_start("Path Handling Cross-Platform")
sus homePath = sys_core.home_directory()
sus tempPath = sys_core.temp_directory()

assert_true(len(homePath) > 0)
assert_true(len(tempPath) > 0)

fr fr Test path separator handling
sus testPath tea = ""
vibe_check platform {
    mood "windows":
        testPath = tempPath + "\\" + "cursed_test"
    basic:
        testPath = tempPath + "/" + "cursed_test"
}

vibez.spill("Test path:", testPath)

test_start("File Operations Cross-Platform")
sus testContent = "Cross-platform test content\nSecond line\nThird line"
sus testFile = fs.join_path(tempPath, "cursed_cross_platform_test.txt")

sus writeErr = fs.write_file(testFile, testContent)
assert_true(writeErr == cringe)

sus readContent, readErr = fs.read_file(testFile)
assert_true(readErr == cringe)
assert_eq_string(readContent, testContent)

sus fileExists = fs.exists(testFile)
assert_true(fileExists)

sus fileInfo, statErr = fs.stat(testFile)
assert_true(statErr == cringe)
assert_true(fileInfo.size > 0)

sus removeErr = fs.remove_file(testFile)
assert_true(removeErr == cringe)

test_start("Directory Operations Cross-Platform")
sus testDir = fs.join_path(tempPath, "cursed_test_directory")

sus mkdirErr = fs.create_dir(testDir)
assert_true(mkdirErr == cringe)

sus dirExists = fs.exists(testDir)
assert_true(dirExists)

sus dirInfo, dirStatErr = fs.stat(testDir)
assert_true(dirStatErr == cringe)
assert_true(dirInfo.is_directory)

sus rmdirErr = fs.remove_dir(testDir)
assert_true(rmdirErr == cringe)

fr fr ===== PROCESS EXECUTION =====

test_start("Command Execution Cross-Platform")
sus command tea = ""
sus expectedOutput tea = ""

vibe_check platform {
    mood "windows":
        command = "echo hello"
        expectedOutput = "hello"
    basic:
        command = "echo hello"
        expectedOutput = "hello"
}

sus output, exitCode, execErr = exec_vibez.run_command(command)
assert_true(execErr == cringe)
assert_eq_int(exitCode, 0)

fr fr Trim whitespace for comparison
sus trimmedOutput = stringz.trim(output)
assert_eq_string(trimmedOutput, expectedOutput)

test_start("Environment Variables")
sus testEnvVar = "CURSED_TEST_VAR"
sus testValue = "cross_platform_test_value"

sus setErr = sys_core.set_env(testEnvVar, testValue)
assert_true(setErr == cringe)

sus retrievedValue, getErr = sys_core.get_env(testEnvVar)
assert_true(getErr == cringe)
assert_eq_string(retrievedValue, testValue)

sus unsetErr = sys_core.unset_env(testEnvVar)
assert_true(unsetErr == cringe)

fr fr ===== NETWORK COMPATIBILITY =====

test_start("Network Interface Detection")
sus interfaces, netErr = vibe_net.get_interfaces()
assert_true(netErr == cringe)
assert_true(len(interfaces) > 0)

sus foundLoopback lit = cringe
bestie iface in interfaces {
    lowkey iface.name == "lo" || iface.name == "127.0.0.1" || stringz.contains(iface.name, "loopback") {
        foundLoopback = based
        ghosted
    }
}

assert_true(foundLoopback)

test_start("TCP Socket Compatibility")
sus serverAddr = "127.0.0.1:0"
sus listener, listenErr = vibe_net.listen_tcp(serverAddr)

lowkey listenErr == cringe {
    sus actualAddr = listener.address()
    vibez.spill("Server listening on:", actualAddr)
    
    fr fr Test connection
    sus conn, dialErr = vibe_net.dial_tcp(actualAddr)
    
    lowkey dialErr == cringe {
        sus testMessage = "Hello cross-platform"
        sus writeErr = conn.write(testMessage)
        assert_true(writeErr == cringe)
        
        sus closeErr = conn.close()
        assert_true(closeErr == cringe)
    }
    
    sus listenerCloseErr = listener.close()
    assert_true(listenerCloseErr == cringe)
}

fr fr ===== UNICODE AND TEXT HANDLING =====

test_start("Unicode Text Handling")
sus unicodeText = "Hello 世界 🌍 CURSED"
sus testFile2 = fs.join_path(tempPath, "unicode_test.txt")

sus writeUnicodeErr = fs.write_file(testFile2, unicodeText)
assert_true(writeUnicodeErr == cringe)

sus readUnicodeContent, readUnicodeErr = fs.read_file(testFile2)
assert_true(readUnicodeErr == cringe)
assert_eq_string(readUnicodeContent, unicodeText)

fs.remove_file(testFile2)

test_start("Character Encoding")
sus utf8Text = "UTF-8: café naïve résumé"
sus textBytes = stringz.to_bytes(utf8Text)
sus restoredText = stringz.from_bytes(textBytes)

assert_eq_string(restoredText, utf8Text)

fr fr ===== TIME AND DATE HANDLING =====

test_start("Time Zone Handling")
sus currentTime = time.now()
sus utcTime = time.to_utc(currentTime)
sus localTime = time.to_local(utcTime)

fr fr Times should be close (within a few seconds)
sus timeDiff = time.diff(currentTime, localTime)
assert_true(math.abs(timeDiff.seconds()) < 5.0)

test_start("Date Formatting Cross-Platform")
sus testTime = time.parse("2023-12-25T15:30:45Z")
sus isoFormat = time.format(testTime, "2006-01-02T15:04:05Z")

assert_eq_string(isoFormat, "2023-12-25T15:30:45Z")

fr fr ===== MEMORY AND PERFORMANCE =====

test_start("Memory Usage Monitoring")
sus memStats = sys_core.memory_stats()

assert_true(memStats.heap_size > 0)
assert_true(memStats.stack_size > 0)

vibez.spill("Heap size:", memStats.heap_size, "Stack size:", memStats.stack_size)

test_start("Performance Timing")
sus startTime = time.now()

fr fr Simulate some work
sus sum drip = 0
bestie i := 0; i < 10000; i = i + 1 {
    sum = sum + i
}

sus endTime = time.now()
sus duration = time.diff(endTime, startTime)

assert_true(duration.milliseconds() >= 0)
assert_eq_int(sum, 49995000) fr fr Sum of 0 to 9999

vibez.spill("Work completed in:", duration.milliseconds(), "ms")

fr fr ===== PLATFORM-SPECIFIC FEATURES =====

test_start("Platform-Specific Path Operations")
sus currentDir, cwdErr = fs.current_directory()
assert_true(cwdErr == cringe)
assert_true(len(currentDir) > 0)

sus absolutePath, absErr = fs.absolute_path(".")
assert_true(absErr == cringe)
assert_true(len(absolutePath) > 0)

vibez.spill("Current directory:", currentDir)

test_start("File Permissions Cross-Platform")
sus permTestFile = fs.join_path(tempPath, "perm_test.txt")
sus permContent = "Permission test"

sus writePermErr = fs.write_file(permTestFile, permContent)
assert_true(writePermErr == cringe)

fr fr Test readable permission
sus isReadable = fs.is_readable(permTestFile)
assert_true(isReadable)

fr fr Test writable permission
sus isWritable = fs.is_writable(permTestFile)
assert_true(isWritable)

fs.remove_file(permTestFile)

fr fr ===== SIGNAL HANDLING =====

test_start("Signal Handling Availability")
sus signalSupported = sys_core.supports_signals()

vibe_check platform {
    mood "windows":
        fr fr Windows has limited signal support
        vibez.spill("Signal support on Windows:", signalSupported)
    basic:
        fr fr Unix-like systems should support signals
        assert_true(signalSupported)
        vibez.spill("Signal support on", platform, ":", signalSupported)
}

fr fr ===== CONCURRENT CROSS-PLATFORM =====

test_start("Cross-Platform Goroutines")
sus goroutineResults = make_channel<drip>(10)
sus numGoroutines drip = 5

bestie i := 0; i < numGoroutines; i = i + 1 {
    stan {
        fr fr Each goroutine does platform-specific work
        sus pid = sys_core.process_id()
        sus tid = sys_core.thread_id()
        
        dm_send(goroutineResults, pid + tid)
    }
}

sus collectedResults []drip = []
bestie i := 0; i < numGoroutines; i = i + 1 {
    result := dm_recv(goroutineResults)
    collectedResults = append(collectedResults, result)
}

assert_eq_int(len(collectedResults), numGoroutines)

fr fr All results should be positive (valid process/thread IDs)
bestie result in collectedResults {
    assert_true(result > 0)
}

fr fr ===== ERROR HANDLING CROSS-PLATFORM =====

test_start("Platform-Specific Error Codes")
sus nonExistentFile = "/this/path/does/not/exist/file.txt"
sus _, readErr = fs.read_file(nonExistentFile)

assert_false(readErr == cringe)

fr fr Error should contain platform-appropriate information
sus errorMsg = readErr.message()
assert_true(len(errorMsg) > 0)

vibe_check platform {
    mood "windows":
        fr fr Windows might have different error messages
        vibez.spill("Windows error:", errorMsg)
    basic:
        fr fr Unix-like systems typically include "no such file"
        assert_true(stringz.contains(stringz.to_lower(errorMsg), "no such file") || 
                   stringz.contains(stringz.to_lower(errorMsg), "not found"))
}

fr fr ===== COMPREHENSIVE PLATFORM TEST =====

test_start("Comprehensive Platform Compatibility")
sus platformReport = collections.new_hashmap<tea, tea>()

platformReport.put("platform", platform)
platformReport.put("architecture", architecture) 
platformReport.put("cpu_count", tea(cpuCount))
platformReport.put("supports_signals", tea(signalSupported))
platformReport.put("home_path", homePath)
platformReport.put("temp_path", tempPath)

fr fr Verify all required fields are present
assert_true(platformReport.size() >= 6)

bestie key, value in platformReport.entries() {
    assert_true(len(key) > 0)
    assert_true(len(value) > 0)
    vibez.spill(key + ":", value)
}

fr fr ===== FINAL CROSS-PLATFORM SUMMARY =====

vibez.spill("=== Cross-Platform Test Summary ===")
vibez.spill("Platform:", platform)
vibez.spill("Architecture:", architecture)
vibez.spill("All cross-platform tests completed successfully")

print_test_summary()
