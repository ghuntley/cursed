// CURSED Enhanced Language Features Demo
// Demonstrates new pattern matching, channel operations, error handling, and defer features

yeet "vibez"
yeet "testz"

slay demo_enhanced_pattern_matching() drip {
    vibez.spill("=== Enhanced Pattern Matching Demo ===")
    
    // Range patterns with 0..10 syntax
    sus value drip = 5
    ready (value) {
        0..10 -> {
            vibez.spill("Value is in range 0 to 10")
        }
        11..20 -> {
            vibez.spill("Value is in range 11 to 20")
        }
        _ -> {
            vibez.spill("Value is outside known ranges")
        }
    }
    
    // Guard patterns with 'when' conditions
    sus user_age drip = 25
    ready (user_age) {
        x when x >= 18 -> {
            vibez.spill("User is adult: ", x)
        }
        x when x < 18 -> {
            vibez.spill("User is minor: ", x)
        }
    }
    
    // Character range patterns
    sus grade_letter tea = 'B'
    ready (grade_letter) {
        'A'..'B' -> vibez.spill("Excellent grade!")
        'C'..'D' -> vibez.spill("Good grade")
        'F' -> vibez.spill("Needs improvement")
        _ -> vibez.spill("Unknown grade")
    }
    
    vibez.spill("✅ Pattern matching demo complete")
    damn 0
}

slay demo_channel_operations() drip {
    vibez.spill("=== Enhanced Channel Operations Demo ===")
    
    // Create unbuffered channel - dm<drip>
    sus unbuffered_ch dm<drip> = dm_create_unbuffered()
    
    // Create buffered channel - dm<drip>[5]
    sus buffered_ch dm<drip>[5] = dm_create_buffered(5)
    
    // Spawn sender goroutine
    stan {
        vibez.spill("Sender: Starting to send values")
        
        // Send using dm_send instead of <-
        dm_send(buffered_ch, 10)
        dm_send(buffered_ch, 20)
        dm_send(buffered_ch, 30)
        
        vibez.spill("Sender: All values sent")
        dm_close(buffered_ch)
    }
    
    // Receive using dm_recv instead of <-
    vibez.spill("Receiver: Starting to receive values")
    bestie based {
        sus received drip = dm_recv(buffered_ch)
        ready (received) {
            null -> {
                vibez.spill("Channel closed, stopping receiver")
                damn
            }
            value -> {
                vibez.spill("Received: ", value)
            }
        }
    }
    
    vibez.spill("✅ Channel operations demo complete")
    damn 0
}

slay demo_error_handling() drip {
    vibez.spill("=== Enhanced Error Handling Demo ===")
    
    // Function that might fail
    slay risky_operation(input drip) drip? {
        ready (input) {
            x when x < 0 -> {
                yikes "Negative input not allowed"
            }
            x when x > 100 -> {
                yikes "Input too large"
            }
            _ -> {
                damn input * 2
            }
        }
    }
    
    // Using ? operator for error propagation
    slay safe_calculation(x drip) drip? {
        sus result drip = risky_operation(x)?  // ? operator propagates errors
        damn result + 10
    }
    
    // Try/catch error handling
    fam {
        sus result1 drip = safe_calculation(5)?
        vibez.spill("Success: ", result1)
        
        sus result2 drip = safe_calculation(-5)?  // This will error
        vibez.spill("This won't print")
    } catch(err) {
        vibez.spill("Caught error: ", err.message)
    }
    
    vibez.spill("✅ Error handling demo complete")
    damn 0
}

slay demo_defer_statements() drip {
    vibez.spill("=== Enhanced Defer Statements Demo ===")
    
    sus resource drip = 42
    
    // Defer cleanup with 'later' keyword
    later {
        vibez.spill("Cleaning up resource: ", resource)
        resource = 0
    }
    
    vibez.spill("Working with resource: ", resource)
    
    // Nested scope with defer
    {
        sus temp_file tea = "temp.txt"
        later {
            vibez.spill("Removing temp file: ", temp_file)
        }
        
        vibez.spill("Using temp file: ", temp_file)
        
        // Early return with proper defer execution
        ready (resource > 40) {
            based -> {
                vibez.spill("Early return triggered")
                damn 1  // Defers still execute in LIFO order
            }
        }
    }
    
    vibez.spill("This line should not be reached")
    damn 0
}

// Main function demonstrating all features
slay main() drip {
    vibez.spill("🚀 CURSED Enhanced Features Demonstration")
    vibez.spill("==========================================")
    
    demo_enhanced_pattern_matching()
    demo_channel_operations()  
    demo_error_handling()
    demo_defer_statements()
    
    vibez.spill("==========================================")
    vibez.spill("✅ All enhanced features demonstrated successfully!")
    
    damn 0
}
