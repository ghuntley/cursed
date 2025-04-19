// FASTA benchmark for Kotlin

import kotlin.system.measureTimeMillis

// Constants for the random number generator
const val IM: Int = 139968
const val IA: Int = 3877
const val IC: Int = 29573
var SEED: Int = 42

// Define DNA sequences
const val ALU = "GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGGGAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGACCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAATACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCAGCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGGAGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCCAGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA"

val IUB_PROB = doubleArrayOf(
    0.27, 0.12, 0.12, 0.27, 0.02,
    0.02, 0.02, 0.02, 0.02, 0.02,
    0.02, 0.02, 0.02, 0.02, 0.02
)

val IUB_CHAR = charArrayOf(
    'a', 'c', 'g', 't', 'B',
    'D', 'H', 'K', 'M', 'N',
    'R', 'S', 'V', 'W', 'Y'
)

val HOMO_SAPIENS_PROB = doubleArrayOf(
    0.3029549426680, 0.1979883004921,
    0.1975473066391, 0.3015094502008
)

val HOMO_SAPIENS_CHAR = charArrayOf(
    'a', 'c', 'g', 't'
)

// Generate a random number
fun genRandom(seed: Int): Pair<Int, Double> {
    val value = (seed * IA + IC) % IM
    return Pair(value, value.toDouble() / IM)
}

// Generate a random FASTA sequence
fun genRandomFasta(n: Int, seed: Int, probs: DoubleArray, chars: CharArray): Pair<Int, String> {
    val length = probs.size
    val buffer = StringBuilder(n)
    var currentSeed = seed
    
    repeat(n) {
        val (newSeed, rnd) = genRandom(currentSeed)
        currentSeed = newSeed
        var r = rnd
        var c = ' '
        
        for (j in 0 until length) {
            if (r < probs[j]) {
                c = chars[j]
                break
            }
            r -= probs[j]
        }
        
        buffer.append(c)
    }
    
    return Pair(currentSeed, buffer.toString())
}

// Repeat a sequence until it reaches the required length
fun repeatFasta(n: Int, seq: String): String {
    val seqLen = seq.length
    val buffer = StringBuilder(n)
    
    for (i in 0 until n) {
        buffer.append(seq[i % seqLen])
    }
    
    return buffer.toString()
}

fun main() {
    val n = 1_000_000 // Default sequence length
    var seed = SEED
    
    val elapsedTime = measureTimeMillis {
        // Write FASTA header and sequence for Homo sapiens Alu
        println(">ONE Homo sapiens alu")
        val aluSeq = repeatFasta(n, ALU)
        println(aluSeq)
        
        // Write FASTA header and random sequence for IUB ambiguity codes
        println(">TWO IUB ambiguity codes")
        val (newSeed, iubSeq) = genRandomFasta(n, seed, IUB_PROB, IUB_CHAR)
        seed = newSeed
        println(iubSeq)
        
        // Write FASTA header and random sequence for Homo sapiens frequency
        println(">THREE Homo sapiens frequency")
        val (_, sapiensSeq) = genRandomFasta(n, seed, HOMO_SAPIENS_PROB, HOMO_SAPIENS_CHAR)
        println(sapiensSeq)
        
        // Calculate approximate memory usage
        val memoryUsage = aluSeq.length + iubSeq.length + sapiensSeq.length
        println("Memory used: ${memoryUsage / 1024} KB")
    }
    
    println("Time taken: $elapsedTime ms")
}