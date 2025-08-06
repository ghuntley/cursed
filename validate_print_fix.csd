yeet "vibez"
yeet "testz"

test_start("core.print fix validation")

fr fr Test that vibez.spill works properly
vibez.spill("✅ vibez.spill() is working!")

fr fr Test multiple prints
vibez.spill("Line 1")
vibez.spill("Line 2") 
vibez.spill("Line 3")

fr fr Test empty string
vibez.spill("")

fr fr Test with newlines in the output
vibez.spill("Multi-line test:")
vibez.spill("  - Item 1")
vibez.spill("  - Item 2")

assert_true(based)  fr fr Test framework works

print_test_summary()
