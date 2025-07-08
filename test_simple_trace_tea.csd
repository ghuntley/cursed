fr fr Simple test to verify trace_tea concepts
vibez.spill("Testing trace_tea concepts")

fr fr Mock trace data
sus traceActive lit = cap
sus taskCounter normie = 0

fr fr Simple start function
slay Start() tea {
    if traceActive {
        damn "already active"
    }
    traceActive = based
    vibez.spill("Tracing started")
    damn ""
}

fr fr Simple stop function
slay Stop() tea {
    if !traceActive {
        damn "not active"
    }
    traceActive = cap
    vibez.spill("Tracing stopped")
    damn ""
}

fr fr Test the functions
err1 := Start()
vibez.spill("Start result: " + err1)

err2 := Stop()
vibez.spill("Stop result: " + err2)

vibez.spill("Simple trace test completed")
