yeet "testz"
yeet "platformz"

test_start("PLATFORMZ Comprehensive Platform Abstraction Tests")

// Test OS detection
sus os_name tea = get_os_name()
assert_not_eq_string(os_name, "")

sus arch tea = get_architecture()
assert_not_eq_string(arch, "")

// Test environment variables
set_env_var("CURSED_TEST", "value123")
sus env_val tea = get_env_var("CURSED_TEST")
assert_eq_string(env_val, "value123")

// Test file system paths
sus home_dir tea = get_home_directory()
assert_not_eq_string(home_dir, "")

sus temp_dir tea = get_temp_directory()
assert_not_eq_string(temp_dir, "")

sus path_sep tea = get_path_separator()
assert_not_eq_string(path_sep, "")

// Test process management
sus process_id drip = get_current_process_id()
assert_true(process_id > 0)

sus parent_id drip = get_parent_process_id()
assert_true(parent_id > 0)

// Test system resources
sus cpu_count drip = get_cpu_count()
assert_true(cpu_count > 0)

sus memory_total drip = get_total_memory()
assert_true(memory_total > 0)

sus memory_free drip = get_free_memory()
assert_true(memory_free > 0)

// Test platform-specific features
ready (os_name == "linux") {
    sus kernel tea = get_kernel_version()
    assert_not_eq_string(kernel, "")
}

ready (os_name == "windows") {
    sus version tea = get_windows_version()
    assert_not_eq_string(version, "")
}

ready (os_name == "darwin") {
    sus macos_ver tea = get_macos_version()
    assert_not_eq_string(macos_ver, "")
}

// Test user information
sus username tea = get_current_username()
assert_not_eq_string(username, "")

sus user_id drip = get_current_user_id()
assert_true(user_id >= 0)

// Test permissions and access
sus is_admin lit = is_administrator()
// Test passes regardless of admin status

sus has_write lit = can_write_to_directory("/tmp")
assert_true(has_write)

// Test network interfaces
sus interfaces tea[value] = get_network_interfaces()
assert_true(len(interfaces) > 0)

// Test system locale
sus locale tea = get_system_locale()
assert_not_eq_string(locale, "")

sus timezone tea = get_system_timezone()
assert_not_eq_string(timezone, "")

print_test_summary()
