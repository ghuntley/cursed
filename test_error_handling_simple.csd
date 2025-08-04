fr fr Test basic yikes error creation
yeet "testz"

test_start("yikes error creation test")

fr fr Create simple error with yikes
sus error_msg tea = "Something went wrong"
sus error_code drip = 42
sus my_error = yikes error_msg, error_code

vibez.spill("Created error:", my_error)

test_start("shook error propagation test")

fr fr Function that might fail
slay risky_function() drip {
    sus random_fail lit = based  fr fr Simulate failure
    if random_fail {
        damn yikes "Function failed", 100
    }
    damn 42
}

fr fr Use shook to propagate error
sus result = shook risky_function()
vibez.spill("Result:", result)

test_start("fam panic recovery test")

fr fr Test fam block with error handling
sus final_result = fam {
    vibez.spill("Trying risky operation...")
    sus value = shook risky_function()
    vibez.spill("Success! Value:", value)
    damn value
} catch(err) {
    vibez.spill("Caught error:", err)
    damn 0  fr fr Return default value
} finally {
    vibez.spill("Cleanup completed")
}

vibez.spill("Final result:", final_result)

print_test_summary()
