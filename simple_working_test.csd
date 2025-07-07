fr fr Simple working test
slay main() {
    vibez.spill("Testing basic math...")
    
    sus a normie = 5
    sus b normie = 3
    sus result normie = a + b
    
    vibez.spill("5 + 3 = " + tea(result))
    
    lowkey result == 8 {
        vibez.spill("✓ Test passed!")
    } highkey {
        vibez.spill("✗ Test failed!")
    }
}
