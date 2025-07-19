# Test monomorphization integration with LLVM codegen

# Generic function that should be monomorphized
slay max<T>(a T, b T) T {
    lowkey a > b {
        damn a
    } sheesh {
        damn b
    }
}

# Function calls that trigger monomorphization
slay main() {
    # This should create max_i32 instance
    sus result1 normie = max(5, 10)
    vibez.spill(result1)
    
    # This should create max_f64 instance
    sus result2 meal = max(3.14, 2.71)
    vibez.spill(result2)
    
    # This should create max_string instance  
    sus result3 tea = max("hello", "world")
    vibez.spill(result3)
}
