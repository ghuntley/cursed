// Test simple function with return statement in both interpreter and compiled modes

fn test_basic_return() -> number {
    return 42;
}

fn main() {
    const result = test_basic_return();
    yap(result);
}
