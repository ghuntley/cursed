// String processing benchmark

package main

import (
	"fmt"
	"math/rand"
	"runtime"
	"strings"
	"time"
)

func processStrings(count, size int) string {
	result := ""

	for i := 0; i < count; i++ {
		str := createRandomString(size)
		processed := processString(str)
		result = result + processed
	}

	return result
}

func createRandomString(size int) string {
	chars := "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
	result := make([]byte, size)

	for i := 0; i < size; i++ {
		result[i] = chars[rand.Intn(len(chars))]
	}

	return string(result)
}

func processString(input string) string {
	result := input

	// Replace all vowels with their uppercase version
	result = strings.ReplaceAll(result, "a", "A")
	result = strings.ReplaceAll(result, "e", "E")
	result = strings.ReplaceAll(result, "i", "I")
	result = strings.ReplaceAll(result, "o", "O")
	result = strings.ReplaceAll(result, "u", "U")

	// Replace all digits with their doubled value
	for i := 0; i < 10; i++ {
		digit := fmt.Sprintf("%d", i)
		doubled := fmt.Sprintf("%d", i*2)
		result = strings.ReplaceAll(result, digit, doubled)
	}

	// Capitalize the first letter
	if len(result) > 0 {
		first := result[:1]
		rest := result[1:]
		upper := strings.ToUpper(first)
		result = upper + rest
	}

	// Reverse the string
	reversed := ""
	for i := len(result) - 1; i >= 0; i-- {
		reversed += string(result[i])
	}

	// Take the first half of the reversed string
	halfLen := len(reversed) / 2
	result = reversed[:halfLen]

	return result
}

func main() {
	rand.Seed(time.Now().UnixNano())
	startTime := time.Now()

	// Process strings of different sizes
	small := processStrings(10000, 10)   // 10,000 strings of length 10
	medium := processStrings(1000, 100)  // 1,000 strings of length 100
	large := processStrings(100, 1000)   // 100 strings of length 1,000

	resultLength := len(small) + len(medium) + len(large)
	fmt.Println("Processed string length:", resultLength)

	elapsed := time.Since(startTime).Milliseconds()
	fmt.Printf("Time taken: %d ms\n", elapsed)

	// Get memory stats
	var mem runtime.MemStats
	runtime.ReadMemStats(&mem)
	fmt.Printf("Memory used: %d KB\n", mem.Alloc/1024)
}