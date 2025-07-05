// test_project - Library module
//
// This is the main library module for your CURSED package.
// Export public functions and types here.

// Example public function
pub func greet(name: string) -> string {
    return "Hello, " + name + "!";
}

// Example public interface
pub interface Greeter {
    func greet(name: string) -> string;
}

// Example struct implementing the interface
pub struct SimpleGreeter {}

impl Greeter for SimpleGreeter {
    func greet(name: string) -> string {
        return "Hello, " + name + " from SimpleGreeter!";
    }
}
