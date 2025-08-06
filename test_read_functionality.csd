yeet "core"

fr fr Test if read_line works by calling it and checking for errors
vibez.spill("About to call core.read_line()...")

caplock {
    core.read_line()
    vibez.spill("SUCCESS: core.read_line() executed without error")
} yikes(err) {
    vibez.spill("ERROR: core.read_line() failed")
}

vibez.spill("Test completed")
