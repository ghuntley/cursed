// Mandelbrot set calculation benchmark for Swift

import Foundation

// Constants
let width = 800
let height = 800
let maxIterations = 100

// Calculate mandelbrot set for a single point
func mandelbrotPoint(cx: Double, cy: Double, maxIter: Int) -> Int {
    var zx = 0.0
    var zy = 0.0
    var iteration = 0
    
    while zx*zx + zy*zy <= 4.0 && iteration < maxIter {
        let temp = zx*zx - zy*zy + cx
        zy = 2.0*zx*zy + cy
        zx = temp
        iteration += 1
    }
    
    return iteration
}

// Calculate entire mandelbrot set
func calculateMandelbrot() -> [[Int]] {
    var result = [[Int]](repeating: [Int](repeating: 0, count: width), count: height)
    
    for y in 0..<height {
        for x in 0..<width {
            let cx = (Double(x) - Double(width)/2.0) * 4.0 / Double(width)
            let cy = (Double(y) - Double(height)/2.0) * 4.0 / Double(height)
            result[y][x] = mandelbrotPoint(cx: cx, cy: cy, maxIter: maxIterations)
        }
    }
    
    return result
}

// Count non-black pixels (points that didn't reach max iterations)
func countNonBlack(result: [[Int]]) -> Int {
    var count = 0
    
    for y in 0..<height {
        for x in 0..<width {
            if result[y][x] < maxIterations {
                count += 1
            }
        }
    }
    
    return count
}

func main() {
    let startTime = Date()
    
    let result = calculateMandelbrot()
    let count = countNonBlack(result: result)
    
    print("Mandelbrot set calculation finished.")
    print("Image size: \(width) x \(height)")
    print("Maximum iterations: \(maxIterations)")
    print("Non-black pixels: \(count)")
    
    let elapsedTime = -startTime.timeIntervalSinceNow * 1000
    print("Time taken: \(elapsedTime) ms")
}

main()