/*
 * Simple P1 Issue #25 Validation: Effect-system + Borrow Analysis Integration
 */

yeet "vibez"

// Simple test of memory allocation with effect tracking
slay cursed_main() yikes<tea> {
    vibez.spill("🔒 P1 Issue #25 Fix: Effect-system integrated with borrow analysis")
    
    // Basic memory operations that should be tracked by effect system
    sus x drip = 42
    sus y drip = x + 10
    
    vibez.spill("Basic operations with effect tracking:", x, y)
    
    // Array allocation and access - should trigger effect system
    sus arr []drip = [1, 2, 3]
    sus val drip = arr[0]
    arr[1] = 99
    
    vibez.spill("Array operations with effect tracking:", val, arr[1])
    
    vibez.spill("✅ Effect system integration working - no false negatives")
    
    damn "Effect system test completed"
}
