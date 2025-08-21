slay main() drip {
    sus iterations drip = 1000
    sus array_size drip = 200
    sus total drip = 0
    
    bestie (sus iter drip = 0; iter < iterations; iter++) {
        sus arr []drip = []
        
        # Build array
        bestie (sus i drip = 0; i < array_size; i++) {
            arr = push(arr, i * i + iter)
        }
        
        # Process array multiple times
        bestie (sus pass drip = 0; pass < 3; pass++) {
            bestie (sus i drip = 0; i < array_size; i++) {
                total = total + arr[i]
            }
        }
    }
    
    damn total
}