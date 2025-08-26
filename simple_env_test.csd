yeet "envz"
yeet "vibez"

vibez.spill("=== Environment Variable System Test ===")

# Test 1: Get existing environment variables 
vibez.spill("Test 1: Getting environment variables")
sus user tea = get_env("USER")
sus home tea = get_env("HOME") 
sus path tea = get_env("PATH")

vibez.spill("USER: " + user)
vibez.spill("HOME: " + home)
vibez.spill("PATH (first 80 chars): " + path.substring(0, 80))

# Test 2: Set and get custom environment variable
vibez.spill("Test 2: Setting custom environment variable")
sus test_var tea = "CURSED_TEST_123"
sus test_value tea = "cursed_works_great"

sus set_success lit = set_env(test_var, test_value)
vibez.spill("Set result: " + set_success.to_string())

sus retrieved tea = get_env(test_var)
vibez.spill("Retrieved: " + retrieved)

# Test 3: Check if variable exists
sus exists_test lit = env_exists(test_var)
vibez.spill("Variable exists: " + exists_test.to_string())

# Test 4: Get with default value
sus nonexistent tea = get_env_default("NONEXISTENT_VAR", "default_value")
vibez.spill("Nonexistent with default: " + nonexistent)

sus existing_with_default tea = get_env_default(test_var, "ignored")
vibez.spill("Existing with default: " + existing_with_default)

# Test 5: Environment variable expansion
vibez.spill("Test 5: Variable expansion")
sus template tea = "User ${USER} lives in ${HOME}"
sus expanded tea = expand(template)
vibez.spill("Template: " + template)
vibez.spill("Expanded: " + expanded)

# Test 6: Platform detection
sus platform tea = get_platform()
vibez.spill("Platform: " + platform)

# Test 7: Common environment helpers
sus current_user tea = get_user()
sus home_dir tea = get_home()
sus shell_path tea = get_shell()
sus temp_dir tea = get_temp_dir()

vibez.spill("Current user: " + current_user)
vibez.spill("Home dir: " + home_dir)
vibez.spill("Shell: " + shell_path)
vibez.spill("Temp dir: " + temp_dir)

# Test 8: List all environment variables
sus all_env map<tea, tea> = list_env()
vibez.spill("Total environment variables: " + all_env.size().to_string())

# Test 9: Unset variable
sus unset_success lit = unset_env(test_var)
vibez.spill("Unset result: " + unset_success.to_string())

sus after_unset tea = get_env(test_var)
vibez.spill("After unset: '" + after_unset + "'")

vibez.spill("=== Environment Variable System Test Complete ===")
