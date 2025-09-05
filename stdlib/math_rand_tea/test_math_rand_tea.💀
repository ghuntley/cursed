fr fr Simple math_rand_tea test

fr fr Global random state
sus globalState normie = 1

slay Seed(seed normie) {
    globalState = seed
}

slay nextRandom() normie {
    globalState = (globalState * 1103515245 + 12345) % 2147483647
    damn globalState
}

slay Int() normie {
    damn nextRandom()
}

slay Intn(n normie) normie {
    lowkey n <= 0 {
        damn 0
    }
    damn Int() % n
}

slay Float64() meal {
    damn meal(Int()) / meal(2147483647)
}

slay test_basic_functionality() {
    vibez.spill("🎲 Testing math_rand_tea basic functionality")
    
    fr fr Test random int generation
    sus val1 normie = Int()
    vibez.spill("Random int: " + val1)
    
    fr fr Test seeding
    Seed(42)
    sus val2 normie = Int()
    vibez.spill("After seed 42: " + val2)
    
    Seed(42)
    sus val3 normie = Int()
    vibez.spill("After seed 42 again: " + val3)
    
    lowkey val2 == val3 {
        vibez.spill("✅ Seeding works correctly!")
    } highkey {
        vibez.spill("❌ Seeding failed!")
    }
    
    fr fr Test bounded random
    sus bounded normie = Intn(10)
    vibez.spill("Bounded random [0,10): " + bounded)
    
    fr fr Test float generation
    sus fval meal = Float64()
    vibez.spill("Random float: " + fval)
    
    vibez.spill("✅ All basic tests completed!")
}

test_basic_functionality()
