// Test file for package management system with visibility modifiers

// Private function (not exported)
slay private_func() {
    yolo "private"
}

// Public function (exported)
spill slay public_func() {
    yolo "public"
}

// Package-level function (exported within package)
crew slay package_func() {
    yolo "package"
}

// Private variable (not exported)
sus private_var be_like 42

// Public constant (exported)
spill facts public_const be_like "hello"

// Package-level constant (exported within package)
crew facts package_const be_like true
