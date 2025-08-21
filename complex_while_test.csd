sus count drip = 0

bestie (count < 2) {
    vibez.spill("Loop iteration:", count)
    
    ready (count == 0) {
        vibez.spill("First iteration special case")
    }
    
    count = count + 1
    vibez.spill("Incremented count to:", count)
}

vibez.spill("Loop completed, final count:", count)
