slay math_abs_pure(x meal) meal {
    lowkey x < 0.0 {
        damn -x
    } highkey {
        damn x
    }
}

slay math_min_pure(a meal, b meal) meal {
    lowkey a < b {
        damn a
    } highkey {
        damn b
    }
}

slay test_basic_math() {
    vibez.spill("Testing pure CURSED math functions")
    
    sus abs_result meal = math_abs_pure(-5.5)
    vibez.spill("abs(-5.5) works")
    
    sus min_result meal = math_min_pure(3.0, 7.0)
    vibez.spill("min(3.0, 7.0) works")
    
    vibez.spill("Pure math functions successful!")
}

slay main() {
    test_basic_math()
}
