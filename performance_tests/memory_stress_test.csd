squad DataPoint {
    spill x drip
    spill y drip
    spill value drip
}

slay memory_stress() {
    sus i drip = 0
    bestie (i < 10000) {
        sus point DataPoint = DataPoint{x: i, y: i * 2, value: i * i}
        # Simulate some work with the struct
        sus temp drip = point.x + point.y + point.value
        i = i + 1
    }
    vibez.spill("Memory stress completed")
}

memory_stress()
