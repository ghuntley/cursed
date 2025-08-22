# Test 3: Control flow - If/else (ready/otherwise), loops (bestie)

# If/else statements (ready/otherwise)
sus age drip = 25
sus score drip = 85

ready (age >= 18) {
    vibez.spill("You are an adult")
} otherwise {
    vibez.spill("You are a minor")
}

ready (score >= 90) {
    vibez.spill("Grade: A")
} otherwise ready (score >= 80) {
    vibez.spill("Grade: B") 
} otherwise ready (score >= 70) {
    vibez.spill("Grade: C")
} otherwise {
    vibez.spill("Grade: F")
}

# While loops (bestie)
sus count drip = 0
vibez.spill("Counting to 5:")
bestie (count < 5) {
    vibez.spill("Count:", count)
    count = count + 1
}

# Nested control structures
sus i drip = 0
vibez.spill("Nested loops and conditions:")
bestie (i < 3) {
    sus j drip = 0
    bestie (j < 3) {
        ready (i == j) {
            vibez.spill("Diagonal at", i, j)
        } otherwise {
            vibez.spill("Position", i, j)
        }
        j = j + 1
    }
    i = i + 1
}

# Break and continue (if implemented)
sus num drip = 0
vibez.spill("Numbers 0-9, skip 5:")
bestie (num < 10) {
    ready (num == 5) {
        num = num + 1
        # continue statement if implemented
    } otherwise {
        vibez.spill("Number:", num)
        num = num + 1
    }
}
