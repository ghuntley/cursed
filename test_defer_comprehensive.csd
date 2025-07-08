slay multipleDefers() {
    vibez.spill("Start of function")
    
    later vibez.spill("First defer")
    later vibez.spill("Second defer")
    later vibez.spill("Third defer")
    
    vibez.spill("Middle of function")
    
    later vibez.spill("Fourth defer")
    
    vibez.spill("End of function")
}

multipleDefers()
