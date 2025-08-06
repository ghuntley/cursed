// JIT Advanced Features Test - Tests complex language features

// Interface definition
collab Drawable {
    slay draw() tea
}

// Struct with interface implementation
squad Circle {
    spill radius meal
    spill x meal  
    spill y meal
}

// Implement interface for Circle
slay (c Circle) draw() tea {
    damn "Drawing circle with radius " + c.radius.(tea) + " at (" + c.x.(tea) + ", " + c.y.(tea) + ")"
}

squad Rectangle {
    spill width meal
    spill height meal
    spill x meal
    spill y meal
}

slay (r Rectangle) draw() tea {
    damn "Drawing rectangle " + r.width.(tea) + "x" + r.height.(tea) + " at (" + r.x.(tea) + ", " + r.y.(tea) + ")"
}

// Generic-like function using type assertions
slay process_drawable(shape Drawable) {
    sus description tea = shape.draw()
    vibez.spill(description)
}

// Channel operations test
slay test_channels() {
    sus ch drip<normie> = dm_make<normie>(10)
    
    // Send values
    stan {
        sus i normie = 0
        bestie (i < 5) {
            dm_send(ch, i)
            i = i + 1
        }
    }
    
    // Receive values
    stan {
        sus received normie = 0
        bestie (received < 5) {
            sus val normie = dm_recv(ch)
            vibez.spillf("Received: {}", val)
            received = received + 1
        }
    }
}

// Pattern matching test
slay test_pattern_matching(value normie) tea {
    clash value {
        0 => damn "Zero"
        1 => damn "One" 
        2 => damn "Two"
        3..10 => damn "Small number"
        _ => damn "Large number"
    }
}

// Error handling with custom errors
slay divide_safe(a normie, b normie) normie {
    lowkey (b == 0) {
        yikes CustomError{ message: "Division by zero", code: 1001 }
    }
    damn a / b
}

// Tuple operations
slay test_tuples() {
    sus point (normie, normie) = (10, 20)
    sus person (tea, normie) = ("Alice", 25)
    
    vibez.spillf("Point: ({}, {})", point.0, point.1)
    vibez.spillf("Person: {} is {} years old", person.0, person.1)
}

// Lambda expressions
slay test_lambdas() {
    sus numbers [5]normie = [1, 2, 3, 4, 5]
    
    // Map operation with lambda
    sus doubled [5]normie
    sus i normie = 0
    bestie (i < 5) {
        doubled[i] = (|x normie| x * 2)(numbers[i])
        i = i + 1
    }
    
    vibez.spill("Original: [1, 2, 3, 4, 5]")
    vibez.spillf("Doubled: [{}, {}, {}, {}, {}]", doubled[0], doubled[1], doubled[2], doubled[3], doubled[4])
}

slay main() {
    vibez.spill("🔬 Starting JIT Advanced Features Tests")
    
    // Test struct and interface
    vibez.spill("\n📐 Testing Structs and Interfaces...")
    sus circle Circle = Circle{ radius: 5.0, x: 10.0, y: 15.0 }
    sus rect Rectangle = Rectangle{ width: 20.0, height: 10.0, x: 0.0, y: 0.0 }
    
    process_drawable(circle.(Drawable))
    process_drawable(rect.(Drawable))
    
    // Test channels
    vibez.spill("\n📡 Testing Channels...")
    test_channels()
    
    // Test pattern matching
    vibez.spill("\n🎯 Testing Pattern Matching...")
    sus i normie = 0
    bestie (i <= 15) {
        sus pattern_result tea = test_pattern_matching(i)
        vibez.spillf("{}: {}", i, pattern_result)
        i = i + 5
    }
    
    // Test error handling
    vibez.spill("\n⚠️ Testing Error Handling...")
    ready {
        sus result normie = divide_safe(10, 2)
        vibez.spillf("10 / 2 = {}", result)
        
        sus error_result normie = divide_safe(10, 0)  // This will throw
        vibez.spillf("Unexpected result: {}", error_result)
    } yikes (err) {
        vibez.spillf("Caught expected error: {}", err.message)
    }
    
    // Test tuples
    vibez.spill("\n📦 Testing Tuples...")
    test_tuples()
    
    // Test lambdas
    vibez.spill("\n🧩 Testing Lambda Expressions...")
    test_lambdas()
    
    vibez.spill("\n✨ JIT Advanced Features Tests Complete")
}

main()
