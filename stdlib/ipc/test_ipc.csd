yeet "testz"
yeet "ipc"

# Comprehensive IPC Module Tests
# Tests all inter-process communication functionality

# ===== IPC Channel Tests =====

test_start("IPC Channel Creation")
sus channel := ipc_create_channel("test_channel", 1024)
sus (name, buffer_size, is_open) := channel
assert_eq_string(name, "test_channel")
assert_eq_int(buffer_size, 1024)
assert_true(is_open)

test_start("IPC Message Sending")
sus channel := ipc_create_channel("msg_channel", 2048)
sus send_result := ipc_send_message(channel, 1, "Hello IPC", 1)
assert_true(send_result)

test_start("IPC Message Receiving")
sus channel := ipc_create_channel("recv_channel", 2048)
sus message := ipc_receive_message(channel)
sus (msg_id, content, priority, timestamp) := message
assert_eq_int(msg_id, 1)
assert_eq_string(content, "Hello from IPC")
assert_eq_int(priority, 1)

test_start("IPC Channel Closing")
sus channel := ipc_create_channel("close_channel", 1024)
sus closed_channel := ipc_close_channel(channel)
sus (name, buffer_size, is_open) := closed_channel
assert_eq_string(name, "close_channel")
assert_false(is_open)

# ===== Process Coordination Tests =====

test_start("Process Registration")
sus process := ipc_register_process(1234, "test_process")
sus (pid, name, status, created_at) := process
assert_eq_int(pid, 1234)
assert_eq_string(name, "test_process")
assert_eq_string(status, "active")

test_start("Process Alive Check")
sus process := ipc_register_process(5678, "alive_process")
sus is_alive := ipc_process_alive(process)
assert_true(is_alive)

test_start("Process Signal Terminate")
sus process := ipc_register_process(9999, "signal_process")
sus signal_result := ipc_signal_process(process, "terminate")
assert_true(signal_result)

test_start("Process Signal Suspend")
sus process := ipc_register_process(8888, "suspend_process")
sus signal_result := ipc_signal_process(process, "suspend")
assert_true(signal_result)

test_start("Process Signal Resume")
sus process := ipc_register_process(7777, "resume_process")
sus signal_result := ipc_signal_process(process, "resume")
assert_true(signal_result)

test_start("Process Unknown Signal")
sus process := ipc_register_process(6666, "unknown_signal_process")
sus signal_result := ipc_signal_process(process, "unknown_signal")
assert_false(signal_result)

# ===== Shared Memory Tests =====

test_start("Shared Memory Creation")
sus memory := ipc_create_shared_memory("test_memory", 4096)
sus (name, size, permissions) := memory
assert_eq_string(name, "test_memory")
assert_eq_int(size, 4096)
assert_eq_int(permissions, 666)

test_start("Shared Memory Write")
sus memory := ipc_create_shared_memory("write_memory", 2048)
sus write_result := ipc_write_shared_memory(memory, 0, "test_data")
assert_true(write_result)

test_start("Shared Memory Write Out of Bounds")
sus memory := ipc_create_shared_memory("bounds_memory", 1024)
sus write_result := ipc_write_shared_memory(memory, 2048, "test_data")
assert_false(write_result)

test_start("Shared Memory Read")
sus memory := ipc_create_shared_memory("read_memory", 2048)
sus data := ipc_read_shared_memory(memory, 0, 100)
assert_eq_string(data, "simulated_data_from_shared_memory")

test_start("Shared Memory Read Out of Bounds")
sus memory := ipc_create_shared_memory("bounds_read_memory", 1024)
sus data := ipc_read_shared_memory(memory, 1024, 100)
assert_eq_string(data, "")

# ===== Message Queue Tests =====

test_start("Message Queue Creation")
sus queue := ipc_create_message_queue("test_queue", 10)
sus (name, max_messages, current_count) := queue
assert_eq_string(name, "test_queue")
assert_eq_int(max_messages, 10)
assert_eq_int(current_count, 0)

