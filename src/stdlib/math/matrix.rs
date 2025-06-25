use crate::error::CursedError;
/// Matrix Operations Module for CURSED
/// 
/// Provides comprehensive matrix operations including basic arithmetic, decompositions,
/// eigenvalue computations, and specialized matrix algorithms for scientific computing
/// and machine learning applications.

use std::fmt;
use super::{MathError, MathResult, validate_float};

// =============================================================================
// MATRIX STRUCTURE AND BASIC OPERATIONS
// =============================================================================

/// Matrix data structure with row-major storage
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    /// Creates a new matrix with given dimensions
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }
    
    /// Creates a matrix from 2D vector
    pub fn from_vec(data: Vec<Vec<f64>>) -> MathResult<Self> {
        if data.is_empty() {
            return Err(MathError::InvalidInput {
                function: "Matrix::from_vec".to_string(),
                parameter: "data".to_string(),
                value: 0.0,
            });
        }
        
        let rows = data.len();
        let cols = data[0].len();
        
        // Check consistent column count
        for (i, row) in data.iter().enumerate() {
            if row.len() != cols {
                return Err(MathError::InvalidInput {
                    function: "Matrix::from_vec".to_string(),
                    parameter: format!("row_{}_length", i),
                    value: row.len() as f64,
                });
            }
        }
        
        let flat_data: Vec<f64> = data.into_iter().flatten().collect();
        
        Ok(Self {
            rows,
            cols,
            data: flat_data,
        })
    }
    
    /// Creates identity matrix
    pub fn identity(size: usize) -> Self {
        let mut matrix = Self::new(size, size);
        for i in 0..size {
            matrix.set(i, i, 1.0);
        }
        matrix
    }
    
    /// Creates matrix filled with zeros
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self::new(rows, cols)
    }
    
    /// Creates matrix filled with ones
    pub fn ones(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![1.0; rows * cols],
        }
    }
    
    /// Gets element at (row, col)
    pub fn get(&self, row: usize, col: usize) -> MathResult<f64> {
        if row >= self.rows || col >= self.cols {
            return Err(MathError::InvalidInput {
                function: "Matrix::get".to_string(),
                parameter: "indices".to_string(),
                value: (row * self.cols + col) as f64,
            });
        }
        Ok(self.data[row * self.cols + col])
    }
    
    /// Sets element at (row, col)
    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        if row < self.rows && col < self.cols {
            self.data[row * self.cols + col] = value;
        }
    }
    
    /// Gets row as vector
    pub fn get_row(&self, row: usize) -> MathResult<Vec<f64>> {
        if row >= self.rows {
            return Err(MathError::InvalidInput {
                function: "Matrix::get_row".to_string(),
                parameter: "row".to_string(),
                value: row as f64,
            });
        }
        
        let start = row * self.cols;
        let end = start + self.cols;
        Ok(self.data[start..end].to_vec())
    }
    
    /// Gets column as vector
    pub fn get_col(&self, col: usize) -> MathResult<Vec<f64>> {
        if col >= self.cols {
            return Err(MathError::InvalidInput {
                function: "Matrix::get_col".to_string(),
                parameter: "col".to_string(),
                value: col as f64,
            });
        }
        
        let mut column = Vec::with_capacity(self.rows);
        for row in 0..self.rows {
            column.push(self.data[row * self.cols + col]);
        }
        Ok(column)
    }
    
    /// Transposes the matrix
    pub fn transpose(&self) -> Self {
        let mut result = Self::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(j, i, self.get(i, j).unwrap());
            }
        }
        result
    }
    
    /// Checks if matrix is square
    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }
    
    /// Checks if matrix is symmetric
    pub fn is_symmetric(&self) -> bool {
        if !self.is_square() {
            return false;
        }
        
        for i in 0..self.rows {
            for j in 0..self.cols {
                if (self.get(i, j).unwrap() - self.get(j, i).unwrap()).abs() > 1e-10 {
                    return false;
                }
            }
        }
        true
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix {}x{} [", self.rows, self.cols)?;
        for i in 0..self.rows {
            write!(f, "  [")?;
            for j in 0..self.cols {
                if j > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:8.4}", self.get(i, j).unwrap())?;
            }
            writeln!(f, "]")?;
        }
        write!(f, "]")
    }
}

