sus i drip = 1

bestie (i <= 100) {
    ready (i % 15 == 0) {
        vibez.spill("FizzBuzz")
    } otherwise ready (i % 3 == 0) {
        vibez.spill("Fizz")
    } otherwise ready (i % 5 == 0) {
        vibez.spill("Buzz")
    } otherwise {
        vibez.spill(i)
    }
    i = i + 1
}
