sus value drip = 150

ready (value) {
    1..50 => vibez.spill("Small")
    51..100 => vibez.spill("Medium")
    _ => vibez.spill("Large or unknown")
}
