// FASTA benchmark - generate and write random DNA sequences

package main

import (
	"fmt"
	"runtime"
	"strings"
	"time"
)

// Constants for the random number generator
const (
	IM    = 139968
	IA    = 3877
	IC    = 29573
	SEED  = 42
)

// Define DNA sequences
const ALU = "GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGGGAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGACCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAATACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCAGCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGGAGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCCAGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA"

var IUB_PROB = []float64{
	0.27, 0.12, 0.12, 0.27, 0.02,
	0.02, 0.02, 0.02, 0.02, 0.02,
	0.02, 0.02, 0.02, 0.02, 0.02,
}

var IUB_CHAR = []string{
	"a", "c", "g", "t", "B",
	"D", "H", "K", "M", "N",
	"R", "S", "V", "W", "Y",
}

var HOMO_SAPIENS_PROB = []float64{
	0.3029549426680, 0.1979883004921,
	0.1975473066391, 0.3015094502008,
}

var HOMO_SAPIENS_CHAR = []string{
	"a", "c", "g", "t",
}

const BUF_SIZE = 1024 * 1024

// Generate a random number
func genRandom(seed *int) float64 {
	value := (*seed * IA + IC) % IM
	*seed = value
	return float64(value) / float64(IM)
}

// Generate a random FASTA sequence
func genRandomFasta(n int, seed *int, probs []float64, chars []string) string {
	length := len(probs)
	var builder strings.Builder
	builder.Grow(n)

	for i := 0; i < n; i++ {
		r := genRandom(seed)
		var c string

		for j := 0; j < length; j++ {
			if r < probs[j] {
				c = chars[j]
				break
			}
			r -= probs[j]
		}

		builder.WriteString(c)
	}

	return builder.String()
}

// Repeat a sequence until it reaches the required length
func repeatFasta(n int, seq string) string {
	seqLen := len(seq)
	var builder strings.Builder
	builder.Grow(n)

	for i := 0; i < n; i++ {
		builder.WriteByte(seq[i%seqLen])
	}

	return builder.String()
}

func main() {
	n := 1000000 // Default sequence length
	seed := SEED
	startTime := time.Now()

	// Write FASTA header and sequence for Homo sapiens Alu
	fmt.Println(">ONE Homo sapiens alu")
	aluSeq := repeatFasta(n, ALU)
	fmt.Println(aluSeq)

	// Write FASTA header and random sequence for IUB ambiguity codes
	fmt.Println(">TWO IUB ambiguity codes")
	iubSeq := genRandomFasta(n, &seed, IUB_PROB, IUB_CHAR)
	fmt.Println(iubSeq)

	// Write FASTA header and random sequence for Homo sapiens frequency
	fmt.Println(">THREE Homo sapiens frequency")
	sapiensSeq := genRandomFasta(n, &seed, HOMO_SAPIENS_PROB, HOMO_SAPIENS_CHAR)
	fmt.Println(sapiensSeq)

	elapsed := time.Since(startTime).Milliseconds()
	fmt.Printf("Time taken: %d ms\n", elapsed)

	// Get memory stats
	var mem runtime.MemStats
	runtime.ReadMemStats(&mem)
	fmt.Printf("Memory used: %d KB\n", mem.Alloc/1024)
}