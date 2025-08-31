// Oracle's test case - minimal variable declaration that caused type mismatch
sus main() -> i32 {
    sus x drip = 42
    yap(x)
    return 0
}
