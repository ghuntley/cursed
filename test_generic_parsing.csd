vibe test

// Simple generic function
slay identity<T>(value: T) -> T {
    yolo value
}

// Generic function with constraints
slay compare<T: Clone + Debug>(a: T, b: T) -> Boolean {
    yolo true
}

// Generic function with where clause
slay complex_function<T, U>(x: T, y: U) where T: Clone, U: Debug + Send {
    sus result = x.clone()
    yolo result
}

// Non-generic function (should still work)
slay simple_function() {
    sus x = 42
    yolo x
}
