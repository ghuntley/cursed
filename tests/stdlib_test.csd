vibe main

yeet "vibez"
yeet "stringz"
yeet "mathz"

slay main() {
    vibez.spill("Standard Library Test")
    
    fr fr Test stringz package
    tea s := "hello, world!"
    
    lowkey stringz.Contains(s, "world") {
        vibez.spill("Found 'world' in string")
    }
    
    tea upper := stringz.ToUpper(s)
    vibez.spill("Uppercase:", upper)
    
    tea parts := stringz.Split(s, ", ")
    vibez.spill("Split result:", parts[0], parts[1])
    
    fr fr Test mathz package
    meal x := mathz.Sqrt(25.0)
    vibez.spill("Square root of 25:", x)
    
    meal y := mathz.Pow(2.0, 10.0)
    vibez.spill("2^10 =", y)
    
    vibez.spill("Pi =", mathz.Pi)
}