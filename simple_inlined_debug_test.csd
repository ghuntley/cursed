# Simple test for inlined function debug information
# This demonstrates basic function inlining with debug preservation

# Small helper function that should be inlined
slay add_two(x drip) drip {
    damn x + 2
}

# Main function 
slay main() drip {
    sus result drip = add_two(5)
    damn result
}
