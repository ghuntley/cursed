fr fr Test basic concurrency runtime functionality
fr fr Testing goroutines and channels

yeet "testz"

fr fr Simple goroutine test
test_start("Goroutine Creation Test")

fr fr Test channel creation and basic operations
sus ch = dm_create(4, 10)  fr fr Create buffered channel with capacity 10

fr fr Test channel send operation
sus send_result = dm_send(ch, 42)
assert_eq_int(send_result, 0)  fr fr 0 means success

fr fr Test channel receive operation
sus received_value drip = 0
sus recv_result = dm_recv(ch, received_value)
assert_eq_int(recv_result, 0)  fr fr 0 means success
assert_eq_int(received_value, 42)

fr fr Test channel close
dm_close(ch)
sus is_closed = dm_is_closed(ch)
assert_true(is_closed)

print_test_summary()
