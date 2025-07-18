yeet "oglogging"

vibez.spill("Testing oglogging module...")

# Test basic logging functionality
Spill("Basic log message test")
vibez.spill("✅ Basic logging works")

# Test log levels
Debug("Debug message")
Info("Info message")
Warn("Warning message")
Error("Error message")
vibez.spill("✅ All log levels work")

# Test constants
vibez.spill("DEBUG=" + DEBUG + " INFO=" + INFO + " WARN=" + WARN + " ERROR=" + ERROR + " FATAL=" + FATAL)
vibez.spill("✅ Constants work")

# Test edge cases
Spill("")
Debug("Test with special chars: 🚀")
vibez.spill("✅ Edge cases work")

vibez.spill("🎉 oglogging module tests passed!")
