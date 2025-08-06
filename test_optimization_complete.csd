fr fr Test comprehensive optimization levels

sus global_var drip = 100

slay expensive_calculation(n drip) drip {
    sus result drip = 0
    sus i drip = 0
    bestie (i < n) {
        result = result + (i * i)
        i = i + 1
    }
    damn result
}

slay main() {
    vibez.spill("Testing optimization with complex calculations...")
    
    fr fr This should be optimized away at higher levels
    sus unused_var drip = 42
    
    fr fr Loop that should be unrolled at O2+
    sus sum drip = 0
    sus j drip = 0
    bestie (j < 10) {
        sum = sum + j
        j = j + 1
    }
    
    fr fr Function call that might be inlined
    sus calc_result drip = expensive_calculation(5)
    
    vibez.spill("Sum: ")
    vibez.spill(sum)
    vibez.spill("Calculation result: ")
    vibez.spill(calc_result)
    
    fr fr Constant propagation test
    sus const_a drip = 5
    sus const_b drip = 10
    sus const_result drip = const_a * const_b
    vibez.spill("Constant result: ")
    vibez.spill(const_result)
}

fr fr Call main
main()
