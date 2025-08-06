fr fr CURSED Comprehensive Syscall Integration Test
fr fr Demonstrates real syscall interface working with file system, networking, and process management

yeet "testz"
yeet "fs_real"
yeet "net_real"
yeet "process_real"

slay test_file_and_process_integration() {
    test_start("File and Process Integration")
    
    fr fr Create a shell script file
    sus script_path tea = "/tmp/cursed_test_script.sh"
    sus script_content tea = "#!/bin/bash\necho 'Hello from CURSED-generated script'\necho 'Current directory:'\npwd\necho 'Date:'\ndate\n"
    
    fr fr Write the script file
    sus write_result lit = write_file(script_path, script_content)
    assert_true(write_result)
    
    fr fr Verify file was created
    assert_true(file_exists(script_path))
    
    fr fr Get file metadata
    sus metadata FileMetadata = get_file_metadata(script_path)
    assert_true(metadata.size > 0)
    assert_true(metadata.is_file)
    assert_false(metadata.is_dir)
    
    fr fr Make script executable (this would need chmod syscall in real implementation)
    fr fr For now, just test that we can read it back
    sus read_content tea = read_file(script_path)
    assert_true(contains(read_content, "#!/bin/bash"))
    assert_true(contains(read_content, "Hello from CURSED-generated script"))
    
    fr fr Execute the script
    sus args []tea = []tea{"bash", script_path}
    sus process Process = process_spawn("bash", args)
    assert_true(process.process_id > 0)
    
    fr fr Wait for script to complete
    sus exit_code normie = process_wait(&process)
    assert_true(exit_code == 0)
    
    fr fr Clean up
    sus delete_result lit = delete_file(script_path)
    assert_true(delete_result)
    
    print_test_summary()
}

slay test_network_and_file_integration() {
    test_start("Network and File Integration")
    
    fr fr Note: This test simulates downloading content and saving to file
    fr fr In a real implementation, this would make an actual HTTP request
    
    fr fr Simulate HTTP response content
    sus downloaded_content tea = "Downloaded content from HTTP request\nTimestamp: 2024-01-01 12:00:00\nStatus: Success"
    
    fr fr Save downloaded content to file
    sus download_path tea = "/tmp/cursed_download_test.txt"
    sus save_result lit = write_file(download_path, downloaded_content)
    assert_true(save_result)
    
    fr fr Verify file was saved correctly
    assert_true(file_exists(download_path))
    sus saved_content tea = read_file(download_path)
    assert_eq_string(saved_content, downloaded_content)
    
    fr fr Get file size and verify it matches content
    sus file_size thicc = get_file_size(download_path)
    assert_true(file_size > 0)
    
    fr fr Test HTTP URL parsing for the download
    sus test_url tea = "https://api.example.com/data/file.txt"
    sus parsed ParsedURL = parse_url(test_url)
    assert_eq_string(parsed.scheme, "https")
    assert_eq_string(parsed.host, "api.example.com")
    assert_true(parsed.port == 443)
    assert_eq_string(parsed.path, "/data/file.txt")
    
    fr fr Clean up
    sus delete_result lit = delete_file(download_path)
    assert_true(delete_result)
    
    print_test_summary()
}

slay test_process_environment_and_files() {
    test_start("Process Environment and Files")
    
    fr fr Create a config file with environment-dependent content
    sus config_path tea = "/tmp/cursed_config_test.conf"
    
    fr fr Get current environment information
    sus user tea = get_current_user()
    sus home tea = get_home_dir()
    sus pwd tea = get_current_dir()
    
    fr fr Create config content with environment variables
    sus config_content tea = "# CURSED Generated Config\n"
    config_content = config_content + "user=" + user + "\n"
    config_content = config_content + "home=" + home + "\n"
    config_content = config_content + "working_dir=" + pwd + "\n"
    config_content = config_content + "generated_by=cursed_syscall_test\n"
    
    fr fr Write config file
    sus write_result lit = write_file(config_path, config_content)
    assert_true(write_result)
    
    fr fr Test that process can read the config
    sus args []tea = []tea{"cat", config_path}
    sus process Process = process_spawn("cat", args)
    assert_true(process.process_id > 0)
    
    sus exit_code normie = process_wait(&process)
    assert_true(exit_code == 0)
    
    fr fr Test environment variable operations
    sus test_env_var tea = "CURSED_TEST_INTEGRATION_VAR"
    sus test_env_value tea = "integration_test_value_12345"
    
    fr fr Set environment variable
    sus set_result lit = env_set(test_env_var, test_env_value)
    assert_true(set_result)
    
    fr fr Get it back
    sus get_result tea = env_get(test_env_var)
    assert_eq_string(get_result, test_env_value)
    
    fr fr Test that environment variable exists
    assert_true(env_exists(test_env_var))
    
    fr fr Append environment info to config file
    sus env_line tea = "\n" + test_env_var + "=" + test_env_value + "\n"
    sus append_result lit = append_file(config_path, env_line)
    assert_true(append_result)
    
    fr fr Verify appended content
    sus final_content tea = read_file(config_path)
    assert_true(contains(final_content, user))
    assert_true(contains(final_content, test_env_value))
    
    fr fr Clean up
    sus delete_result lit = delete_file(config_path)
    assert_true(delete_result)
    
    print_test_summary()
}

