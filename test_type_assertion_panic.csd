// Test type assertions that should panic

sus str_val tea = "hello"
vibez.spill("About to attempt unsafe string to int conversion...")

// This should panic with a detailed error message
sus int_val normie = str_val.(normie)

// This line should never be reached
vibez.spill("This should not print")
