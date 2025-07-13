vibez.spill("Testing basic functionality...")

# Test just core functionality first
sus test_config map = {
    "default_timeout_ms": 30000,
    "max_concurrent_processes": 100
}
vibez.spill("✅ Config created")

sus test_stats map = {
    "commands_executed": 0,
    "processes_created": 0
}
vibez.spill("✅ Stats created")

vibez.spill("Basic test completed successfully!")
