// Test basic type assertions in CURSED

// Test safe type assertions (value.(type)?)
sus x normie = 42
sus y drip = x.(drip)?  // Safe conversion from int to float
vibez.spill("Safe conversion successful:", y)

// Test unsafe type assertions (value.(type))
sus a drip = 3.14
sus b normie = a.(normie)  // Unsafe conversion from float to int (may panic)
vibez.spill("Unsafe conversion successful:", b)

// Test type assertion with different types
sus str_val tea = "hello"
sus bool_val lit = based

// This should succeed
sus int_from_bool normie = bool_val.(normie)
vibez.spill("Boolean to int:", int_from_bool)

// This should fail with panic (commented out for safety)
// sus int_from_str normie = str_val.(normie)
