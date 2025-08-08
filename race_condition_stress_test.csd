stan { sus i drip = 0; bestie (i < 100) { vibez.spill("Goroutine A:", i); i = i + 1 } }
stan { sus j drip = 0; bestie (j < 100) { vibez.spill("Goroutine B:", j); j = j + 1 } }
stan { sus k drip = 0; bestie (k < 100) { vibez.spill("Goroutine C:", k); k = k + 1 } }
vibez.spill("Main thread done")
