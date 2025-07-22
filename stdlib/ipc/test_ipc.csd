yeet "testz"
yeet "ipc"

fr fr Test IPC module functionality
test_start("IPC Module Comprehensive Tests")

fr fr ==============================================================================
fr fr INITIALIZATION TESTS
fr fr ==============================================================================

test_start("IPC Initialization")
assert_true(ipc.init_ipc())
print_test_summary()

test_start("IPC Configuration")
sus config map = {
    "max_message_size": 32768,
    "max_queue_size": 500,
    "timeout_ms": 3000
}
assert_true(ipc.configure_ipc(config))

sus retrieved_config map = ipc.get_ipc_config()
assert_eq_int(retrieved_config.get("max_message_size"), 32768)
assert_eq_int(retrieved_config.get("max_queue_size"), 500)
assert_eq_int(retrieved_config.get("timeout_ms"), 3000)
print_test_summary()

fr fr ==============================================================================
fr fr NAMED PIPES TESTS
fr fr ==============================================================================

test_start("Named Pipes Creation")
assert_true(ipc.create_named_pipe("test_pipe", 1024))
assert_false(ipc.create_named_pipe("test_pipe", 1024)) fr fr Should fail - already exists
print_test_summary()

test_start("Named Pipes Readers/Writers")
assert_true(ipc.open_pipe_reader("test_pipe", "reader1"))
assert_true(ipc.open_pipe_writer("test_pipe", "writer1"))
assert_false(ipc.open_pipe_reader("nonexistent_pipe", "reader1")) fr fr Should fail
print_test_summary()

test_start("Named Pipes Data Transfer")
assert_true(ipc.write_to_pipe("test_pipe", "Hello, World!"))
assert_true(ipc.write_to_pipe("test_pipe", "Second message"))

sus received1 tea = ipc.read_from_pipe("test_pipe")
assert_eq_string(received1, "Hello, World!")

sus received2 tea = ipc.read_from_pipe("test_pipe")
assert_eq_string(received2, "Second message")

sus empty tea = ipc.read_from_pipe("test_pipe")
assert_eq_string(empty, "") fr fr No more data
print_test_summary()

test_start("Named Pipes Buffer Overflow")
fr fr Create small pipe and fill it
assert_true(ipc.create_named_pipe("small_pipe", 2))
assert_true(ipc.write_to_pipe("small_pipe", "msg1"))
assert_true(ipc.write_to_pipe("small_pipe", "msg2"))
assert_false(ipc.write_to_pipe("small_pipe", "msg3")) fr fr Should fail - buffer full
print_test_summary()

fr fr ==============================================================================
fr fr MESSAGE QUEUES TESTS
fr fr ==============================================================================

test_start("Message Queue Creation")
assert_true(ipc.create_message_queue("test_queue", 10))
assert_false(ipc.create_message_queue("test_queue", 10)) fr fr Should fail - already exists
print_test_summary()

test_start("Message Queue Priority Handling")
assert_true(ipc.send_message("test_queue", "low_priority", ipc.MSG_PRIORITY_LOW))
assert_true(ipc.send_message("test_queue", "high_priority", ipc.MSG_PRIORITY_HIGH))
assert_true(ipc.send_message("test_queue", "normal_priority", ipc.MSG_PRIORITY_NORMAL))
assert_true(ipc.send_message("test_queue", "urgent_priority", ipc.MSG_PRIORITY_URGENT))

fr fr Messages should be received in priority order: urgent, high, normal, low
sus msg1 map = ipc.receive_message("test_queue")
assert_eq_string(msg1.get("content"), "urgent_priority")
assert_eq_int(msg1.get("priority"), ipc.MSG_PRIORITY_URGENT)

sus msg2 map = ipc.receive_message("test_queue")
assert_eq_string(msg2.get("content"), "high_priority")

sus msg3 map = ipc.receive_message("test_queue")
assert_eq_string(msg3.get("content"), "normal_priority")

sus msg4 map = ipc.receive_message("test_queue")
assert_eq_string(msg4.get("content"), "low_priority")
print_test_summary()

