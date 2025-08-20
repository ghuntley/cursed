// Enhanced FFI Enum Type Mapping Demonstration
// Shows proper C enum type mapping with different sizes and marshaling

// Example C enum declarations that would be parsed by the FFI system:
/*
// Basic C enum (defaults to int size)
enum Color {
    Red = 0,
    Green = 1,
    Blue = 2
};

// Enum with explicit size specification
enum Status : unsigned char {
    OK = 0,
    WARNING = 1,
    ERROR = 255
};

// Enum with packed attribute (minimal size)
enum __attribute__((packed)) Direction {
    North = 0,
    South = 1,
    East = 2,
    West = 3
};

// Enum with different underlying types
enum Priority : short {
    Low = -10,
    Normal = 0,
    High = 10,
    Critical = 100
};
*/

// Generated CURSED enum bindings (what the FFI system produces):

// Color enum binding (int-sized, signed)
enum Color {
    Red = 0,
    Green = 1,
    Blue = 2
}

// Type alias for C interop
type Color_Raw = normie

// Conversion functions
slay Color_to_raw(value Color) Color_Raw {
    damn @intFromEnum(value)
}

slay raw_to_Color(value Color_Raw) Color {
    damn @enumFromInt(value)
}

// Status enum binding (char-sized, unsigned)
enum Status {
    OK = 0,
    WARNING = 1,
    ERROR = 255
}

// Type alias for C interop (unsigned char maps to smol in CURSED)
type Status_Raw = smol

// Conversion functions with bounds checking
slay Status_to_raw(value Status) Status_Raw {
    sus raw normie = @intFromEnum(value)
    // Validate fits in unsigned char range (0-255)
    ready (raw < 0 || raw > 255) {
        yikes "Status value out of range for unsigned char"
    }
    damn @as(smol, raw)
}

slay raw_to_Status(value Status_Raw) Status {
    sus int_val normie = @as(normie, value)
    damn @enumFromInt(int_val)
}

// Direction enum binding (packed, minimal size)
enum Direction {
    North = 0,
    South = 1,
    East = 2,
    West = 3
}

// Type alias for packed enum (char-sized)
type Direction_Raw = smol

// Priority enum binding (short-sized, signed)
enum Priority {
    Low = -10,
    Normal = 0,
    High = 10,
    Critical = 100
}

// Type alias for short (maps to smol in CURSED for 16-bit)
type Priority_Raw = smol

// Conversion functions for Priority
slay Priority_to_raw(value Priority) Priority_Raw {
    sus raw normie = @intFromEnum(value)
    // Validate fits in short range (-32768 to 32767)
    ready (raw < -32768 || raw > 32767) {
        yikes "Priority value out of range for short"
    }
    damn @as(smol, raw)
}

slay raw_to_Priority(value Priority_Raw) Priority {
    sus int_val normie = @as(normie, value)
    damn @enumFromInt(int_val)
}

// FFI function declarations using the enum types
extern "C" {
    library "graphics"
    slay set_pixel_color(x normie, y normie, color Color_Raw) vibes
    slay get_pixel_color(x normie, y normie) Color_Raw
}

extern "C" {
    library "system"  
    slay get_system_status() Status_Raw
    slay set_log_priority(priority Priority_Raw) vibes
}

// Example usage with automatic marshaling
slay demo_enum_ffi() {
    vibez.spill("=== FFI Enum Type Mapping Demo ===")
    
    // Using Color enum with FFI
    sus color Color = Color.Red
    sus raw_color Color_Raw = Color_to_raw(color)
    vibez.spill("Color.Red as raw value:", raw_color)
    
    // Call C function with enum parameter
    set_pixel_color(100, 50, raw_color)
    vibez.spill("Set pixel at (100, 50) to red")
    
    // Get enum value back from C function
    sus returned_color Color_Raw = get_pixel_color(100, 50)
    sus parsed_color Color = raw_to_Color(returned_color)
    vibez.spill("Retrieved color:", parsed_color)
    
    // Using Status enum (different size)
    sus status_raw Status_Raw = get_system_status()
    sus status Status = raw_to_Status(status_raw)
    
    sick (status) {
        when Status.OK -> vibez.spill("System status: OK")
        when Status.WARNING -> vibez.spill("System status: WARNING")
        when Status.ERROR -> vibez.spill("System status: ERROR")
    }
    
    // Using Priority enum (signed values)
    sus priority Priority = Priority.High
    sus priority_raw Priority_Raw = Priority_to_raw(priority)
    set_log_priority(priority_raw)
    vibez.spill("Set log priority to High (", @as(normie, priority_raw), ")")
    
    // Demonstrate size validation
    fam {
        sus invalid_status Status_Raw = 300  // > 255, invalid for unsigned char
        sus parsed_invalid Status = raw_to_Status(invalid_status)
        vibez.spill("This shouldn't print - value too large")
    } shook (error) {
        vibez.spill("Caught expected error for out-of-range value:", error)
    }
}

// Type information introspection
slay show_enum_type_info() {
    vibez.spill("=== Enum Type Information ===")
    
    // Color: int-sized (4 bytes), signed
    vibez.spill("Color enum: 4 bytes, signed, range: -2147483648 to 2147483647")
    
    // Status: char-sized (1 byte), unsigned  
    vibez.spill("Status enum: 1 byte, unsigned, range: 0 to 255")
    
    // Direction: packed char-sized (1 byte), signed
    vibez.spill("Direction enum: 1 byte, signed, range: -128 to 127")
    
    // Priority: short-sized (2 bytes), signed
    vibez.spill("Priority enum: 2 bytes, signed, range: -32768 to 32767")
}

// Main demonstration
slay main() vibes {
    demo_enum_ffi()
    vibez.spill("")
    show_enum_type_info()
}
