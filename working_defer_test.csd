yeet "testz"

fr fr Working Defer Implementation Test
test_start("Defer Statement Implementation")

fr fr Global variable to track defer execution
sus defer_trace tea = ""

fr fr Test basic defer execution
slay test_basic_defer() tea {
    defer_trace = ""
    
    later {
        defer_trace = defer_trace + "cleanup1 "
    }
    
    defer_trace = defer_trace + "main "
    
    later {
        defer_trace = defer_trace + "cleanup2 "
    }
    
    defer_trace = defer_trace + "end "
    
    damn defer_trace
}

sus result1 := test_basic_defer()
vibez.spill("Defer result: " + result1)
assert_true(result1.len() > 0)

fr fr Test defer with early return
slay test_defer_early_return(should_return lit) tea {
    defer_trace = ""
    
    later {
        defer_trace = defer_trace + "always_cleanup "
    }
    
    defer_trace = defer_trace + "start "
    
    if should_return {
        later {
            defer_trace = defer_trace + "early_cleanup "
        }
        damn defer_trace + "early_return"
    }
    
    defer_trace = defer_trace + "normal "
    damn defer_trace + "normal_return"
}

sus early_result := test_defer_early_return(based)
vibez.spill("Early return result: " + early_result)

sus normal_result := test_defer_early_return(cringe)
vibez.spill("Normal return result: " + normal_result)

fr fr Test defer with nested scopes
slay test_defer_nested() tea {
    defer_trace = ""
    
    later {
        defer_trace = defer_trace + "outer_end "
    }
    
    defer_trace = defer_trace + "outer_start "
    
    fr fr Create nested scope with block
    {
        later {
            defer_trace = defer_trace + "inner_end "
        }
        
        defer_trace = defer_trace + "inner_start "
    }
    
    defer_trace = defer_trace + "outer_middle "
    
    damn defer_trace
}

sus nested_result := test_defer_nested()
vibez.spill("Nested defer result: " + nested_result)

print_test_summary()
