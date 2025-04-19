// Mandelbrot set calculation benchmark

// Size constants
const WIDTH = 800;
const HEIGHT = 800;
const MAX_ITERATIONS = 100;

// Calculate the Mandelbrot set
function calculateMandelbrot(maxIterations) {
    const result = new Array(HEIGHT);
    
    for (let y = 0; y < HEIGHT; y++) {
        result[y] = new Array(WIDTH);
        for (let x = 0; x < WIDTH; x++) {
            const cx = (x - WIDTH / 2.0) * 4.0 / WIDTH;
            const cy = (y - HEIGHT / 2.0) * 4.0 / HEIGHT;
            
            let zx = 0.0;
            let zy = 0.0;
            let iteration = 0;
            
            while (zx*zx + zy*zy <= 4.0 && iteration < maxIterations) {
                const temp = zx*zx - zy*zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = temp;
                iteration++;
            }
            
            result[y][x] = iteration;
        }
    }
    
    return result;
}

// Count non-black pixels in the result
function countNonBlack(result, maxIterations) {
    let count = 0;
    
    for (let y = 0; y < HEIGHT; y++) {
        for (let x = 0; x < WIDTH; x++) {
            if (result[y][x] < maxIterations) {
                count++;
            }
        }
    }
    
    return count;
}

function main() {
    const startTime = Date.now();
    
    const result = calculateMandelbrot(MAX_ITERATIONS);
    const count = countNonBlack(result, MAX_ITERATIONS);
    
    console.log("Mandelbrot set calculation finished.");
    console.log(`Image size: ${WIDTH} x ${HEIGHT}`);
    console.log(`Maximum iterations: ${MAX_ITERATIONS}`);
    console.log(`Non-black pixels: ${count}`);
    
    const elapsed = Date.now() - startTime;
    console.log(`Time taken: ${elapsed} ms`);
    
    // Get memory stats
    const memoryUsageInMB = process.memoryUsage().heapUsed / 1024 / 1024;
    console.log(`Memory used: ${Math.round(memoryUsageInMB * 1024)} KB`);
}

main();