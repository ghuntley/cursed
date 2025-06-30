// Test visibility modifiers in CURSED syntax

// Public function (exported)
spill slay add(a, b) {
    yolo a + b
}

// Private function (not exported)
slay internal_helper() {
    yolo "helper"
}

// Package-level function
crew slay package_helper() {
    yolo "package-level"
}

// Public constant
spill facts PI be_like 3.14159

// Private variable 
sus debug_flag be_like true

// Package constant
crew facts VERSION be_like "1.0.0"
