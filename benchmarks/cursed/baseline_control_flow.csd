# Baseline Benchmark 5: Complex Control Flow
# Tests: Nested conditions, switch statements, pattern matching, complex loops

yeet "mathz"
yeet "timez"

slay complex_conditions(n drip) drip {
    sus result drip = 0
    
    ready (n % 2 == 0) {
        ready (n % 4 == 0) {
            ready (n % 8 == 0) {
                result = n * 3
            } otherwise {
                result = n * 2
            }
        } otherwise {
            result = n + 10
        }
    } otherwise {
        ready (n % 3 == 0) {
            ready (n % 9 == 0) {
                result = n / 3
            } otherwise {
                result = n - 5
            }
        } otherwise {
            result = n + 1
        }
    }
    
    damn result
}

slay pattern_matching_sim(value drip) drip {
    sus category drip = value % 10
    sus result drip = 0
    
    # Simulate pattern matching with nested conditions
    ready (category == 0) {
        result = value * 10
    } otherwise ready (category == 1) {
        result = value * 5
    } otherwise ready (category == 2) {
        result = value * 3
    } otherwise ready (category == 3) {
        result = value * 2
    } otherwise ready (category >= 4 && category <= 6) {
        result = value + 100
    } otherwise {
        result = value - 50
    }
    
    damn result
}

slay nested_loops(depth drip, width drip) drip {
    sus total drip = 0
    
    bestie (sus i drip = 0; i < depth; i++) {
        bestie (sus j drip = 0; j < width; j++) {
            bestie (sus k drip = 0; k < width/2; k++) {
                total = total + complex_conditions(i + j + k)
                total = total + pattern_matching_sim(i * j * k + 1)
            }
        }
    }
    
    damn total
}

slay state_machine_sim(iterations drip) drip {
    sus state drip = 0
    sus result drip = 0
    
    bestie (sus i drip = 0; i < iterations; i++) {
        ready (state == 0) {
            result = result + i
            ready (i % 5 == 0) {
                state = 1
            }
        } otherwise ready (state == 1) {
            result = result * 2
            ready (i % 7 == 0) {
                state = 2
            }
        } otherwise ready (state == 2) {
            result = result - 10
            ready (i % 11 == 0) {
                state = 0
            }
        }
    }
    
    damn result
}

slay benchmark_control_flow() drip {
    sus iterations drip = 10000
    sus total drip = 0
    
    bestie (sus i drip = 0; i < iterations; i++) {
        total = total + complex_conditions(i)
        total = total + pattern_matching_sim(i * 2)
        
        ready (i % 100 == 0) {
            total = total + nested_loops(5, 10)
        }
        
        ready (i % 50 == 0) {
            total = total + state_machine_sim(20)
        }
    }
    
    damn total
}

slay main() drip {
    sus start drip = timez.now_microseconds()
    sus result drip = benchmark_control_flow()
    sus end drip = timez.now_microseconds()
    
    vibez.spill("Control flow benchmark result:", result)
    vibez.spill("Execution time (μs):", end - start)
    
    damn 0
}
