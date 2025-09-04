fr fr CURSED Panic/Recovery Example
fr fr Demonstrates the panic/recovery system with Gen Z slang

slay main_character() {
    println("Starting panic/recovery demo...")
    
    // Example 1: Basic panic handling
    catch {
        sus risky_value = divide_by_zero(10, 0)
        println("This won't print")
    } recover {
        println("Caught a panic! Continuing...")
    }
    
    // Example 2: Conditional panic
    sus user_input = -5
    lowkey (user_input < 0) {
        yeet_error "Invalid input: value must be positive"
    }
    
    // Example 3: Nested panic handling
    catch {
        catch {
            sus dangerous_operation = risky_calculation()
            println("Inner operation completed")
        } recover {
            println("Inner recovery triggered")
            yeet_error "Re-throwing error from inner handler"
        }
    } recover {
        println("Outer recovery caught the re-thrown error")
    }
    
    // Example 4: Recovery with error information
    catch {
        process_file("nonexistent.txt")
    } recover {
        println("File operation failed, using default values")
        // Continue with default behavior
    }
    
    println("Demo completed successfully!")
}

slay divide_by_zero(normie a, normie b) normie {
    lowkey (b == 0) {
        yeet_error "Division by zero is not allowed!"
    }
    damn a / b
}

slay risky_calculation() normie {
    sus result = 42
    lowkey (result > 40) {
        yeet_error "Result too large!"
    }
    damn result
}

slay process_file(tea filename) tea {
    // Simulate file processing that might fail
    lowkey (filename == "nonexistent.txt") {
        yeet_error "File not found"
    }
    damn "File processed successfully"
}
