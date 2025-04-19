#!/usr/bin/env php
<?php
// Mandelbrot set calculation benchmark

// Size constants
define('WIDTH', 800);
define('HEIGHT', 800);
define('MAX_ITERATIONS', 100);

// Calculate the Mandelbrot set
function calculate_mandelbrot($max_iterations) {
    $result = array();
    
    for ($y = 0; $y < HEIGHT; $y++) {
        $result[$y] = array();
        for ($x = 0; $x < WIDTH; $x++) {
            $cx = ($x - WIDTH / 2.0) * 4.0 / WIDTH;
            $cy = ($y - HEIGHT / 2.0) * 4.0 / HEIGHT;
            
            $zx = 0.0;
            $zy = 0.0;
            $iteration = 0;
            
            while ($zx*$zx + $zy*$zy <= 4.0 && $iteration < $max_iterations) {
                $temp = $zx*$zx - $zy*$zy + $cx;
                $zy = 2.0 * $zx * $zy + $cy;
                $zx = $temp;
                $iteration++;
            }
            
            $result[$y][$x] = $iteration;
        }
    }
    
    return $result;
}

// Count non-black pixels
function count_non_black($result, $max_iterations) {
    $count = 0;
    
    for ($y = 0; $y < HEIGHT; $y++) {
        for ($x = 0; $x < WIDTH; $x++) {
            if ($result[$y][$x] < $max_iterations) {
                $count++;
            }
        }
    }
    
    return $count;
}

// Main function
function main() {
    $start_time = microtime(true);
    
    $result = calculate_mandelbrot(MAX_ITERATIONS);
    $count = count_non_black($result, MAX_ITERATIONS);
    
    echo "Mandelbrot set calculation finished.\n";
    echo "Image size: " . WIDTH . " x " . HEIGHT . "\n";
    echo "Maximum iterations: " . MAX_ITERATIONS . "\n";
    echo "Non-black pixels: " . $count . "\n";
    
    $elapsed = (microtime(true) - $start_time) * 1000;
    echo "Time taken: $elapsed ms\n";
    
    // Get memory stats
    $memoryUsed = memory_get_peak_usage(true) / 1024;
    echo "Memory used: $memoryUsed KB\n";
}

main();