test_start("Message Queue Push")
sus queue := ipc_create_message_queue("push_queue", 5)
sus message := IpcMessage(1, "queue_message", 1, 1640995200)
sus updated_queue := ipc_queue_push(queue, message)
sus (name, max_messages, current_count) := updated_queue
assert_eq_int(current_count, 1)

test_start("Message Queue Pop")
sus queue := ipc_create_message_queue("pop_queue", 5)
sus message := IpcMessage(1, "test_message", 1, 1640995200)
sus queue_with_message := ipc_queue_push(queue, message)
sus (popped_message, updated_queue) := ipc_queue_pop(queue_with_message)
sus (msg_id, content, priority, timestamp) := popped_message
assert_eq_int(msg_id, 1)
assert_eq_string(content, "queued_message")

test_start("Message Queue Pop Empty")
sus empty_queue := ipc_create_message_queue("empty_queue", 5)
sus (popped_message, unchanged_queue) := ipc_queue_pop(empty_queue)
sus (msg_id, content, priority, timestamp) := popped_message
assert_eq_int(msg_id, 0)
assert_eq_string(content, "")

# ===== Semaphore Tests =====

test_start("Semaphore Creation")
sus semaphore := ipc_create_semaphore("test_semaphore", 3)
sus (name, value, waiting_count) := semaphore
assert_eq_string(name, "test_semaphore")
assert_eq_int(value, 3)
assert_eq_int(waiting_count, 0)

test_start("Semaphore Acquire Available")
sus semaphore := ipc_create_semaphore("acquire_semaphore", 2)
sus updated_semaphore := ipc_semaphore_acquire(semaphore)
sus (name, value, waiting_count) := updated_semaphore
assert_eq_int(value, 1)
assert_eq_int(waiting_count, 0)

test_start("Semaphore Acquire Unavailable")
sus semaphore := ipc_create_semaphore("unavailable_semaphore", 0)
sus updated_semaphore := ipc_semaphore_acquire(semaphore)
sus (name, value, waiting_count) := updated_semaphore
assert_eq_int(value, 0)
assert_eq_int(waiting_count, 1)

test_start("Semaphore Release")
sus semaphore := ipc_create_semaphore("release_semaphore", 1)
sus updated_semaphore := ipc_semaphore_release(semaphore)
sus (name, value, waiting_count) := updated_semaphore
assert_eq_int(value, 2)

test_start("Semaphore Release with Waiting")
sus semaphore := (name, 0, 2)  # Create semaphore with waiting processes
sus updated_semaphore := ipc_semaphore_release(semaphore)
sus (name, value, waiting_count) := updated_semaphore
assert_eq_int(waiting_count, 1)

# ===== Named Pipe Tests =====

test_start("Named Pipe Creation")
sus pipe := ipc_create_named_pipe("/tmp/test_pipe", 644)
sus (path, permissions, is_open) := pipe
assert_eq_string(path, "/tmp/test_pipe")
assert_eq_int(permissions, 644)
assert_true(is_open)

test_start("Named Pipe Open for Read")
sus pipe := ipc_create_named_pipe("/tmp/read_pipe", 644)
sus opened_pipe := ipc_open_pipe_read(pipe)
sus (path, permissions, is_open) := opened_pipe
assert_true(is_open)

test_start("Named Pipe Write")
sus pipe := ipc_create_named_pipe("/tmp/write_pipe", 644)
sus write_result := ipc_pipe_write(pipe, "pipe_data")
assert_true(write_result)

test_start("Named Pipe Read")
sus pipe := ipc_create_named_pipe("/tmp/read_pipe", 644)
sus data := ipc_pipe_read(pipe)
assert_eq_string(data, "data_from_named_pipe")

test_start("Named Pipe Write to Closed")
sus closed_pipe := ("/tmp/closed_pipe", 644, cap)
sus write_result := ipc_pipe_write(closed_pipe, "test_data")
assert_false(write_result)

