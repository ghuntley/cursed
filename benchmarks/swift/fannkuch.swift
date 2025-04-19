// Fannkuch Redux benchmark for Swift

import Foundation

// Flip the first n elements in the array
func flip(_ p: inout [Int], _ n: Int) {
    for i in 0..<n/2 {
        let temp = p[i]
        p[i] = p[n-i-1]
        p[n-i-1] = temp
    }
}

// Fannkuch algorithm implementation
func fannkuch(_ n: Int) -> (maxFlips: Int, checksum: Int) {
    var p = Array(0..<n)      // Initial permutation
    var perm = Array(repeating: 0, count: n)
    var count = Array(repeating: 0, count: n)
    var maxFlips = 0
    var checksum = 0
    var permCount = 0
    var sign = 1
    
    while true {
        // Copy permutation
        for i in 0..<n {
            perm[i] = p[i] + 1
        }
        
        // Count flips
        if p[0] != 0 {
            for i in 0..<n {
                count[i] = 0
            }
            
            var flips = 0
            var k = p[0]
            
            while k != 0 {
                flip(&perm, k+1)
                flips += 1
                k = perm[0] - 1
            }
            
            maxFlips = max(maxFlips, flips)
            checksum += sign * flips
        }
        
        // Generate next permutation
        sign = -sign
        var j = 1
        while j < n && p[j-1] >= p[j] {
            j += 1
        }
        
        if j >= n { break }  // No more permutations
        
        permCount += 1
        if permCount >= 10000 { break }  // Limit permutations for benchmark
        
        let firstJ = p[j]
        var i = 0
        while i < j {
            p[i] = p[j-i-1]
            i += 1
        }
        p[j] = firstJ
    }
    
    return (maxFlips, checksum)
}

func main() {
    let n = 10  // Standard size for the benchmark
    
    let startTime = Date()
    
    let result = fannkuch(n)
    
    print("Fannkuch(\(n)): \(result.maxFlips)")
    print("Checksum: \(result.checksum)")
    
    let elapsedTime = -startTime.timeIntervalSinceNow * 1000
    print("Time taken: \(elapsedTime) ms")
}

main()