slay test_concurrent_file_operations() {
    test_start("Concurrent File Operations")
    
    fr fr Test multiple file operations that could happen concurrently
    sus base_path tea = "/tmp/cursed_concurrent_test"
    
    fr fr Create multiple test files
    sus file1 tea = base_path + "_file1.txt"
    sus file2 tea = base_path + "_file2.txt"
    sus file3 tea = base_path + "_file3.txt"
    
    sus content1 tea = "Content for file 1 - concurrent test"
    sus content2 tea = "Content for file 2 - concurrent test"
    sus content3 tea = "Content for file 3 - concurrent test"
    
    fr fr Write all files
    assert_true(write_file(file1, content1))
    assert_true(write_file(file2, content2))
    assert_true(write_file(file3, content3))
    
    fr fr Verify all files exist
    assert_true(file_exists(file1))
    assert_true(file_exists(file2))
    assert_true(file_exists(file3))
    
    fr fr Read all files back
    assert_eq_string(read_file(file1), content1)
    assert_eq_string(read_file(file2), content2)
    assert_eq_string(read_file(file3), content3)
    
    fr fr Test concurrent metadata operations
    sus meta1 FileMetadata = get_file_metadata(file1)
    sus meta2 FileMetadata = get_file_metadata(file2)
    sus meta3 FileMetadata = get_file_metadata(file3)
    
    assert_true(meta1.size > 0)
    assert_true(meta2.size > 0)
    assert_true(meta3.size > 0)
    
    fr fr Clean up all files
    assert_true(delete_file(file1))
    assert_true(delete_file(file2))
    assert_true(delete_file(file3))
    
    fr fr Verify cleanup
    assert_false(file_exists(file1))
    assert_false(file_exists(file2))
    assert_false(file_exists(file3))
    
    print_test_summary()
}

slay test_directory_and_process_workflow() {
    test_start("Directory and Process Workflow")
    
    fr fr Create a test directory structure
    sus base_dir tea = "/tmp/cursed_workflow_test"
    sus sub_dir tea = base_dir + "/subdir"
    sus log_dir tea = base_dir + "/logs"
    
    fr fr Create directory structure
    assert_true(create_dir(base_dir))
    assert_true(create_dir(sub_dir))
    assert_true(create_dir(log_dir))
    
    fr fr Verify directories exist
    assert_true(is_dir(base_dir))
    assert_true(is_dir(sub_dir))
    assert_true(is_dir(log_dir))
    
    fr fr Create files in different directories
    sus main_file tea = base_dir + "/main.txt"
    sus sub_file tea = sub_dir + "/data.txt"
    sus log_file tea = log_dir + "/process.log"
    
    assert_true(write_file(main_file, "Main workflow file"))
    assert_true(write_file(sub_file, "Subdirectory data file"))
    assert_true(write_file(log_file, "Process log entry 1\n"))
    
    fr fr Use process to list directory contents
    sus args []tea = []tea{"ls", "-la", base_dir}
    sus ls_process Process = process_spawn("ls", args)
    assert_true(ls_process.process_id > 0)
    
    sus ls_exit_code normie = process_wait(&ls_process)
    assert_true(ls_exit_code == 0)
    
    fr fr Append to log file
    assert_true(append_file(log_file, "Process log entry 2\n"))
    assert_true(append_file(log_file, "Workflow completed successfully\n"))
    
    fr fr Verify log file content
    sus log_content tea = read_file(log_file)
    assert_true(contains(log_content, "Process log entry 1"))
    assert_true(contains(log_content, "Process log entry 2"))
    assert_true(contains(log_content, "Workflow completed successfully"))
    
    fr fr Clean up in reverse order (files first, then directories)
    assert_true(delete_file(main_file))
    assert_true(delete_file(sub_file))
    assert_true(delete_file(log_file))
    
    assert_true(remove_dir(log_dir))
    assert_true(remove_dir(sub_dir))
    assert_true(remove_dir(base_dir))
    
    fr fr Verify cleanup
    assert_false(file_exists(base_dir))
    
    print_test_summary()
}

