// Test mathz.add_two() function that was failing in compiled mode
sus main() -> i32 {
    sus result drip = mathz.add_two(5, 3)
    yap("Testing mathz.add_two(5, 3):")
    yap(result) 
    return 0
}
