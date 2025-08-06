// Simple CURSED Memory Management Demo
// Tests basic memory allocation and tracking

yeet "vibez"

// Basic memory allocations
sus small_object drip = 42
sus string_object tea = "Hello, CURSED memory management!"
sus array_object = [1, 2, 3, 4, 5]

// Create a structured object
sus complex_object = {
    name: "test_object",
    value: 100,
    nested: {
        items: [10, 20, 30],
        active: based
    }
}

// Print the objects to verify they work
vibez.spill("Small object: " + small_object.to_string())
vibez.spill("String object: " + string_object)
vibez.spill("Array length: " + array_object.length.to_string())
vibez.spill("Complex object name: " + complex_object.name)
vibez.spill("Complex object nested active: " + complex_object.nested.active.to_string())

// Test basic function calls with objects
slay process_object(obj) {
    vibez.spill("Processing object: " + obj.to_string())
    damn obj
}

sus result = process_object(small_object)
vibez.spill("Function result: " + result.to_string())

// Test loops with memory allocation
sus loop_objects = []
bestie (sus i drip = 0; i < 10; i = i + 1) {
    sus obj = {
        id: i,
        data: "Object " + i.to_string()
    }
    loop_objects.append(obj)
}

vibez.spill("Created " + loop_objects.length.to_string() + " objects in loop")

// Test conditional allocation
grr (loop_objects.length > 5) {
    sus bonus_object = {type: "bonus", value: 999}
    vibez.spill("Bonus object created: " + bonus_object.type)
}

vibez.spill("Memory management demo completed successfully!")
