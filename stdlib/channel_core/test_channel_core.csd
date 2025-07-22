yeet "testz"
yeet "channel_core"
yeet "goroutine_core"

fr fr Test Channel Core System
test_start("Channel Core System Tests")

fr fr Initialize dependencies
init_goroutine_scheduler()

fr fr Test 1: Channel system initialization
sus init_result lit = init_channel_system()
assert_true(init_result)
print_test_status("Channel system initialization", init_result)

fr fr Test 2: Unbuffered channel creation
sus unbuffered_id normie = make_channel(0, "tea")
assert_true(unbuffered_id > 0)
assert_true(channel_exists(unbuffered_id))

sus unbuffered_info Channel = get_channel_info(unbuffered_id)
assert_eq_int(unbuffered_info.channel_type, CHANNEL_UNBUFFERED)
assert_eq_int(unbuffered_info.buffer_size, 0)
print_test_status("Unbuffered channel creation", unbuffered_id > 0)

fr fr Test 3: Buffered channel creation
sus buffered_id normie = make_channel(5, "normie")
assert_true(buffered_id > 0)
assert_true(channel_exists(buffered_id))

sus buffered_info Channel = get_channel_info(buffered_id)
assert_eq_int(buffered_info.channel_type, CHANNEL_BUFFERED)
assert_eq_int(buffered_info.buffer_size, 5)
print_test_status("Buffered channel creation", buffered_id > 0)

fr fr Test 4: Buffered channel send operations
sus send_result1 ChannelResult = channel_send(buffered_id, "value1")
assert_true(send_result1.success)
assert_true(!send_result1.would_block)

sus send_result2 ChannelResult = channel_send(buffered_id, "value2")
assert_true(send_result2.success)

sus send_result3 ChannelResult = channel_send(buffered_id, "value3")
assert_true(send_result3.success)
print_test_status("Buffered channel sends", send_result1.success && send_result2.success)

fr fr Test 5: Buffered channel receive operations
sus recv_result1 ChannelResult = channel_receive(buffered_id)
assert_true(recv_result1.success)
assert_eq_string(recv_result1.value, "value1")

sus recv_result2 ChannelResult = channel_receive(buffered_id)
assert_true(recv_result2.success)
assert_eq_string(recv_result2.value, "value2")
print_test_status("Buffered channel receives", recv_result1.success && recv_result2.success)

fr fr Test 6: Channel closure
sus close_result lit = close_channel(buffered_id)
assert_true(close_result)

sus closed_info Channel = get_channel_info(buffered_id)
assert_true(closed_info.is_closed)
assert_eq_int(closed_info.channel_type, CHANNEL_CLOSED)
print_test_status("Channel closure", close_result)

fr fr Test 7: Send to closed channel
sus send_to_closed ChannelResult = channel_send(buffered_id, "should_fail")
assert_true(!send_to_closed.success)
assert_true(send_to_closed.channel_closed)
print_test_status("Send to closed channel", send_to_closed.channel_closed)

fr fr Test 8: Receive from closed channel
sus recv_from_closed ChannelResult = channel_receive(buffered_id)
assert_true(!recv_from_closed.success)
assert_true(recv_from_closed.channel_closed)
print_test_status("Receive from closed channel", recv_from_closed.channel_closed)

fr fr Test 9: Buffer overflow protection
reset_channel_system()
init_channel_system()

sus small_buffer_id normie = make_channel(2, "tea")
assert_true(small_buffer_id > 0)

fr fr Fill buffer
sus fill1 ChannelResult = channel_send(small_buffer_id, "item1")
sus fill2 ChannelResult = channel_send(small_buffer_id, "item2")
assert_true(fill1.success && fill2.success)

fr fr Try to overfill
sus overflow ChannelResult = channel_send(small_buffer_id, "overflow")
assert_true(!overflow.success)
assert_true(overflow.would_block)
print_test_status("Buffer overflow protection", overflow.would_block)

fr fr Test 10: Channel statistics
sus stats_before map[tea]normie = get_channel_stats()
sus stats_channel_id normie = make_channel(3, "test")
sus stats_after map[tea]normie = get_channel_stats()

assert_true(stats_after["total_created"] > stats_before["total_created"])
assert_true(stats_after["active_channels"] > stats_before["active_channels"])
print_test_status("Channel statistics", stats_after["total_created"] > stats_before["total_created"])

fr fr Test 11: Multiple channel types
reset_channel_system()
init_channel_system()

sus type_unbuffered normie = make_channel(0, "tea")
sus type_buffered normie = make_channel(10, "normie")

sus multi_stats map[tea]normie = get_channel_stats()
assert_true(multi_stats["unbuffered_channels"] >= 1)
assert_true(multi_stats["buffered_channels"] >= 1)
assert_eq_int(multi_stats["closed_channels"], 0)
print_test_status("Multiple channel types", based)

fr fr Test 12: Health check
sus health_result lit = channel_health_check()
assert_true(health_result)
print_test_status("Channel health check", health_result)

fr fr Test 13: Invalid operations
sus invalid_send ChannelResult = channel_send(99999, "invalid")
assert_true(!invalid_send.success)

sus invalid_recv ChannelResult = channel_receive(99999)
assert_true(!invalid_recv.success)

sus invalid_close lit = close_channel(99999)
assert_true(!invalid_close)

sus invalid_info Channel = get_channel_info(99999)
assert_eq_int(invalid_info.id, 0)
print_test_status("Invalid operations", based)

fr fr Test 14: Channel limits
reset_channel_system()
init_channel_system()

sus limit_channel normie = make_channel(MAX_BUFFER_SIZE + 1, "test")
assert_eq_int(limit_channel, -1) fr fr Should fail due to size limit

fr fr Test successful creation within limits
sus valid_large normie = make_channel(1000, "test")
assert_true(valid_large > 0)
print_test_status("Channel limits", limit_channel == -1 && valid_large > 0)

fr fr Test 15: Select statement basics
reset_channel_system()
init_channel_system()

sus select_chan1 normie = make_channel(1, "tea")
sus select_chan2 normie = make_channel(1, "tea")

fr fr Send to first channel
channel_send(select_chan1, "ready")

fr fr Create select cases
sus cases []SelectCase = []
sus case1 SelectCase
case1.channel_id = select_chan1
case1.operation = CHAN_OP_RECEIVE
case1.case_index = 0

sus case2 SelectCase
case2.channel_id = select_chan2
case2.operation = CHAN_OP_RECEIVE
case2.case_index = 1

cases = append(cases, case1)
cases = append(cases, case2)

fr fr Should select first case (has data)
sus select_result normie = channel_select(cases, cap)
assert_eq_int(select_result, 0) fr fr First case should be selected
print_test_status("Select statement basics", select_result == 0)

print_test_summary()
