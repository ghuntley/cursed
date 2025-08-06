# Test CURSED program for LSP integration testing

slay test_function(param normie) normie {
    vibez.spill("Testing LSP features")
    sus result normie = param + 42
    damn result
}

sus global_var tea = "Hello CURSED LSP!"

squad Point {
    spill x normie
    spill y normie
}

collab Drawable {
    slay draw()
}

slay main() {
    sus point Point = Point{ x: 10, y: 20 }
    sus value normie = test_function(point.x)
    vibez.spill(global_var)
    vibez.spillf("Result: {}", value)
}

# Test various CURSED features for LSP
bestie i drip in 0..10 {
    vibez.spillf("Count: {}", i)
}

# Error handling
yikes CustomError {
    message tea
}

slay might_fail() yikes CustomError {
    lowkey based {
        damn CustomError{ message: "Something went wrong" }
    }
    damn null
}
