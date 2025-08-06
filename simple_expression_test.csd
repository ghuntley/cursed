# Simple expression test to verify basic functionality

# Test basic arithmetic
sus a normie = 10
sus b normie = 20
sus sum normie = a + b

vibez.spill("Sum:", sum)

# Test array literal (simple)
sus numbers [normie] = [1, 2, 3]

# Test function call
slay double_it(x normie) normie {
    damn x * 2
}

sus doubled normie = double_it(5)
vibez.spill("Doubled:", doubled)

vibez.spill("Simple test completed")
