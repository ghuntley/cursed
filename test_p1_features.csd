yeet "testz"
yeet "vibez"
yeet "concurrenz"

// Test generics with constraints
slay identity[T](value T) T {
    damn value
}

slay add[T: Numeric](a T, b T) T {
    damn a + b
}

squad Box[T] {
    spill value T
}

// Test error propagation with context
yikes CustomError {
    spill message tea
    spill code drip
}

slay risky_operation() yikes CustomError {
    lowkey based {
        shook CustomError{message: "Something went wrong", code: 500}
    }
    damn 42
}

slay test_error_handling() {
    fam {
        sus result drip = risky_operation()
        vibez.spill("Operation succeeded")
    } catch err {
        vibez.spill("Caught error with stack trace")
    }
}

// Test select statements for concurrency
slay test_select_concurrency() {
    sus ch1 dm<drip> = concurrenz.make_channel(1)
    sus ch2 dm<drip> = concurrenz.make_channel(1)
    
    stan {
        concurrenz.send(ch1, 42)
    }
    
    stan {
        concurrenz.send(ch2, 100)
    }
    
    ready {
        mood val1 := concurrenz.recv(ch1) -> {
            vibez.spill("Received from channel 1")
        }
        mood val2 := concurrenz.recv(ch2) -> {
            vibez.spill("Received from channel 2")
        }
        basic -> {
            vibez.spill("No channels ready")
        }
    }
}

slay test_generics() {
    // Test generic function calls
    sus int_result drip = identity[drip](42)
    sus float_result meal = identity[meal](3.14)
    
    // Test generic with constraints
    sus sum drip = add[drip](10, 20)
    
    // Test generic struct
    sus int_box Box[drip] = Box[drip]{value: 123}
    sus str_box Box[tea] = Box[tea]{value: "hello"}
    
    vibez.spill("Generics working!")
}

slay main() {
    vibez.spill("Testing P1 High Priority Features")
    
    test_generics()
    test_error_handling() 
    test_select_concurrency()
    
    vibez.spill("All P1 features working!")
    damn 0
}
