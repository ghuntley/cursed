// Test CURSED error handling syntax

slay test_error_handling() {
    // Error variable declaration
    yikes err

    // Error variable with type
    yikes typed_err : tea

    // Error variable with value
    yikes valued_err = yikes("Something went wrong")

    // Error propagation
    sus result = risky_operation() shook

    // Panic recovery block
    fam {
        dangerous_operation()
    } sus panic_value {
        vibez.spill("Recovered from panic:", panic_value)
    }

    // Error value expressions
    sus simple_error = yikes("File not found")
    sus detailed_error = yikes{
        message: "Connection failed",
        code: 500,
        details: "Unable to connect to server"
    }
}

vibez.spill("Error handling syntax test compiled successfully!")
