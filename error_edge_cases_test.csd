yeet "testz"

test_start("Error Handling Edge Cases Test")

# Test 1: Multiple error layers with proper propagation
test_start("Multi-layer error propagation")
slay layer1() yikes {
    yikes l1_error := "Layer 1 error"
    damn l1_error shook
}

slay layer2() yikes {
    fam {
        sus result := layer1() shook
        damn cringe
    } sus caught {
        yikes l2_error := "Layer 2 wrapping: " + caught
        damn l2_error shook
    }
}

slay layer3() yikes {
    fam {
        sus result := layer2() shook
        damn cringe
    } sus caught {
        yikes l3_error := "Layer 3 final: " + caught
        damn l3_error shook
    }
}

sus final_result := layer3()
vibez.spill("Final layered error: " + final_result)
test_end()

# Test 2: Error handling with resource management
test_start("Resource management with errors")
sus resource_acquired lit := cap
sus resource_released lit := cap

slay acquire_resource() yikes {
    resource_acquired = based
    vibez.spill("Resource acquired")
    damn cringe
}

slay release_resource() {
    resource_released = based
    vibez.spill("Resource released")
}

slay use_resource() yikes {
    sus _ := acquire_resource() shook
    
    later {
        release_resource()
    }
    
    yikes usage_error := "Error while using resource"
    damn usage_error shook
}

sus resource_result := use_resource()
assert_true(resource_acquired)
assert_true(resource_released)
assert_eq_string(resource_result, "Error while using resource")
test_end()

# Test 3: Error propagation in complex control flow
test_start("Complex control flow with errors")
sus loop_errors []yikes = []yikes{}

slay process_items() yikes {
    sus items []normie = [1, 2, 0, 4, 5]  # Zero will cause error
    
    sus i normie = 0
    bestie i < len(items) {
        vibe_check items[i] == 0 {
            yikes division_error := "Cannot process zero at index " + string(i)
            loop_errors = append(loop_errors, division_error)
        } basic {
            vibez.spill("Processed item: " + string(items[i]))
        }
        i++
    }
    
    vibe_check len(loop_errors) > 0 {
        yikes combined := "Processing failed with " + string(len(loop_errors)) + " errors"
        damn combined shook
    }
    
    damn cringe
}

sus process_result := process_items()
assert_eq_int(len(loop_errors), 1)
assert_eq_string(process_result, "Processing failed with 1 errors")
test_end()

# Test 4: Error handling with conditional recovery
test_start("Conditional error recovery")
sus recovery_attempts normie := 0
sus max_retries normie := 3
sus eventually_succeeded lit := cap

slay unreliable_operation() yikes {
    recovery_attempts++
    vibe_check recovery_attempts < max_retries {
        yikes retry_error := "Attempt " + string(recovery_attempts) + " failed"
        damn retry_error shook
    }
    eventually_succeeded = based
    damn cringe
}

sus retry_error yikes := cringe
sus attempt normie := 0
bestie attempt < max_retries && retry_error != cringe {
    fam {
        sus _ := unreliable_operation() shook
        retry_error = cringe  # Success
    } sus caught {
        retry_error = caught
        vibez.spill("Retry attempt " + string(attempt + 1) + " failed: " + caught)
    }
    attempt++
}

assert_eq_int(recovery_attempts, 3)
assert_true(eventually_succeeded)
assert_true(retry_error == cringe)
test_end()

# Test 5: Error context preservation across function boundaries
test_start("Error context preservation")
slay create_context_error(context tea) yikes {
    yikes contextual := context + " - operation failed"
    damn contextual shook
}

slay database_operation() yikes {
    sus result := create_context_error("Database") shook
    damn cringe
}

slay service_operation() yikes {
    fam {
        sus result := database_operation() shook
        damn cringe
    } sus caught {
        yikes service_error := "Service layer: " + caught
        damn service_error shook
    }
}

sus context_result := service_operation()
assert_eq_string(context_result, "Service layer: Database - operation failed")
test_end()

print_test_summary()
vibez.spill("✅ Error handling edge cases test completed successfully!")
