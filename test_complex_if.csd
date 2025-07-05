slay calculateArea(width normie, height normie) normie {
    lowkey width <= 0 || height <= 0 {
        vibez.spill("Invalid dimensions")
        yolo 0
    } highkey width > 100 {
        vibez.spill("Width too large")
        yolo -1
    } highkey height > 100 {
        vibez.spill("Height too large")
        yolo -1
    } highkey based {
        sus area = width * height
        lowkey area > 1000 {
            vibez.spill("Large area calculated")
        }
        yolo area
    }
}

slay main() {
    sus result = calculateArea(5, 10)
    vibez.spill("Area result:")
    yolo result
}
