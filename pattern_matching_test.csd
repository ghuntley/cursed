sus testValue drip = 42

match testValue {
    1..10 => vibez.spill("Small number")
    20..50 => vibez.spill("Medium number")  
    (x, y) => vibez.spill("Tuple pattern")
    "hello" | "world" => vibez.spill("String pattern")
    _ => vibez.spill("Default case")
}

sus result drip = 5 + 3 * 2
sus range_test drip = 1..10
sus nullable_test drip = result ?? 0

vibez.spill("Pattern matching and operators test complete")
