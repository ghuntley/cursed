slay test_function(name tea) {
    vibez.spill("Function called with: " + name)
}

vibez.spill("Before function call")
test_function("hello")
vibez.spill("After function call")
