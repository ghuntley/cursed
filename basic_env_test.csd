// Basic test to debug environment variables

vibez.spill("Testing runtime_get_env...")

// Direct runtime call
(home_value, home_error) := runtime_get_env("HOME")

lowkey home_error == "" {
    vibez.spill("HOME is:", home_value)
} otherwise {
    vibez.spill("HOME error:", home_error)
}

// Test PATH too
(path_value, path_error) := runtime_get_env("PATH")

lowkey path_error == "" {
    vibez.spill("PATH found, length:", len_str(path_value))
} otherwise {
    vibez.spill("PATH error:", path_error)
}

vibez.spill("Done testing")
