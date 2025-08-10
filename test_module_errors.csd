fr fr Test valid imports
yeet "vibez"
yeet "mathz"

fr fr Test invalid module (should show error but continue)
yeet "nonexistent"

vibez.spill("Testing module system")

fr fr Test calling function from loaded module
sus result drip = mathz.multiply_two(4, 7)
vibez.spill("4 * 7 =", result)

fr fr Test multiple functions from same module
vibez.print_header("Multiple Functions Test")
vibez.print_success("Module system working!")

fr fr Test function that doesn't exist (should show runtime error)
fr fr vibez.nonexistent_function()  // Comment this out since it would cause compile error

vibez.spill("Test completed successfully")
