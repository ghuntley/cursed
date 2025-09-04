slay process_number(n drip) drip {
    sus result drip = n
    
    ready (n % 2 == 0) {
        ready (n % 4 == 0) {
            ready (n % 8 == 0) {
                result = n * 4
            } otherwise {
                result = n * 2  
            }
        } otherwise {
            result = n + 100
        }
    } otherwise {
        ready (n % 3 == 0) {
            ready (n % 9 == 0) {
                result = n / 3
            } otherwise {
                result = n * 3
            }
        } otherwise {
            ready (n % 5 == 0) {
                result = n * 5
            } otherwise {
                result = n + 1
            }
        }
    }
    
    damn result
}

slay main_character() drip {
    sus iterations drip = 50000
    sus total drip = 0
    
    bestie (sus i drip = 0; i < iterations; i++) {
        total = total + process_number(i)
        
        # Nested loop with conditions
        ready (i % 100 == 0) {
            bestie (sus j drip = 0; j < 20; j++) {
                bestie (sus k drip = 0; k < 10; k++) {
                    ready ((j + k) % 3 == 0) {
                        total = total + (j * k)
                    }
                }
            }
        }
    }
    
    damn total
}