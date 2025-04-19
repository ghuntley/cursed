// Fannkuch redux benchmark

package main

import (
	"fmt"
	"runtime"
	"time"
)

// Reverse the first n elements of the array
func flip(p []int, n int) {
	for i := 0; i < n/2; i++ {
		temp := p[i]
		p[i] = p[n-i-1]
		p[n-i-1] = temp
	}
}

// Count flips required to flip elements to get back to original order
func fannkuch(n int) int {
	p := make([]int, n)
	perm := make([]int, n)
	count := make([]int, n)
	maxFlips := 0
	checksum := 0

	// Initialize permutation
	for i := 0; i < n; i++ {
		p[i] = i
	}

	permCount := 0
	sign := 1

	for {
		// Copy permutation to perm
		for i := 0; i < n; i++ {
			perm[i] = p[i] + 1
		}

		first := p[0]
		if first != 0 {
			// Count flips
			for i := 0; i < n; i++ {
				count[i] = 0
			}

			flips := 0
			for perm[0] != 1 {
				k := perm[0] - 1
				flip(perm, k)
				flips++
				perm[0] = k + 1
			}

			if flips > maxFlips {
				maxFlips = flips
			}

			checksum += sign * flips
		}

		// Generate next permutation
		sign = -sign
		j := 1
		for p[j-1] < p[j] {
			j++
			if j == n {
				break
			}
		}
		permCount++

		if j == n {
			break
		}

		firstJ := p[j]
		for i := 0; i < j; i++ {
			if i%2 == 0 {
				temp := p[i]
				p[i] = p[j-i]
				p[j-i] = temp
			} else {
				temp := p[i]
				p[i] = p[j-i-1]
				p[j-i-1] = temp
			}
		}

		if j < 2 {
			j = 1
			for i := 1; i < n; i++ {
				if p[i-1] > p[i] {
					j = i + 1
				}
			}
			for i := 0; i < j-1; i++ {
				k := i
				temp := p[i]
				for k < j-1 {
					k++
					p[k-1] = p[k]
				}
				p[j-1] = temp
			}
		} else {
			j--
			firstJ = p[j]
			for i := j; i > 0; i-- {
				p[i] = p[i-1]
			}
			p[0] = firstJ
		}

		if permCount >= 10000 {
			break
		}
	}

	return maxFlips
}

func main() {
	n := 10
	startTime := time.Now()

	result := fannkuch(n)

	fmt.Printf("Fannkuch(%d): %d\n", n, result)

	elapsed := time.Since(startTime).Milliseconds()
	fmt.Printf("Time taken: %d ms\n", elapsed)

	// Get memory stats
	var mem runtime.MemStats
	runtime.ReadMemStats(&mem)
	fmt.Printf("Memory used: %d KB\n", mem.Alloc/1024)
}