test_start("Message Queue Overflow")
fr fr Create small queue and fill it
assert_true(ipc.create_message_queue("small_queue", 2))
assert_true(ipc.send_message("small_queue", "msg1", ipc.MSG_PRIORITY_NORMAL))
assert_true(ipc.send_message("small_queue", "msg2", ipc.MSG_PRIORITY_NORMAL))
assert_false(ipc.send_message("small_queue", "msg3", ipc.MSG_PRIORITY_NORMAL)) fr fr Should fail
print_test_summary()

test_start("Message Queue Empty Read")
sus empty_msg map = ipc.receive_message("empty_queue_test")
assert_eq_int(empty_msg.size(), 0) fr fr Empty map for non-existent queue

fr fr Empty existing queue
ipc.receive_message("small_queue")
ipc.receive_message("small_queue")
sus empty_msg2 map = ipc.receive_message("small_queue")
assert_eq_int(empty_msg2.size(), 0) fr fr Empty map for empty queue
print_test_summary()

fr fr ==============================================================================
fr fr SHARED MEMORY TESTS
fr fr ==============================================================================

test_start("Shared Memory Creation")
assert_true(ipc.create_shared_memory("test_shm", 1024))
assert_false(ipc.create_shared_memory("test_shm", 1024)) fr fr Should fail - already exists
print_test_summary()

test_start("Shared Memory Process Attachment")
assert_true(ipc.attach_shared_memory("test_shm", "process1"))
assert_true(ipc.attach_shared_memory("test_shm", "process2"))
assert_false(ipc.attach_shared_memory("nonexistent_shm", "process1")) fr fr Should fail
print_test_summary()

test_start("Shared Memory Data Operations")
assert_true(ipc.write_shared_memory("test_shm", "key1", "value1"))
assert_true(ipc.write_shared_memory("test_shm", "key2", "value2"))

sus value1 tea = ipc.read_shared_memory("test_shm", "key1")
assert_eq_string(value1, "value1")

sus value2 tea = ipc.read_shared_memory("test_shm", "key2")
assert_eq_string(value2, "value2")

fr fr Test overwriting
assert_true(ipc.write_shared_memory("test_shm", "key1", "new_value"))
sus new_value tea = ipc.read_shared_memory("test_shm", "key1")
assert_eq_string(new_value, "new_value")
print_test_summary()

test_start("Shared Memory Size Limits")
assert_false(ipc.create_shared_memory("huge_shm", 2097152)) fr fr Should fail - too large (2MB > 1MB limit)
print_test_summary()

fr fr ==============================================================================
fr fr SEMAPHORE TESTS
fr fr ==============================================================================

test_start("Semaphore Creation")
assert_true(ipc.create_semaphore("test_sem", 3))
assert_false(ipc.create_semaphore("test_sem", 3)) fr fr Should fail - already exists
print_test_summary()

test_start("Semaphore Wait/Signal Operations")
fr fr Test successful waits
assert_true(ipc.semaphore_wait("test_sem", "process1")) fr fr 3 -> 2
assert_true(ipc.semaphore_wait("test_sem", "process2")) fr fr 2 -> 1
assert_true(ipc.semaphore_wait("test_sem", "process3")) fr fr 1 -> 0

fr fr This should fail (semaphore at 0)
assert_false(ipc.semaphore_wait("test_sem", "process4"))

fr fr Signal to release
assert_true(ipc.semaphore_signal("test_sem")) fr fr 0 -> 1

fr fr Now wait should succeed
assert_true(ipc.semaphore_wait("test_sem", "process5")) fr fr 1 -> 0
print_test_summary()

test_start("Semaphore Value Limits")
assert_false(ipc.create_semaphore("huge_sem", 65536)) fr fr Should fail - too large
print_test_summary()

fr fr ==============================================================================
fr fr UNIX SOCKET TESTS
fr fr ==============================================================================

test_start("Unix Socket Creation")
assert_true(ipc.create_unix_socket("test_socket", "stream"))
assert_false(ipc.create_unix_socket("test_socket", "stream")) fr fr Should fail - already exists
print_test_summary()

