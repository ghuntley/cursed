fr fr Advanced Pattern Matching Validation Test
yeet "testz"

fr fr Define enums for pattern matching
peep Color {
    Red,
    Green,
    Blue,
    RGB(normie, normie, normie)
}

peep Shape {
    Circle(meal),
    Rectangle(meal, meal),
    Point(meal, meal)
}

peep Option<T> {
    Some(T),
    None
}

fr fr Define structs for pattern matching
squad Person {
    spill name tea
    spill age normie
    spill active lit
}

squad Point2D {
    spill x meal
    spill y meal
}

slay test_pattern_matching() {
    test_start("Basic Pattern Matching Test")
    
    fr fr Test enum pattern matching
    sus color Color = RGB(255, 128, 0)
    sus color_name tea = match color {
        Red => "red",
        Green => "green", 
        Blue => "blue",
        RGB(r, g, b) => "custom_rgb"
    }
    assert_eq_string(color_name, "custom_rgb")
    
    test_start("Struct Pattern Matching Test")
    
    fr fr Test struct destructuring
    sus person Person = Person{name: "Alice", age: 30, active: based}
    sus greeting tea = match person {
        Person{name: "Alice", age, active: based} => "Hello Alice!",
        Person{name, age, active: cringe} => "Inactive user",
        Person{name, age, active} if age > 65 => "Senior user",
        _ => "Regular user"
    }
    assert_eq_string(greeting, "Hello Alice!")
    
    test_start("Guard Expression Test")
    
    fr fr Test pattern matching with guards
    sus number normie = 42
    sus classification tea = match number {
        x if x < 0 => "negative",
        x if x == 0 => "zero",
        x if x > 0 && x < 10 => "small_positive",
        x if x >= 10 && x < 100 => "medium_positive",
        _ => "large_positive"
    }
    assert_eq_string(classification, "medium_positive")
    
    test_start("Complex Pattern Matching Test")
    
    fr fr Test nested pattern matching
    sus point Point2D = Point2D{x: 3.0, y: 4.0}
    sus quadrant tea = match point {
        Point2D{x: 0.0, y: 0.0} => "origin",
        Point2D{x, y} if x > 0.0 && y > 0.0 => "first_quadrant",
        Point2D{x, y} if x < 0.0 && y > 0.0 => "second_quadrant", 
        Point2D{x, y} if x < 0.0 && y < 0.0 => "third_quadrant",
        Point2D{x, y} if x > 0.0 && y < 0.0 => "fourth_quadrant",
        _ => "on_axis"
    }
    assert_eq_string(quadrant, "first_quadrant")
    
    test_start("Option Pattern Matching Test")
    
    fr fr Test generic enum pattern matching
    sus some_value Option<normie> = Some(42)
    sus none_value Option<normie> = None
    
    sus result1 normie = match some_value {
        Some(x) => x * 2,
        None => 0
    }
    
    sus result2 normie = match none_value {
        Some(x) => x * 2,
        None => -1
    }
    
    assert_eq_int(result1, 84)
    assert_eq_int(result2, -1)
    
    test_start("Exhaustiveness Checking Test")
    
    fr fr Test that all enum variants are covered
    sus shape Shape = Circle(5.0)
    sus area meal = match shape {
        Circle(radius) => 3.14159 * radius * radius,
        Rectangle(width, height) => width * height,
        Point(x, y) => 0.0  fr fr Points have no area
    }
    assert_eq_float(area, 78.53975)
    
    print_test_summary()
}

test_pattern_matching()
