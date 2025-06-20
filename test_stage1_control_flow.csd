// Test Stage 1: Control Structures
// Testing lowkey/highkey (if/else), bestie (for), periodt (loop termination)

sus x: normie = 10
sus y: normie = 5

// Test if/else (lowkey/highkey)
lowkey (x > y) {
    print("x is greater than y")
} highkey {
    print("x is not greater than y") 
}

// Test simple loop with bestie
bestie (sus i: normie = 0; i < 5; i++) {
    print(i)
    lowkey (i == 3) {
        periodt  // break
    }
}

// Test while-style loop
sus counter: normie = 0
bestie (counter < 3) {
    print(counter)
    counter = counter + 1
}

// Test nested control structures
bestie (sus outer: normie = 0; outer < 3; outer++) {
    bestie (sus inner: normie = 0; inner < 2; inner++) {
        sus product: normie = outer * inner
        print(product)
        lowkey (product > 2) {
            periodt
        }
    }
}
