// Mandelbrot set calculation benchmark for C++

#include <iostream>
#include <vector>
#include <chrono>

// Constants
const int WIDTH = 800;
const int HEIGHT = 800;
const int MAX_ITERATIONS = 100;

// Calculate Mandelbrot set for a single point
int mandelbrotPoint(double cx, double cy, int maxIter) {
    double zx = 0.0;
    double zy = 0.0;
    int iteration = 0;
    
    while (zx*zx + zy*zy <= 4.0 && iteration < maxIter) {
        double temp = zx*zx - zy*zy + cx;
        zy = 2.0*zx*zy + cy;
        zx = temp;
        iteration++;
    }
    
    return iteration;
}

// Calculate the entire Mandelbrot set
std::vector<std::vector<int>> calculateMandelbrot() {
    std::vector<std::vector<int>> result(HEIGHT, std::vector<int>(WIDTH));
    
    for (int y = 0; y < HEIGHT; y++) {
        for (int x = 0; x < WIDTH; x++) {
            double cx = (double(x) - double(WIDTH)/2.0) * 4.0 / double(WIDTH);
            double cy = (double(y) - double(HEIGHT)/2.0) * 4.0 / double(HEIGHT);
            result[y][x] = mandelbrotPoint(cx, cy, MAX_ITERATIONS);
        }
    }
    
    return result;
}

// Count non-black pixels
int countNonBlack(const std::vector<std::vector<int>>& result) {
    int count = 0;
    
    for (int y = 0; y < HEIGHT; y++) {
        for (int x = 0; x < WIDTH; x++) {
            if (result[y][x] < MAX_ITERATIONS) {
                count++;
            }
        }
    }
    
    return count;
}

int main() {
    auto startTime = std::chrono::high_resolution_clock::now();
    
    auto result = calculateMandelbrot();
    int count = countNonBlack(result);
    
    std::cout << "Mandelbrot set calculation finished." << std::endl;
    std::cout << "Image size: " << WIDTH << " x " << HEIGHT << std::endl;
    std::cout << "Maximum iterations: " << MAX_ITERATIONS << std::endl;
    std::cout << "Non-black pixels: " << count << std::endl;
    
    auto endTime = std::chrono::high_resolution_clock::now();
    auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(endTime - startTime);
    std::cout << "Time taken: " << elapsed.count() << " ms" << std::endl;
    
    return 0;
}