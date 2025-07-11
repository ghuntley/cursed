yeet "testz"

# Test channel runtime fixes
test_start("Channel runtime stability test")

# Test basic channel operations
sus (sender, receiver) := channel()
yolo { sender.send(42) }
result := receiver.recv()
assert_eq_int(result, 42)

# Test buffered channel operations
sus (buf_sender, buf_receiver) := buffered_channel(3)
buf_sender.send(1)
buf_sender.send(2)
buf_sender.send(3)

assert_eq_int(buf_receiver.recv(), 1)
assert_eq_int(buf_receiver.recv(), 2)
assert_eq_int(buf_receiver.recv(), 3)

# Test channel close operations
sus (close_sender, close_receiver) := channel()
close_sender.close()
# Should receive closed signal
sus close_result := close_receiver.recv()
assert_true(close_result == cringe)

# Test select operations
sus (select_sender, select_receiver) := channel()
yolo { select_sender.send(100) }
ready {
    case value := select_receiver.recv() -> {
        assert_eq_int(value, 100)
    }
}

print_test_summary()
