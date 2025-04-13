# crypto_subtle_drip (crypto/subtle)

## Overview
The `crypto_subtle_drip` module provides functions for performing constant-time operations to avoid side-channel attacks in cryptographic implementations. These functions help prevent timing attacks by ensuring that the time taken to execute operations does not depend on the values of sensitive data.

## Core Functions

```csd
// ConstantTimeCompare returns 1 if the two slices, x and y, have equal contents,
// and 0 otherwise. The time taken is a function of the length of the slices and
// is independent of the contents.
func ConstantTimeCompare(x, y []byte) int

// ConstantTimeByteEq returns 1 if x == y and 0 otherwise.
// This function executes in constant time.
func ConstantTimeByteEq(x, y byte) int

// ConstantTimeEq returns 1 if x == y and 0 otherwise.
// This function executes in constant time.
func ConstantTimeEq(x, y int32) int

// ConstantTimeLessOrEq returns 1 if x <= y and 0 otherwise.
// This function executes in constant time.
func ConstantTimeLessOrEq(x, y int) int

// ConstantTimeSelect returns x if v is 1 and y if v is 0.
// This function executes in constant time.
func ConstantTimeSelect(v, x, y int) int

// ConstantTimeCopy copies the contents of y into x (a slice of equal length)
// if v is 1. If v is 0, x is left unchanged. This function executes in constant time.
func ConstantTimeCopy(v int, x, y []byte)
```

## Enhanced Features

- **Constant-Time String Operations**: String comparison without timing leaks
  ```csd
  match := crypto_subtle_drip.ConstantTimeStringCompare(str1, str2)
  ```

- **Constant-Time Integer Operations**: Arithmetic without timing leaks
  ```csd
  result := crypto_subtle_drip.ConstantTimeAdd(a, b)
  product := crypto_subtle_drip.ConstantTimeMul(a, b)
  ```

- **Secret Data Handling**: Safe operations for sensitive values
  ```csd
  safeSecret := crypto_subtle_drip.NewSecretBytes([]byte("password123"))
  // Memory is automatically zeroed when no longer needed
  ```

- **Constant-Time Conditional Selection**: Choose between values securely
  ```csd
  chosenBytes := crypto_subtle_drip.ConstantTimeSelectBytes(condition, trueValue, falseValue)
  ```

- **Blinded Memory Access**: Prevent cache-timing attacks
  ```csd
  value := crypto_subtle_drip.BlindedAccess(array, index)
  ```

## Usage Examples

