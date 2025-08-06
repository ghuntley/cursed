// Comprehensive test file demonstrating CURSED error diagnostics system
// This file intentionally contains various types of errors to showcase
// the diagnostic capabilities of the CURSED compiler

yeet "testz"

// 1. LEXICAL ERRORS - Testing tokenization issues

slay testLexicalErrors() {
    // Unterminated string literals
    sus message1 tea = "This string is not terminated
    sus message2 tea = 'Character not terminated
    
    // Invalid escape sequences
    sus escaped tea = "Invalid escape: \q and \z"
    
    // Invalid number formats
    sus badNumber1 normie = 123.456.789  // Multiple decimal points
    sus badNumber2 normie = 99999999999999999999999999999  // Overflow
    sus badFloat meal = 1.2e  // Incomplete scientific notation
    
    // Unterminated comment
    /*
    This comment is not properly closed...
    
    sus unreachable normie = 42
}

// 2. SYNTAX/PARSE ERRORS - Testing parser capabilities

slay testParseErrors() {
    // Missing expressions
    sus x normie = 
    sus y tea = "hello" +
    
    // Unbalanced delimiters
    bestie (i drip = 0; i < 10; i = i + 1 {
        vibez.spill("Missing closing parenthesis")
    }
    
    // Wrong bracket types
    sus array = [1, 2, 3)  // Mismatched brackets
    sus function_call = someFunction(1, 2, 3]  // Wrong closing delimiter
    
    // Invalid function declarations
    function invalidSyntax() {  // Should use 'slay' keyword
        damn 42
    }
    
    def alsoInvalid() {  // Not a CURSED keyword
        vibez.spill("error")
    }
    
    // Missing semicolons in struct definitions
    squad Person {
        name tea  // Missing semicolon
        age normie,  // Inconsistent punctuation
        height meal
    }  // Missing semicolon after struct
    
    // Invalid parameter syntax
    slay badParameters(param1, param2 without types) {
        damn param1
    }
    
    // Missing return type arrow
    slay badReturnType() normie  // Should be -> normie
        damn 42
    }
}

// 3. SEMANTIC ERRORS - Testing type system and scoping

slay testSemanticErrors() {
    // Undefined variables
    vibez.spill(undefinedVariable)
    sus result = anotherUndefinedVar + 42
    
    // Undefined functions
    sus value = unknownFunction(1, 2, 3)
    nonExistentProcedure()
    
    // Type mismatches
    sus number normie = "string value"  // Wrong type assignment
    sus text tea = 42  // Number to string
    sus flag lit = "not a boolean"  // String to boolean
    sus floatVal meal = based  // Boolean to float
    
    // Duplicate definitions
    sus duplicateName tea = "first"
    sus duplicateName normie = 42  // Same name, different type
    
    slay duplicateFunction() {
        vibez.spill("first definition")
    }
    
    slay duplicateFunction() {  // Duplicate function
        vibez.spill("second definition")
    }
    
    // Invalid assignments to immutable variables
    facts constantValue normie = 100
    constantValue = 200  // Cannot reassign immutable
    
    // Incorrect arithmetic operations
    sus text tea = "hello"
    sus invalid = text + 42  // Cannot add string and number
    sus alsoBad = based * 3  // Cannot multiply boolean
    
    // Scope violations
    lowkey (based) {
        sus scopedVar normie = 42
    }
    vibez.spill(scopedVar)  // Variable out of scope
    
    // Array/indexing errors
    sus numbers = [1, 2, 3]
    sus outOfBounds = numbers[10]  // Index out of bounds
    sus wrongIndex = numbers["hello"]  // String index on array
}

// 4. INTERFACE AND STRUCT ERRORS

collab Drawable {
    slay draw();
    slay getArea() meal;
}

squad Circle {
    radius meal;
}

squad Rectangle {
    width meal;
    height meal;
}

impl Drawable slay Rectangle {
    slay draw() {
        vibez.spill("Drawing rectangle")
    }
    // Missing getArea implementation
}

slay testInterfaceErrors() {
    sus circle Circle = Circle { radius: 5.0 }
    sus drawable Drawable = circle  // Circle doesn't implement Drawable
    
    sus rect Rectangle = Rectangle { width: 10.0, height: 5.0 }
    sus rectArea = rect.getArea()  // Method not implemented
    
    // Invalid field access
    sus invalidAccess = circle.diameter  // Field doesn't exist
    sus wrongType = rect.width + rect.nonExistent  // Non-existent field
}

// 5. GENERIC TYPE ERRORS

slay genericFunction<T>(value T) T {
    damn value
}

slay testGenericErrors() {
    // Invalid generic instantiation
    sus result1 = genericFunction<UnknownType>(42)  // Unknown type
    
    // Generic constraint violations
    slay constrainedFunction<T where T: Display>(value T) {
        vibez.spill(value.toString())
    }
    
    squad NoDisplay {
        value normie;
    }
    
    sus noDisplayValue = NoDisplay { value: 42 }
    constrainedFunction(noDisplayValue)  // Doesn't implement Display
}

// 6. CONCURRENCY ERRORS

slay testConcurrencyErrors() {
    sus channel dm<normie> = dm<normie>()
    
    // Send to closed channel
    close(channel)
    channel <- 42  // Error: sending to closed channel
    
    // Type mismatch in channel operations
    sus stringChannel dm<tea> = dm<tea>()
    stringChannel <- 42  // Wrong type for channel
    
    // Potential deadlock
    stan {
        channel <- 1
        channel <- 2  // May deadlock if no receiver
    }
    
    // Invalid select usage
    select {
        ready channel <- 42:
            vibez.spill("Sent")
        // Missing default case may cause deadlock
    }
}

// 7. PATTERN MATCHING ERRORS

slay testPatternMatchingErrors() {
    sus value normie = 42
    
    // Non-exhaustive patterns
    match value {
        1 => vibez.spill("one"),
        2 => vibez.spill("two"),
        // Missing default case
    }
    
    // Invalid patterns
    match value {
        x + 1 => vibez.spill("invalid expression pattern"),  // Expressions not allowed
        42.0 => vibez.spill("wrong type"),  // Float pattern for int value
        "string" => vibez.spill("string pattern for int"),  // Type mismatch
        _ => vibez.spill("default"),
    }
    
    // Invalid variable binding
    match value {
        x where x > 100 => vibez.spill("too large"),  // Invalid where clause syntax
        invalid_binding* => vibez.spill("bad binding"),  // Invalid binding syntax
        _ => vibez.spill("default"),
    }
}

// 8. MODULE AND IMPORT ERRORS

yeet "nonexistent/module"  // Module not found
yeet "std/badpath"  // Invalid module path
yeet ""  // Empty import path
yeet "module_with_typo"  // Typo in module name

// 9. ADVANCED TYPE SYSTEM ERRORS

// Recursive type without indirection
squad RecursiveStruct {
    value normie;
    next RecursiveStruct;  // Should be *RecursiveStruct
}

// Invalid type operations
slay testAdvancedTypeErrors() {
    // Invalid casts
    sus number normie = 42
    sus text = number as tea  // Invalid cast from int to string
    
    // Invalid pointer operations
    sus ptr *normie = &number
    sus invalid = *text  // Cannot dereference non-pointer
    
    // Invalid array operations
    sus numbers [5]normie = [1, 2, 3, 4, 5]
    sus tooManyElements [3]normie = [1, 2, 3, 4, 5]  // Array size mismatch
    
    // Invalid slice operations
    sus slice = numbers[1:10]  // Slice end beyond array bounds
    sus negativeSlice = numbers[-1:3]  // Negative index
}

// 10. RUNTIME ERROR DEMONSTRATIONS

slay testRuntimeErrors() {
    // Division by zero
    sus zero normie = 0
    sus divisionByZero = 42 / zero
    
    // Index out of bounds
    sus array [3]normie = [1, 2, 3]
    sus outOfBounds = array[5]
    
    // Null pointer dereference simulation
    sus nullPtr *normie = nah
    sus dereferenceNull = *nullPtr
    
    // Stack overflow (infinite recursion)
    slay infiniteRecursion() {
        infiniteRecursion()  // Will cause stack overflow
    }
    
    infiniteRecursion()
}

// 11. CONTROL FLOW ERRORS

slay testControlFlowErrors() {
    // Unreachable code
    damn 42
    vibez.spill("This code is unreachable")  // Warning: unreachable
    
    // Invalid break/continue
    ghosted  // Break outside of loop
    simp     // Continue outside of loop
    
    // Invalid return types
    slay shouldReturnNumber() normie {
        damn "string instead of number"  // Type mismatch
    }
    
    // Missing return statement
    slay missingReturn() normie {
        vibez.spill("No return statement")
        // Missing return
    }
}

// 12. VISIBILITY AND ACCESS ERRORS

slay testVisibilityErrors() {
    // Accessing private members (if privacy system exists)
    squad PrivateStruct {
        priv hiddenField normie;
        spill publicField tea;
    }
    
    sus private = PrivateStruct { hiddenField: 42, publicField: "hello" }
    vibez.spill(private.hiddenField)  // Accessing private field
}

// 13. MEMORY AND RESOURCE ERRORS

slay testMemoryErrors() {
    // Use after free (if manual memory management exists)
    sus ptr = allocate(normie)
    free(ptr)
    sus invalid = *ptr  // Use after free
    
    // Double free
    sus anotherPtr = allocate(tea)
    free(anotherPtr)
    free(anotherPtr)  // Double free
    
    // Memory leak (unused allocation)
    allocate(normie)  // Allocated but never freed or used
}

// Main function demonstrating comprehensive error handling
slay main() {
    // This main function calls all test functions
    // Most will have errors that should be caught by the diagnostic system
    
    test_start("Comprehensive Error Diagnostics Test")
    
    testLexicalErrors()
    testParseErrors()
    testSemanticErrors()
    testInterfaceErrors()
    testGenericErrors()
    testConcurrencyErrors()
    testPatternMatchingErrors()
    testAdvancedTypeErrors()
    testRuntimeErrors()
    testControlFlowErrors()
    testVisibilityErrors()
    testMemoryErrors()
    
    print_test_summary()
    
    vibez.spill("If you see this message, some error detection failed!")
}
