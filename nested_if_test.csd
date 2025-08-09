sus age drip = 25
sus score drip = 85

ready (age >= 18) {
    ready (score >= 80) {
        vibez.spill("Adult with high score")
    } otherwise {
        vibez.spill("Adult with low score")
    }
} otherwise {
    vibez.spill("Minor")
}
