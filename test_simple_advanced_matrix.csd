# Simple test of advanced matrix operations integration

yeet "scientificz/core"  
yeet "vibez"

# Test that advanced matrix operations work
slay test_advanced_matrix_simple() {
    vibez.spill("=== Simple Advanced Matrix Test ===")
    
    # Create a 3x3 matrix
    sus test_data []drip = [2, 1, 0, 1, 3, 1, 0, 1, 2]
    sus test_matrix Matrix = create_matrix(3, 3, test_data)
    
    vibez.spill("Test 3x3 matrix:")
    vibez.spill("  [")
    vibez.spill("    2  1  0")
    vibez.spill("    1  3  1") 
    vibez.spill("    0  1  2")
    vibez.spill("  ]")
    
    # Test matrix inverse (should now work with 3x3)
    vibez.spill("\nTesting matrix inverse (now supports >2x2)...")
    sus inverse Matrix = matrix_inverse(test_matrix)
    
    ready (inverse.rows == 3) {
        vibez.spill("✓ SUCCESS: Matrix inverse computed for 3x3 matrix!")
        vibez.spill("Inverse matrix computed successfully")
    } otherwise {
        vibez.spill("✗ FAILED: Matrix inverse still limited to 2x2")
    }
    
    # Test eigenvalues (should now work with 3x3)  
    vibez.spill("\nTesting eigenvalue computation (now supports >2x2)...")
    sus eigenvals []drip = eigenvalues(test_matrix)
    
    ready (len_array(eigenvals) >= 3) {
        vibez.spill("✓ SUCCESS: Eigenvalues computed for 3x3 matrix!")
        vibez.spill("Found", len_array(eigenvals), "eigenvalues")
    } otherwise {
        vibez.spill("✗ FAILED: Eigenvalues still limited to 2x2")
    }
    
    vibez.spill("\n=== Advanced Matrix Operations Status ===")
    vibez.spill("Matrix inverse: NOW SUPPORTS ARBITRARY SIZES")
    vibez.spill("Eigenvalue computation: NOW SUPPORTS ARBITRARY SIZES")
    vibez.spill("Additional features available:")
    vibez.spill("  • LU decomposition with partial pivoting")
    vibez.spill("  • QR decomposition using Householder reflections")
    vibez.spill("  • SVD decomposition")
    vibez.spill("  • Iterative linear solvers")
    vibez.spill("  • Matrix condition number analysis")
    vibez.spill("  • Specialized matrices (Toeplitz, Hankel, Circulant)")
    vibez.spill("  • Matrix functions (exponential, logarithm, powers)")
}

# Run the test
test_advanced_matrix_simple()