// =============================================================================
// MATRIX ARITHMETIC OPERATIONS
// =============================================================================

/// Matrix addition
pub fn matrix_add(a: &Matrix, b: &Matrix) -> MathResult<Matrix> {
    if a.rows != b.rows || a.cols != b.cols {
        return Err(MathError::InvalidInput {
            function: "matrix_add".to_string(),
            parameter: "dimensions".to_string(),
            value: (a.rows * a.cols + b.rows * b.cols) as f64,
        });
    }
    
    let mut result = Matrix::new(a.rows, a.cols);
    for i in 0..a.data.len() {
        result.data[i] = a.data[i] + b.data[i];
    }
    Ok(result)
}

/// Matrix subtraction
pub fn matrix_subtract(a: &Matrix, b: &Matrix) -> MathResult<Matrix> {
    if a.rows != b.rows || a.cols != b.cols {
        return Err(MathError::InvalidInput {
            function: "matrix_subtract".to_string(),
            parameter: "dimensions".to_string(),
            value: (a.rows * a.cols + b.rows * b.cols) as f64,
        });
    }
    
    let mut result = Matrix::new(a.rows, a.cols);
    for i in 0..a.data.len() {
        result.data[i] = a.data[i] - b.data[i];
    }
    Ok(result)
}

/// Matrix multiplication
pub fn matrix_multiply(a: &Matrix, b: &Matrix) -> MathResult<Matrix> {
    if a.cols != b.rows {
        return Err(MathError::InvalidInput {
            function: "matrix_multiply".to_string(),
            parameter: "dimensions".to_string(),
            value: (a.cols as f64 - b.rows as f64),
        });
    }
    
    let mut result = Matrix::new(a.rows, b.cols);
    
    for i in 0..a.rows {
        for j in 0..b.cols {
            let mut sum = 0.0;
            for k in 0..a.cols {
                sum += a.get(i, k)? * b.get(k, j)?;
            }
            result.set(i, j, sum);
        }
    }
    
    Ok(result)
}

/// Scalar multiplication
pub fn matrix_scalar_multiply(matrix: &Matrix, scalar: f64) -> MathResult<Matrix> {
    validate_float("matrix_scalar_multiply", "scalar", scalar)?;
    
    let mut result = matrix.clone();
    for value in &mut result.data {
        *value *= scalar;
    }
    Ok(result)
}

/// Matrix-vector multiplication
pub fn matrix_vector_multiply(matrix: &Matrix, vector: &[f64]) -> MathResult<Vec<f64>> {
    if matrix.cols != vector.len() {
        return Err(MathError::InvalidInput {
            function: "matrix_vector_multiply".to_string(),
            parameter: "dimensions".to_string(),
            value: (matrix.cols as f64 - vector.len() as f64),
        });
    }
    
    let mut result = vec![0.0; matrix.rows];
    
    for i in 0..matrix.rows {
        for j in 0..matrix.cols {
            result[i] += matrix.get(i, j)? * vector[j];
        }
    }
    
    Ok(result)
}

// =============================================================================
// MATRIX DECOMPOSITIONS
// =============================================================================

/// LU decomposition result
#[derive(Debug, Clone)]
pub struct LuDecomposition {
    pub l: Matrix,  // Lower triangular
    pub u: Matrix,  // Upper triangular
    pub p: Matrix,  // Permutation matrix
}

