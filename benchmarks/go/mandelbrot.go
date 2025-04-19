// Mandelbrot set calculation benchmark

package main

import (
	"fmt"
	"runtime"
	"time"
)

// Size constants
const (
	WIDTH          = 800
	HEIGHT         = 800
	MAX_ITERATIONS = 100
)

// Calculate the Mandelbrot set
func calculateMandelbrot(maxIterations int) [][]int {
	result := make([][]int, HEIGHT)
	for y := 0; y < HEIGHT; y++ {
		result[y] = make([]int, WIDTH)
		for x := 0; x < WIDTH; x++ {
			cx := (float64(x) - float64(WIDTH)/2.0) * 4.0 / float64(WIDTH)
			cy := (float64(y) - float64(HEIGHT)/2.0) * 4.0 / float64(HEIGHT)

			zx, zy := 0.0, 0.0
			iteration := 0

			for zx*zx+zy*zy <= 4.0 && iteration < maxIterations {
				temp := zx*zx - zy*zy + cx
				zy = 2.0*zx*zy + cy
				zx = temp
				iteration++
			}

			result[y][x] = iteration
		}
	}

	return result
}

// Count non-black pixels in the result
func countNonBlack(result [][]int, maxIterations int) int {
	count := 0
	for y := 0; y < HEIGHT; y++ {
		for x := 0; x < WIDTH; x++ {
			if result[y][x] < maxIterations {
				count++
			}
		}
	}
	return count
}

func main() {
	startTime := time.Now()

	result := calculateMandelbrot(MAX_ITERATIONS)
	count := countNonBlack(result, MAX_ITERATIONS)

	fmt.Printf("Mandelbrot set calculation finished.\n")
	fmt.Printf("Image size: %d x %d\n", WIDTH, HEIGHT)
	fmt.Printf("Maximum iterations: %d\n", MAX_ITERATIONS)
	fmt.Printf("Non-black pixels: %d\n", count)

	elapsed := time.Since(startTime).Milliseconds()
	fmt.Printf("Time taken: %d ms\n", elapsed)

	// Get memory stats
	var mem runtime.MemStats
	runtime.ReadMemStats(&mem)
	fmt.Printf("Memory used: %d KB\n", mem.Alloc/1024)
}