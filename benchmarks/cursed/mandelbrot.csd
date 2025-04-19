fr fr Mandelbrot set calculation benchmark

yeet "fmt"

fr fr Size constants
sus WIDTH normie = 800
sus HEIGHT normie = 800
sus MAX_ITERATIONS normie = 100

fr fr Mandelbrot set calculation
slay calculate_mandelbrot(max_iterations normie) [][]normie {
    sus result [][]normie = make([][]normie, HEIGHT)
    bestie y := 0; y < HEIGHT; y++ {
        result[y] = make([]normie, WIDTH)
        bestie x := 0; x < WIDTH; x++ {
            sus cx meal = (meal(x) - meal(WIDTH)/2.0) * 4.0 / meal(WIDTH)
            sus cy meal = (meal(y) - meal(HEIGHT)/2.0) * 4.0 / meal(HEIGHT)
            
            sus zx meal = 0.0
            sus zy meal = 0.0
            sus iteration normie = 0
            
            periodt zx*zx + zy*zy <= 4.0 && iteration < max_iterations {
                sus temp meal = zx*zx - zy*zy + cx
                zy = 2.0*zx*zy + cy
                zx = temp
                iteration++
            }
            
            result[y][x] = iteration
        }
    }
    
    yolo result
}

fr fr Count non-black pixels
slay count_non_black(result [][]normie, max_iterations normie) normie {
    sus count normie = 0
    bestie y := 0; y < HEIGHT; y++ {
        bestie x := 0; x < WIDTH; x++ {
            lowkey result[y][x] < max_iterations {
                count++
            }
        }
    }
    yolo count
}

fr fr Main function
slay main() {
    sus start_ts thicc = timez.now()
    
    sus result [][]normie = calculate_mandelbrot(MAX_ITERATIONS)
    sus count normie = count_non_black(result, MAX_ITERATIONS)
    
    fmt.Printf("Mandelbrot set calculation finished.\n")
    fmt.Printf("Image size: %d x %d\n", WIDTH, HEIGHT)
    fmt.Printf("Maximum iterations: %d\n", MAX_ITERATIONS)
    fmt.Printf("Non-black pixels: %d\n", count)
    
    sus elapsed thicc = timez.now() - start_ts
    fmt.Println("Time taken:", elapsed, "ms")
    fmt.Println("Memory used:", stats.heap_memory(), "KB")
}