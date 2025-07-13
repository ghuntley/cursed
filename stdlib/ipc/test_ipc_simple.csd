yeet "ipc"

vibez.spill("=== IPC Module Simple Test Suite ===")

# Test initialization
vibez.spill("Testing IPC initialization...")
sus init_result lit = ipc.init_ipc()
vibez.spill("Init result: " + core.tea(core.normie(init_result)))

# Test named pipes
vibez.spill("\nTesting named pipes...")
sus pipe_result lit = ipc.create_named_pipe("test_pipe", 1024)
vibez.spill("Pipe creation: " + core.tea(core.normie(pipe_result)))

sus write_result lit = ipc.write_to_pipe("test_pipe", "Hello IPC!")
vibez.spill("Write operation: " + core.tea(core.normie(write_result)))

sus read_data tea = ipc.read_from_pipe("test_pipe")
vibez.spill("Read data: " + read_data)

# Test message queues
vibez.spill("\nTesting message queues...")
sus queue_result lit = ipc.create_message_queue("test_queue", 10)
vibez.spill("Queue creation: " + core.tea(core.normie(queue_result)))

sus send_result lit = ipc.send_message("test_queue", "Test message", 5)
vibez.spill("Send message: " + core.tea(core.normie(send_result)))

sus received_msg map = ipc.receive_message("test_queue")
if received_msg.size() > 0 {
    vibez.spill("Received: " + received_msg.get("content"))
} else {
    vibez.spill("No message received")
}

# Test connectivity
vibez.spill("\nTesting connectivity...")
sus connectivity_result lit = ipc.test_ipc_connectivity()
vibez.spill("Connectivity test: " + core.tea(core.normie(connectivity_result)))

# Test module info
vibez.spill("\nModule info:")
sus module_info tea = ipc.get_module_info()
vibez.spill(module_info)

# Test cleanup
vibez.spill("\nTesting cleanup...")
sus cleanup_result lit = ipc.cleanup_ipc()
vibez.spill("Cleanup result: " + core.tea(core.normie(cleanup_result)))

vibez.spill("\n=== IPC Simple Test Suite Completed ===")
