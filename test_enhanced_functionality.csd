yeet "testz"
yeet "mathz"
yeet "vibez" 
yeet "stringz"
yeet "atomic_drip"
yeet "concurrenz"

slay test_enhanced_features() {
    vibez.spill("Testing enhanced CURSED functionality")
    
    // Test enhanced variables and expressions
    sus x drip = 42
    sus y drip = mathz.add(x, 8)
    sus result drip = 2 + 3 * 4 + 5  // Test operator precedence
    
    vibez.spill("Enhanced arithmetic working!")
    
    // Test string operations with new runtime support
    sus name tea = "Alice"
    sus greeting tea = stringz.concat("Hello, ", name)
    sus char_at_0 normie = stringz.char_at(name, 0)
    
    vibez.spill("String operations working!")
    
    // Test array operations
    sus numbers [5]drip = [1, 2, 3, 4, 5]
    sus first drip = numbers[0]
    
    vibez.spill("Array operations working!")
    
    // Test struct with enhanced field access
    squad Point {
        spill x drip
        spill y drip
    }
    
    sus p1 Point = Point{x: 10, y: 20}
    sus px drip = p1.x
    
    vibez.spill("Struct operations working!")
    
    // Test atomic operations
    sus atomic_counter atomic_drip.AtomicI32
    atomic_drip.store(atomic_counter, 100)
    sus counter_value drip = atomic_drip.load(atomic_counter)
    
    vibez.spill("Atomic operations working!")
    
    // Test defer statements  
    later {
        vibez.spill("Defer cleanup executed!")
    }
    
    // Test pattern matching
    sus value drip = 42
    match value {
        mood 0 -> vibez.spill("Zero")
        mood 42 -> vibez.spill("Answer to everything!")
        basic -> vibez.spill("Other")
    }
    
    vibez.spill("All enhanced features working!")
}

slay main() {
    test_enhanced_features()
    damn 0
}
