vibez.spill("Testing shook/fam error handling...")
shook {
    vibez.spill("Inside shook block")
    yikes "Error in shook block"
    vibez.spill("This should not execute")
} fam err {
    vibez.spill("Caught error:", err)
}
vibez.spill("Program completed!")
