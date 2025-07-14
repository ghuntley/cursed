# CURSED Advanced Pattern Matching Test Suite
# Testing comprehensive pattern matching semantics

yeet "testz"

# Test basic literal patterns
sus test_value normie = 42

vibe_check test_value {
    mood 42:
        vibez.spill("Matched literal 42")
    mood 24:
        vibez.spill("Matched literal 24") 
    basic:
        vibez.spill("No match")
}

# Test range patterns  
sus score normie = 85

vibe_check score {
    mood 90..100:
        vibez.spill("Grade A")
    mood 80..89:
        vibez.spill("Grade B")
    mood 70..79:
        vibez.spill("Grade C")
    basic:
        vibez.spill("Grade F")
}

# Test character range patterns
sus letter sip = 'm'

vibe_check letter {
    mood 'a'..'j':
        vibez.spill("Letter in first half")
    mood 'k'..'z':
        vibez.spill("Letter in second half")
    basic:
        vibez.spill("Not a lowercase letter")
}

# Test wildcard and variable patterns
sus input normie = 15

vibe_check input {
    mood x when x > 20:
        vibez.spill("Large number: %d", x)
    mood x when x > 10:
        vibez.spill("Medium number: %d", x)
    mood x:
        vibez.spill("Small number: %d", x)
}

# Test tuple destructuring patterns
sus point := (3, 4)

vibe_check point {
    mood (0, 0):
        vibez.spill("Origin point")
    mood (x, 0):
        vibez.spill("On X-axis at: %d", x)
    mood (0, y):
        vibez.spill("On Y-axis at: %d", y)
    mood (x, y) when x == y:
        vibez.spill("On diagonal: (%d, %d)", x, y)
    mood (x, y):
        vibez.spill("Point at: (%d, %d)", x, y)
}

# Test Or patterns
sus day_of_week normie = 6

vibe_check day_of_week {
    mood 1 | 2 | 3 | 4 | 5:
        vibez.spill("Weekday")
    mood 6 | 7:
        vibez.spill("Weekend")
    basic:
        vibez.spill("Invalid day")
}

# Test boolean patterns with exhaustiveness
sus flag lit = based

vibe_check flag {
    mood based:
        vibez.spill("Flag is true")
    mood cap:
        vibez.spill("Flag is false")
    # No default needed - exhaustive for boolean
}

# Test guards with complex conditions
sus temperature normie = 25

vibe_check temperature {
    mood t when t < 0:
        vibez.spill("Freezing: %d°C", t)
    mood t when t >= 0 && t < 10:
        vibez.spill("Cold: %d°C", t)
    mood t when t >= 10 && t < 20:
        vibez.spill("Cool: %d°C", t)
    mood t when t >= 20 && t < 30:
        vibez.spill("Warm: %d°C", t)
    mood t when t >= 30:
        vibez.spill("Hot: %d°C", t)
}

# Test nested pattern matching  
sus nested_data := ((1, 2), (3, 4))

vibe_check nested_data {
    mood ((a, b), (c, d)) when a + b == c:
        vibez.spill("Special relationship: %d+%d=%d", a, b, c)
    mood ((a, b), (c, d)):
        vibez.spill("Nested tuple: ((%d,%d), (%d,%d))", a, b, c, d)
    basic:
        vibez.spill("Not a nested tuple")
}

vibez.spill("Advanced pattern matching tests completed")
