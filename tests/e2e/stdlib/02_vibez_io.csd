yeet "testz"
yeet "vibez"

test_start("Vibez I/O Library Tests")

# Test basic output functions
vibez.spill("Testing basic spill function")
vibez.spill("Number output: " + str(42))
vibez.spill("Boolean output: " + str(based))

# Test formatted output
sus name tea = "CURSED"
sus version drip = 1
sus active lit = based

vibez.spillf("Language: %s", name)
vibez.spillf("Version: %d", version)
vibez.spillf("Active: %t", active)
vibez.spillf("Combined: %s v%d (active: %t)", name, version, active)

# Test different data types
sus integer drip = 42
sus floating meal = 3.14159
sus character rune = 'A'
sus text tea = "Hello, World!"

vibez.spillf("Integer: %d", integer)
vibez.spillf("Float: %.2f", floating)
vibez.spillf("Character: %c", character)
vibez.spillf("String: %s", text)

# Test with arrays
sus numbers := [1, 2, 3, 4, 5]
vibez.spill("Array contents:")
range i, num in numbers {
    vibez.spillf("  [%d] = %d", i, num)
}

# Test with structs
squad Person {
    spill name tea
    spill age drip
    spill active lit
}

sus person Person = Person{
    name: "Alice", 
    age: 30, 
    active: based
}

vibez.spillf("Person: %s, age %d, active: %t", 
    person.name, person.age, person.active)

# Test error output
vibez.spill_err("This is an error message")
vibez.spill_err("Error code: " + str(404))

# Test with complex formatting
sus pi meal = 3.14159265359
vibez.spillf("Pi with different precision:")
vibez.spillf("  %.1f", pi)
vibez.spillf("  %.3f", pi)
vibez.spillf("  %.6f", pi)

# Test newline and special characters
vibez.spill("Line 1\nLine 2")
vibez.spill("Tab\tseparated\tvalues")
vibez.spill("Quote: \"Hello\"")

# Test numeric formatting
sus large_number drip = 1234567
sus small_number meal = 0.001234

vibez.spillf("Large: %d", large_number)
vibez.spillf("Small: %.6f", small_number)
vibez.spillf("Scientific: %.2e", small_number)

# Test boolean formatting
vibez.spillf("True: %t", based)
vibez.spillf("False: %t", cringe)

# Validation through string comparison
sus test_string tea = "Test: " + str(42)
assert_eq_string(test_string, "Test: 42")

sus test_concat tea = name + " v" + str(version)
assert_eq_string(test_concat, "CURSED v1")

vibez.spill("All vibez I/O tests completed successfully")

print_test_summary()
