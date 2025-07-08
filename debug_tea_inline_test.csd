# Debug Tea Module - Inline test (functions defined directly in file)

# Debug levels
sus DEBUG_LEVEL_NONE normie = 0
sus DEBUG_LEVEL_ERROR normie = 1
sus DEBUG_LEVEL_WARN normie = 2
sus DEBUG_LEVEL_INFO normie = 3
sus DEBUG_LEVEL_DEBUG normie = 4
sus DEBUG_LEVEL_TRACE normie = 5

# Global debug state
sus debug_enabled lit = based
sus debug_level normie = DEBUG_LEVEL_INFO

# Debug utility functions
slay enable_debug() {
    debug_enabled = based
}

slay disable_debug() {
    debug_enabled = cap
}

slay debug_info(message tea) {
    bestie debug_enabled {
        vibez.spill("[INFO] " + message)
    }
}

slay debug_print_summary() {
    bestie debug_enabled {
        vibez.spill("=== DEBUG SUMMARY ===")
        vibez.spill("Debug utilities module loaded successfully")
    }
}

# Test the functions
vibez.spill("Testing debug_tea functions inline")
enable_debug()
debug_info("This is a test message")
debug_print_summary()
vibez.spill("Debug tea inline test completed")
