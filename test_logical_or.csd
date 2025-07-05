slay test_logical_or(a normie, b normie) normie {
    lowkey a <= 0 || b <= 0 {
        vibez.spill("At least one is non-positive")
        yolo 0
    }
    vibez.spill("Both are positive")
    yolo 1
}

slay main() {
    sus result1 = test_logical_or(5, -3)
    sus result2 = test_logical_or(5, 3)
    vibez.spill("Results:")
    yolo result1 + result2
}
