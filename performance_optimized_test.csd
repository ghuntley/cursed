# Performance test with optimizations enabled
yeet "mathz"
yeet "stringz"
yeet "arrayz"

# Function to test call overhead optimizations
slay compute_intensive(iterations drip) drip {
    sus total drip = 0
    sus i drip = 0
    bestie (i < iterations) {
        # Multiple function calls to test overhead reduction
        sus val1 drip = abs_normie(i - 50)
        sus val2 drip = abs_normie(val1 * 2)
        total = total + val2
        i = i + 1
    }
    damn total
}

# Variable lookup optimization test
sus large_data []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
sus result drip = 0
sus counter drip = 0

# Heavy computation with frequent variable access
bestie (counter < 500) {
    sus index drip = counter % len(large_data)
    sus value drip = large_data[index]
    sus computed drip = compute_intensive(20)
    result = result + value + computed
    counter = counter + 1
}

# String operations for memory allocation testing
sus message tea = "Performance test result: "
sus final_message tea = message + "optimized"

vibez.spill(final_message, result)