```csd
// Compare two secrets in constant time
func compareHMACs() {
  // Simulate two HMAC values to compare (in a real scenario, one would be computed and one received)
  expectedHMAC := []byte{0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08}
  receivedHMAC := []byte{0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08} // Same for this example
  
  // Compare in constant time
  result := crypto_subtle_drip.ConstantTimeCompare(expectedHMAC, receivedHMAC)
  
  vibez.spill("HMACs match: %v", result == 1)
  
  // Try with a different value
  differentHMAC := []byte{0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x09} // Last byte is different
  result = crypto_subtle_drip.ConstantTimeCompare(expectedHMAC, differentHMAC)
  
  vibez.spill("Different HMACs match: %v", result == 1)
  
  // Try with HMACs of different lengths
  shortHMAC := []byte{0x01, 0x02, 0x03, 0x04}
  result = crypto_subtle_drip.ConstantTimeCompare(expectedHMAC, shortHMAC)
  
  vibez.spill("HMACs of different lengths match: %v", result == 1)
}

// Constant time byte equality
func byteEquality() {
  byte1 := byte(0x42)
  byte2 := byte(0x42)
  byte3 := byte(0x43)
  
  result1 := crypto_subtle_drip.ConstantTimeByteEq(byte1, byte2)
  result2 := crypto_subtle_drip.ConstantTimeByteEq(byte1, byte3)
  
  vibez.spill("Byte equality (same): %v", result1 == 1)
  vibez.spill("Byte equality (different): %v", result2 == 1)
}

// Constant time integer equality
func integerEquality() {
  int1 := int32(1000)
  int2 := int32(1000)
  int3 := int32(2000)
  
  result1 := crypto_subtle_drip.ConstantTimeEq(int1, int2)
  result2 := crypto_subtle_drip.ConstantTimeEq(int1, int3)
  
  vibez.spill("Integer equality (same): %v", result1 == 1)
  vibez.spill("Integer equality (different): %v", result2 == 1)
}

// Constant time selection based on a condition
func constTimeSelect() {
  // Select between two values based on a condition
  condition := 1 // True condition
  trueVal := 42
  falseVal := 24
  
  selected := crypto_subtle_drip.ConstantTimeSelect(condition, trueVal, falseVal)
  vibez.spill("Selected value (condition=1): %d", selected)
  
  // Try with condition = 0
  condition = 0
  selected = crypto_subtle_drip.ConstantTimeSelect(condition, trueVal, falseVal)
  vibez.spill("Selected value (condition=0): %d", selected)
}

// Constant time conditional copying
func constTimeCopy() {
  // Define two byte slices
  dest := []byte{0xFF, 0xFF, 0xFF, 0xFF, 0xFF}
  src := []byte{0x01, 0x02, 0x03, 0x04, 0x05}
  
  // Copy src to dest if condition is 1
  condition := 1
  crypto_subtle_drip.ConstantTimeCopy(condition, dest, src)
  
  vibez.spill("After copy (condition=1): %v", dest)
  
  // Reset destination
  dest = []byte{0xFF, 0xFF, 0xFF, 0xFF, 0xFF}
  
  // Copy src to dest if condition is 0 (should not copy)
  condition = 0
  crypto_subtle_drip.ConstantTimeCopy(condition, dest, src)
  
  vibez.spill("After copy (condition=0): %v", dest)
}

// Check if x <= y in constant time
func constTimeLessOrEq() {
  // Test cases
  test := func(x, y int) {
    result := crypto_subtle_drip.ConstantTimeLessOrEq(x, y)
    vibez.spill("%d <= %d? %v", x, y, result == 1)
  }
  
  test(5, 10)  // 5 <= 10, should be true
  test(10, 5)  // 10 <= 5, should be false
  test(7, 7)   // 7 <= 7, should be true
  test(-3, 4)  // -3 <= 4, should be true
}

// Real-world example: Secure token validation
func secureTokenValidation() {
  // Simulate a stored token (e.g., in a database)
  storedToken := []byte{0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF}
  
  // Simulate a received token from a user
  receivedToken := []byte{0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF}
  
  // Validate the token in constant time
  isValid := crypto_subtle_drip.ConstantTimeCompare(storedToken, receivedToken) == 1
  
  if isValid {
    vibez.spill("Token is valid")
  } else {
    vibez.spill("Token is invalid")
  }
  
  // Simulate a timing attack:
  // A naive comparison might break early when it finds a mismatch
  naiveCompare := func(a, b []byte) bool {
    if len(a) != len(b) {
      return false
    }
    for i := 0; i < len(a); i++ {
      if a[i] != b[i] {
        return false // Exit early on first mismatch - LEAKS TIMING INFORMATION!
      }
    }
    return true
  }
  
  // Try different tokens with varying match lengths to demonstrate the problem
  tokens := [][]byte{
    []byte{0x00, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF}, // First byte wrong
    []byte{0x01, 0x00, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF}, // Second byte wrong
    []byte{0x01, 0x23, 0x00, 0x67, 0x89, 0xAB, 0xCD, 0xEF}, // Third byte wrong
    []byte{0x01, 0x23, 0x45, 0x00, 0x89, 0xAB, 0xCD, 0xEF}, // Fourth byte wrong
  }
  
  vibez.spill("\nDemonstrating timing attack vulnerability:")
  for i, token := range tokens {
    // Using secure comparison - time is independent of match length
    isValidSecure := crypto_subtle_drip.ConstantTimeCompare(storedToken, token) == 1
    
    // Using naive comparison - time depends on match length!
    isValidNaive := naiveCompare(storedToken, token)
    
    vibez.spill("Token %d: Secure: %v, Naive: %v (Matching prefix: %d bytes)", 
      i+1, isValidSecure, isValidNaive, findMatchingPrefix(storedToken, token))
  }
  
  vibez.spill("In a real attack, the timing difference would reveal information about the token!")
}

// Helper function to find the matching prefix length (for demonstration)
func findMatchingPrefix(a, b []byte) int {
  minLen := len(a)
  if len(b) < minLen {
    minLen = len(b)
  }
  
  for i := 0; i < minLen; i++ {
    if a[i] != b[i] {
      return i
    }
  }
  
  return minLen
}

// Using enhanced features
func enhancedFeaturesExample() {
  // Constant-time string comparison
  str1 := "secure-password"
  str2 := "secure-password"
  str3 := "wrong-password"
  
  match1 := crypto_subtle_drip.ConstantTimeStringCompare(str1, str2)
  match2 := crypto_subtle_drip.ConstantTimeStringCompare(str1, str3)
  
  vibez.spill("String comparison:")
  vibez.spill("  Matching strings: %v", match1 == 1)
  vibez.spill("  Different strings: %v", match2 == 1)
  
  // Constant-time integer operations
  a := 1234
  b := 5678
  
  sum := crypto_subtle_drip.ConstantTimeAdd(a, b)
  diff := crypto_subtle_drip.ConstantTimeSub(a, b)
  product := crypto_subtle_drip.ConstantTimeMul(a, b)
  isEqual := crypto_subtle_drip.ConstantTimeIntEq(a, a) // Should be 1
  
  vibez.spill("\nConstant-time integer operations:")
  vibez.spill("  %d + %d = %d", a, b, sum)
  vibez.spill("  %d - %d = %d", a, b, diff)
  vibez.spill("  %d * %d = %d", a, b, product)
  vibez.spill("  %d == %d? %v", a, a, isEqual == 1)
  
  // Secret data handling
  vibez.spill("\nSecret data handling:")
  
  // Create a secret
  password := "super-secret-password-123"
  secret := crypto_subtle_drip.NewSecretBytes([]byte(password))
  
  vibez.spill("  Created secret with length: %d bytes", secret.Len())
  
  // Use the secret (e.g., for authentication)
  isCorrect := secret.ConstantTimeCompare([]byte("wrong-password")) == 1
  vibez.spill("  Password check: %v", isCorrect)
  
  // Secret is automatically zeroed when scope ends or when Clear() is called
  secret.Clear()
  vibez.spill("  Secret cleared from memory")
  
  // Constant-time conditional selection of byte slices
  vibez.spill("\nConstant-time byte selection:")
  trueBytes := []byte{0x01, 0x02, 0x03, 0x04}
  falseBytes := []byte{0xFF, 0xFE, 0xFD, 0xFC}
  
  // Select based on condition
  selected1 := crypto_subtle_drip.ConstantTimeSelectBytes(1, trueBytes, falseBytes)
  selected2 := crypto_subtle_drip.ConstantTimeSelectBytes(0, trueBytes, falseBytes)
  
  vibez.spill("  Selected (condition=1): %v", selected1)
  vibez.spill("  Selected (condition=0): %v", selected2)
  
  // Blinded memory access
  vibez.spill("\nBlinded memory access:")
  
  // Array of values
  array := []byte{10, 20, 30, 40, 50, 60, 70, 80}
  
  // Access array elements in a way that doesn't reveal the index through timing
  index1 := 2  // We want array[2]
  index2 := 5  // We want array[5]
  
  value1 := crypto_subtle_drip.BlindedAccess(array, index1)
  value2 := crypto_subtle_drip.BlindedAccess(array, index2)
  
  vibez.spill("  Blinded access at index %d: %d", index1, value1)
  vibez.spill("  Blinded access at index %d: %d", index2, value2)
  vibez.spill("  (Normal access would be %d and %d)", array[index1], array[index2])
}
```

## Implementation Guidelines

- Implement all functions with strict constant-time guarantees
- Ensure no branching based on sensitive data
- Prevent compiler optimizations that might break constant-time behavior
- Use bitwise operations instead of branches where possible
- Be resistant to compiler and CPU optimizations
- Avoid table lookups indexed by secret data
- Provide thorough documentation about constant-time guarantees
- Include testing against timing attacks
- Optimize for both security and performance
- Support common cryptographic use cases
- Ensure functions work correctly for all possible inputs
- Use memory barriers where necessary to prevent optimizations