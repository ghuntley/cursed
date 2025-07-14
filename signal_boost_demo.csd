yeet "signal_boost"

vibez.spill("🔥 Signal Boost Demo - Unix Signal Handling That Hits Different! 📡")
vibez.spill("")

# Demo 1: Signal Constants
vibez.spill("📊 Signal Constants:")
vibez.spill("SIGTERM = " + SIGTERM)
vibez.spill("SIGINT = " + SIGINT)
vibez.spill("SIGUSR1 = " + SIGUSR1)
vibez.spill("SIGRTMIN = " + SIGRTMIN)
vibez.spill("")

# Demo 2: Signal Names
vibez.spill("📖 Signal Name Resolution:")
vibez.spill("Signal 15 = " + signal_get_name(15))
vibez.spill("Signal 2 = " + signal_get_name(2))
vibez.spill("Signal 34 = " + signal_get_name(34))
vibez.spill("")

# Demo 3: Signal Safety
vibez.spill("🛡️ Signal Safety Checks:")
lowkey signal_is_safe_handler(SIGTERM) {
    vibez.spill("✅ SIGTERM is safe to handle")
} else {
    vibez.spill("❌ SIGTERM is not safe to handle")
}

lowkey signal_is_safe_handler(SIGKILL) {
    vibez.spill("✅ SIGKILL is safe to handle")
} else {
    vibez.spill("❌ SIGKILL is not safe to handle")
}
vibez.spill("")

# Demo 4: Signal Masking
vibez.spill("🔒 Signal Masking Operations:")
sus mask SignalMask = signal_create_mask()
signal_mask_add(&mask, SIGTERM)
signal_mask_add(&mask, SIGINT)

lowkey signal_mask_contains(mask, SIGTERM) {
    vibez.spill("✅ SIGTERM is in the mask")
}

lowkey signal_mask_contains(mask, SIGUSR1) {
    vibez.spill("✅ SIGUSR1 is in the mask")
} else {
    vibez.spill("❌ SIGUSR1 is not in the mask")
}
vibez.spill("")

# Demo 5: Module Info
vibez.spill("💪 Module Information:")
vibez.spill(signal_boost_info())

vibez.spill("")
vibez.spill("🎉 Signal Boost Demo Complete - Ready for System Programming!")
