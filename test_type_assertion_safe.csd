// Test safe type assertions that don't panic

sus x normie = 42
sus y drip = x.(drip)?  // Safe conversion - should succeed
vibez.spill("Safe int to float:", y)

sus str_val tea = "hello"
sus safe_int normie = str_val.(normie)?  // Safe conversion - should return 0 (default value)
vibez.spill("Safe string to int (default):", safe_int)

sus bool_val lit = based
sus safe_float drip = bool_val.(drip)?  // Safe conversion - should succeed
vibez.spill("Safe bool to float:", safe_float)

sus byte_val byte = 255
sus safe_char sip = byte_val.(sip)?  // Safe conversion - should succeed
vibez.spill("Safe byte to char:", safe_char)
