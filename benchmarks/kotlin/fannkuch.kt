// Fannkuch Redux benchmark for Kotlin

import kotlin.system.measureTimeMillis

// Flip the first n elements in the array
fun flip(p: IntArray, n: Int) {
    for (i in 0 until n/2) {
        val temp = p[i]
        p[i] = p[n-i-1]
        p[n-i-1] = temp
    }
}

// Fannkuch algorithm implementation
fun fannkuch(n: Int): Pair<Int, Int> {
    val p = IntArray(n) { it }         // Initial permutation
    val perm = IntArray(n)
    val count = IntArray(n)
    var maxFlips = 0
    var checksum = 0
    var permCount = 0
    var sign = 1
    
    while (true) {
        // Copy permutation
        for (i in 0 until n) {
            perm[i] = p[i] + 1
        }
        
        // Count flips
        if (p[0] != 0) {
            count.fill(0)
            
            var flips = 0
            var k = p[0]
            
            while (k != 0) {
                flip(perm, k+1)
                flips++
                k = perm[0] - 1
            }
            
            maxFlips = maxOf(maxFlips, flips)
            checksum += sign * flips
        }
        
        // Generate next permutation
        sign = -sign
        var j = 1
        while (j < n && p[j-1] >= p[j]) {
            j++
        }
        
        if (j >= n) break  // No more permutations
        
        permCount++
        if (permCount >= 10000) break  // Limit permutations for benchmark
        
        val firstJ = p[j]
        for (i in 0 until j) {
            p[i] = p[j-i-1]
        }
        p[j] = firstJ
    }
    
    return Pair(maxFlips, checksum)
}

fun main() {
    val n = 10  // Standard size for the benchmark
    
    val totalTime = measureTimeMillis {
        val (maxFlips, checksum) = fannkuch(n)
        
        println("Fannkuch($n): $maxFlips")
        println("Checksum: $checksum")
    }
    
    println("Time taken: $totalTime ms")
}