fr fr Advanced error handling test with complex scenarios
yeet "testz"

test_start("complex error scenarios")

fr fr Error type definition
squad CustomError {
    spill message tea
    spill code drip
    spill details tea
}

fr fr Function with multiple error conditions
slay complex_operation(input drip) drip {
    if input < 0 {
        damn yikes "Negative input not allowed", 1001
    }
    if input == 0 {
        damn yikes "Zero division error", 1002
    }
    if input > 100 {
        damn yikes "Input too large", 1003
    }
    damn input * 2
}

fr fr Test error propagation chain
slay chain_operation(value drip) drip {
    sus step1 = shook complex_operation(value)
    sus step2 = shook complex_operation(step1)
    damn step2
}

fr fr Test nested fam blocks
sus nested_result = fam {
    vibez.spill("Starting nested operation...")
    
    sus inner_result = fam {
        vibez.spill("Inner operation...")
        sus dangerous_value = shook complex_operation(-5)
        damn dangerous_value
    } catch(inner_err) {
        vibez.spill("Inner catch:", inner_err)
        damn 10  fr fr Recover with safe value
    }
    
    sus final_step = shook complex_operation(inner_result)
    damn final_step
} catch(outer_err) {
    vibez.spill("Outer catch:", outer_err)
    damn -1  fr fr Final fallback
} finally {
    vibez.spill("Nested operation completed")
}

vibez.spill("Nested result:", nested_result)

fr fr Test error with custom data
test_start("structured error test")

sus structured_error = yikes "Database connection failed", 2001
vibez.spill("Structured error:", structured_error)

fr fr Test multiple error conditions
bestie i := -10; i <= 110; i = i + 25 {
    sus test_result = fam {
        sus value = shook complex_operation(i)
        vibez.spillf("Operation with {} succeeded: {}", i, value)
        damn value
    } catch(err) {
        vibez.spillf("Operation with {} failed: {}", i, err)
        damn 0
    }
}

print_test_summary()
