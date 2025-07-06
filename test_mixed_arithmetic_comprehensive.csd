slay test_mixed_arithmetic() {
    fr fr Test Integer + Float
    sus result1 normie = 5 + 3.14
    vibez.spill("5 + 3.14 =")
    vibez.spill(result1)
    
    fr fr Test Float + Integer
    sus result2 normie = 3.14 + 5
    vibez.spill("3.14 + 5 =")
    vibez.spill(result2)
    
    fr fr Test Integer * Float
    sus result3 normie = 10 * 2.5
    vibez.spill("10 * 2.5 =")
    vibez.spill(result3)
    
    fr fr Test Float / Integer
    sus result4 normie = 15.0 / 3
    vibez.spill("15.0 / 3 =")
    vibez.spill(result4)
    
    fr fr Test comparison operations
    lowkey 5 > 3.14 {
        vibez.spill("5 > 3.14 is based")
    }
    
    lowkey 2.5 < 10 {
        vibez.spill("2.5 < 10 is based")
    }
}

slay main() {
    test_mixed_arithmetic()
    yolo 0
}
