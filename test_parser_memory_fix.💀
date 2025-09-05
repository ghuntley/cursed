// Simple test to verify parser memory leak fix
vibes demo;

fn main() {
    tea x = 42;
    spill "Hello World";
    // This should cause an error to test the errdefer cleanup
    invalid_syntax_here();
}
