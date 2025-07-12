fr fr Test math_rand_tea with direct function calls
yeet "math_rand_tea"

slay simple_test() {
    vibez.spill("Testing math_rand_tea module...")
    
    fr fr Test basic random generation
    sus val normie = Int()
    vibez.spill("Random int: " + val)
    
    fr fr Test seeding
    Seed(42)
    sus val1 normie = Int()
    vibez.spill("After seed 42: " + val1)
    
    Seed(42)
    sus val2 normie = Int()
    vibez.spill("After seed 42 again: " + val2)
    
    if val1 == val2 {
        vibez.spill("✅ Seeding works correctly!")
    } else {
        vibez.spill("❌ Seeding failed!")
    }
    
    fr fr Test other functions
    sus float_val meal = Float64()
    vibez.spill("Random float: " + float_val)
    
    sus bounded_val normie = Intn(100)
    vibez.spill("Random bounded int [0,100): " + bounded_val)
    
    vibez.spill("Test completed!")
}

simple_test()
