// Test const generics with bounds checking to prevent ICE in optimizer

// Valid const generic with bounds
slay fixed_array<const N: drip>(arr: [N]drip) drip {
    damn arr[0] + arr[1]
}

// Invalid const generic that should be caught (negative value)
slay invalid_negative<const M: drip>() drip {
    damn M * 2
}

// Const generic with array bounds
slay safe_buffer<const SIZE: drip>(data: [SIZE]tea) tea {
    ready (SIZE > 0) {
        damn data[0]
    } otherwise {
        damn "empty"
    }
}

// Test usage
slay main() drip {
    sus test_array [5]drip = [1, 2, 3, 4, 5]
    sus result drip = fixed_array(test_array)
    
    // This should pass bounds checking
    sus safe_result tea = safe_buffer(["hello", "world"])
    
    // This should be caught by bounds checking (negative const generic)
    // sus invalid_result drip = invalid_negative<-1>()
    
    damn result
}
