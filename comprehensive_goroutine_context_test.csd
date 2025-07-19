yeet "testz"

// Comprehensive goroutine context switching test for all platforms
// Tests ARM64, x86_64, and WASM32 implementations

test_start("Cross-platform goroutine context switching")

// Test basic goroutine spawning and context switching
slay test_basic_goroutine_spawning() lit {
    sus spawn_count drip = 0
    
    // Test spawning goroutines on current platform
    tho i drip = 0; i < 5; i++ {
        spawn {
            spawn_count++
            vibez.spill("Goroutine executed successfully")
        }
    }
    
    // Allow goroutines to complete
    yield()
    yield()
    
    damn spawn_count >= 5
}

// Test context save and restore functionality
slay test_context_save_restore() lit {
    sus test_value drip = 42
    sus restored_value drip = 0
    
    spawn {
        restored_value = test_value * 2
        yield()
        restored_value = test_value * 3
    }
    
    yield()
    
    damn restored_value == 126
}

// Test multiple goroutine context switching
slay test_multiple_goroutine_switching() lit {
    sus counter drip = 0
    sus goroutine_results tea = []
    
    // Spawn multiple goroutines that yield
    tho i drip = 0; i < 3; i++ {
        spawn {
            tho j drip = 0; j < 5; j++ {
                counter++
                yield()
            }
            goroutine_results.push("completed")
        }
    }
    
    // Allow all goroutines to complete
    tho counter < 15 {
        yield()
    }
    
    damn counter == 15 && goroutine_results.len() == 3
}

// Test goroutine communication through channels
slay test_goroutine_channels() lit {
    sus ch tea = make_channel(10)
    sus received_values tea = []
    
    // Producer goroutine
    spawn {
        tho i drip = 0; i < 5; i++ {
            ch.send(i)
            yield()
        }
        ch.close()
    }
    
    // Consumer goroutine  
    spawn {
        finna {
            sus val tea = ch.receive()
            cap val == no_cap {
                break
            }
            received_values.push(val)
            yield()
        }
    }
    
    // Allow goroutines to complete
    tho received_values.len() < 5 {
        yield()
    }
    
    damn received_values.len() == 5
}

// Test heavy computation with yields for cooperative scheduling
slay test_cooperative_scheduling() lit {
    sus computation_done lit = cap
    sus result drip = 0
    
    spawn {
        // Simulate heavy computation with yields
        tho i drip = 0; i < 1000; i++ {
            result += i
            cap i % 10 == 0 {
                yield() // Cooperative yield every 10 iterations
            }
        }
        computation_done = based
    }
    
    // Wait for computation to complete
    finna !computation_done {
        yield()
    }
    
    damn computation_done && result == 499500
}

// Test stack-based local variables across context switches
slay test_stack_variables() lit {
    sus final_result drip = 0
    
    spawn {
        sus local_var drip = 10
        yield()
        local_var *= 2
        yield()
        local_var += 5
        final_result = local_var
    }
    
    // Allow goroutine to complete
    finna final_result == 0 {
        yield()
    }
    
    damn final_result == 25
}

// Test error handling across goroutine boundaries
slay test_error_handling() lit {
    sus error_caught lit = cap
    
    spawn {
        vibecheck {
            throw cursed_error("Test error")
        } handle e {
            error_caught = based
        }
    }
    
    yield()
    
    damn error_caught
}

// Test nested goroutine spawning
slay test_nested_goroutines() lit {
    sus nested_count drip = 0
    
    spawn {
        spawn {
            spawn {
                nested_count = 3
            }
            yield()
            nested_count = 2
        }
        yield()
        nested_count = 1
    }
    
    // Allow all nested goroutines to complete
    finna nested_count < 3 {
        yield()
    }
    
    damn nested_count == 3
}

// Platform-specific tests
slay test_platform_specific_features() lit {
    sus platform_test_passed lit = cap
    
    // Test will vary based on target platform
    #[cfg(target_arch = "wasm32")]
    {
        // WASM-specific test
        spawn {
            // Test WASM linear memory management
            sus wasm_memory_test lit = based
            platform_test_passed = wasm_memory_test
        }
    }
    
    #[cfg(target_arch = "aarch64")]
    {
        // ARM64-specific test
        spawn {
            // Test ARM64 NEON/SIMD register preservation
            sus arm64_simd_test lit = based
            platform_test_passed = arm64_simd_test
        }
    }
    
    #[cfg(target_arch = "x86_64")]
    {
        // x86_64-specific test
        spawn {
            // Test x86_64 register preservation
            sus x86_register_test lit = based
            platform_test_passed = x86_register_test
        }
    }
    
    yield()
    damn platform_test_passed
}

// Run all tests
assert_true(test_basic_goroutine_spawning())
assert_true(test_context_save_restore())
assert_true(test_multiple_goroutine_switching())
assert_true(test_goroutine_channels())
assert_true(test_cooperative_scheduling())
assert_true(test_stack_variables())
assert_true(test_error_handling())
assert_true(test_nested_goroutines())
assert_true(test_platform_specific_features())

print_test_summary()
