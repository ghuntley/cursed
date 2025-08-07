// Test match expressions with return values
sus value drip = 7

sus result tea = match value {
    0 => "zero"
    1..5 => "small"
    6..10 => "medium"  
    11..20 => "large"
    _ => "very large"
}

vibez.spill("Result:", result)

// Test enum patterns
mood Color {
    Red
    Green  
    Blue
    Custom(drip, drip, drip)
}

sus color Color = Color.Red

sus color_name tea = match color {
    Color.Red => "red"
    Color.Green => "green"
    Color.Blue => "blue"
    Color.Custom(r, g, b) => "custom RGB"
    _ => "unknown"
}

vibez.spill("Color:", color_name)

// Test nested patterns
sus nested_data (drip, [drip]) = (1, [2, 3, 4])

match nested_data {
    (0, []) => vibez.spill("Empty")
    (1, [first, ...rest]) => {
        vibez.spill("First element:", first)
        vibez.spill("Rest:", rest)
    }
    (n, array) ready (n > 5 && array.length > 3) => {
        vibez.spill("Complex case")
    }
    _ => vibez.spill("Other case")
}

// Test wildcard and variable binding
sus any_value = "test"

match any_value {
    x => vibez.spill("Bound to variable x:", x)
}
