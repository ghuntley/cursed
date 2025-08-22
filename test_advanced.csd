# Test 5: Advanced features - concurrency, error handling, etc.

# Import required modules
yeet "vibez"
yeet "concurrenz" 

# Test error handling with yikes/fam pattern
slay divide(a drip, b drip) yikes<drip> {
    ready (b == 0) {
        yikes "Division by zero!"
    }
    damn a / b
}

# Test error handling
sus result drip = divide(10, 2) fam {
    when "Division by zero!" -> {
        vibez.spill("Caught division by zero error")
        damn 0
    }
    when _ -> {
        vibez.spill("Caught unknown error")
        damn -1
    }
}
vibez.spill("Division result:", result)

# Test error case
sus error_result drip = divide(10, 0) fam {
    when "Division by zero!" -> {
        vibez.spill("Successfully caught division by zero")
        damn -999
    }
}
vibez.spill("Error result:", error_result)

# Test concurrency (goroutines and channels) if implemented
vibez.spill("Testing basic concurrency...")

# Simple goroutine test
go {
    vibez.spill("Hello from goroutine!")
}

# Channel operations test (if channels work)
sus ch chan<drip> = make_channel()

go {
    ch <- 42
    vibez.spill("Sent 42 to channel")
}

go {
    sus value drip = <-ch
    vibez.spill("Received from channel:", value)
}

# Wait a moment for goroutines (if sleep works)
# sleep(100)

# Test arrays and more complex data structures
sus array []drip = [1, 2, 3, 4, 5]
vibez.spill("Array:", array[0], array[1], array[2])

# Test struct-like behavior (if available)
# squad Person {
#     name tea,
#     age drip
# }
# 
# sus person Person = {name: "Alice", age: 30}
# vibez.spill("Person:", person.name, person.age)

vibez.spill("Advanced features test complete")
