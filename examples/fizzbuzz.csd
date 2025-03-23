fr fr FizzBuzz implementation in CURSED
fr fr Prints numbers from 1 to n, but:
fr fr - For multiples of 3, print "Fizz"
fr fr - For multiples of 5, print "Buzz"
fr fr - For multiples of both 3 and 5, print "FizzBuzz"

vibe main

yeet (
    "vibez"
    "vibe_life"
    "stringz"
)

slay fizzbuzz(n normie) {
    bestie i := 1; i <= n; i++ {
        lowkey i % 3 == 0 && i % 5 == 0 {
            vibez.spill("FizzBuzz")
        } highkey lowkey i % 3 == 0 {
            vibez.spill("Fizz")
        } highkey lowkey i % 5 == 0 {
            vibez.spill("Buzz")
        } highkey {
            vibez.spill(i)
        }
    }
}

slay main() {
    fr fr Default number of iterations
    sus n normie = 100
    
    fr fr Check for command-line argument
    lowkey len(vibe_life.Args) > 1 {
        sus arg tea = vibe_life.Args[1]
        sus parsed, err = stringz.Atoi(arg)
        
        lowkey err == cap {
            n = parsed
        } highkey {
            vibez.spill("Invalid input, using default n =", n)
        }
    }
    
    vibez.spillf("Running FizzBuzz up to %d:\n", n)
    
    fr fr Run the fizzbuzz algorithm
    fizzbuzz(n)
    
    vibez.spill("No cap, we vibin'!")
} 