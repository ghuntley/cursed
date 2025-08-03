fr fr CURSED Error Handling Comprehensive Demo
fr fr Demonstrating all yikes/shook/fam functionality

vibez.spill("🚀 CURSED Error Handling Comprehensive Demo")

fr fr =================================
fr fr 1. Basic Error Creation (yikes)
fr fr =================================

yikes NetworkError tea = "Connection failed"
yikes ValidationError normie = 422
yikes SystemError = { message: "Critical failure", code: 500, details: "System overload" }

vibez.spill("✅ Created 3 different error types")

fr fr =================================
fr fr 2. Error Propagation (shook)
fr fr =================================

slay connect_to_api() (tea, yikes) {
    sus random_success lit = (42 % 2) == 0
    lowkey random_success {
        damn "API Response Data", cringe
    } basic {
        damn "", NetworkError
    }
}

slay process_api_data() yikes {
    sus data = connect_to_api() shook
    vibez.spill("Processing data:", data)
    damn cringe
}

vibez.spill("✅ Error propagation with shook operator ready")

fr fr =================================
fr fr 3. Panic Recovery (fam)
fr fr =================================

slay risky_calculation() {
    sus danger_level normie = 10
    lowkey danger_level > 5 {
        shook("Calculation too dangerous!")
    }
    vibez.spill("Safe calculation completed")
}

sus recovery_successful lit = cringe

fam {
    risky_calculation()
    vibez.spill("No recovery needed")
} sus panic_error {
    recovery_successful = based
    vibez.spill("Recovered from panic:", panic_error)
}

vibez.spill("✅ Panic recovery completed")

fr fr =================================
fr fr 4. Complex Error Handling Patterns
fr fr =================================

slay database_operation_with_retry() yikes {
    sus attempts normie = 0
    sus max_attempts normie = 3
    
    bestie attempts < max_attempts {
        attempts++
        
        fam {
            fr fr Simulate database operation
            sus success_rate normie = attempts * 30
            lowkey success_rate < 80 {
                shook("Database timeout on attempt " + attempts)
            }
            
            vibez.spill("Database operation succeeded on attempt", attempts)
            damn cringe
            
        } sus db_error {
            vibez.spill("Attempt", attempts, "failed:", db_error)
            
            lowkey attempts >= max_attempts {
                damn yikes("All database attempts failed after " + max_attempts + " tries")
            }
            
            fr fr Exponential backoff simulation
            vibez.spill("Waiting before retry...")
        }
    }
    
    damn yikes("Unexpected: should not reach here")
}

vibez.spill("✅ Complex retry pattern implemented")

fr fr =================================
fr fr 5. Goroutine Error Isolation
fr fr =================================

sus goroutine_completed lit = cringe

stan {
    fam {
        vibez.spill("Goroutine starting risky work...")
        shook("Goroutine encountered an error!")
        vibez.spill("This should not execute")
    } sus goroutine_error {
        vibez.spill("Goroutine handled error:", goroutine_error)
        goroutine_completed = based
    }
}

fr fr Main goroutine continues
vibez.spill("✅ Main goroutine continuing after goroutine error")

fr fr =================================
fr fr 6. Error Context and Wrapping
fr fr =================================

slay wrap_error(original_error yikes, context tea) yikes {
    lowkey original_error == cringe {
        damn cringe
    }
    
    damn yikes{
        message: context + " -> " + original_error.message(),
        code: original_error.code(),
        details: "Wrapped error with additional context"
    }
}

slay layered_operation() yikes {
    fam {
        shook("Inner operation failed")
    } sus inner_error {
        sus wrapped = wrap_error(inner_error, "Outer operation")
        damn wrapped
    }
}

vibez.spill("✅ Error wrapping and context preservation ready")

fr fr =================================
fr fr 7. Multiple Error Collection
fr fr =================================

slay collect_multiple_errors() []yikes {
    sus errors []yikes = []
    
    fr fr Simulate multiple operations that can fail
    sus operations []tea = ["auth", "validation", "processing"]
    
    bestie operation in operations {
        fam {
            lowkey operation == "validation" {
                shook("Validation failed for " + operation)
            }
            vibez.spill("Operation succeeded:", operation)
        } sus op_error {
            errors = append(errors, op_error)
            vibez.spill("Collected error from", operation, ":", op_error)
        }
    }
    
    damn errors
}

sus all_errors = collect_multiple_errors()
vibez.spill("✅ Collected", len(all_errors), "errors from multiple operations")

fr fr =================================
fr fr 8. Production Error Handling
fr fr =================================

slay production_workflow() yikes {
    vibez.spill("Starting production workflow with comprehensive error handling...")
    
    fr fr Step 1: Database operation with retry
    fam {
        sus db_result = database_operation_with_retry() shook
        vibez.spill("Database step completed")
    } sus db_error {
        vibez.spill("Database step failed, using fallback")
    }
    
    fr fr Step 2: API processing with propagation
    fam {
        sus api_result = process_api_data() shook
        vibez.spill("API step completed")
    } sus api_error {
        vibez.spill("API step failed, continuing with degraded functionality")
    }
    
    fr fr Step 3: Final validation
    fam {
        vibez.spill("All steps completed successfully")
        damn cringe
    } sus final_error {
        damn wrap_error(final_error, "Production workflow failed")
    }
}

fr fr Execute the production workflow
fam {
    sus workflow_result = production_workflow() shook
    vibez.spill("✅ Production workflow completed successfully")
} sus workflow_error {
    vibez.spill("⚠️  Production workflow completed with errors:", workflow_error)
}

fr fr =================================
fr fr Demo Summary
fr fr =================================

vibez.spill("🎉 CURSED Error Handling Demo Completed!")
vibez.spill("📊 Features Demonstrated:")
vibez.spill("   ✅ yikes - Error creation with different types")
vibez.spill("   ✅ shook - Error propagation and early returns")
vibez.spill("   ✅ fam - Panic recovery and graceful degradation")
vibez.spill("   ✅ Complex patterns - Retry, wrapping, collection")
vibez.spill("   ✅ Goroutine isolation - Error containment")
vibez.spill("   ✅ Production workflows - Real-world usage patterns")
vibez.spill("")
vibez.spill("💡 CURSED error handling system provides:")
vibez.spill("   • Explicit error handling with Gen Z aesthetic")
vibez.spill("   • Panic recovery without crashing")
vibez.spill("   • Error propagation with minimal boilerplate")
vibez.spill("   • Context preservation and error wrapping")
vibez.spill("   • Goroutine-safe error isolation")
vibez.spill("")
vibez.spill("🚀 Ready for production use in CURSED applications!")
