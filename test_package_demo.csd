// Demonstration file showing package management features

// Public interface - these will be exported
spill slay calculate_area(radius) {
    yolo PI * radius * radius
}

spill facts PI be_like 3.14159

// Package-level utilities - accessible within package
crew slay format_result(value) {
    yolo "Result: " + value
}

crew facts VERSION be_like "1.0.0"

// Private implementation details - not exported
slay internal_helper() {
    yolo "This is hidden from external modules"
}

sus debug_mode be_like false

// Main function to demonstrate usage
spill slay main() {
    sus area be_like calculate_area(5.0)
    sus formatted be_like format_result(area)
    yolo formatted
}