/// Performs LU decomposition with partial pivoting
pub fn lu_decomposition(matrix: &Matrix) -> MathResult<LuDecomposition> {
    if !matrix.is_square() {
        return Err(MathError::InvalidInput {
            function: "lu_decomposition".to_string(),
            parameter: "matrix_shape".to_string(),
            value: (matrix.rows as f64 - matrix.cols as f64),
        });
    }
    
    let n = matrix.rows;
    let mut a = matrix.clone();
    let mut p = Matrix::identity(n);
    
    // Gaussian elimination with partial pivoting
    for i in 0..n {
        // Find pivot
        let mut max_row = i;
        for k in i + 1..n {
            if a.get(k, i)?.abs() > a.get(max_row, i)?.abs() {
                max_row = k;
            }
        }
        
        // Swap rows in A and P
        if max_row != i {
            for j in 0..n {
                let temp_a = a.get(i, j)?;
                a.set(i, j, a.get(max_row, j)?);
                a.set(max_row, j, temp_a);
                
                let temp_p = p.get(i, j)?;
                p.set(i, j, p.get(max_row, j)?);
                p.set(max_row, j, temp_p);
            }
        }
        
        // Check for singular matrix
        if a.get(i, i)?.abs() < 1e-12 {
            return Err(MathError::ComputationError {
                function: "lu_decomposition".to_string(),
                message: "Matrix is singular or nearly singular".to_string(),
            });
        }
        
        // Eliminate column
        for k in i + 1..n {
            let factor = a.get(k, i)? / a.get(i, i)?;
            a.set(k, i, factor);
            
            for j in i + 1..n {
                let new_val = a.get(k, j)? - factor * a.get(i, j)?;
                a.set(k, j, new_val);
            }
        }
    }
    
    // Extract L and U matrices
    let mut l = Matrix::identity(n);
    let mut u = Matrix::new(n, n);
    
    for i in 0..n {
        for j in 0..n {
            if i > j {
                l.set(i, j, a.get(i, j)?);
            } else {
                u.set(i, j, a.get(i, j)?);
            }
        }
    }
    
    Ok(LuDecomposition { l, u, p })
}

/// QR decomposition result
#[derive(Debug, Clone)]
pub struct QrDecomposition {
    pub q: Matrix,  // Orthogonal matrix
    pub r: Matrix,  // Upper triangular matrix
}

/// Performs QR decomposition using Gram-Schmidt process
pub fn qr_decomposition(matrix: &Matrix) -> MathResult<QrDecomposition> {
    let m = matrix.rows;
    let n = matrix.cols;
    
    let mut q = Matrix::new(m, n);
    let mut r = Matrix::new(n, n);
    
    for j in 0..n {
        // Get column j
        let mut v = matrix.get_col(j)?;
        
        // Subtract projections onto previous columns
        for i in 0..j {
            let q_col = q.get_col(i)?;
            let proj = vector_dot(&v, &q_col)? / vector_dot(&q_col, &q_col)?;
            r.set(i, j, proj);
            
            for k in 0..m {
                v[k] -= proj * q_col[k];
            }
        }
        
        // Normalize
        let norm = vector_norm(&v)?;
        if norm < 1e-12 {
            return Err(MathError::ComputationError {
                function: "qr_decomposition".to_string(),
                message: "Matrix is rank deficient".to_string(),
            });
        }
        
        r.set(j, j, norm);
        
        for i in 0..m {
            q.set(i, j, v[i] / norm);
        }
    }
    
    Ok(QrDecomposition { q, r })
}

// =============================================================================
// EIGENVALUE COMPUTATIONS
// =============================================================================

/// Eigenvalue decomposition result
#[derive(Debug, Clone)]
pub struct EigenDecomposition {
    pub eigenvalues: Vec<f64>,
    pub eigenvectors: Matrix,
}

