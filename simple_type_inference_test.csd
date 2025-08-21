# Simple Type Inference Edge Case Test

yeet "vibez"

# Test complex generic function call inference
slay generic_map<T, U>(items []T, mapper slay(T) U) []U {
    sus results []U = []
    bestie (item in items) {
        results.append(mapper(item))
    }
    damn results
}

slay to_string(x drip) tea {
    damn "number: " + x.to_string()
}

slay main() drip {
    vibez.spill("Testing enhanced type inference...")
    
    # This should trigger our enhanced constraint generation and inference
    sus numbers []drip = [1, 2, 3, 4, 5]
    sus strings []tea = generic_map(numbers, to_string)
    
    vibez.spill("Mapped {} numbers to strings", numbers.len())
    vibez.spill("First result: {}", strings[0])
    
    vibez.spill("✅ Enhanced type inference working correctly!")
    damn 0
}
