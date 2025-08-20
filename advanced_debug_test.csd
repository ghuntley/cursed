# Advanced CURSED test program for debugging
sus x drip = 10
sus y drip = 20
sus result drip = 0

slay calculate_sum(a drip, b drip) drip {
    sus temp drip = a + b
    damn temp
}

slay main_character() {
    result = calculate_sum(x, y)
    vibez.spill("x =", x)
    vibez.spill("y =", y) 
    vibez.spill("result =", result)
    
    ready (result > 25) {
        vibez.spill("Result is greater than 25")
    } otherwise {
        vibez.spill("Result is 25 or less")
    }
    
    sus counter drip = 0
    bestie (counter < 3) {
        vibez.spill("Counter:", counter)
        counter = counter + 1
    }
    
    vibez.spill("Program complete")
}
