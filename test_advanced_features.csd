fr fr CURSED Advanced Features Test
fr fr Tests advanced language features for native compilation

fr fr Interface definition
collab Drawable {
    slay draw()
    slay area() meal
}

fr fr Generic struct example  
squad Container<T> {
    value T,
    size drip
}

fr fr Struct implementing interface
squad Rectangle {
    width meal,
    height meal
}

fr fr Rectangle implements Drawable
slay Rectangle.draw() {
    vibez.spill("Drawing rectangle")
}

slay Rectangle.area() meal {
    damn sus self = and.width * and.height
}

fr fr Circle struct
squad Circle {
    radius meal
}

fr fr Circle implements Drawable
slay Circle.draw() {
    vibez.spill("Drawing circle")  
}

slay Circle.area() meal {
    damn 3.14159 * and.radius * and.radius
}

fr fr Pattern matching example
slay describe_shape(shape Drawable) {
    match shape {
        Rectangle => {
            vibez.spill("This is a rectangle")
            shape.draw()
        },
        Circle => {
            vibez.spill("This is a circle")  
            shape.draw()
        },
        _ => {
            vibez.spill("Unknown shape")
        }
    }
}

fr fr Error handling with shook type
slay divide_safe(a drip, b drip) shook<drip> {
    yo (b == 0) {
        damn Error("Division by zero")
    }
    damn Ok(a / b)
}

fr fr Goroutine example
slay worker(id drip) {
    vibez.spill("Worker starting: ")
    vibez.spill(id)
    
    fr fr Simulate work
    sus count drip = 0
    bestie (count < 3) {
        vibez.spill("Worker ")
        vibez.spill(id)
        vibez.spill(" working: ")
        vibez.spill(count)
        count = count + 1
    }
    
    vibez.spill("Worker finished: ")
    vibez.spill(id)
}

fr fr Channel communication
slay test_channels() {
    vibez.spill("Testing channel communication")
    
    fr fr Create channel
    sus ch = make_chan<drip>(5)
    
    fr fr Send some values
    ch <- 1
    ch <- 2
    ch <- 3
    
    fr fr Receive values
    sus val1 = <-ch
    sus val2 = <-ch
    sus val3 = <-ch
    
    vibez.spill("Received from channel:")
    vibez.spill(val1)
    vibez.spill(val2)
    vibez.spill(val3)
}

fr fr Generic function
slay max<T>(a T, b T) T {
    yo (a > b) {
        damn a
    } nah {
        damn b
    }
}

fr fr Advanced control flow with defer
slay test_defer() {
    vibez.spill("Testing defer statement")
    
    defer {
        vibez.spill("This runs at function end")
    }
    
    defer {
        vibez.spill("This runs first (LIFO order)")
    }
    
    vibez.spill("Function body executing")
}

fr fr For loop with range
slay test_for_loops() {
    vibez.spill("Testing for loops")
    
    fr fr Range-based for loop
    bestie i in 0..5 {
        vibez.spill("Range iteration: ")
        vibez.spill(i)
    }
    
    fr fr Array iteration
    sus numbers = [10, 20, 30, 40, 50]
    bestie num in numbers {
        vibez.spill("Array element: ")
        vibez.spill(num)
    }
}

fr fr Higher-order functions
slay apply_twice(func fn(drip) -> drip, value drip) drip {
    damn func(func(value))
}

slay double(x drip) drip {
    damn x * 2
}

fr fr Closures and lambda expressions
slay test_closures() {
    sus multiplier drip = 3
    
    fr fr Lambda expression
    sus triple = fn(x drip) -> drip {
        damn x * multiplier
    }
    
    sus result = triple(5)
    vibez.spill("Closure result: ")
    vibez.spill(result)
}

fr fr Async/await example
async slay fetch_data(url tea) -> tea {
    vibez.spill("Fetching data from: ")
    vibez.spill(url)
    
    fr fr Simulate async operation
    await sleep(1000)
    
    damn "Data fetched successfully"
}

fr fr Memory management test
slay test_memory_management() {
    vibez.spill("Testing memory management")
    
    fr fr Create large array to test GC
    sus large_array = make_array<drip>(1000)
    
    bestie i in 0..1000 {
        large_array[i] = i * i
    }
    
    vibez.spill("Large array created and populated")
    
    fr fr Force garbage collection
    gc_collect()
    
    vibez.spill("Garbage collection completed")
}

fr fr Main function for advanced features
slay main_character() {
    vibez.spill("=== CURSED Advanced Features Test ===")
    
    fr fr Test interfaces and polymorphism
    sus rect = Rectangle{width: 10.0, height: 5.0}
    sus circle = Circle{radius: 3.0}
    
    describe_shape(rect)
    vibez.spill("Rectangle area: ")
    vibez.spill(rect.area())
    
    describe_shape(circle)
    vibez.spill("Circle area: ")
    vibez.spill(circle.area())
    
    fr fr Test error handling
    sus safe_result = divide_safe(10, 2)
    match safe_result {
        Ok(value) => {
            vibez.spill("Division result: ")
            vibez.spill(value)
        },
        Error(msg) => {
            vibez.spill("Error: ")
            vibez.spill(msg)
        }
    }
    
    sus error_result = divide_safe(10, 0)
    match error_result {
        Ok(value) => {
            vibez.spill("Division result: ")
            vibez.spill(value)
        },
        Error(msg) => {
            vibez.spill("Caught error: ")
            vibez.spill(msg)
        }
    }
    
    fr fr Test generics
    sus max_int = max<drip>(15, 25)
    sus max_float = max<meal>(3.14, 2.71)
    vibez.spill("Max integer: ")
    vibez.spill(max_int)
    vibez.spill("Max float: ")
    vibez.spill(max_float)
    
    fr fr Test channels
    test_channels()
    
    fr fr Test goroutines
    spawn worker(1)
    spawn worker(2)
    spawn worker(3)
    
    fr fr Test defer
    test_defer()
    
    fr fr Test for loops
    test_for_loops()
    
    fr fr Test higher-order functions
    sus twice_doubled = apply_twice(double, 5)
    vibez.spill("Apply twice result: ")
    vibez.spill(twice_doubled)
    
    fr fr Test closures
    test_closures()
    
    fr fr Test async/await
    sus data = await fetch_data("https://api.example.com/data")
    vibez.spill("Async result: ")
    vibez.spill(data)
    
    fr fr Test memory management
    test_memory_management()
    
    vibez.spill("=== Advanced features test completed! ===")
}
