# Type Switch Demo for CURSED Language
# Demonstrates runtime type checking with variable binding

vibez.spill("=== Type Switch Demo ===")

# Test with different primitive types
sus int_val normie = 42
sus string_val tea = "hello"
sus bool_val lit = based

# Function to demonstrate type switching
slay describe_value(value normie) tea {
    damn typecheck value is {
        normie n -> "Integer value found"
        tea s -> "String value found"
        lit b -> "Boolean value found"
        _ -> "Unknown type"
    }
}

# Demo basic type switches
vibez.spill("Testing integer:")
vibez.spill(typecheck int_val is {
    normie -> "✅ Correctly identified as integer"
    _ -> "❌ Failed to identify integer"
})

vibez.spill("Testing string:")
vibez.spill(typecheck string_val is {
    tea -> "✅ Correctly identified as string"
    _ -> "❌ Failed to identify string"
})

vibez.spill("Testing boolean:")
vibez.spill(typecheck bool_val is {
    lit -> "✅ Correctly identified as boolean"
    _ -> "❌ Failed to identify boolean"
})

# Test with variable binding
vibez.spill("Variable binding test:")
sus bound_result tea = typecheck int_val is {
    normie bound_var -> "Bound variable received the value"
    _ -> "Binding failed"
}
vibez.spill(bound_result)

# Test wildcard pattern
vibez.spill("Wildcard pattern test:")
sus wildcard_result tea = typecheck int_val is {
    tea -> "Not a string"
    lit -> "Not a boolean"
    _ -> "✅ Wildcard caught the integer"
}
vibez.spill(wildcard_result)

vibez.spill("=== Type Switch Demo Complete ===")
