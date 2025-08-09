sus outer_var drip = 100
sus array_data []drip = [10, 20, 30]

ready (outer_var > 50) {
    sus inner_var drip = 5
    sus j drip = 0
    
    bestie (j < 3) {
        vibez.spill("outer_var:", outer_var)
        vibez.spill("inner_var:", inner_var)
        vibez.spill("array_data[", j, "]:", array_data[j])
        j = j + 1
    }
}

vibez.spill("Finished nested scoping test")
