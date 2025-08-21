# Simple Arrays Benchmark  
slay main() drip {
    sus iterations drip = 1000
    sus size drip = 100
    sus total drip = 0
    
    bestie (sus iter drip = 0; iter < iterations; iter++) {
        sus arr []drip = []
        
        # Fill array
        bestie (sus i drip = 0; i < size; i++) {
            arr = push(arr, i * 2 + 1)
        }
        
        # Process array
        sus sum drip = 0
        bestie (sus i drip = 0; i < size; i++) {
            sum = sum + arr[i]
        }
        
        total = total + sum
    }
    
    vibez.spill("Arrays result:", total)
    damn 0
}
