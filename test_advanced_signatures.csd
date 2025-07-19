# Test file for advanced function signature parsing

/// This is a basic function with documentation
pub slay add_numbers(x normie, y normie) -> normie {
    damn x + y
}

/// Function with variadic parameters
slay printf_style(format tea, ...args normie) {
    vibez.spill(format)
}

/// Function returning tuple
slay get_coordinates() -> (normie, normie) {
    damn (10, 20)
}

/// Function with function pointer parameter
slay callback_function(handler fn(normie) -> lit) -> lit {
    damn handler(42)
}

/// Generic function with bounds
slay sort_data<T: Clone + Debug>(items [T]) where T: Ord {
    # Implementation would go here
}

/// Async function
async slay fetch_data(url tea) -> tea {
    # Async implementation
    damn "data"
}

/// Unsafe function with pointer
unsafe slay raw_access(ptr *normie) -> normie {
    # Unsafe pointer access
    damn 42
}

/// Complex function with multiple advanced features
/// Documentation spanning multiple lines
pub async slay complex_function<T, U>(
    mut data T,
    processor fn(T) -> U,
    options (tea, normie),
    ...extensions ParserExtension
) -> Result<U, ProcessError> 
where T: Clone + Debug,
      U: Send + Sync {
    # Complex implementation
    damn Ok(processor(data))
}

/// Function with nested array types
slay process_matrix(matrix [[normie; 10]; 20], buffer []byte) {
    # Matrix processing
}

/// Function with mutable parameters
slay modify_data(mut buffer []byte, mut count normie) {
    # Data modification
}

/// Function with default parameters
slay connect(host tea = "localhost", port normie = 8080) {
    # Connection logic
}

/// Function returning function pointer
slay create_handler() -> fn(normie) -> lit {
    # Return function pointer
}
