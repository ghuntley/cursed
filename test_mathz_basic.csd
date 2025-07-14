// Test basic mathz constants
sus pi_val meal = 3.14159265358979323846
sus e_val meal = 2.71828182845904523536

vibez.spill("Pi:")
vibez.spill(pi_val)
vibez.spill("E:")
vibez.spill(e_val)

// Test simple function
slay TestAbs(x meal) meal {
    lowkey x < 0.0 {
        damn 0.0 - x
    }
    damn x
}

sus result meal = TestAbs(0.0 - 5.5)
vibez.spill("Abs result:")
vibez.spill(result)
