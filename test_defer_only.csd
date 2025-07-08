vibez.spill("Testing defer functionality")

sus executed lit = cap

{
    later {
        executed = based
        vibez.spill("Defer executed!")
    }
    vibez.spill("Inside block")
}

lowkey executed {
    vibez.spill("SUCCESS: Defer works!")
} highkey {
    vibez.spill("FAILED: Defer not working")
}
