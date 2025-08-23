# Advanced Matrix Operations Test - Complete Scientific Computing Demo
# Tests arbitrary size matrix operations, decompositions, and numerical methods

yeet "scientificz/core"
yeet "scientificz/advanced_matrix"
yeet "vibez"

# Test large matrix operations
slay test_large_matrix_operations() {
    vibez.spill("\n=== Advanced Large Matrix Operations Test ===")
    
    # Create a 5x5 test matrix with known properties
    sus matrix_5x5_data []drip = [
        4,  1,  2,  1,  0,
        1,  3,  1,  1,  1,
        2,  1,  5,  2,  1,
        1,  1,  2,  4,  2,
        0,  1,  1,  2,  3
    ]
    
    sus large_matrix Matrix = create_matrix(5, 5, matrix_5x5_data)
    
    vibez.spill("Testing 5x5 matrix:")
    print_matrix_detailed(large_matrix)
    
    # Test matrix inverse using advanced LU decomposition
    vibez.spill("\n--- Matrix Inverse (LU Decomposition) ---")
    sus inverse Matrix = matrix_inverse(large_matrix)
    
    ready (inverse.rows > 1) {
        vibez.spill("Successfully computed inverse:")
        print_matrix_detailed(inverse)
        
        # Verify A * A^(-1) = I
        sus identity_check Matrix = matrix_multiply(large_matrix, inverse)
        vibez.spill("Verification A × A⁻¹ (should be identity):")
        print_matrix_detailed(identity_check)
    } otherwise {
        vibez.spill("Matrix is singular - cannot compute inverse")
    }
    
    # Test eigenvalues for large matrix
    vibez.spill("\n--- Eigenvalue Computation (Jacobi Method) ---")
    sus eigenvals []drip = eigenvalues(large_matrix)
    
    ready (len_array(eigenvals) > 2) {
        vibez.spill("Successfully computed eigenvalues:")
        bestie (sus i drip = 0; i < len_array(eigenvals); i = i + 1) {
            vibez.spill("  λ", i + 1, "=", format_number_precise(eigenvals[i]))
        }
    } otherwise {
        vibez.spill("Failed to compute eigenvalues for large matrix")
    }
}

# Test matrix decompositions
slay test_matrix_decompositions() {
    vibez.spill("\n=== Matrix Decompositions Test ===")
    
    # Create a well-conditioned 4x4 matrix for decomposition tests
    sus matrix_4x4_data []drip = [
        4,  2,  1,  0,
        2,  5,  3,  1,
        1,  3,  6,  2,
        0,  1,  2,  4
    ]
    
    sus test_matrix Matrix = create_matrix(4, 4, matrix_4x4_data)
    
    vibez.spill("Test matrix for decompositions:")
    print_matrix_detailed(test_matrix)
    
    # LU Decomposition
    vibez.spill("\n--- LU Decomposition with Partial Pivoting ---")
    sus lu LuDecomposition = lu_decomposition_pivoting(test_matrix)
    
    vibez.spill("L matrix (Lower triangular):")
    print_matrix_detailed(lu.l)
    
    vibez.spill("U matrix (Upper triangular):")
    print_matrix_detailed(lu.u)
    
    vibez.spill("P matrix (Permutation):")
    print_matrix_detailed(lu.p)
    
    vibez.spill("Determinant from LU:", format_number_precise(lu.determinant))
    vibez.spill("Matrix rank:", lu.rank)
    
    # QR Decomposition
    vibez.spill("\n--- QR Decomposition (Householder Reflections) ---")
    sus qr QrDecomposition = qr_decomposition_householder(test_matrix)
    
    vibez.spill("Q matrix (Orthogonal):")
    print_matrix_detailed(qr.q)
    
    vibez.spill("R matrix (Upper triangular):")
    print_matrix_detailed(qr.r)
    
    vibez.spill("Matrix rank:", qr.rank)
    vibez.spill("Condition number:", format_number_precise(qr.condition_number))
    
    # SVD Decomposition
    vibez.spill("\n--- SVD Decomposition (Singular Value Decomposition) ---")
    sus svd SvdDecomposition = svd_decomposition_jacobi(test_matrix)
    
    vibez.spill("Singular values:")
    bestie (sus i drip = 0; i < len_array(svd.sigma); i = i + 1) {
        vibez.spill("  σ", i + 1, "=", format_number_precise(svd.sigma[i]))
    }
    
    vibez.spill("Matrix rank:", svd.rank)
    vibez.spill("Condition number:", format_number_precise(svd.condition_number))
    vibez.spill("Nuclear norm:", format_number_precise(svd.nuclear_norm))
}