test_start("Unix Socket Listen/Connect")
assert_true(ipc.listen_unix_socket("test_socket", "server_process"))
assert_true(ipc.connect_unix_socket("test_socket", "client1"))
assert_true(ipc.connect_unix_socket("test_socket", "client2"))

fr fr Try connecting to non-listening socket
assert_true(ipc.create_unix_socket("inactive_socket", "stream"))
assert_false(ipc.connect_unix_socket("inactive_socket", "client3")) fr fr Should fail
print_test_summary()

fr fr ==============================================================================
fr fr PROCESS MANAGEMENT TESTS
fr fr ==============================================================================

test_start("Process Registration")
assert_true(ipc.register_process("proc1", "Test Process 1"))
assert_true(ipc.register_process("proc2", "Test Process 2"))

sus proc_info map = ipc.get_process_info("proc1")
assert_eq_string(proc_info.get("name"), "Test Process 1")
assert_eq_string(proc_info.get("id"), "proc1")
assert_true(proc_info.get("active"))
print_test_summary()

test_start("Process Unregistration")
assert_true(ipc.unregister_process("proc1"))
assert_false(ipc.unregister_process("nonexistent_proc"))

sus updated_info map = ipc.get_process_info("proc1")
assert_false(updated_info.get("active"))

sus active_processes [tea] = ipc.list_active_processes()
assert_eq_int(active_processes.length(), 1) fr fr Only proc2 should be active
print_test_summary()

fr fr ==============================================================================
fr fr UTILITY AND STATISTICS TESTS
fr fr ==============================================================================

test_start("IPC Resource Listing")
sus resources [tea] = ipc.list_ipc_resources()
assert_true(resources.length() > 0) fr fr Should have some resources from previous tests

sus pipe_info map = ipc.get_ipc_resource_info("pipe_test_pipe")
assert_eq_string(pipe_info.get("name"), "test_pipe")
assert_eq_string(pipe_info.get("type"), ipc.IPC_TYPE_PIPE)
print_test_summary()

test_start("IPC Statistics")
sus stats map = ipc.get_ipc_statistics()
assert_true(stats.get("pipes_created") > 0)
assert_true(stats.get("queues_created") > 0)
assert_true(stats.get("shared_segments_created") > 0)
assert_true(stats.get("semaphores_created") > 0)
assert_true(stats.get("sockets_created") > 0)
assert_true(stats.get("messages_sent") > 0)
assert_true(stats.get("messages_received") > 0)
print_test_summary()

fr fr ==============================================================================
fr fr CONNECTIVITY TESTS
fr fr ==============================================================================

test_start("IPC Connectivity Test")
assert_true(ipc.test_ipc_connectivity())
print_test_summary()

fr fr ==============================================================================
fr fr ERROR HANDLING TESTS
fr fr ==============================================================================

test_start("Error Handling - Invalid Operations")
fr fr Test operations on non-existent resources
assert_false(ipc.write_to_pipe("nonexistent_pipe", "data"))
assert_eq_string(ipc.read_from_pipe("nonexistent_pipe"), "")
assert_false(ipc.send_message("nonexistent_queue", "msg", ipc.MSG_PRIORITY_NORMAL))

sus empty_msg map = ipc.receive_message("nonexistent_queue")
assert_eq_int(empty_msg.size(), 0)

assert_false(ipc.write_shared_memory("nonexistent_shm", "key", "value"))
assert_eq_string(ipc.read_shared_memory("nonexistent_shm", "key"), "")
assert_false(ipc.semaphore_wait("nonexistent_sem", "process"))
assert_false(ipc.semaphore_signal("nonexistent_sem"))
print_test_summary()

fr fr ==============================================================================
fr fr CLEANUP TESTS
fr fr ==============================================================================

test_start("Resource Cleanup")
assert_true(ipc.cleanup_resource_type("pipe"))
assert_true(ipc.cleanup_resource_type("queue"))
assert_true(ipc.cleanup_resource_type("shm"))
assert_true(ipc.cleanup_resource_type("sem"))
assert_true(ipc.cleanup_resource_type("socket"))
print_test_summary()

test_start("Full IPC Cleanup")
assert_true(ipc.cleanup_ipc())

