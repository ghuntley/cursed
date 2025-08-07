sus x drip = 15
sus y drip = x * 2 + 5
vibez.spill("Values:", x, y)

lowkey x > 10 {
    vibez.spill("x is greater than 10")
    lowkey y > 30 {
        vibez.spill("y is also greater than 30")
    }
}

sus sum drip = 0
bestie i := 1; i <= 5; i++ {
    sum = sum + i
    vibez.spill("Step", i, "Sum:", sum)
}

vibez.spill("Final sum:", sum)