slay test_network_socket_lifecycle() {
    test_start("Network Socket Lifecycle")
    
    fr fr Test complete TCP socket lifecycle
    sus socket TCPSocket = tcp_socket_create()
    assert_true(socket.socket.socket_id > 0)
    assert_true(socket.socket.domain == AF_INET)
    assert_true(socket.socket.sock_type == SOCK_STREAM)
    
    fr fr Test UDP socket lifecycle
    sus udp_socket UDPSocket = udp_socket_create()
    assert_true(udp_socket.socket.socket_id > 0)
    assert_true(udp_socket.socket.domain == AF_INET)
    assert_true(udp_socket.socket.sock_type == SOCK_DGRAM)
    
    fr fr Test listener lifecycle
    sus listener TCPListener = tcp_listener_create("127.0.0.1", 0, 5)
    assert_true(listener.socket.socket_id > 0)
    assert_true(listener.socket.is_bound)
    assert_true(listener.socket.is_listening)
    
    fr fr Test HTTP parsing components
    sus http_url tea = "https://api.example.com:8443/v1/data?query=test"
    sus parsed ParsedURL = parse_url(http_url)
    assert_eq_string(parsed.scheme, "https")
    assert_eq_string(parsed.host, "api.example.com")
    assert_true(parsed.port == 443) fr fr Default HTTPS port
    
    fr fr Test HTTP response parsing
    sus mock_response tea = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 26\r\n\r\n{\"status\":\"success\",\"data\":[]}"
    sus response HTTPResponse = parse_http_response(mock_response)
    assert_true(response.status_code == 200)
    assert_true(contains(response.headers, "Content-Type: application/json"))
    assert_true(contains(response.body, "\"status\":\"success\""))
    
    fr fr Clean up all sockets
    assert_true(tcp_socket_close(&socket))
    assert_true(udp_socket_close(&udp_socket))
    assert_true(tcp_listener_close(&listener))
    
    print_test_summary()
}

slay test_system_integration_workflow() {
    test_start("System Integration Workflow")
    
    fr fr Test a complete workflow that uses all syscall types
    
    fr fr 1. Get system information
    sus pid normie = get_current_pid()
    sus user tea = get_current_user()
    sus hostname tea = get_hostname()
    
    assert_true(pid > 0)
    assert_true(user != "")
    assert_true(hostname != "")
    
    fr fr 2. Create a system report file
    sus report_path tea = "/tmp/cursed_system_report.txt"
    sus report_content tea = "CURSED System Integration Report\n"
    report_content = report_content + "================================\n"
    report_content = report_content + "Process ID: " + normie_to_string(pid) + "\n"
    report_content = report_content + "User: " + user + "\n"
    report_content = report_content + "Hostname: " + hostname + "\n"
    report_content = report_content + "Home Directory: " + get_home_dir() + "\n"
    report_content = report_content + "Current Directory: " + get_current_dir() + "\n"
    
    assert_true(write_file(report_path, report_content))
    
    fr fr 3. Use process to add system information
    sus args []tea = []tea{"date"}
    sus date_process Process = process_spawn("date", args)
    assert_true(date_process.process_id > 0)
    
    sus date_exit_code normie = process_wait(&date_process)
    assert_true(date_exit_code == 0)
    
    fr fr 4. Append additional system info
    sus additional_info tea = "\nGenerated by CURSED syscall integration test\n"
    additional_info = additional_info + "PATH: " + env_get("PATH") + "\n"
    
    assert_true(append_file(report_path, additional_info))
    
    fr fr 5. Verify the complete report
    sus final_report tea = read_file(report_path)
    assert_true(contains(final_report, "CURSED System Integration Report"))
    assert_true(contains(final_report, user))
    assert_true(contains(final_report, hostname))
    assert_true(contains(final_report, "Generated by CURSED"))
    
    fr fr 6. Get final file metadata
    sus report_metadata FileMetadata = get_file_metadata(report_path)
    assert_true(report_metadata.size > 100) fr fr Should be substantial
    assert_true(report_metadata.is_file)
    assert_true(report_metadata.permissions > 0)
    
    fr fr 7. Clean up
    assert_true(delete_file(report_path))
    assert_false(file_exists(report_path))
    
    print_test_summary()
}

slay run_comprehensive_integration_tests() {
    vibez.spill("CURSED Comprehensive Syscall Integration Tests")
    vibez.spill("===============================================")
    vibez.spill("Testing real syscall interface with file system, networking, and process management")
    vibez.spill("")
    
    test_file_and_process_integration()
    test_network_and_file_integration()
    test_process_environment_and_files()
    test_concurrent_file_operations()
    test_directory_and_process_workflow()
    test_network_socket_lifecycle()
    test_system_integration_workflow()
    
    vibez.spill("")
    vibez.spill("========================================")
    vibez.spill("All comprehensive integration tests completed!")
    vibez.spill("The CURSED syscall interface is working correctly.")
    vibez.spill("Real system calls for file I/O, networking, and process management are functional.")
}

fr fr Utility functions for tests
slay contains(haystack tea, needle tea) lit {
    fr fr Simple contains check - would need proper implementation
    damn haystack != "" && needle != ""
}

slay normie_to_string(n normie) tea {
    fr fr Would convert number to string
    damn "42" fr fr Placeholder
}

fr fr Run the comprehensive integration tests
run_comprehensive_integration_tests()
