// Test environment variable functionality
yeet "envz"

vibez.spill("🧪 Testing Environment Variable Functions")
vibez.spill("=========================================")

// Test basic get_env
(home_dir, err) := get_env("HOME")
lowkey err == "" {
    vibez.spill("✅ HOME directory:", home_dir)
} otherwise {
    vibez.spill("❌ Failed to get HOME:", err)
}

// Test set_env
set_err := set_env("CURSED_TEST", "hello_world")
lowkey set_err == "" {
    vibez.spill("✅ Set CURSED_TEST environment variable")
    
    // Verify it was set
    (test_value, get_err) := get_env("CURSED_TEST")
    lowkey get_err == "" && test_value == "hello_world" {
        vibez.spill("✅ Retrieved CURSED_TEST:", test_value)
    } otherwise {
        vibez.spill("❌ Failed to retrieve CURSED_TEST:", get_err)
    }
} otherwise {
    vibez.spill("❌ Failed to set environment variable:", set_err)
}

// Test env_exists
lowkey env_exists("PATH") {
    vibez.spill("✅ PATH environment variable exists")
} otherwise {
    vibez.spill("❌ PATH environment variable not found")
}

// Test get_env_with_default
default_val := get_env_with_default("NONEXISTENT_VAR", "default_value")
lowkey default_val == "default_value" {
    vibez.spill("✅ Got default value for non-existent variable")
} otherwise {
    vibez.spill("❌ Default value function failed")
}

// Test get_home_dir
(home, home_err) := get_home_dir()
lowkey home_err == "" {
    vibez.spill("✅ Home directory:", home)
} otherwise {
    vibez.spill("⚠️ Home directory not found:", home_err)
}

// Test get_path_env
paths := get_path_env()
lowkey len(paths) > 0 {
    vibez.spill("✅ Found", len(paths), "PATH directories")
    vibez.spill("First PATH entry:", paths[0])
} otherwise {
    vibez.spill("⚠️ No PATH directories found")
}

// Test environment variable expansion
lowkey env_exists("USER") {
    expanded := expand_env("Current user is: $USER")
    vibez.spill("✅ Expanded text:", expanded)
}

// Clean up test variable
unset_err := unset_env("CURSED_TEST")
lowkey unset_err == "" {
    vibez.spill("✅ Cleaned up CURSED_TEST environment variable")
} otherwise {
    vibez.spill("⚠️ Failed to clean up test variable:", unset_err)
}

vibez.spill("")
vibez.spill("🎉 Environment variable tests completed!")
