// Test FFI enum functionality
// Basic enum demonstration

enum Color {
    Red = 0,
    Green = 1,
    Blue = 2
}

slay main() vibes {
    vibez.spill("FFI Enum Type Mapping Test")
    
    sus color Color = Color.Red
    vibez.spill("Selected color:", color)
    
    // Basic enum value testing
    sus red_value drip = @intFromEnum(Color.Red)
    sus green_value drip = @intFromEnum(Color.Green)
    sus blue_value drip = @intFromEnum(Color.Blue)
    
    vibez.spill("Red value:", red_value)
    vibez.spill("Green value:", green_value)
    vibez.spill("Blue value:", blue_value)
    
    // Pattern matching with enums
    sick (color) {
        when Color.Red -> vibez.spill("Color is red!")
        when Color.Green -> vibez.spill("Color is green!")
        when Color.Blue -> vibez.spill("Color is blue!")
    }
}
