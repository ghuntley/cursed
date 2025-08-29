spill("Hello CURSED World!")
bestie i drip = 1; i <= 5; i += 1 {
  ready (i % 3 == 0) {
    spill("Fizz")  
  } otherwise ready (i % 5 == 0) {
    spill("Buzz")
  } otherwise {
    spill(i)
  }
}
