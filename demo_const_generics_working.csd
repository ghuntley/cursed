// Demonstration that const generics now work safely without ICE

// Simple const generic function - this used to cause ICE in optimizer
slay safe_array<const SIZE: drip>(value: drip) [SIZE]drip {
    sus result [SIZE]drip
    bestie (i = 0; i < SIZE; i += 1) {
        result[i] = value + @as(drip, i)
    }
    damn result
}

// Another example with bounds checking
slay buffer_copy<const N: drip>(src: [N]tea, dest: [N]tea) drip {
    bestie (i = 0; i < N; i += 1) {
        dest[i] = src[i]
    }
    damn @as(drip, N)
}

slay main() drip {
    // These work safely now - no more ICE!
    sus arr1 [3]drip = safe_array<3>(10)
    sus arr2 [5]drip = safe_array<5>(20)
    
    vibez.spill("Array 1:", arr1[0], arr1[1], arr1[2])
    vibez.spill("Array 2:", arr2[0], arr2[1], arr2[2], arr2[3], arr2[4])
    
    damn 0
}
