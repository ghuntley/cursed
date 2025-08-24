# WASM Memory Leak Test - Multiple allocations to expose memory leaks

yeet "vibez"

slay main() {
    vibez.spill("Starting WASM memory leak test")
    
    # Allocate memory multiple times to expose leaks
    bestie (sus i drip = 0; i < 100; i = i + 1) {
        sus test_array []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        sus test_string tea = "Memory allocation test iteration number: "
        
        # Force some memory allocations
        sus combined tea = test_string + "test"
        vibez.spill("Iteration " + str(i) + " - allocated array and strings")
        
        # These allocations should be freed but aren't in current WASM runtime
    }
    
    vibez.spill("Completed 100 allocation iterations")
}
