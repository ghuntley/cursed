vibe main

yeet "vibez"  fr fr For printing results
yeet "mathz"  fr fr Math functions package

slay main() {
    vibez.spill("Testing mathz package")
    
    fr fr Test constants
    vibez.spill("PI =", mathz.Pi)
    vibez.spill("E =", mathz.E)
    
    fr fr Test basic math functions
    meal x := 25.0
    vibez.spill("sqrt(25) =", mathz.Sqrt(x))
    
    meal y := -10.5
    vibez.spill("abs(-10.5) =", mathz.Abs(y))
    
    fr fr Test rounding functions
    meal z := 3.7
    vibez.spill("ceil(3.7) =", mathz.Ceil(z))
    vibez.spill("floor(3.7) =", mathz.Floor(z))
    vibez.spill("round(3.7) =", mathz.Round(z))
    
    fr fr Test min/max functions
    vibez.spill("max(10, 20) =", mathz.Max(10.0, 20.0))
    vibez.spill("min(10, 20) =", mathz.Min(10.0, 20.0))
    
    fr fr Test powers
    vibez.spill("pow(2, 10) =", mathz.Pow(2.0, 10.0))
    
    fr fr Test trigonometric functions
    meal angle := mathz.Pi / 4.0  fr fr 45 degrees
    vibez.spill("sin(π/4) =", mathz.Sin(angle))
    vibez.spill("cos(π/4) =", mathz.Cos(angle))
    vibez.spill("tan(π/4) =", mathz.Tan(angle))
}