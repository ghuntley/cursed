# Comprehensive Interface Composition Test

# Simple composition
collab SimpleInterface with BaseInterface {
    slay simple_method() lit
}

# Composition with alias
collab AliasedInterface with UtilityInterface as Utils {
    slay aliased_method() tea
}

# Composition with method exclusion
collab ExcludedInterface with LegacyInterface except deprecated_method, old_function {
    slay excluded_method() normie
}

# Composition with method renaming
collab RenamedInterface with SourceInterface rename old_name -> new_name, legacy_func -> modern_func {
    slay renamed_method() lit
}

# Complex composition with multiple features
collab ComplexInterface with
    FirstInterface as First except method1,
    SecondInterface rename old -> new,
    ThirdInterface as Third except deprecated rename legacy -> modern {
    slay complex_method() tea
}

# Multiple compositions
collab MultiCompositionInterface with
    InterfaceA as A,
    InterfaceB as B except unwanted_method,
    InterfaceC rename func1 -> function1 {
    slay multi_method() normie
}

vibez.spill("All interface composition patterns tested successfully!")
