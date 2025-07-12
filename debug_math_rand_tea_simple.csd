fr fr Simple test for math_rand_tea module without testz
yeet "math_rand_tea"

slay simple_test() {
    vibez.spill("Testing math_rand_tea module...")
    
    fr fr Test basic random generation
    sus val normie = math_rand_tea.Int()
    vibez.spill("Random int: " + val)
    
    fr fr Test seeding
    math_rand_tea.Seed(42)
    sus val1 normie = math_rand_tea.Int()
    vibez.spill("After seed 42: " + val1)
    
    math_rand_tea.Seed(42)
    sus val2 normie = math_rand_tea.Int()
    vibez.spill("After seed 42 again: " + val2)
    
    if val1 == val2 {
        vibez.spill("✅ Seeding works correctly!")
    } else {
        vibez.spill("❌ Seeding failed!")
    }
    
    vibez.spill("Test completed!")
}

simple_test()
