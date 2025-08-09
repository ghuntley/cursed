fr fr Test file for demonstrating advanced REPL features
fr fr This file shows various CURSED language constructs

sus greeting tea = "Hello, CURSED REPL!"
vibez.spill(greeting)

sus x drip = 42
sus y drip = 24
sus sum drip = x + y
vibez.spill("Sum:", sum)

slay multiply(a drip, b drip) drip {
    damn a * b
}

sus result drip = multiply(6, 7)
vibez.spill("6 * 7 =", result)

sus numbers []drip = [1, 2, 3, 4, 5]
vibez.spill("Array length:", len(numbers))

ready (result > 40) {
    vibez.spill("Result is greater than 40!")
}

sus counter drip = 0
bestie (counter < 3) {
    vibez.spill("Counter:", counter)
    counter = counter + 1
}

vibez.spill("Advanced REPL test complete!")
