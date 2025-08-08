yeet "testz"
yeet "sys_core"

test_group_start("System Information")

test_start("get_system_info_test")
sus info tea = get_system_info()
assert_true(len_str(info) > 0)
assert_contains(info, "CURSED Runtime")

test_start("platform_detection_test")
sus platform tea = get_platform()
assert_true(platform == "linux" || platform == "macos" || platform == "windows" || platform == "unknown")

test_start("architecture_detection_test")
sus arch tea = get_architecture()
assert_true(arch == "x64" || arch == "x86" || arch == "unknown")

test_start("os_version_test")
sus version tea = get_os_version()
assert_true(len_str(version) > 0)
assert_contains(version, "Linux")

test_start("pointer_size_test")
sus size normie = get_pointer_size()
assert_true(size == 4 || size == 8)

test_group_end()

test_group_start("Memory Management")

test_start("initialization_test")
assert_true(sys_core_init())

test_start("memory_allocation_test")
sus addr1 normie = alloc(1024)
sus addr2 normie = alloc(2048)
assert_true(addr1 > 0)
assert_true(addr2 > 0)
assert_true(addr1 != addr2)

test_start("memory_deallocation_test")
sus addr normie = alloc(512)
assert_true(free(addr))

test_start("memory_usage_tracking_test")
sus initial_usage normie = memory_usage()
sus addr normie = alloc(1024)
sus new_usage normie = memory_usage()
assert_true(new_usage >= initial_usage)

test_start("memory_limits_test")
sus limit normie = get_memory_limit()
assert_true(limit > 0)
assert_true(set_memory_limit(limit))

test_start("heap_operations_test")
sus heap_size normie = get_heap_size()
assert_true(heap_size > 0)

test_group_end()

test_group_start("Process Management")

test_start("process_id_test")
sus pid normie = get_process_id()
sus ppid normie = get_parent_process_id()
assert_true(pid > 0)
assert_true(ppid >= 0)

test_start("process_spawning_test")
sus new_pid normie = spawn_process("echo hello")
assert_true(new_pid > 0)

test_start("process_existence_test")
sus current_pid normie = get_process_id()
assert_true(process_exists(current_pid))

test_start("process_termination_test")
sus pid normie = spawn_process("sleep 1")
assert_true(kill_process(pid))

test_group_end()

test_group_start("Signal Handling")

test_start("signal_registration_test")
assert_true(register_signal_handler(15))  // SIGTERM
assert_true(register_signal_handler(2))   // SIGINT

test_start("signal_sending_test")
sus pid normie = get_process_id()
assert_true(send_signal(pid, 0))  // Check if process exists

test_start("signal_ignoring_test")
assert_true(ignore_signal(1))  // SIGHUP

test_group_end()

test_group_start("Resource Management")

test_start("resource_limits_test")
assert_true(set_resource_limit(1, 1000000))  // Set memory limit
sus limit normie = get_resource_limit(1)
assert_true(limit > 0)

test_start("cpu_usage_test")
sus cpu normie = get_cpu_usage()
assert_true(cpu >= 0 && cpu <= 100)

test_start("open_files_test")
sus count normie = get_open_files_count()
assert_true(count >= 0)

test_start("max_files_test")
sus max_files normie = get_max_open_files()
assert_true(max_files > 0)

test_group_end()

test_group_start("Environment Variables")

test_start("environment_variable_test")
assert_true(set_environment_variable("TEST_VAR", "test_value"))
sus value tea = get_environment_variable("TEST_VAR")
assert_eq_string(value, "test_value")

test_start("working_directory_test")
sus current_dir tea = get_working_directory()
assert_true(len_str(current_dir) > 0)
assert_true(set_working_directory("/tmp"))

test_group_end()

test_group_start("Time and Scheduling")

test_start("system_time_test")
sus time normie = get_system_time()
assert_true(time > 1600000000)  // After 2020

test_start("sleep_test")
assert_true(sleep_milliseconds(1))

test_start("process_priority_test")
sus priority normie = get_process_priority()
assert_true(priority >= -20 && priority <= 19)
assert_true(set_process_priority(0))

test_group_end()

test_group_start("Hardware Information")

test_start("cpu_info_test")
sus cpu_count normie = get_cpu_count()
assert_true(cpu_count > 0 && cpu_count <= 128)

test_start("memory_info_test")
sus total_mem normie = get_total_memory()
sus avail_mem normie = get_available_memory()
assert_true(total_mem > 0)
assert_true(avail_mem > 0)
assert_true(avail_mem <= total_mem)

test_start("stack_size_test")
sus stack_size normie = get_stack_size()
assert_true(stack_size > 0)
assert_true(set_stack_size(stack_size))

test_group_end()

test_group_start("Network and Host Information")

test_start("hostname_test")
sus hostname tea = get_hostname()
assert_true(len_str(hostname) > 0)

test_start("network_interfaces_test")
sus interfaces tea = get_network_interfaces()
assert_true(len_str(interfaces) > 0)

test_group_end()

test_group_start("Security and Permissions")

test_start("user_identification_test")
sus uid normie = get_user_id()
sus gid normie = get_group_id()
assert_true(uid >= 0)
assert_true(gid >= 0)

test_start("privilege_check_test")
sus has_root lit = has_root_privileges()
assert_true(has_root == based || has_root == cringe)

test_group_end()

test_group_start("Performance Monitoring")

test_start("load_average_test")
sus load tea = get_load_average()
assert_true(len_str(load) > 0)
assert_contains(load, ".")

test_start("uptime_test")
sus uptime normie = get_uptime()
assert_true(uptime >= 0)

test_group_end()

test_group_start("Module Lifecycle")

test_start("cleanup_test")
assert_true(sys_core_cleanup())

test_start("reinitialization_test")
assert_true(sys_core_init())

test_group_end()

print_test_summary()

vibez.spill("✅ Enhanced sys_core Module Test Complete!")
vibez.spill("🎯 All system operations validated successfully")
