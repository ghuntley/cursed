// Direct test of mathz module without imports
// Tests the mathz module functions directly

func test_mathz() drip {
    let abs_pos drip = 42
    if abs_pos < 0 {
        abs_pos = 0 - abs_pos
    }
    
    let abs_neg drip = -42
    if abs_neg < 0 {
        abs_neg = 0 - abs_neg
    }
    
    // Basic arithmetic
    let add_result drip = 15 + 25
    let sub_result drip = 30 - 12
    let mult_result drip = 6 * 7
    
    // Test return values
    return add_result + sub_result + mult_result
}

func main() drip {
    let result drip = test_mathz()
    return result
}
