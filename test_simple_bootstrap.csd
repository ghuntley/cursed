yeet "bootstrap"

vibez.spill("Testing bootstrap allocator...")

// Initialize bootstrap
sus init_result lit = bootstrap_init()
if init_result {
    vibez.spill("Bootstrap initialization successful")
} else {
    vibez.spill("Bootstrap initialization failed")
}

// Test basic allocation
sus ptr *byte = cursed_malloc(64)
if ptr != cringe {
    vibez.spill("Basic allocation successful")
    cursed_free(ptr)
    vibez.spill("Basic deallocation successful")
} else {
    vibez.spill("Basic allocation failed")
}

vibez.spill("Bootstrap test completed")