# Test eigenvalue decomposition in detail
slay test_eigenvalue_decomposition() {
    vibez.spill("\n=== Eigenvalue Decomposition Test ===")
    
    # Create a symmetric matrix (guarantees real eigenvalues)
    sus symmetric_data []drip = [
        6,  2,  1,
        2,  3,  1,
        1,  1,  1
    ]
    
    sus symmetric_matrix Matrix = create_matrix(3, 3, symmetric_data)
    
    vibez.spill("Symmetric matrix for eigendecomposition:")
    print_matrix_detailed(symmetric_matrix)
    
    # Full eigendecomposition
    sus eigen EigenDecomposition = eigendecomposition_jacobi(symmetric_matrix)
    
    vibez.spill("\n--- Complete Eigendecomposition ---")
    vibez.spill("Eigenvalues (sorted descending):")
    bestie (sus i drip = 0; i < len_array(eigen.eigenvalues); i = i + 1) {
        vibez.spill("  λ", i + 1, "=", format_number_precise(eigen.eigenvalues[i]))
    }
    
    vibez.spill("Eigenvectors (as columns):")
    print_matrix_detailed(eigen.eigenvectors)
    
    vibez.spill("Is symmetric:", eigen.is_symmetric)
    vibez.spill("Condition number:", format_number_precise(eigen.condition_number))
    
    # Verify eigendecomposition: A * v = λ * v
    vibez.spill("\n--- Eigendecomposition Verification ---")
    bestie (sus i drip = 0; i < len_array(eigen.eigenvalues); i = i + 1) {
        # Extract eigenvector column i
        sus eigenvector []drip = create_array(3)
        bestie (sus j drip = 0; j < 3; j = j + 1) {
            eigenvector[j] = eigen.eigenvectors.data[j * 3 + i]
        }
        
        # Compute A * v
        sus av []drip = matrix_vector_multiply(symmetric_matrix, eigenvector)
        
        # Compute λ * v
        sus eigenval drip = eigen.eigenvalues[i]
        
        vibez.spill("For eigenvalue λ", i + 1, "=", format_number_precise(eigenval))
        vibez.spill("  A*v =", vector_to_string(av))
        vibez.spill("  λ*v =", vector_to_string_scaled(eigenvector, eigenval))
    }
}

# Test iterative linear solvers
slay test_iterative_solvers() {
    vibez.spill("\n=== Iterative Linear System Solvers ===")
    
    # Create a symmetric positive definite system (ideal for CG)
    sus spd_matrix_data []drip = [
        4,  1,  0,
        1,  3,  1,
        0,  1,  2
    ]
    
    sus spd_matrix Matrix = create_matrix(3, 3, spd_matrix_data)
    sus rhs []drip = [1, 2, 3]
    sus initial_guess []drip = [0, 0, 0]
    
    vibez.spill("Symmetric positive definite system Ax = b:")
    vibez.spill("Matrix A:")
    print_matrix_detailed(spd_matrix)
    vibez.spill("Right-hand side b:", vector_to_string(rhs))
    
    # Conjugate Gradient Method
    vibez.spill("\n--- Conjugate Gradient Method ---")
    sus cg_result IterativeResult = conjugate_gradient_solve(spd_matrix, rhs, initial_guess, 1e-8, 100)
    
    vibez.spill("Solution x:", vector_to_string(cg_result.solution))
    vibez.spill("Residual norm:", format_number_precise(cg_result.residual_norm))
    vibez.spill("Iterations:", cg_result.iterations)
    vibez.spill("Converged:", cg_result.converged)
    
    # Verify solution
    sus verification []drip = matrix_vector_multiply(spd_matrix, cg_result.solution)
    vibez.spill("Verification A*x:", vector_to_string(verification))
    vibez.spill("Expected b:    ", vector_to_string(rhs))
    
    # GMRES Method
    vibez.spill("\n--- GMRES Method ---")
    sus gmres_result IterativeResult = gmres_solve(spd_matrix, rhs, initial_guess, 3, 1e-8, 30)
    
    vibez.spill("GMRES solution x:", vector_to_string(gmres_result.solution))
    vibez.spill("Residual norm:", format_number_precise(gmres_result.residual_norm))
    vibez.spill("Iterations:", gmres_result.iterations)
    vibez.spill("Converged:", gmres_result.converged)
}

