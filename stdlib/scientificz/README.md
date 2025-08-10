# scientificz - Scientific Computing Module

## Overview
Comprehensive scientific computing library providing statistical analysis, numerical methods, data visualization, and mathematical modeling capabilities - all implemented in pure CURSED for high-performance scientific applications.

## Features

### Statistical Analysis
- **Descriptive Statistics**: Mean, median, mode, variance, standard deviation, quartiles
- **Probability Distributions**: Normal, binomial, Poisson, exponential, chi-square
- **Hypothesis Testing**: t-tests, ANOVA, chi-square tests, correlation analysis
- **Regression Analysis**: Linear, polynomial, multiple regression with R-squared
- **Time Series**: Moving averages, trend analysis, seasonality detection

### Numerical Methods
- **Linear Algebra**: Matrix operations, eigenvalues, SVD, LU decomposition
- **Calculus**: Numerical integration, differentiation, root finding
- **Optimization**: Gradient descent, Newton's method, simplex algorithm
- **Differential Equations**: Euler's method, Runge-Kutta, finite differences
- **Interpolation**: Linear, polynomial, spline interpolation

### Data Processing
- **Data Structures**: Matrices, vectors, data frames, time series
- **Data Cleaning**: Missing value handling, outlier detection, normalization
- **Data Transformation**: Scaling, encoding, feature selection
- **Signal Processing**: FFT, filtering, convolution, spectral analysis
- **Image Processing**: Basic operations, filters, transformations

### Visualization
- **Plotting**: Line plots, scatter plots, histograms, box plots
- **Charts**: Bar charts, pie charts, heatmaps, contour plots
- **3D Graphics**: Surface plots, 3D scatter plots, mesh visualization
- **Statistical Plots**: Q-Q plots, probability plots, residual analysis
- **Export**: SVG, PNG, PDF output formats

## Usage Examples

### Basic Statistics
```cursed
yeet "scientificz"

sus data []drip = [12, 15, 18, 20, 22, 25, 28, 30, 32, 35]
sus stats Statistics = calculate_statistics(data)

vibez.spill("Mean:", stats.mean)
vibez.spill("Std Dev:", stats.std_dev)
vibez.spill("Correlation with Y:", pearson_correlation(data, y_data))
```

### Matrix Operations
```cursed
yeet "scientificz"

sus matrix_a Matrix = create_matrix(3, 3, [1, 2, 3, 4, 5, 6, 7, 8, 9])
sus matrix_b Matrix = create_matrix(3, 3, [9, 8, 7, 6, 5, 4, 3, 2, 1])

sus result Matrix = matrix_multiply(matrix_a, matrix_b)
sus inverse Matrix = matrix_inverse(matrix_a)
sus eigenvals []drip = eigenvalues(matrix_a)
```

### Data Visualization
```cursed
yeet "scientificz"

sus x_data []drip = [1, 2, 3, 4, 5]
sus y_data []drip = [2, 4, 6, 8, 10]

sus plot Plot = create_line_plot(x_data, y_data, "Linear Relationship")
add_trend_line(plot, LINEAR)
save_plot(plot, "linear_analysis.svg")
```

### Regression Analysis
```cursed
yeet "scientificz"

sus regression Regression = linear_regression(x_data, y_data)
vibez.spill("Slope:", regression.slope)
vibez.spill("R-squared:", regression.r_squared)

sus predictions []drip = predict(regression, [6, 7, 8])
```

## API Reference

### Statistical Functions
- `calculate_statistics(data: []drip) -> Statistics`: Comprehensive stats
- `pearson_correlation(x: []drip, y: []drip) -> drip`: Correlation coefficient
- `t_test(sample1: []drip, sample2: []drip) -> TTestResult`: Student's t-test
- `anova(groups: [][]drip) -> AnovaResult`: Analysis of variance
- `normal_distribution(mean: drip, std_dev: drip) -> Distribution`: Normal dist

### Matrix Operations
- `create_matrix(rows: drip, cols: drip, data: []drip) -> Matrix`: Create matrix
- `matrix_multiply(a: Matrix, b: Matrix) -> Matrix`: Matrix multiplication
- `matrix_inverse(m: Matrix) -> Matrix`: Matrix inversion
- `eigenvalues(m: Matrix) -> []drip`: Calculate eigenvalues
- `svd(m: Matrix) -> SvdResult`: Singular value decomposition

### Numerical Methods
- `numerical_integrate(func: tea, a: drip, b: drip) -> drip`: Integration
- `find_root(func: tea, initial: drip) -> drip`: Root finding
- `gradient_descent(func: tea, initial: []drip) -> []drip`: Optimization
- `runge_kutta(func: tea, initial: drip, step: drip) -> []drip`: ODE solver

### Visualization Functions
- `create_line_plot(x: []drip, y: []drip, title: tea) -> Plot`: Line plot
- `create_histogram(data: []drip, bins: drip) -> Plot`: Histogram
- `create_scatter_plot(x: []drip, y: []drip) -> Plot`: Scatter plot
- `save_plot(plot: Plot, filename: tea)`: Export plot

Built for research, data analysis, and scientific computing in CURSED.
