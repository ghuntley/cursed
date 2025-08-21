# Simple Control Flow Benchmark
slay complex_conditions(n drip) drip {
    sus result drip = 0
    
    ready (n % 2 == 0) {
        ready (n % 4 == 0) {
            result = n * 3
        } otherwise {
            result = n * 2
        }
    } otherwise {
        ready (n % 3 == 0) {
            result = n / 3
        } otherwise {
            result = n + 1
        }
    }
    
    damn result
}

slay main() drip {
    sus iterations drip = 10000
    sus total drip = 0
    
    bestie (sus i drip = 0; i < iterations; i++) {
        total = total + complex_conditions(i)
        
        ready (i % 100 == 0) {
            bestie (sus j drip = 0; j < 10; j++) {
                total = total + complex_conditions(i + j)
            }
        }
    }
    
    vibez.spill("Control flow result:", total)
    damn 0
}