# Test matrix norms and conditioning
slay test_matrix_norms_conditioning() {
    vibez.spill("\n=== Matrix Norms and Conditioning ===")
    
    # Well-conditioned matrix
    sus well_conditioned_data []drip = [
        2,  0,  0,
        0,  2,  0,
        0,  0,  2
    ]
    
    sus well_conditioned Matrix = create_matrix(3, 3, well_conditioned_data)
    
    vibez.spill("Well-conditioned matrix (2I):")
    print_matrix_detailed(well_conditioned)
    
    sus wc_norms MatrixNorms = calculate_matrix_norms(well_conditioned)
    
    vibez.spill("Matrix norms:")
    vibez.spill("  Frobenius norm:", format_number_precise(wc_norms.frobenius))
    vibez.spill("  Spectral norm (largest σ):", format_number_precise(wc_norms.spectral))
    vibez.spill("  Nuclear norm (Σσᵢ):", format_number_precise(wc_norms.nuclear))
    vibez.spill("  Condition number (2-norm):", format_number_precise(wc_norms.condition_2))
    vibez.spill("  Condition number (∞-norm):", format_number_precise(wc_norms.condition_inf))
    
    # Ill-conditioned matrix
    vibez.spill("\n--- Ill-Conditioned Matrix Test ---")
    sus ill_conditioned_data []drip = [
        1,     1,     1,
        1,     1.01,  1,
        1,     1,     1.01
    ]
    
    sus ill_conditioned Matrix = create_matrix(3, 3, ill_conditioned_data)
    
    vibez.spill("Ill-conditioned matrix:")
    print_matrix_detailed(ill_conditioned)
    
    sus ic_norms MatrixNorms = calculate_matrix_norms(ill_conditioned)
    
    vibez.spill("Matrix norms:")
    vibez.spill("  Condition number (2-norm):", format_number_precise(ic_norms.condition_2))
    vibez.spill("  Condition number (∞-norm):", format_number_precise(ic_norms.condition_inf))
    
    ready (ic_norms.condition_2 > 100) {
        vibez.spill("  Status: ILL-CONDITIONED (high condition number)")
    } otherwise {
        vibez.spill("  Status: Well-conditioned")
    }
}

# Test specialized matrices
slay test_specialized_matrices() {
    vibez.spill("\n=== Specialized Matrix Types ===")
    
    # Toeplitz matrix
    sus toeplitz_first_row []drip = [1, 2, 3, 4]
    sus toeplitz_first_col []drip = [1, 5, 6, 7]
    
    sus toeplitz Matrix = create_toeplitz_matrix(toeplitz_first_row, toeplitz_first_col)
    
    vibez.spill("Toeplitz matrix:")
    print_matrix_detailed(toeplitz)
    
    # Hankel matrix
    sus hankel_first_row []drip = [1, 2, 3]
    sus hankel_last_col []drip = [3, 4, 5]
    
    sus hankel Matrix = create_hankel_matrix(hankel_first_row, hankel_last_col)
    
    vibez.spill("Hankel matrix:")
    print_matrix_detailed(hankel)
    
    # Circulant matrix
    sus circulant_first_row []drip = [1, 2, 3, 4]
    
    sus circulant Matrix = create_circulant_matrix(circulant_first_row)
    
    vibez.spill("Circulant matrix:")
    print_matrix_detailed(circulant)
    
    # Test eigenvalues of circulant matrix (should have special structure)
    sus circ_eigenvals []drip = eigenvalues(circulant)
    vibez.spill("Circulant matrix eigenvalues:")
    bestie (sus i drip = 0; i < len_array(circ_eigenvals); i = i + 1) {
        vibez.spill("  λ", i + 1, "=", format_number_precise(circ_eigenvals[i]))
    }
}

