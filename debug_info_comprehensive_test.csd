fr fr Comprehensive CURSED Debug Information Test Program
fr fr This program tests all debug info features including:
fr fr - Functions with debug symbols
fr fr - Variables with proper debug types
fr fr - Structs with field debug info
fr fr - Interfaces with method debug info
fr fr - Source location tracking
fr fr - Scope management

yeet "testz"

fr fr Define a CURSED struct with debug info
squad Point {
    spill x meal
    spill y meal
    spill label tea
}

fr fr Define an interface with debug info
collab Drawable {
    slay draw()
    slay area() meal
}

fr fr Implement the interface
flex Point => Drawable {
    slay draw() {
        vibez.spillf("Drawing point at ({}, {}): {}", x, y, label)
    }
    
    slay area() meal {
        damn 0.0  fr fr Points have no area
    }
}

fr fr Function with debug info and local variables
slay calculate_distance(p1 Point, p2 Point) meal {
    fr fr Local variables with debug info
    sus dx meal = p1.x - p2.x
    sus dy meal = p1.y - p2.y
    sus distance_squared meal = dx * dx + dy * dy
    
    fr fr CURSED doesn't have sqrt, so we'll return squared distance
    damn distance_squared
}

fr fr Function with different CURSED types for debug info testing
slay type_showcase(
    int_param normie,
    string_param tea,
    float_param meal,
    bool_param lit,
    small_param smol,
    large_param thicc,
    byte_param sip
) {
    vibez.spillf("normie: {}", int_param)
    vibez.spillf("tea: {}", string_param)
    vibez.spillf("meal: {}", float_param)
    vibez.spillf("lit: {}", bool_param)
    vibez.spillf("smol: {}", small_param)
    vibez.spillf("thicc: {}", large_param)
    vibez.spillf("sip: {}", byte_param)
}

fr fr Main function with comprehensive debug testing
slay main() {
    test_start("Debug Info Comprehensive Test")
    
    fr fr Create struct instances with debug info
    sus origin Point = Point{x: 0.0, y: 0.0, label: "Origin"}
    sus destination Point = Point{x: 3.0, y: 4.0, label: "Destination"}
    
    fr fr Test function calls with debug info
    sus distance meal = calculate_distance(origin, destination)
    vibez.spillf("Distance squared: {}", distance)
    
    fr fr Test interface method calls with debug info
    origin.draw()
    destination.draw()
    
    sus origin_area meal = origin.area()
    vibez.spillf("Origin area: {}", origin_area)
    
    fr fr Test all CURSED types for debug info
    type_showcase(
        42,                    fr fr normie
        "Debug test string",   fr fr tea
        3.14159,              fr fr meal
        based,                fr fr lit
        127,                  fr fr smol
        9223372036854775807,  fr fr thicc
        255                   fr fr sip
    )
    
    fr fr Test local variables in different scopes
    sus outer_var normie = 100
    {
        sus inner_var normie = 200
        sus scoped_string tea = "Inner scope"
        vibez.spillf("Outer: {}, Inner: {}, String: {}", outer_var, inner_var, scoped_string)
    }
    
    fr fr Test arrays and complex data structures
    sus numbers []normie = [1, 2, 3, 4, 5]
    sus coordinates []Point = [origin, destination]
    
    vibez.spillf("Numbers array length: {}", numbers.len())
    vibez.spillf("Coordinates array length: {}", coordinates.len())
    
    fr fr Loop with debug info
    sus counter normie = 0
    bestie i := 0; i < numbers.len(); i = i + 1 {
        counter = counter + numbers[i]
        vibez.spillf("Running sum: {}", counter)
    }
    
    assert_eq_int(counter, 15)
    print_test_summary()
    
    vibez.spill("🎯 Debug info comprehensive test completed!")
}

main()
