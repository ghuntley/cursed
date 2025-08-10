# Test P1 Issue #22: Error recovery in release builds
# This file has intentional syntax errors to test error recovery

sus x drip = 10
sus y drip = 20 // missing semicolon 
sus z = "missing type"

ready (x > 5) 
    vibez.spill("x is greater than 5")
otherwise
    vibez.spill("x is not greater than 5")
end

# Malformed function
slay badFunction(a drip b drip) drip { // missing comma
    damn a + b
}

# Valid code after errors
sus result drip = x + y
vibez.spill("Result:", result)
