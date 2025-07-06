// Test the fix for "Expected expression" errors with comments
slay add(a normie, b normie) normie {
    // This used to cause "Expected expression" when // was tokenized as two slashes
    yolo a + b  // Return the sum
}
