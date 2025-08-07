fr fr Advanced Error Handling Test for CURSED

vibez.spill("Testing basic yikes error creation...")
yikes "This is a test error message"

vibez.spill("Testing shook/fam error handling...")
shook {
    vibez.spill("Inside shook block - about to create error")
    yikes "Error in shook block"
    vibez.spill("This should not execute")
} fam err {
    vibez.spill("Caught error in fam block!")
    vibez.spill("Error details:", err)
}

vibez.spill("Testing function with error handling...")
slay risky_function() {
    vibez.spill("Starting risky operation")
    yikes "Function error occurred"
    damn "never reached"
}

shook {
    sus result drip = risky_function()
    vibez.spill("Function completed successfully:", result)
} fam function_err {
    vibez.spill("Function threw error:", function_err)
}

vibez.spill("Error handling tests completed!")
