// FASTA benchmark for Swift

import Foundation

// Constants for the random number generator
let IM: Int32 = 139968
let IA: Int32 = 3877
let IC: Int32 = 29573
let SEED: Int32 = 42

// Define DNA sequences
let ALU = "GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGGGAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGACCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAATACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCAGCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGGAGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCCAGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA"

let IUB_PROB: [Double] = [
    0.27, 0.12, 0.12, 0.27, 0.02,
    0.02, 0.02, 0.02, 0.02, 0.02,
    0.02, 0.02, 0.02, 0.02, 0.02,
]

let IUB_CHAR: [Character] = [
    "a", "c", "g", "t", "B",
    "D", "H", "K", "M", "N",
    "R", "S", "V", "W", "Y",
]

let HOMO_SAPIENS_PROB: [Double] = [
    0.3029549426680, 0.1979883004921,
    0.1975473066391, 0.3015094502008,
]

let HOMO_SAPIENS_CHAR: [Character] = [
    "a", "c", "g", "t",
]

// Generate a random number
func genRandom(seed: inout Int32) -> Double {
    let value = (seed * IA + IC) % IM
    seed = value
    return Double(value) / Double(IM)
}

// Generate a random FASTA sequence
func genRandomFasta(n: Int, seed: inout Int32, probs: [Double], chars: [Character]) -> String {
    let length = probs.count
    var buffer = ""
    buffer.reserveCapacity(n)
    
    for _ in 0..<n {
        var r = genRandom(seed: &seed)
        var c: Character = " "
        
        for j in 0..<length {
            if r < probs[j] {
                c = chars[j]
                break
            }
            r -= probs[j]
        }
        
        buffer.append(c)
    }
    
    return buffer
}

// Repeat a sequence until it reaches the required length
func repeatFasta(n: Int, seq: String) -> String {
    let seqLength = seq.count
    var buffer = ""
    buffer.reserveCapacity(n)
    
    let seqArray = Array(seq)
    for i in 0..<n {
        buffer.append(seqArray[i % seqLength])
    }
    
    return buffer
}

func main() {
    let n = 1_000_000 // Default sequence length
    var seed = SEED
    let startTime = Date()
    
    // Write FASTA header and sequence for Homo sapiens Alu
    print(">ONE Homo sapiens alu")
    let aluSeq = repeatFasta(n: n, seq: ALU)
    print(aluSeq)
    
    // Write FASTA header and random sequence for IUB ambiguity codes
    print(">TWO IUB ambiguity codes")
    let iubSeq = genRandomFasta(n: n, seed: &seed, probs: IUB_PROB, chars: IUB_CHAR)
    print(iubSeq)
    
    // Write FASTA header and random sequence for Homo sapiens frequency
    print(">THREE Homo sapiens frequency")
    let sapiensSeq = genRandomFasta(n: n, seed: &seed, probs: HOMO_SAPIENS_PROB, chars: HOMO_SAPIENS_CHAR)
    print(sapiensSeq)
    
    let elapsed = Date().timeIntervalSince(startTime) * 1000
    print("Time taken: \(elapsed) ms")
    
    // Calculate approximate memory usage
    let memoryUsage = aluSeq.count + iubSeq.count + sapiensSeq.count
    print("Memory used: \(memoryUsage / 1024) KB")
}

main()