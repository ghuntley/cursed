# CURSED Comprehensive Test
slay multiply(x drip, y drip) drip { damn x * y }
slay factorial(n drip) drip {
    sus result drip = 1
    sus i drip = 1
    bestie (i <= n) {
        result = result * i
        i = i + 1
    }
    damn result
}

vibez.spill("Function test:", multiply(6, 7))
vibez.spill("Factorial test:", factorial(5))

sus x drip = 5
ready (x) {
    1 => vibez.spill("One")
    5 => vibez.spill("Five - pattern matching works!")
    _ => vibez.spill("Other")
}

sus count drip = 0
bestie (count < 3) {
    vibez.spill("Loop iteration:", count)
    count = count + 1
}

ready (x > 3 && count >= 3) {
    vibez.spill("Advanced conditions work!")
}

vibez.spill("All major features working correctly!")