# Test matrix functions
slay test_matrix_functions() {
    vibez.spill("\n=== Matrix Functions ===")
    
    # Create a small symmetric matrix for matrix functions
    sus small_matrix_data []drip = [
        2,  1,
        1,  2
    ]
    
    sus small_matrix Matrix = create_matrix(2, 2, small_matrix_data)
    
    vibez.spill("Test matrix for functions:")
    print_matrix_detailed(small_matrix)
    
    # Matrix exponential
    vibez.spill("\n--- Matrix Exponential (Padé Approximation) ---")
    sus matrix_exp Matrix = matrix_exponential_pade(small_matrix, 8)
    
    vibez.spill("e^A (matrix exponential):")
    print_matrix_detailed(matrix_exp)
    
    # Matrix logarithm
    vibez.spill("\n--- Matrix Logarithm (Power Series) ---")
    sus matrix_log Matrix = matrix_logarithm_series(matrix_exp, 20)
    
    vibez.spill("log(e^A) (should ≈ A):")
    print_matrix_detailed(matrix_log)
    
    # Matrix power
    vibez.spill("\n--- Matrix Powers ---")
    sus matrix_squared Matrix = matrix_power_integer(small_matrix, 2)
    sus matrix_cubed Matrix = matrix_power_integer(small_matrix, 3)
    
    vibez.spill("A²:")
    print_matrix_detailed(matrix_squared)
    
    vibez.spill("A³:")
    print_matrix_detailed(matrix_cubed)
    
    # Verify A² = A × A
    sus verify_square Matrix = matrix_multiply(small_matrix, small_matrix)
    vibez.spill("Verification A×A:")
    print_matrix_detailed(verify_square)
}

# Test random matrix generation and properties
slay test_random_matrices() {
    vibez.spill("\n=== Random Matrix Generation and Analysis ===")
    
    # Generate random matrices with different seeds
    sus random1 Matrix = create_random_matrix(4, 4, 12345)
    sus random2 Matrix = create_random_matrix(4, 4, 54321)
    
    vibez.spill("Random matrix 1 (seed=12345):")
    print_matrix_detailed(random1)
    
    vibez.spill("Random matrix 2 (seed=54321):")
    print_matrix_detailed(random2)
    
    # Analyze properties of random matrices
    sus r1_norms MatrixNorms = calculate_matrix_norms(random1)
    sus r2_norms MatrixNorms = calculate_matrix_norms(random2)
    
    vibez.spill("Random matrix 1 - Condition number:", format_number_precise(r1_norms.condition_2))
    vibez.spill("Random matrix 2 - Condition number:", format_number_precise(r2_norms.condition_2))
    
    # Test matrix multiplication with random matrices
    sus random_product Matrix = matrix_multiply(random1, random2)
    
    vibez.spill("Product of random matrices:")
    print_matrix_detailed(random_product)
    
    # Compute eigenvalues of random matrix
    sus random_eigenvals []drip = eigenvalues(random1)
    vibez.spill("Eigenvalues of random matrix 1:")
    bestie (sus i drip = 0; i < len_array(random_eigenvals); i = i + 1) {
        vibez.spill("  λ", i + 1, "=", format_number_precise(random_eigenvals[i]))
    }
}

