slay simple_factorial(n drip) drip {
    ready (n <= 1) { damn 1 }
    sus temp drip = n - 1
    sus recursive_result drip = simple_factorial(temp)
    sus final_result drip = n * recursive_result
    damn final_result
}

vibez.spill("3! =", simple_factorial(3))
