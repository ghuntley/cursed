// Mandelbrot set calculation benchmark

#include <stdio.h>
#include <stdlib.h>
#include <time.h>

// Size constants
#define WIDTH 800
#define HEIGHT 800
#define MAX_ITERATIONS 100

// Calculate the Mandelbrot set
int** calculate_mandelbrot(int max_iterations) {
    int** result = (int**)malloc(HEIGHT * sizeof(int*));
    for (int y = 0; y < HEIGHT; y++) {
        result[y] = (int*)malloc(WIDTH * sizeof(int));
        for (int x = 0; x < WIDTH; x++) {
            double cx = (double)x - (double)WIDTH/2.0;
            cx = (cx * 4.0) / (double)WIDTH;
            double cy = (double)y - (double)HEIGHT/2.0;
            cy = (cy * 4.0) / (double)HEIGHT;
            
            double zx = 0.0, zy = 0.0;
            int iteration = 0;
            
            while (zx*zx + zy*zy <= 4.0 && iteration < max_iterations) {
                double temp = zx*zx - zy*zy + cx;
                zy = 2.0*zx*zy + cy;
                zx = temp;
                iteration++;
            }
            
            result[y][x] = iteration;
        }
    }
    
    return result;
}

// Count non-black pixels
int count_non_black(int** result, int max_iterations) {
    int count = 0;
    for (int y = 0; y < HEIGHT; y++) {
        for (int x = 0; x < WIDTH; x++) {
            if (result[y][x] < max_iterations) {
                count++;
            }
        }
    }
    return count;
}

// Free the 2D array
void free_result(int** result) {
    for (int y = 0; y < HEIGHT; y++) {
        free(result[y]);
    }
    free(result);
}

int main() {
    clock_t start = clock();
    
    int** result = calculate_mandelbrot(MAX_ITERATIONS);
    int count = count_non_black(result, MAX_ITERATIONS);
    
    printf("Mandelbrot set calculation finished.\n");
    printf("Image size: %d x %d\n", WIDTH, HEIGHT);
    printf("Maximum iterations: %d\n", MAX_ITERATIONS);
    printf("Non-black pixels: %d\n", count);
    
    // Free the memory
    free_result(result);
    
    // Calculate elapsed time
    clock_t end = clock();
    double elapsed = (double)(end - start) * 1000.0 / CLOCKS_PER_SEC;
    printf("Time taken: %.2f ms\n", elapsed);
    
    // Note: C doesn't have a standard way to get memory usage
    printf("Memory monitoring not available for C implementation\n");
    
    return 0;
}