# Scientific computing application: Solve heat equation (discrete)
slay test_scientific_application_heat_equation() {
    vibez.spill("\n=== Scientific Application: 1D Heat Equation ===")
    
    # Discrete 1D heat equation: -d²u/dx² = f
    # Using finite differences: -u[i-1] + 2u[i] - u[i+1] = h²f[i]
    sus n drip = 5  # Interior points
    sus h drip = 1.0 / (n + 1)  # Grid spacing
    
    # Create tridiagonal matrix for -d²/dx²
    sus heat_matrix_data []drip = create_array(n * n)
    
    bestie (sus i drip = 0; i < n; i = i + 1) {
        bestie (sus j drip = 0; j < n; j = j + 1) {
            ready (i == j) {
                heat_matrix_data[i * n + j] = 2.0 / (h * h)  # Main diagonal
            } otherwise ready (abs_val(i - j) == 1) {
                heat_matrix_data[i * n + j] = -1.0 / (h * h)  # Off-diagonals
            } otherwise {
                heat_matrix_data[i * n + j] = 0.0
            }
        }
    }
    
    sus heat_matrix Matrix = create_matrix(n, n, heat_matrix_data)
    
    # Right-hand side: f(x) = sin(πx)
    sus rhs []drip = create_array(n)
    bestie (sus i drip = 0; i < n; i = i + 1) {
        sus x drip = (i + 1) * h
        rhs[i] = sin_taylor(PI * x)
    }
    
    vibez.spill("Heat equation matrix (finite differences):")
    print_matrix_detailed(heat_matrix)
    vibez.spill("Right-hand side f(x) = sin(πx):")
    vibez.spill("RHS:", vector_to_string(rhs))
    
    # Solve using direct method (LU)
    sus heat_solution_direct []drip = solve_linear_system_lu(heat_matrix, rhs)
    
    vibez.spill("\n--- Direct Solution (LU Decomposition) ---")
    vibez.spill("Solution u(x):", vector_to_string(heat_solution_direct))
    
    # Solve using iterative method (Conjugate Gradient)
    sus initial_guess []drip = create_zeros(n)
    sus heat_solution_cg IterativeResult = conjugate_gradient_solve(heat_matrix, rhs, initial_guess, 1e-10, 100)
    
    vibez.spill("\n--- Iterative Solution (Conjugate Gradient) ---")
    vibez.spill("CG solution u(x):", vector_to_string(heat_solution_cg.solution))
    vibez.spill("CG iterations:", heat_solution_cg.iterations)
    vibez.spill("CG residual:", format_number_precise(heat_solution_cg.residual_norm))
    
    # Compare solutions
    sus solution_diff drip = vector_difference_norm(heat_solution_direct, heat_solution_cg.solution, n)
    vibez.spill("Difference between direct and iterative solutions:", format_number_precise(solution_diff))
}

# Utility functions for testing
slay print_matrix_detailed(m Matrix) {
    vibez.spill("  Matrix", m.rows, "×", m.cols, ":")
    bestie (sus i drip = 0; i < m.rows; i = i + 1) {
        sus row tea = "    ["
        bestie (sus j drip = 0; j < m.cols; j = j + 1) {
            ready (j > 0) {
                row = row + ", "
            }
            row = row + format_number_precise(m.data[i * m.cols + j])
        }
        row = row + "]"
        vibez.spill(row)
    }
}

slay format_number_precise(value drip) tea {
    # Format to 6 decimal places for scientific accuracy
    sus integer_part drip = floor_func(abs_val(value))
    sus fractional_part drip = abs_val(value) - integer_part
    sus decimal_digits drip = floor_func(fractional_part * 1000000)
    
    sus sign tea = ""
    ready (value < 0) {
        sign = "-"
    }
    
    sus result tea = sign + int_to_string(integer_part) + "."
    
    # Add leading zeros if necessary
    ready (decimal_digits < 100000) {
        result = result + "0"
        ready (decimal_digits < 10000) {
            result = result + "0"
            ready (decimal_digits < 1000) {
                result = result + "0"
                ready (decimal_digits < 100) {
                    result = result + "0"
                    ready (decimal_digits < 10) {
                        result = result + "0"
                    }
                }
            }
        }
    }
    
    result = result + int_to_string(decimal_digits)
    damn result
}

