// String processing benchmark for Swift

import Foundation

// Create a random string of specified length
func createRandomString(length: Int) -> String {
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    return String((0..<length).map { _ in chars.randomElement()! })
}

// Process a string with various operations
func processString(_ input: String) -> String {
    // Replace all vowels with uppercase version
    var result = input
        .replacingOccurrences(of: "a", with: "A")
        .replacingOccurrences(of: "e", with: "E")
        .replacingOccurrences(of: "i", with: "I")
        .replacingOccurrences(of: "o", with: "O")
        .replacingOccurrences(of: "u", with: "U")
    
    // Replace digits with doubled value
    for i in 0...9 {
        result = result.replacingOccurrences(of: String(i), with: String(i * 2))
    }
    
    // Capitalize first letter if string is not empty
    if let firstChar = result.first {
        result = String(firstChar).uppercased() + result.dropFirst()
    }
    
    // Reverse the string
    let reversed = String(result.reversed())
    
    // Take first half
    let halfLength = reversed.count / 2
    return String(reversed.prefix(halfLength))
}

// Process multiple strings of different sizes
func processStrings(count: Int, size: Int) -> String {
    var result = ""
    
    for _ in 0..<count {
        let str = createRandomString(length: size)
        let processed = processString(str)
        result += processed
    }
    
    return result
}

func main() {
    let startTime = Date()
    
    // Process strings of different sizes
    let small = processStrings(count: 10000, size: 10)     // 10,000 strings of length 10
    let medium = processStrings(count: 1000, size: 100)    // 1,000 strings of length 100
    let large = processStrings(count: 100, size: 1000)     // 100 strings of length 1,000
    
    let resultLength = small.count + medium.count + large.count
    print("Processed string length: \(resultLength)")
    
    let elapsedTime = -startTime.timeIntervalSinceNow * 1000
    print("Time taken: \(elapsedTime) ms")
}

main()