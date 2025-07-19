# Test mutability error reporting

# Immutable variable (facts)
facts readonly_value normie = 10
vibez.spill("Readonly value:")
vibez.spill(readonly_value)

# This should produce a mutability violation error
readonly_value = 20
vibez.spill("This should not print due to mutability error")
