fr fr Test working features

fr fr Test constants
facts (
    PI = 3.14159
    MAX_SIZE = 1000
    MIN_SIZE = 1
)

fr fr Test basic types
sus small_num smol = 42
sus medium_num mid = 32767
sus large_num thicc = 9223372036854775807
sus unsigned_byte byte = 255
sus unicode_char rune = 65

fr fr Test variables
sus name tea = "CURSED"
sus is_working lit = based

fr fr Test function
slay calculate_area(radius meal) meal {
    yolo PI * radius * radius
}

fr fr Main function
slay main() {
    vibez.spill("Testing working implementations...")
    
    fr fr Test constants
    vibez.spill("PI: " + PI)
    vibez.spill("MAX_SIZE: " + MAX_SIZE)
    
    fr fr Test basic types
    vibez.spill("Small number: " + small_num)
    vibez.spill("Medium number: " + medium_num)
    vibez.spill("Large number: " + large_num)
    vibez.spill("Unsigned byte: " + unsigned_byte)
    vibez.spill("Unicode char: " + unicode_char)
    
    fr fr Test variables
    vibez.spill("Name: " + name)
    vibez.spill("Is working: " + is_working)
    
    fr fr Test function call
    sus area := calculate_area(5.0)
    vibez.spill("Area: " + area)
    
    fr fr Test goroutines
    stan vibez.spill("Goroutine spawned!")
    
    vibez.spill("All working features tested successfully!")
}
