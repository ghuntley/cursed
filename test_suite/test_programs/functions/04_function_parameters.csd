vibe main

yeet "vibez"

slay no_params() {
    damn 42
}

slay one_param(x) {
    damn x * 2
}

slay multiple_params(a, b, c) {
    damn a + b + c
}

slay main_character() {
    vibez.spill("=== Function Parameters Test ===")
    
    vibez.spill("No parameters:")
    vibez.spill(no_params())
    
    vibez.spill("One parameter:")
    vibez.spill(one_param(10))
    
    vibez.spill("Multiple parameters:")
    vibez.spill(multiple_params(1, 2, 3))
    
    vibez.spill("Parameters with expressions:")
    vibez.spill(multiple_params(5 + 1, 3 * 2, 10 - 2))
    
    vibez.spill("=== Test Complete ===")
}
