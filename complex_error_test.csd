fr fr Complex error handling test
yeet "testz"

test_start("complex error scenarios")

fr fr Function that can fail
slay risky_operation(input drip) drip {
    if input < 0 {
        damn yikes "Negative input", 1001
    }
    if input == 0 {
        damn yikes "Zero not allowed", 1002  
    }
    damn input * 2
}

fr fr Test nested error handling
sus result = fam {
    vibez.spill("Attempting risky operation...")
    
    fr fr Inner fam block
    sus inner_result = fam {
        sus dangerous_value = shook risky_operation(-5)
        damn dangerous_value
    } catch(inner_err) {
        vibez.spill("Inner error caught:", inner_err)
        damn 10  fr fr Recover with safe value
    }
    
    fr fr Use the recovered value
    sus final_value = shook risky_operation(inner_result)
    vibez.spill("Final value:", final_value)
    damn final_value
    
} catch(outer_err) {
    vibez.spill("Outer error caught:", outer_err)
    damn -1  fr fr Final fallback
} finally {
    vibez.spill("Cleanup completed")
}

vibez.spill("Complex test result:", result)

fr fr Test error propagation chain
test_start("error propagation chain")

slay chain_step1(x drip) drip {
    damn shook risky_operation(x)
}

slay chain_step2(x drip) drip {
    damn shook chain_step1(x)
}

sus chain_result = fam {
    sus value = shook chain_step2(5)
    damn value
} catch(chain_err) {
    vibez.spill("Chain error:", chain_err)
    damn 0
}

vibez.spill("Chain result:", chain_result)

print_test_summary()