fr fr Verify resources are cleaned up
sus resources_after [tea] = ipc.list_ipc_resources()
assert_eq_int(resources_after.length(), 0)
print_test_summary()

fr fr ==============================================================================
fr fr RESET AND REINITIALIZE TESTS
fr fr ==============================================================================

test_start("IPC System Reset")
ipc.reset_ipc()

fr fr Test that system works after reset
assert_true(ipc.create_named_pipe("post_reset_pipe", 512))
assert_true(ipc.write_to_pipe("post_reset_pipe", "reset_test"))
sus reset_msg tea = ipc.read_from_pipe("post_reset_pipe")
assert_eq_string(reset_msg, "reset_test")
print_test_summary()

fr fr ==============================================================================
fr fr MODULE INFO TESTS
fr fr ==============================================================================

test_start("Module Information")
sus module_info tea = ipc.get_module_info()
assert_true(module_info.contains("ipc"))
assert_true(module_info.contains("CURSED"))
print_test_summary()

fr fr ==============================================================================
fr fr INTEGRATION TESTS
fr fr ==============================================================================

test_start("Multi-Resource Integration Test")
fr fr Create resources of each type and test interaction
assert_true(ipc.create_named_pipe("integration_pipe", 1024))
assert_true(ipc.create_message_queue("integration_queue", 20))
assert_true(ipc.create_shared_memory("integration_shm", 2048))
assert_true(ipc.create_semaphore("integration_sem", 5))
assert_true(ipc.create_unix_socket("integration_socket", "stream"))

fr fr Register processes
assert_true(ipc.register_process("integrator1", "Integration Process 1"))
assert_true(ipc.register_process("integrator2", "Integration Process 2"))

fr fr Test coordination between resources
assert_true(ipc.semaphore_wait("integration_sem", "integrator1"))
assert_true(ipc.write_shared_memory("integration_shm", "status", "processing"))
assert_true(ipc.send_message("integration_queue", "task_started", ipc.MSG_PRIORITY_HIGH))
assert_true(ipc.write_to_pipe("integration_pipe", "log: task initiated"))

fr fr Verify state
sus status tea = ipc.read_shared_memory("integration_shm", "status")
assert_eq_string(status, "processing")

sus task_msg map = ipc.receive_message("integration_queue")
assert_eq_string(task_msg.get("content"), "task_started")

sus log_msg tea = ipc.read_from_pipe("integration_pipe")
assert_eq_string(log_msg, "log: task initiated")

fr fr Complete task
assert_true(ipc.semaphore_signal("integration_sem"))
assert_true(ipc.write_shared_memory("integration_shm", "status", "completed"))
print_test_summary()

fr fr ==============================================================================
fr fr PERFORMANCE TESTS
fr fr ==============================================================================

test_start("High Volume Message Processing")
assert_true(ipc.create_message_queue("perf_queue", 1000))

fr fr Send many messages
sus i normie = 0
while i < 100 {
    sus msg_content tea = "performance_test_" + core.tea(i)
    assert_true(ipc.send_message("perf_queue", msg_content, ipc.MSG_PRIORITY_NORMAL))
    i = i + 1
}

fr fr Receive all messages
sus received_count normie = 0
while received_count < 100 {
    sus perf_msg map = ipc.receive_message("perf_queue")
    if perf_msg.size() > 0 {
        received_count = received_count + 1
    } else {
        break fr fr No more messages
    }
}

assert_eq_int(received_count, 100)
print_test_summary()

fr fr ==============================================================================
fr fr DEBUGGING TESTS
fr fr ==============================================================================

test_start("Debug State Dump")
fr fr This test just verifies the dump function doesn't crash
ipc.dump_ipc_state()
assert_true(based) fr fr If we get here, dump_ipc_state() succeeded
print_test_summary()

fr fr ==============================================================================
fr fr FINAL CLEANUP
fr fr ==============================================================================

test_start("Final Cleanup")
assert_true(ipc.cleanup_ipc())
print_test_summary()

vibez.spill("=== IPC Module Test Suite Completed ===")
print_test_summary()
