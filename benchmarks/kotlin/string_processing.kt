// String processing benchmark for Kotlin

import kotlin.random.Random
import kotlin.system.measureTimeMillis

// Create a random string of specified length
fun createRandomString(length: Int): String {
    val chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    return (1..length)
        .map { chars[Random.nextInt(chars.length)] }
        .joinToString("")
}

// Process a string with various operations
fun processString(input: String): String {
    // Replace all vowels with uppercase version
    var result = input
        .replace("a", "A")
        .replace("e", "E")
        .replace("i", "I")
        .replace("o", "O")
        .replace("u", "U")
    
    // Replace digits with doubled value
    for (i in 0..9) {
        result = result.replace(i.toString(), (i * 2).toString())
    }
    
    // Capitalize first letter if string is not empty
    if (result.isNotEmpty()) {
        result = result[0].uppercaseChar() + result.substring(1)
    }
    
    // Reverse the string
    val reversed = result.reversed()
    
    // Take the first half
    return reversed.substring(0, reversed.length / 2)
}

// Process multiple strings of different sizes
fun processStrings(count: Int, size: Int): String {
    var result = ""
    repeat(count) {
        val str = createRandomString(size)
        val processed = processString(str)
        result += processed
    }
    return result
}

fun main() {
    val totalTime = measureTimeMillis {
        // Process strings of different sizes
        val small = processStrings(10000, 10)    // 10,000 strings of length 10
        val medium = processStrings(1000, 100)   // 1,000 strings of length 100
        val large = processStrings(100, 1000)    // 100 strings of length 1,000
        
        val resultLength = small.length + medium.length + large.length
        println("Processed string length: $resultLength")
    }
    
    println("Time taken: $totalTime ms")
}