test_start("Named Pipe Read from Closed")
sus closed_pipe := ("/tmp/closed_pipe", 644, cap)
sus data := ipc_pipe_read(closed_pipe)
assert_eq_string(data, "")

# ===== IPC Utility Tests =====

test_start("IPC Statistics")
sus stats := ipc_get_stats()
sus (active_channels, active_processes, shared_memory_segments) := stats
assert_eq_int(active_channels, 5)
assert_eq_int(active_processes, 3)
assert_eq_int(shared_memory_segments, 2)

test_start("IPC Cleanup")
sus cleanup_result := ipc_cleanup()
assert_true(cleanup_result)

test_start("IPC Health Check")
sus health_result := ipc_health_check()
assert_true(health_result)

# ===== Complex IPC Scenarios =====

test_start("IPC Complex Channel Communication")
sus channel := ipc_create_channel("complex_channel", 4096)
sus send_result1 := ipc_send_message(channel, 1, "Message 1", 1)
sus send_result2 := ipc_send_message(channel, 2, "Message 2", 2)
assert_true(send_result1)
assert_true(send_result2)

sus received_msg := ipc_receive_message(channel)
sus (msg_id, content, priority, timestamp) := received_msg
assert_eq_int(msg_id, 1)

test_start("IPC Process Coordination Flow")
sus process1 := ipc_register_process(1001, "worker1")
sus process2 := ipc_register_process(1002, "worker2")

sus alive1 := ipc_process_alive(process1)
sus alive2 := ipc_process_alive(process2)
assert_true(alive1)
assert_true(alive2)

sus signal_result := ipc_signal_process(process1, "suspend")
assert_true(signal_result)

test_start("IPC Shared Memory Operations Flow")
sus memory := ipc_create_shared_memory("flow_memory", 8192)
sus write1 := ipc_write_shared_memory(memory, 0, "start_data")
sus write2 := ipc_write_shared_memory(memory, 100, "middle_data")
sus write3 := ipc_write_shared_memory(memory, 200, "end_data")

assert_true(write1)
assert_true(write2)
assert_true(write3)

sus read_data := ipc_read_shared_memory(memory, 0, 50)
assert_eq_string(read_data, "simulated_data_from_shared_memory")

test_start("IPC Message Queue Full Flow")
sus queue := ipc_create_message_queue("full_flow_queue", 3)
sus msg1 := IpcMessage(1, "msg1", 1, 1640995200)
sus msg2 := IpcMessage(2, "msg2", 2, 1640995201)
sus msg3 := IpcMessage(3, "msg3", 3, 1640995202)

sus queue1 := ipc_queue_push(queue, msg1)
sus queue2 := ipc_queue_push(queue1, msg2)
sus queue3 := ipc_queue_push(queue2, msg3)

sus (name, max_messages, current_count) := queue3
assert_eq_int(current_count, 3)

# Try to push to full queue
sus msg4 := IpcMessage(4, "msg4", 4, 1640995203)
sus queue4 := ipc_queue_push(queue3, msg4)
sus (name_full, max_full, count_full) := queue4
assert_eq_int(count_full, 3)  # Should remain at 3

test_start("IPC Semaphore Coordination")
sus semaphore := ipc_create_semaphore("coordination_semaphore", 1)

# Simulate process 1 acquiring
sus sem1 := ipc_semaphore_acquire(semaphore)
sus (name1, value1, waiting1) := sem1
assert_eq_int(value1, 0)

# Simulate process 2 trying to acquire (should wait)
sus sem2 := ipc_semaphore_acquire(sem1)
sus (name2, value2, waiting2) := sem2
assert_eq_int(value2, 0)
assert_eq_int(waiting2, 1)

# Process 1 releases
sus sem3 := ipc_semaphore_release(sem2)
sus (name3, value3, waiting3) := sem3
assert_eq_int(waiting3, 0)  # Process 2 should be woken up

print_test_summary()
