// Mandelbrot set calculation benchmark for Kotlin

import kotlin.math.sqrt
import kotlin.system.measureTimeMillis

// Constants
const val WIDTH = 800
const val HEIGHT = 800
const val MAX_ITERATIONS = 100

// Calculate Mandelbrot set for a single point
fun mandelbrotPoint(cx: Double, cy: Double, maxIter: Int): Int {
    var zx = 0.0
    var zy = 0.0
    var iteration = 0
    
    while (zx*zx + zy*zy <= 4.0 && iteration < maxIter) {
        val temp = zx*zx - zy*zy + cx
        zy = 2.0*zx*zy + cy
        zx = temp
        iteration++
    }
    
    return iteration
}

// Calculate entire Mandelbrot set
fun calculateMandelbrot(): Array<IntArray> {
    val result = Array(HEIGHT) { IntArray(WIDTH) }
    
    for (y in 0 until HEIGHT) {
        for (x in 0 until WIDTH) {
            val cx = (x.toDouble() - WIDTH/2.0) * 4.0 / WIDTH
            val cy = (y.toDouble() - HEIGHT/2.0) * 4.0 / HEIGHT
            result[y][x] = mandelbrotPoint(cx, cy, MAX_ITERATIONS)
        }
    }
    
    return result
}

// Count non-black pixels
fun countNonBlack(result: Array<IntArray>): Int {
    var count = 0
    
    for (y in 0 until HEIGHT) {
        for (x in 0 until WIDTH) {
            if (result[y][x] < MAX_ITERATIONS) {
                count++
            }
        }
    }
    
    return count
}

fun main() {
    val totalTime = measureTimeMillis {
        val result = calculateMandelbrot()
        val count = countNonBlack(result)
        
        println("Mandelbrot set calculation finished.")
        println("Image size: $WIDTH x $HEIGHT")
        println("Maximum iterations: $MAX_ITERATIONS")
        println("Non-black pixels: $count")
    }
    
    println("Time taken: $totalTime ms")
}