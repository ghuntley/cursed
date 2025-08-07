fr fr Stack Trace Test for CURSED Error Handling

slay level_three() {
    vibez.spill("Level 3 function")
    yikes "Deep stack error"
}

slay level_two() {
    vibez.spill("Level 2 function") 
    level_three()
}

slay level_one() {
    vibez.spill("Level 1 function")
    level_two()
}

vibez.spill("Testing nested function stack trace...")
shook {
    level_one()
} fam stack_err {
    vibez.spill("Caught nested error with stack trace:")
    vibez.spill(stack_err)
}

vibez.spill("Stack trace test completed!")
