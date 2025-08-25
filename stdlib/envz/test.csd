fr fr envz module tests - Environment Variable Management Testing
yeet "envz"
yeet "testz"

fr fr Basic environment variable tests
slay test_basic_env_operations() {
    envz.init_envz()
    
    # Test setting and getting
    testz.assert_eq_bool(envz.set("TEST_VAR", "test_value"), based)
    testz.assert_eq_string(envz.get("TEST_VAR"), "test_value")
    
    # Test existence check
    testz.assert_eq_bool(envz.exists("TEST_VAR"), based)
    testz.assert_eq_bool(envz.exists("NONEXISTENT_VAR"), cap)
    
    # Test unsetting
    testz.assert_eq_bool(envz.unset("TEST_VAR"), based)
    testz.assert_eq_string(envz.get("TEST_VAR"), "")
    testz.assert_eq_bool(envz.exists("TEST_VAR"), cap)
    
    testz.pass("Basic environment operations")
}

fr fr Environment expansion tests
slay test_env_expansion() {
    envz.init_envz()
    
    envz.set("USER", "testuser")
    envz.set("HOME", "/home/testuser")
    
    # Test ${VAR} expansion
    sus result1 tea = envz.expand("Hello ${USER}")
    testz.assert_eq_string(result1, "Hello testuser")
    
    # Test $VAR expansion
    sus result2 tea = envz.expand("Home is $HOME")
    testz.assert_eq_string(result2, "Home is /home/testuser")
    
    # Test complex expansion
    sus result3 tea = envz.expand("${USER} lives in ${HOME}")
    testz.assert_eq_string(result3, "testuser lives in /home/testuser")
    
    testz.pass("Environment expansion")
}

fr fr Path manipulation tests  
slay test_path_manipulation() {
    envz.init_envz()
    
    # Test getting path
    sus paths [tea] = envz.get_path()
    testz.assert_gt_int(arrayz.len(paths), 0)
    
    # Test adding to path
    testz.assert_eq_bool(envz.add_to_path("/new/path"), based)
    sus new_paths [tea] = envz.get_path() 
    testz.assert_eq_string(new_paths[0], "/new/path")
    
    # Test removing from path
    testz.assert_eq_bool(envz.remove_from_path("/new/path"), based)
    sus filtered_paths [tea] = envz.get_path()
    # Should not contain /new/path anymore
    
    testz.pass("Path manipulation")
}

fr fr Platform detection tests
slay test_platform_detection() {
    envz.init_envz()
    
    sus platform tea = envz.get_platform()
    testz.assert_ne_string(platform, "")
    
    sus separator tea = envz.get_path_separator()
    testz.assert_true(separator == ":" || separator == ";")
    
    sus path_var tea = envz.get_path_var() 
    testz.assert_eq_string(path_var, "PATH")
    
    testz.pass("Platform detection")
}

fr fr Common environment variable tests
slay test_common_env_vars() {
    envz.init_envz()
    
    sus home tea = envz.get_home()
    testz.assert_ne_string(home, "")
    
    sus user tea = envz.get_user()
    testz.assert_ne_string(user, "")
    
    sus shell tea = envz.get_shell()
    testz.assert_ne_string(shell, "")
    
    sus editor tea = envz.get_editor()
    testz.assert_ne_string(editor, "")
    
    sus temp_dir tea = envz.get_temp_dir()
    testz.assert_ne_string(temp_dir, "")
    
    testz.pass("Common environment variables")
}

fr fr Edge case tests
slay test_edge_cases() {
    envz.init_envz()
    
    # Test empty key
    testz.assert_eq_bool(envz.set("", "value"), cap)
    testz.assert_eq_string(envz.get(""), "")
    
    # Test long values
    sus long_value tea = "a".repeat(1000)
    testz.assert_eq_bool(envz.set("LONG_VAR", long_value), based)
    testz.assert_eq_string(envz.get("LONG_VAR"), long_value)
    
    # Test expansion with missing variable
    sus result tea = envz.expand("Missing: ${MISSING_VAR}")
    testz.assert_eq_string(result, "Missing: ")
    
    testz.pass("Edge cases")
}

fr fr Utility function tests
slay test_utilities() {
    envz.init_envz()
    
    envz.set("TEST1", "value1")
    envz.set("TEST2", "value2")
    
    sus count normie = envz.get_env_count()
    testz.assert_gt_int(count, 1)
    
    testz.assert_eq_bool(envz.is_modified(), based)
    
    envz.reset_modified_flag()
    testz.assert_eq_bool(envz.is_modified(), cap)
    
    # Test getting all keys
    sus keys [tea] = envz.get_keys()
    testz.assert_gt_int(arrayz.len(keys), 0)
    
    testz.pass("Utility functions")
}

fr fr Run all tests
slay test_all() {
    testz.test_start("envz module tests")
    
    test_basic_env_operations()
    test_env_expansion()
    test_path_manipulation()
    test_platform_detection()
    test_common_env_vars()
    test_edge_cases()
    test_utilities()
    
    testz.print_test_summary()
}

test_all()