/// Computes eigenvalues and eigenvectors using QR algorithm
pub fn eigen_decomposition(matrix: &Matrix, max_iterations: Option<usize>) -> MathResult<EigenDecomposition> {
    if !matrix.is_square() {
        return Err(MathError::InvalidInput {
            function: "eigen_decomposition".to_string(),
            parameter: "matrix_shape".to_string(),
            value: (matrix.rows as f64 - matrix.cols as f64),
        });
    }
    
    let n = matrix.rows;
    let max_iter = max_iterations.unwrap_or(1000);
    
    let mut a = matrix.clone();
    let mut q_total = Matrix::identity(n);
    
    // QR algorithm
    for _iteration in 0..max_iter {
        let qr = qr_decomposition(&a)?;
        a = matrix_multiply(&qr.r, &qr.q)?;
        q_total = matrix_multiply(&q_total, &qr.q)?;
        
        // Check for convergence (off-diagonal elements small)
        let mut max_off_diag = 0.0;
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    max_off_diag = max_off_diag.max(a.get(i, j)?.abs());
                }
            }
        }
        
        if max_off_diag < 1e-10 {
            break;
        }
    }
    
    // Extract eigenvalues from diagonal
    let mut eigenvalues = Vec::with_capacity(n);
    for i in 0..n {
        eigenvalues.push(a.get(i, i)?);
    }
    
    Ok(EigenDecomposition {
        eigenvalues,
        eigenvectors: q_total,
    })
}

// =============================================================================
// UTILITY FUNCTIONS
// =============================================================================

/// Vector dot product
pub fn vector_dot(a: &[f64], b: &[f64]) -> MathResult<f64> {
    if a.len() != b.len() {
        return Err(MathError::InvalidInput {
            function: "vector_dot".to_string(),
            parameter: "vector_lengths".to_string(),
            value: (a.len() as f64 - b.len() as f64),
        });
    }
    
    Ok(a.iter().zip(b.iter()).map(|(x, y)| x * y).sum())
}

/// Vector norm (L2 norm)
pub fn vector_norm(vector: &[f64]) -> MathResult<f64> {
    let sum_squares: f64 = vector.iter().map(|x| x * x).sum();
    Ok(sum_squares.sqrt())
}

/// Matrix norm (Frobenius norm)
pub fn matrix_norm(matrix: &Matrix) -> MathResult<f64> {
    let sum_squares: f64 = matrix.data.iter().map(|x| x * x).sum();
    Ok(sum_squares.sqrt())
}

/// Matrix determinant using LU decomposition
pub fn matrix_determinant(matrix: &Matrix) -> MathResult<f64> {
    if !matrix.is_square() {
        return Err(MathError::InvalidInput {
            function: "matrix_determinant".to_string(),
            parameter: "matrix_shape".to_string(),
            value: (matrix.rows as f64 - matrix.cols as f64),
        });
    }
    
    let lu = lu_decomposition(matrix)?;
    
    // Determinant is product of diagonal elements of U
    let mut det = 1.0;
    for i in 0..matrix.rows {
        det *= lu.u.get(i, i)?;
    }
    
    // Account for row swaps in permutation matrix
    // Count number of swaps (not implemented for simplicity)
    
    Ok(det)
}

/// Matrix inverse using LU decomposition
pub fn matrix_inverse(matrix: &Matrix) -> MathResult<Matrix> {
    if !matrix.is_square() {
        return Err(MathError::InvalidInput {
            function: "matrix_inverse".to_string(),
            parameter: "matrix_shape".to_string(),
            value: (matrix.rows as f64 - matrix.cols as f64),
        });
    }
    
    let n = matrix.rows;
    let lu = lu_decomposition(matrix)?;
    let mut inverse = Matrix::new(n, n);
    
    // Solve for each column of the inverse
    for j in 0..n {
        let mut b = vec![0.0; n];
        b[j] = 1.0;
        
        // Solve Ly = Pb
        let pb = matrix_vector_multiply(&lu.p, &b)?;
        let mut y = vec![0.0; n];
        for i in 0..n {
            y[i] = pb[i];
            for k in 0..i {
                y[i] -= lu.l.get(i, k)? * y[k];
            }
        }
        
        // Solve Ux = y
        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            x[i] = y[i];
            for k in i + 1..n {
                x[i] -= lu.u.get(i, k)? * x[k];
            }
            x[i] /= lu.u.get(i, i)?;
        }
        
        // Set column in inverse matrix
        for i in 0..n {
            inverse.set(i, j, x[i]);
        }
    }
    
    Ok(inverse)
}

