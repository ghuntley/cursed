yikes "test error"

vibez.spill("Testing yikes statement")

shook { yikes "test error" } fam err { vibez.spill("Caught:", err) }

vibez.spill("After error handling")

shook { 
    vibez.spill("This will work fine") 
} fam err { 
    vibez.spill("No error should occur") 
}

vibez.spill("All error handling tests complete")
