fr fr Fannkuch redux benchmark

yeet "fmt"

fr fr Reverse the first n elements of the array
slay flip(p []normie, n normie) {
    bestie i := 0; i < n/2; i++ {
        sus temp normie = p[i]
        p[i] = p[n-i-1]
        p[n-i-1] = temp
    }
}

fr fr Count flips required to flip elements to get back to original order
slay fannkuch(n normie) normie {
    sus p []normie = make([]normie, n)
    sus perm []normie = make([]normie, n)
    sus count []normie = make([]normie, n)
    sus maxFlips normie = 0
    sus checksum normie = 0
    
    fr fr Initialize permutation
    bestie i := 0; i < n; i++ {
        p[i] = i
    }
    
    sus permCount normie = 0
    sus sign normie = 1
    
    periodt true {
        fr fr Copy permutation to perm
        bestie i := 0; i < n; i++ {
            perm[i] = p[i] + 1
        }
        
        sus first normie = p[0]
        lowkey first != 0 {
            fr fr Count flips
            bestie i := 0; i < n; i++ {
                count[i] = 0
            }
            
            sus flips normie = 0
            periodt perm[0] != 1 {
                sus k normie = perm[0] - 1
                flip(perm, k)
                flips++
                perm[0] = k + 1
            }
            
            lowkey flips > maxFlips {
                maxFlips = flips
            }
            
            checksum += sign * flips
        }
        
        fr fr Generate next permutation
        sign = -sign
        sus j normie = 1
        periodt p[j-1] < p[j] {
            j++
            lowkey j == n {
                ghosted
            }
        }
        permCount++
        
        sus first_j normie = p[j]
        bestie i := 0; i < j; i++ {
            lowkey i%2 == 0 {
                sus temp normie = p[i]
                p[i] = p[j-i]
                p[j-i] = temp
            } highkey {
                sus temp normie = p[i]
                p[i] = p[j-i-1]
                p[j-i-1] = temp
            }
        }
        
        lowkey j < 2 {
            j = 1
            bestie i := 1; i < n; i++ {
                lowkey p[i-1] > p[i] {
                    j = i + 1
                }
            }
            bestie i := 0; i < j-1; i++ {
                sus k normie = i
                sus temp normie = p[i]
                periodt k < j-1 {
                    k++
                    p[k-1] = p[k]
                }
                p[j-1] = temp
            }
        } highkey {
            j--
            first_j = p[j]
            bestie i := j; i > 0; i-- {
                p[i] = p[i-1]
            }
            p[0] = first_j
        }
        
        lowkey permCount >= 10000 {
            ghosted
        }
    }
    
    yolo maxFlips
}

slay main() {
    sus n normie = 10
    sus start_ts thicc = timez.now()
    
    sus result normie = fannkuch(n)
    
    fmt.Printf("Fannkuch(%d): %d\n", n, result)
    
    sus elapsed thicc = timez.now() - start_ts
    fmt.Println("Time taken:", elapsed, "ms")
    fmt.Println("Memory used:", stats.heap_memory(), "KB")
}