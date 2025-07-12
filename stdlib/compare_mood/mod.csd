# CURSED compare_mood stdlib module
# Pure CURSED implementation of comparison utilities

# Comparison result constants
sus LessThan normie = -1
sus Equal normie = 0
sus GreaterThan normie = 1

# Basic integer comparison
slay CompareInt(a normie, b normie) normie {
    if a < b {
        damn LessThan
    }
    if a > b {
        damn GreaterThan
    }
    damn Equal
}

# String comparison
slay CompareString(a tea, b tea) normie {
    # Simple lexicographical comparison
    if a < b {
        damn LessThan
    }
    if a > b {
        damn GreaterThan
    }
    damn Equal
}

# Float comparison
slay CompareFloat(a meal, b meal) normie {
    if a < b {
        damn LessThan
    }
    if a > b {
        damn GreaterThan
    }
    damn Equal
}

# Boolean comparison (cap < based)
slay CompareBool(a lit, b lit) normie {
    if a == b {
        damn Equal
    }
    if a == cap && b == based {
        damn LessThan
    }
    damn GreaterThan
}

# Equality check for integers
slay EqualInt(a normie, b normie) lit {
    damn a == b
}

# Equality check for strings
slay EqualString(a tea, b tea) lit {
    damn a == b
}

# Equality check for floats
slay EqualFloat(a meal, b meal) lit {
    damn a == b
}

# Equality check for booleans
slay EqualBool(a lit, b lit) lit {
    damn a == b
}

# Less than check for integers
slay LessInt(a normie, b normie) lit {
    damn a < b
}

# Less than check for strings
slay LessString(a tea, b tea) lit {
    damn a < b
}

# Less than check for floats
slay LessFloat(a meal, b meal) lit {
    damn a < b
}

# Greater than check for integers
slay GreaterInt(a normie, b normie) lit {
    damn a > b
}

# Greater than check for strings
slay GreaterString(a tea, b tea) lit {
    damn a > b
}

# Greater than check for floats
slay GreaterFloat(a meal, b meal) lit {
    damn a > b
}

# Less than or equal check for integers
slay LessEqualInt(a normie, b normie) lit {
    damn a <= b
}

# Less than or equal check for strings
slay LessEqualString(a tea, b tea) lit {
    damn a <= b
}

# Less than or equal check for floats
slay LessEqualFloat(a meal, b meal) lit {
    damn a <= b
}

# Greater than or equal check for integers
slay GreaterEqualInt(a normie, b normie) lit {
    damn a >= b
}

# Greater than or equal check for strings
slay GreaterEqualString(a tea, b tea) lit {
    damn a >= b
}

# Greater than or equal check for floats
slay GreaterEqualFloat(a meal, b meal) lit {
    damn a >= b
}

# Three-way comparison helper
slay ThreeWay(less_condition lit, greater_condition lit) normie {
    if less_condition {
        damn LessThan
    }
    if greater_condition {
        damn GreaterThan
    }
    damn Equal
}

# Min function for integers
slay MinInt(a normie, b normie) normie {
    if a < b {
        damn a
    }
    damn b
}

# Max function for integers
slay MaxInt(a normie, b normie) normie {
    if a > b {
        damn a
    }
    damn b
}

# Min function for floats
slay MinFloat(a meal, b meal) meal {
    if a < b {
        damn a
    }
    damn b
}

# Max function for floats
slay MaxFloat(a meal, b meal) meal {
    if a > b {
        damn a
    }
    damn b
}

# Min function for strings
slay MinString(a tea, b tea) tea {
    if a < b {
        damn a
    }
    damn b
}

# Max function for strings
slay MaxString(a tea, b tea) tea {
    if a > b {
        damn a
    }
    damn b
}

# Clamp function for integers
slay ClampInt(value normie, min normie, max normie) normie {
    if value < min {
        damn min
    }
    if value > max {
        damn max
    }
    damn value
}

# Clamp function for floats
slay ClampFloat(value meal, min meal, max meal) meal {
    if value < min {
        damn min
    }
    if value > max {
        damn max
    }
    damn value
}

# Sign function for integers
slay SignInt(value normie) normie {
    if value < 0 {
        damn -1
    }
    if value > 0 {
        damn 1
    }
    damn 0
}

# Sign function for floats
slay SignFloat(value meal) normie {
    if value < 0.0 {
        damn -1
    }
    if value > 0.0 {
        damn 1
    }
    damn 0
}

# Absolute value for integers
slay AbsInt(value normie) normie {
    if value < 0 {
        damn -value
    }
    damn value
}

# Absolute value for floats
slay AbsFloat(value meal) meal {
    if value < 0.0 {
        damn -value
    }
    damn value
}

# Distance between two integers
slay DistanceInt(a normie, b normie) normie {
    damn AbsInt(a - b)
}

# Distance between two floats
slay DistanceFloat(a meal, b meal) meal {
    damn AbsFloat(a - b)
}

# Check if value is between two bounds (inclusive)
slay BetweenInt(value normie, min normie, max normie) lit {
    damn value >= min && value <= max
}

# Check if value is between two bounds (inclusive)
slay BetweenFloat(value meal, min meal, max meal) lit {
    damn value >= min && value <= max
}

# Check if value is between two bounds (exclusive)
slay BetweenExclusiveInt(value normie, min normie, max normie) lit {
    damn value > min && value < max
}

# Check if value is between two bounds (exclusive)
slay BetweenExclusiveFloat(value meal, min meal, max meal) lit {
    damn value > min && value < max
}
