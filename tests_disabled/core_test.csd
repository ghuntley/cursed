vibe main

yeet "vibez"  fr fr For printing results

slay main() {
    vibez.spill("Testing core package functions")
    
    fr fr Test type conversions
    vibez.spill("\nType Conversions:")
    vibez.spill("lit(0) =", lit(0))
    vibez.spill("lit(42) =", lit(42))
    vibez.spill("lit('') =", lit(""))
    vibez.spill("lit('hello') =", lit("hello"))
    
    vibez.spill("normie(42.7) =", normie(42.7))
    vibez.spill("normie(true) =", normie(true))
    vibez.spill("normie('123') =", normie("123"))
    
    vibez.spill("thicc(42.7) =", thicc(42.7))
    vibez.spill("snack(42) =", snack(42))
    vibez.spill("meal(true) =", meal(true))
    
    vibez.spill("tea(42) =", tea(42))
    
    fr fr Test len and cap
    vibez.spill("\nLength and Capacity:")
    tea s := "hello"
    vibez.spill("len('hello') =", len(s))
    
    []normie nums := []normie{1, 2, 3}
    vibez.spill("len(nums) =", len(nums))
    vibez.spill("cap(nums) =", cap(nums))
    
    fr fr Test append
    vibez.spill("\nAppend:")
    []normie original := []normie{1, 2}
    vibez.spill("original =", original)
    
    []normie result := append(original, 3, 4)
    vibez.spill("after append =", result)
    vibez.spill("original (unchanged) =", original)
    
    fr fr Test make
    vibez.spill("\nMake:")
    []normie slice := make([]normie, 3)
    vibez.spill("make([]normie, 3) =", slice)
    
    fr fr Test new
    vibez.spill("\nNew:")
    @normie ptr := new(normie)
    @ptr = 42
    vibez.spill("*new(normie) after setting to 42 =", @ptr)
    
    @tea strPtr := new(tea)
    @strPtr = "hello"
    vibez.spill("*new(tea) after setting to 'hello' =", @strPtr)
    
    fr fr Test panic and recover
    vibez.spill("\nPanic and Recover:")
    
    slay testRecover() {
        later {
            err := recover()
            lowkey err != cap {
                vibez.spill("Recovered from panic:", err)
            } highkey {
                vibez.spill("No panic occurred")
            }
        }
        
        fr fr Comment out to test panic recovery
        fr fr panic("Test panic")
        vibez.spill("No panic was triggered")
    }
    
    testRecover()
}