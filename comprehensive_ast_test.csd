// Comprehensive test of all new AST types

// Array expressions
sus numbers = [1, 2, 3, 4, 5]
sus names = ["Alice", "Bob", "Charlie"]
sus mixed = [1, "hello", based]

// Struct definitions and struct expressions
squad Point {
    spill x drip
    spill y drip
}

squad Circle {
    spill center Point
    spill radius drip
}

// Struct literals with field initializers
sus origin = Point{
    x: 0.0,
    y: 0.0
}

sus myCircle = Circle{
    center: Point{x: 10, y: 20},
    radius: 5.0
}

// Method calls on objects
slay test_methods() {
    sus p = Point{x: 1, y: 2}
    p.toString()
    p.distance(origin)
    myCircle.getArea()
}

// Error handling with yikes/shook/fam
slay risky_operation() {
    yikes "Division by zero", 1001
}

slay error_handling_example() {
    fam {
        sus result = shook risky_operation()
        vibez.spill("Success!")
    } catch(error) {
        vibez.spill("Caught error")
    }
}

// Chained method calls
slay chained_calls() {
    sus text = "hello"
    text.toUpper().trim().length()
}

// Array access and method calls combined
slay array_method_combo() {
    sus points = [Point{x: 1, y: 2}, Point{x: 3, y: 4}]
    points[0].distance(points[1])
}
