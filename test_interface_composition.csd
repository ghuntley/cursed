collab TestInterface with ComposedInterface {
    slay test_method() lit
}

collab AdvancedInterface with BaseInterface as Base, OtherInterface except deprecated_method {
    slay advanced_method() tea
}

collab ComplexInterface with 
    FirstInterface as First except old_method,
    SecondInterface rename old_name -> new_name {
    slay complex_method() normie
}

vibez.spill("Interface composition test complete!")