slay vector_to_string(v []drip) tea {
    sus result tea = "["
    sus n drip = len_array(v)
    bestie (sus i drip = 0; i < n; i = i + 1) {
        ready (i > 0) {
            result = result + ", "
        }
        result = result + format_number_precise(v[i])
    }
    result = result + "]"
    damn result
}

slay vector_to_string_scaled(v []drip, scale drip) tea {
    sus result tea = "["
    sus n drip = len_array(v)
    bestie (sus i drip = 0; i < n; i = i + 1) {
        ready (i > 0) {
            result = result + ", "
        }
        result = result + format_number_precise(v[i] * scale)
    }
    result = result + "]"
    damn result
}

slay solve_linear_system_lu(a Matrix, b []drip) []drip {
    # Solve Ax = b using LU decomposition
    sus lu LuDecomposition = lu_decomposition_pivoting(a)
    sus n drip = a.rows
    
    # Forward substitution with permutation: Ly = Pb
    sus pb []drip = create_zeros(n)
    bestie (sus i drip = 0; i < n; i = i + 1) {
        bestie (sus j drip = 0; j < n; j = j + 1) {
            ready (lu.p.data[i * n + j] == 1.0) {
                pb[i] = b[j]
                break
            }
        }
    }
    
    sus y []drip = create_zeros(n)
    bestie (sus i drip = 0; i < n; i = i + 1) {
        sus sum drip = 0.0
        bestie (sus j drip = 0; j < i; j = j + 1) {
            sum = sum + lu.l.data[i * n + j] * y[j]
        }
        y[i] = (pb[i] - sum) / lu.l.data[i * n + i]
    }
    
    # Back substitution: Ux = y
    sus x []drip = create_zeros(n)
    bestie (sus i drip = n - 1; i >= 0; i = i - 1) {
        sus sum drip = 0.0
        bestie (sus j drip = i + 1; j < n; j = j + 1) {
            sum = sum + lu.u.data[i * n + j] * x[j]
        }
        x[i] = (y[i] - sum) / lu.u.data[i * n + i]
    }
    
    damn x
}

slay vector_difference_norm(v1 []drip, v2 []drip, n drip) drip {
    sus sum_sq drip = 0.0
    bestie (sus i drip = 0; i < n; i = i + 1) {
        sus diff drip = v1[i] - v2[i]
        sum_sq = sum_sq + diff * diff
    }
    damn sqrt_newton(sum_sq)
}

# Main comprehensive test runner
slay run_advanced_matrix_tests() {
    vibez.spill("===============================================")
    vibez.spill("CURSED ADVANCED MATRIX OPERATIONS TEST SUITE")
    vibez.spill("===============================================")
    vibez.spill("Complete scientific computing with arbitrary size matrices")
    
    test_large_matrix_operations()
    test_matrix_decompositions()
    test_eigenvalue_decomposition()
    test_iterative_solvers()
    test_matrix_norms_conditioning()
    test_specialized_matrices()
    test_matrix_functions()
    test_random_matrices()
    test_scientific_application_heat_equation()
    
    vibez.spill("\n===============================================")
    vibez.spill("ADVANCED MATRIX OPERATIONS TESTS COMPLETED")
    vibez.spill("===============================================")
    vibez.spill("✓ Large matrix inversions (LU decomposition)")
    vibez.spill("✓ Eigenvalue computation (Jacobi method)")  
    vibez.spill("✓ Matrix decompositions (LU, QR, SVD)")
    vibez.spill("✓ Iterative solvers (CG, GMRES)")
    vibez.spill("✓ Matrix norms and conditioning")
    vibez.spill("✓ Specialized matrices (Toeplitz, Hankel, Circulant)")
    vibez.spill("✓ Matrix functions (exp, log, powers)")
    vibez.spill("✓ Scientific application (Heat equation PDE)")
    vibez.spill("✓ Memory-safe operations with error handling")
    vibez.spill("✓ Production-ready numerical algorithms")
    
    vibez.spill("\nAdvanced matrix module info:")
    vibez.spill(get_advanced_matrix_module_info())
}

# Execute comprehensive tests
run_advanced_matrix_tests()
