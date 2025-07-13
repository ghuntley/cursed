vibez.spill("Testing IPC module with signal_boost...")

yeet "signal_boost"
vibez.spill("signal_boost imported")

yeet "ipc"
vibez.spill("IPC module imported successfully")

# Test initialization
sus result lit = ipc.init_ipc()
vibez.spill("IPC initialization completed")

# Test basic functionality
ipc.create_named_pipe("test", 1024)
vibez.spill("Created named pipe")

ipc.write_to_pipe("test", "hello")
vibez.spill("Wrote to pipe")

sus data tea = ipc.read_from_pipe("test")
vibez.spill("Read from pipe: " + data)

vibez.spill("IPC test completed successfully")
