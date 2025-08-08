stan { sus i drip = 0; bestie (i < 50) { vibez.spill("Thread 1:", i); i = i + 1 } }
stan { sus j drip = 0; bestie (j < 50) { vibez.spill("Thread 2:", j); j = j + 1 } }
stan { sus k drip = 0; bestie (k < 50) { vibez.spill("Thread 3:", k); k = k + 1 } }
stan { sus l drip = 0; bestie (l < 50) { vibez.spill("Thread 4:", l); l = l + 1 } }
stan { sus m drip = 0; bestie (m < 50) { vibez.spill("Thread 5:", m); m = m + 1 } }
vibez.spill("Main thread spawned 5 goroutines")
wait_all()
vibez.spill("All goroutines completed successfully")
