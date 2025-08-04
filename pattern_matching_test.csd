fr fr Test pattern matching with multiple enum variants
yeet "testz"

fr fr Define enum with multiple variants
enum Color {
    Red,
    Green(normie),
    Blue(normie, normie),
    Custom(tea),
}

fr fr Test function that uses pattern matching
slay test_pattern_matching() {
    sus colors []Color = [
        Color.Red,
        Color.Green(128),
        Color.Blue(100, 200),
        Color.Custom("purple"),
    ]
    
    bestie color in colors {
        sus result tea = match color {
            Color.Red => "red color",
            Color.Green(value) => spillf("green with value: {}", value),
            Color.Blue(r, g) => spillf("blue with r:{}, g:{}", r, g),
            Color.Custom(name) => spillf("custom color: {}", name),
        }
        vibez.spill(result)
    }
}

fr fr Test with guards and nested patterns
slay test_pattern_guards() {
    sus test_color Color = Color.Green(255)
    
    sus result tea = match test_color {
        Color.Red => "basic red",
        Color.Green(value) if value > 200 => "bright green",
        Color.Green(value) => spillf("dim green: {}", value),
        Color.Blue(r, g) if r == g => "equal blue components",
        Color.Blue(r, g) => spillf("different blue r:{}, g:{}", r, g),
        Color.Custom(name) if name.starts_with("dark") => "dark custom color",
        Color.Custom(name) => spillf("other custom: {}", name),
    }
    
    vibez.spill(result)
}

fr fr Test exhaustiveness checking
slay test_exhaustiveness() {
    sus color Color = Color.Red
    
    fr fr This should compile - all variants covered
    sus result1 tea = match color {
        Color.Red => "red",
        Color.Green(_) => "green",
        Color.Blue(_, _) => "blue", 
        Color.Custom(_) => "custom",
    }
    
    fr fr This should also compile - wildcard covers remaining
    sus result2 tea = match color {
        Color.Red => "red",
        Color.Green(value) if value > 100 => "bright green",
        _ => "other",
    }
    
    vibez.spill(result1)
    vibez.spill(result2)
}

fr fr Test nested enum patterns
enum Shape {
    Circle(normie),
    Rectangle(normie, normie),
    Colored(Color, normie),
}

slay test_nested_patterns() {
    sus shapes []Shape = [
        Shape.Circle(10),
        Shape.Rectangle(5, 8),
        Shape.Colored(Color.Red, 100),
        Shape.Colored(Color.Green(128), 50),
    ]
    
    bestie shape in shapes {
        sus description tea = match shape {
            Shape.Circle(radius) => spillf("circle r:{}", radius),
            Shape.Rectangle(w, h) => spillf("rect {}x{}", w, h),
            Shape.Colored(Color.Red, area) => spillf("red shape, area:{}", area),
            Shape.Colored(Color.Green(intensity), area) => spillf("green shape intensity:{}, area:{}", intensity, area),
            Shape.Colored(_, area) => spillf("other colored shape, area:{}", area),
        }
        vibez.spill(description)
    }
}

fr fr Run all tests
test_start("Pattern Matching with Multiple Variants")

test_pattern_matching()
test_pattern_guards()
test_exhaustiveness()
test_nested_patterns()

print_test_summary()
