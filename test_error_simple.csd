// Simple error handling test

slay test_panic() {
    yeet_error "test panic"
}

slay main() {
    test_panic()
}
