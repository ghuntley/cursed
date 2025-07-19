# Test mutability tracking implementation

# Mutable variable declaration (sus)
sus mutable_var normie = 42
vibez.spill("Mutable variable initial value:")
vibez.spill(mutable_var)

# Immutable variable declaration (facts)
facts immutable_var normie = 100
vibez.spill("Immutable variable value:")
vibez.spill(immutable_var)

# Valid mutation of mutable variable
mutable_var = 84
vibez.spill("Mutable variable after assignment:")
vibez.spill(mutable_var)

# This should cause a mutability error when type checking is enabled
# immutable_var = 200  # Uncommenting this should produce an error

vibez.spill("Mutability tracking test completed successfully!")
