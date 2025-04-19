// Mandelbrot set calculation benchmark

use std::time::Instant;

// Size constants
const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const MAX_ITERATIONS: usize = 100;

// Calculate the Mandelbrot set
fn calculate_mandelbrot(max_iterations: usize) -> Vec<Vec<usize>> {
    let mut result = vec![vec![0; WIDTH]; HEIGHT];
    
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let cx = (x as f64 - WIDTH as f64 / 2.0) * 4.0 / WIDTH as f64;
            let cy = (y as f64 - HEIGHT as f64 / 2.0) * 4.0 / HEIGHT as f64;
            
            let mut zx = 0.0;
            let mut zy = 0.0;
            let mut iteration = 0;
            
            while zx*zx + zy*zy <= 4.0 && iteration < max_iterations {
                let temp = zx*zx - zy*zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = temp;
                iteration += 1;
            }
            
            result[y][x] = iteration;
        }
    }
    
    result
}

// Count non-black pixels in the result
fn count_non_black(result: &Vec<Vec<usize>>, max_iterations: usize) -> usize {
    let mut count = 0;
    
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if result[y][x] < max_iterations {
                count += 1;
            }
        }
    }
    
    count
}

fn main() {
    let start_time = Instant::now();
    
    let result = calculate_mandelbrot(MAX_ITERATIONS);
    let count = count_non_black(&result, MAX_ITERATIONS);
    
    println!("Mandelbrot set calculation finished.");
    println!("Image size: {} x {}", WIDTH, HEIGHT);
    println!("Maximum iterations: {}", MAX_ITERATIONS);
    println!("Non-black pixels: {}", count);
    
    let elapsed = start_time.elapsed();
    println!("Time taken: {} ms", elapsed.as_millis());
    
    // Get approximate memory usage
    let memory_usage = std::mem::size_of::<Vec<Vec<usize>>>() + (HEIGHT * WIDTH * std::mem::size_of::<usize>());
    println!("Memory used: {} KB", memory_usage / 1024);
}